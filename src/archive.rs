use std::{
    fs::File,
    io::{copy, BufReader, BufWriter},
    path::Path,
};

use flate2::bufread::GzDecoder;
use infer::Type;
use tar::Archive;

pub struct TarInfo {
    pub name: String,
    pub size: u64,
}

pub fn get_file_type(path: &Path) -> Type {
    infer::get_from_path(path).unwrap().unwrap()
}

pub fn decompress_file(source: &Path, destination: &Path) {
    let source_file_buffer = BufReader::new(File::open(source).unwrap());
    let mut destination_file_buffer = BufWriter::new(File::create(destination).unwrap());

    let file_type = get_file_type(source);
    match file_type.mime_type() {
        "application/gzip" | "application/x-gzip" => {
            let mut decoder = GzDecoder::new(source_file_buffer);
            copy(&mut decoder, &mut destination_file_buffer).unwrap();
        }
        "application/bzip2" | "application/x-bzip2" => {
            let mut decoder = bzip2::bufread::BzDecoder::new(source_file_buffer);
            copy(&mut decoder, &mut destination_file_buffer).unwrap();
        }
        _ => {}
    }
}

pub fn get_tar_infos(path: &Path) -> Vec<TarInfo> {
    let archive_file_buffer = BufReader::new(File::open(path).unwrap());
    let mut archive = Archive::new(archive_file_buffer);
    archive
        .entries()
        .unwrap()
        .map(|entry| {
            let file = entry.unwrap();

            TarInfo {
                name: file
                    .header()
                    .path()
                    .unwrap()
                    .into_owned()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                size: file.header().size().unwrap(),
            }
        })
        .collect()
}

pub fn unpacking_archive(source: &Path, destination: &Path) {
    let source_file_buffer = BufReader::new(File::open(source).unwrap());
    let mut archive = Archive::new(source_file_buffer);

    archive.unpack(destination).unwrap();
}

pub fn _unpack_file(file: &File, index: usize, destination: &Path) {
    let source_file_buffer = BufReader::new(file);
    let mut archive = Archive::new(source_file_buffer);
    archive
        .entries()
        .unwrap()
        .nth(index)
        .unwrap()
        .unwrap()
        .unpack_in(destination)
        .unwrap();
}
