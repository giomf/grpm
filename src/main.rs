mod archive;
mod config;
mod database;
mod print;
mod repo;

use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{command, Arg, Command, ArgMatches};
use config::Config;
use database::Database;
use tempfile::NamedTempFile;

use crate::database::Package;

fn create_arg_matches()-> ArgMatches{
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
    .get_matches()

}

fn main() {
    let matches = create_arg_matches();
    let config = Config::new();
    let database = Database::new(Config::get_database_path()).unwrap();


    match matches.subcommand() {
        Some(("install", subcommand)) => {
            let repo = subcommand.get_one::<String>("Repository").unwrap();
            install(
                &database,
                &repo,
                &config.token.unwrap(),
                config.install_path.as_ref(),
            );
        }
        Some(("uninstall", subcommand)) => {
            let package_name = subcommand.get_one::<String>("Package").unwrap();
            uninstall(&database, &package_name);
        }
        Some(("list", _)) => {
            list(&database);
        }
        _ => {}
    }
}

fn list(database: &Database) {
    let packages = database.get_all().unwrap();
    if packages.is_empty() {
        println!("No packages installed yet");
        return;
    }
    print::print_packages(packages);
}

fn install(database: &Database, repo: &str, token: &str, install_path: &Path) {
    let tmp_download_file = NamedTempFile::new().unwrap();
    let tmp_decompress_file = NamedTempFile::new().unwrap();

    let repo_info = repo::get_repo_infos(repo, token).unwrap();
    print::print_repo_info(&repo_info);
    let choosen_asset_index = print::print_index_question("Choose an asset to download");
    let asset = &repo_info.assets[choosen_asset_index];

    println!("Downloading {}...", asset.name);
    repo::download_asset(asset, tmp_download_file.path()).unwrap();

    println!("Decompressing {}...", asset.name);
    archive::decompress_file(tmp_download_file.path(), tmp_decompress_file.path());

    println!("Reading {}...", asset.name);
    let tar_infos = archive::get_tar_infos(tmp_decompress_file.path());

    if tar_infos.len() > 1 {
        println!("Multiple files found in archive. Aborting!");
        return;
    }

    println!(
        "Installing {} to {}",
        tar_infos[0].name,
        install_path.to_str().unwrap()
    );
    archive::unpacking_archive(tmp_decompress_file.path(), install_path);

    let package = Package {
        name: repo_info.name,
        version: repo_info.version,
        path: install_path.to_string_lossy().to_string(),
        binary: tar_infos[0].name.to_string(),
    };

    database.put(&package.name, &package).unwrap();

    println!("Done!");
}

fn uninstall(database: &Database, package_name: &str) {
    let package = database.get(package_name).unwrap();
    if package.is_some() {
        let package = package.unwrap();
        let path = PathBuf::from(package.path).join(package.binary);
        fs::remove_file(path).unwrap();
        database.remove(package_name).unwrap();
    }
}
