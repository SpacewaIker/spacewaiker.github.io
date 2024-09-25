use base64::prelude::*;
use cached::proc_macro::cached;
use chrono::Utc;
use futures::{future::join_all, TryFutureExt};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use reqwest::{Client, RequestBuilder};

use crate::utils::reroot;

/// Fetches and parses content from a given path
///
/// The path should be a relative path to the content file in the repository.
/// The file will be parsed as a TOML file.
#[cached]
pub async fn get_content(path: String) -> toml::Table {
    #[cfg(debug_assertions)]
    leptos::logging::log!("Loading content for path: {path}");

    let client = create_client();

    let req = client.get(format!(
        "https://api.github.com/repos/SpacewaIker/spacewaiker.github.io/contents/{path}.toml?ref=content"
    ));

    fetch_parse_content(req).await
}

/// Creates a `reqwest::Client` with the necessary headers for GitHub API requests
#[cached]
fn create_client() -> Client {
    #[cfg(debug_assertions)]
    leptos::logging::log!("Creating client");
    let github_pat = env!("GH_API_PAT").trim();

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

/// Fetches the list of items in a directory
///
/// # Panics
/// If the JSON response from the GitHub API is not an array
#[cached]
pub async fn get_directory_items(directory: String) -> Vec<String> {
    #[cfg(debug_assertions)]
    leptos::logging::log!("Loading items for directory: {directory}");

    let client = create_client();

    client.get(format!("https://api.github.com/repos/SpacewaIker/spacewaiker.github.io/contents/{directory}?ref=content"))
        .send()
        .and_then(reqwest::Response::json::<serde_json::Value>)
        .and_then(|json| async move {
            let listing = json.as_array().expect("Directory listing is not an array");

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

                let content = get_content(format!("{directory}/{file_name}").to_owned()).await;

                (file_name, content)
            })).await;

            items.sort_by_key(|(_, table)| {
                let date = table.get("date").unwrap().as_table().unwrap();
                match date.get("end") {
                    Some(toml::Value::String(_)) => Utc::now().to_rfc3339().parse::<toml::value::Datetime>().unwrap(),
                    Some(toml::Value::Datetime(d)) => d.to_owned(),
                    _ => date.get("start").unwrap().as_datetime().unwrap().to_owned(),
                }
            });
            items.reverse();

            let ids = items.iter().map(|(id, _)| id.to_owned()).collect();

            Ok(ids)
        })
        .unwrap_or_else(|_| Vec::new())
        .await
}

/// Fetches and parses content from a given request
///
/// The request should be a `reqwest::RequestBuilder` with the necessary headers for GitHub API requests.
/// It is sent, then parsed as a JSON object, and the content is extracted from it. The content
/// is decoded from base64, parsed as a TOML file, and rerooted.
async fn fetch_parse_content(request: RequestBuilder) -> toml::Table {
    request
        .send()
        .and_then(reqwest::Response::json::<serde_json::Value>)
        .and_then(|json| async move {
            let content = json
                .get("content")
                .expect("Failed to get content")
                .as_str()
                .expect("Content is not a string")
                .replace('\n', "");

            let content = BASE64_STANDARD
                .decode(content)
                .expect("Failed to decode content from base64");
            let content =
                String::from_utf8(content).expect("Failed to parse content as UTF-8 string");
            let content = toml::from_str(&content).expect("Failed to parse content as TOML");
            let content = reroot(content);
            Ok(content)
        })
        .unwrap_or_else(|_| toml::value::Table::new())
        .await
}
