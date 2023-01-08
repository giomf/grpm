mod archive;
mod config;
mod print;
mod repo;

use std::path::Path;

use clap::{command, Arg, Command};
use tempfile::NamedTempFile;

const INSTALL_PATH: &str = "~/.local/bin/";

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("install").about("Installs a release").arg(
                Arg::new("Repository")
                    .help("Repository owner/repository")
                    .required(true),
            ),
        )
        .arg(
            Arg::new("Config")
                .short('c')
                .long("config")
                .help("Configuration path")
                .default_value("~/.config/grpm/config.toml"),
        )
        .get_matches();

    let config_path = shellexpand::tilde(matches.get_one::<String>("Config").unwrap());
    let config_path = Path::new(config_path.as_ref());
    let config = config::parse_config(config_path);

    match matches.subcommand() {
        Some(("install", subcommand)) => {
            let repo = subcommand.get_one::<String>("Repository").unwrap();
            install(&repo, &config.token);
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
