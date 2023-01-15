use std::{
    fs::File,
    io::copy,
    path::Path,
};

use attohttpc::{Error, RequestBuilder};
use octocrab::models::{repos::Release, Repository};

pub struct Repoinfo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub version: String,
    pub assets: Vec<AssetInfo>,
}

pub struct AssetInfo {
    pub name: String,
    pub url: String,
    pub size: i64,
    pub download_count: i64,
}

fn build_api_query(url: &str, token: &str) -> RequestBuilder {
    build_query(url)
        .bearer_auth(token)
        .header("ACCEPT", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
}

fn build_query(url: &str) -> RequestBuilder {
    attohttpc::get(url).header("User-Agent", "grpm")
}

fn get_repo(repo: &str, token: &str) -> Result<Repository, Error> {
    let url = format!("https://api.github.com/repos/{}", repo);
    let response = build_api_query(&url, token).send()?.error_for_status()?;
    Ok(response.json()?)
}

fn get_latest_release(repo: &str, token: &str) -> Result<Release, Error> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let response = build_api_query(&url, token).send()?.error_for_status()?;
    Ok(response.json()?)
}

pub fn get_repo_infos(repo: &str, token: &str) -> Result<Repoinfo, Error> {
    let repository = get_repo(repo, token)?;
    let release = get_latest_release(repo, token)?;

    let repo_info = Repoinfo {
        name: repository.name,
        full_name: repo.to_string(),
        description: repository.description,
        version: release.tag_name,
        assets: release
            .assets
            .iter()
            .map(|asset| AssetInfo {
                name: String::from(&asset.name),
                size: asset.size,
                url: asset.browser_download_url.to_string(),
                download_count: asset.download_count,
            })
            .collect(),
    };

    Ok(repo_info)
}

pub fn download_asset(asset: &AssetInfo, destination: &Path) -> Result<(), Error> {
    let mut response = build_query(&asset.url).send()?.error_for_status()?;
    let mut destination_file_buffer = std::io::BufWriter::new(File::create(destination).unwrap());
    copy(&mut response, &mut destination_file_buffer).unwrap();

    Ok(())
}
