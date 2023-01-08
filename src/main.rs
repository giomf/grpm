mod archive;
mod config;
mod print;
mod repo;
mod database;

use std::path::Path;

use clap::{command, Arg, Command};
use tempfile::NamedTempFile;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("install").about("Installs a release").arg(
                Arg::new("Repository")
                    .help("Repository owner/repository")
                    .required(true),
            ),
        )
        .get_matches();

    config::create_default_folders();
    let config = config::get_config();

    match matches.subcommand() {
        Some(("install", subcommand)) => {
            let repo = subcommand.get_one::<String>("Repository").unwrap();
            install(&repo, &config.token.unwrap(), config.install_path.as_ref());
        }
        _ => {}
    }
}

pub fn install(repo: &str, token: &str, install_path: &Path) {

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

    println!("Installing {} to {}", tar_infos[0].name, install_path.to_str().unwrap());
    archive::unpacking_archive(tmp_decompress_file.path(), install_path);

    println!("Done!");
}
