use prettytable::{format, Cell, Row, Table};

use crate::{
    archive::TarInfo,
    repo::{AssetInfo, Repoinfo},
};

pub fn print_repo_info(repo: &Repoinfo) {
    let repo_table = create_repo_table(&repo);
    let asset_table = create_asset_table(&repo.assets);

    repo_table.print_tty(true).unwrap();
    println!("");
    asset_table.print_tty(true).unwrap();
    println!("");
}

pub fn _print_binaries(tar_infos: &Vec<TarInfo>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    let headers = Row::new(vec![
        Cell::new("#").style_spec("b"),
        Cell::new("Name").style_spec("b"),
        Cell::new("Size (MB)").style_spec("br"),
    ]);
    table.set_titles(headers);

    for (i, tar_info) in tar_infos.iter().enumerate() {
        let index = &i.to_string();
        let name = &tar_info.name;
        let size = &format!("{:.2}", tar_info.size as f32 / 1000000 as f32);
        let tar_row = Row::new(vec![
            Cell::new(index),
            Cell::new(name),
            Cell::new(size).style_spec("r"),
        ]);
        table.add_row(tar_row);
    }

    table.print_tty(true).unwrap();
    println!("");
}

pub fn print_index_question(question: &str) -> usize {
    print!("{}: ", question);
    text_io::read!()
}

fn create_repo_table(repo: &Repoinfo) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    let name = &repo.name;
    let version = &repo.version;
    let description = &repo.description;

    let repo_row = Row::new(vec![
        Cell::new("Package:"),
        Cell::new(name).style_spec("Fcb"),
    ]);
    let version_row = Row::new(vec![
        Cell::new("Version:"),
        Cell::new(version).style_spec("Fgb"),
    ]);

    table.add_row(repo_row);
    table.add_row(version_row);

    if let Some(description) = description {
        let description_row = Row::new(vec![
            Cell::new("Description:"),
            Cell::new(&description).style_spec("b"),
        ]);
        table.add_row(description_row);
    }

    table
}

fn create_asset_table(assets: &Vec<AssetInfo>) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    let headers = Row::new(vec![
        Cell::new("#").style_spec("b"),
        Cell::new("Name").style_spec("b"),
        Cell::new("Size (MB)").style_spec("br"),
        Cell::new("Downloads").style_spec("br"),
    ]);

    table.set_titles(headers);

    for (i, asset) in assets.iter().enumerate() {
        let index = &i.to_string();
        let name = &asset.name;
        let size = &format!("{:.2}", asset.size as f32 / 1000000 as f32);
        let downloads = &asset.download_count.to_string();
        let asset_row = Row::new(vec![
            Cell::new(index),
            Cell::new(name),
            Cell::new(size).style_spec("r"),
            Cell::new(downloads).style_spec("r"),
        ]);
        table.add_row(asset_row);
    }

    table
}
