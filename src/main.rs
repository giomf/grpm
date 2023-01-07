mod archive;
mod print;
mod repo;

use std::path::Path;

use clap::{command, Arg, Command};
use tempfile::NamedTempFile;

const INSTALL_PATH: &str = "~/.local/bin/";

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("install")
                .about("Installs a release")
                .arg(
                    Arg::new("Token")
                        .short('t')
                        .long("token")
                        .help("Github access token")
                        .required(true),
                )
                .arg(
                    Arg::new("Repository")
                        .short('r')
                        .long("repo")
                        .help("Repository owner/repository")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", subcommand)) => {
            let token = subcommand.get_one::<String>("Token").unwrap();
            let repo = subcommand.get_one::<String>("Repository").unwrap();
            install(&repo, &token);
        }
        _ => {}
    }
}

pub fn install(repo: &str, token: &str) {
    let install_path = shellexpand::tilde(INSTALL_PATH);

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

    println!("Installing {} to {}", tar_infos[0].name, &install_path);
    archive::unpacking_archive(tmp_decompress_file.path(), Path::new(install_path.as_ref()));

    println!("Done!");
}
