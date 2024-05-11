use std::collections::HashMap;

use base64::prelude::*;
use futures::future::join_all;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use reqwest::Client;

use crate::{utils::reroot, PageData};

pub async fn load_data() -> (
    HashMap<String, toml::Table>,
    String,
    PageData,
    PageData,
    PageData,
) {
    // get and parse content
    let mut content_map = HashMap::new();

    let client = create_client();

    let (projects_index, projects) = fetch_directory(&client, "projects").await;
    let projects_ids = projects
        .iter()
        .map(|(id, _)| id.clone())
        .collect::<Vec<String>>();
    content_map.extend(projects);

    let (experience_index, experience) = fetch_directory(&client, "experience").await;
    let experience_ids = experience
        .iter()
        .map(|(id, _)| id.clone())
        .collect::<Vec<String>>();
    content_map.extend(experience);

    let (education_index, education) = fetch_directory(&client, "education").await;
    let education_ids = education
        .iter()
        .map(|(id, _)| id.clone())
        .collect::<Vec<String>>();
    content_map.extend(education);

    (
        content_map,
        String::from("en"),
        PageData {
            index: projects_index,
            ids: projects_ids,
        },
        PageData {
            index: experience_index,
            ids: experience_ids,
        },
        PageData {
            index: education_index,
            ids: education_ids,
        },
    )
}

fn create_client() -> Client {
    let github_pat = include_str!("../.github_pat").trim();

    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    let auth = format!("Bearer {github_pat}");
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );

    Client::builder().default_headers(headers).build().unwrap()
}

async fn fetch_directory(
    client: &Client,
    directory: &str,
) -> (toml::Table, Vec<(String, toml::Table)>) {
    let listing_json = client
        .get(format!("https://api.github.com/repos/SpacewaIker/portfolio-v2/contents/{directory}?ref=content"))
        .send()
        .await
        .expect("GET Request for directory failed")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse directory listing as JSON object");
    let listing = listing_json
        .as_array()
        .expect("Directory listing is not an array");

    let mut items = join_all(listing.iter().map(|repo_file| async {
        let file_name = repo_file
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .to_owned();

        let content_json = client
            .get(repo_file.get("url").unwrap().as_str().unwrap())
            .send()
            .await
            .expect("GET Request for file failed")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to parse file as JSON object");

        let content = content_json
            .get("content")
            .expect("Failed to get content from file")
            .as_str()
            .expect("Content is not a string")
            .replace('\n', "");
        let content = BASE64_STANDARD
            .decode(content)
            .expect("Failed to decode content from base64");
        let content = String::from_utf8(content).expect("Failed to parse content as UTF-8 string");
        let content = toml::from_str(&content).expect("Failed to parse content as TOML");
        let content = reroot(content);

        (file_name, content)
    }))
    .await;

    items.sort_by_key(|(_, table)| {
        let date = table
            .get("en")
            .unwrap()
            .as_table()
            .unwrap()
            .get("date")
            .unwrap()
            .as_table()
            .unwrap();
        date.get("end")
            .unwrap_or_else(|| date.get("start").unwrap())
            .as_datetime()
            .unwrap()
            .to_owned()
    });
    items.reverse();

    let index_json = client.get(
        format!("https://api.github.com/repos/SpacewaIker/portfolio-v2/contents/{directory}.toml?ref=content"),
    )
        .send()
        .await
        .expect("GET Request for index failed")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse index as JSON");
    let index = index_json
        .get("content")
        .expect("Failed to get index content")
        .as_str()
        .expect("Index content is not a string")
        .replace('\n', "");
    let index = BASE64_STANDARD
        .decode(index)
        .expect("Failed to decode index from base64");
    let index = String::from_utf8(index).expect("Failed to parse index as UTF-8 string");
    let index = toml::from_str(&index).expect("Failed to parse index as TOML");
    let index = reroot(index);

    (index, items)
}
