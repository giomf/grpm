mod archive;
mod config;
mod database;
mod print;
mod repo;

use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use clap::{command, Arg, ArgMatches, Command};
use config::Config;
use database::Database;
use repo::RepoInfo;
use tempfile::NamedTempFile;

use crate::database::Package;

fn create_arg_matches() -> ArgMatches {
    command!()
        .subcommand(
            Command::new("install").about("Installs a package").arg(
                Arg::new("Repository")
                    .help("Repository owner/repository")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("uninstall").about("Uninstalls a package").arg(
                Arg::new("Package")
                    .help("The package to uninstall")
                    .required(true),
            ),
        )
        .subcommand(Command::new("list").about("Lists all installed packages"))
        .subcommand(Command::new("update").about("Updates all installed packages"))
        .get_matches()
}

fn main() {
    let matches = create_arg_matches();
    let config = Config::new();
    let database = Database::new(Config::get_database_path()).unwrap();

    match matches.subcommand() {
        Some(("install", subcommand)) => {
            let repo = subcommand.get_one::<String>("Repository").unwrap();
            if let Err(error) = install(
                &database,
                &repo,
                &config.token.unwrap(),
                config.install_path.as_ref(),
            ) {
                handle_error(error);
            }
        }

        Some(("uninstall", subcommand)) => {
            let package_name = subcommand.get_one::<String>("Package").unwrap();
            if let Err(error) = uninstall(&database, &package_name) {
                handle_error(error);
            }
        }
        Some(("list", _)) => {
            if let Err(error) = list(&database) {
                handle_error(error);
            }
        }
        Some(("update", _)) => {
            if let Err(error) = update(&database, &config.token.unwrap()) {
                handle_error(error);
            }
        }
        _ => {}
    }
}

fn handle_error(error: Box<dyn Error>) {
    if let Some(error) = error.downcast_ref::<attohttpc::Error>() {
        eprintln!("HTTP Error: {}", error);
    } else if let Some(err) = error.downcast_ref::<io::Error>() {
        eprintln!("I/O Error: {}", err);
    } else if let Some(err) = error.downcast_ref::<jammdb::Error>() {
        eprintln!("Database Error: {}", err);
    } else {
        eprintln!("Unknown Error: {}", error);
    }
}

fn update(database: &Database, token: &str) -> Result<(), Box<dyn Error>> {

    let installed_packages = database.get_all()?;
    let mut updateable_packages: Vec<(Package, RepoInfo)> = Vec::new();
    if installed_packages.is_empty() {
        println!("No packages installed yet");
        return Ok(());
    }

    for package in installed_packages {
        let repo_info = repo::get_repo_infos(&package.full_name, token)?;
        if package.version == repo_info.version {
            updateable_packages.push((package, repo_info));
        }
    }

    if updateable_packages.is_empty() {
        println!("No updates available");
        return Ok(());
    }

    print::print_updates(&updateable_packages);
    Ok(())
}

fn list(database: &Database) -> Result<(), Box<dyn Error>> {
    let packages = database.get_all()?;
    if packages.is_empty() {
        println!("No packages installed yet");
    } else {
        print::print_packages(&packages);
    }
    Ok(())
}

fn install(
    database: &Database,
    repo: &str,
    token: &str,
    install_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let tmp_download_file = NamedTempFile::new()?;
    let tmp_decompress_file = NamedTempFile::new()?;

    let repo_info = repo::get_repo_infos(repo, token)?;
    print::print_repo_info(&repo_info);
    let choosen_asset_index = print::print_index_question("Choose an asset to download");
    let asset = &repo_info.assets[choosen_asset_index];

    println!("Downloading {}...", asset.name);
    repo::download_asset(asset, tmp_download_file.path())?;

    println!("Decompressing {}...", asset.name);
    archive::decompress_file(tmp_download_file.path(), tmp_decompress_file.path());

    println!("Reading {}...", asset.name);
    let tar_infos = archive::get_tar_infos(tmp_decompress_file.path());

    if tar_infos.len() > 1 {
        println!("Multiple files found in archive. Aborting!");
        return Ok(());
    }

    println!(
        "Installing {} to {}",
        tar_infos[0].name,
        install_path.to_str().unwrap()
    );
    archive::unpacking_archive(tmp_decompress_file.path(), install_path);

    let package = Package {
        name: repo_info.name,
        full_name: repo_info.full_name,
        version: repo_info.version,
        path: install_path.to_string_lossy().to_string(),
        binary: tar_infos[0].name.to_string(),
    };

    database.put(&package.name, &package).unwrap();
    println!("Done!");
    Ok(())
}

fn uninstall(database: &Database, package_name: &str) -> Result<(), Box<dyn Error>> {
    let package = database.get(package_name).unwrap();
    if package.is_some() {
        let package = package.unwrap();
        let path = PathBuf::from(package.path).join(package.binary);
        fs::remove_file(path)?;
        database.remove(package_name)?;
    }
    Ok(())
}
