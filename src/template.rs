use std::collections::{HashMap, HashSet};

use serde::Serialize;
use tera::{Context, Tera};

use crate::{screenshot::ScreenshotState, Example, ImageUrl, Kind, Run, SnapshotViewerUrl};

#[derive(Debug, Serialize, Default)]
struct StringRun {
    date: String,
    commit: String,
    results: HashMap<String, HashMap<String, Kind>>,
    screenshots: HashMap<String, HashMap<String, (ImageUrl, ScreenshotState, SnapshotViewerUrl)>>,
    logs: HashMap<String, HashMap<String, String>>,
}

impl From<Run> for StringRun {
    fn from(value: Run) -> Self {
        StringRun {
            date: value.date.clone(),
            commit: value.commit.clone(),
            results: value
                .results
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(|(k, v)| (k.to_string(), v.clone())).collect(),
                    )
                })
                .collect(),
            screenshots: value
                .screenshots
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(|(k, v)| (k.to_string(), v.clone())).collect(),
                    )
                })
                .collect(),

            logs: value.logs.clone(),
        }
    }
}

pub fn build_site(
    runs: Vec<Run>,
    all_examples: Vec<Example>,
    all_mobile_platforms: HashSet<String>,
    all_wasm_platforms: HashSet<String>,
) {
    let runs: Vec<StringRun> = runs.into_iter().map(|r| r.into()).collect();

    let mut all_wasm_platforms: Vec<String> = all_wasm_platforms.into_iter().collect();
    all_wasm_platforms.sort_by(|a, b| {
        fn sort_key(tag: &str) -> (u8, u8, u8) {
            let tag_lower = tag.to_lowercase();
            let api = if tag_lower.contains("webgpu") {
                0
            } else if tag_lower.contains("webgl2") {
                1
            } else {
                2
            };
            let browser = if tag_lower.contains("chromium") {
                0
            } else if tag_lower.contains("firefox") {
                1
            } else if tag_lower.contains("webkit") {
                2
            } else {
                3
            };
            let os = if tag_lower.starts_with("linux") {
                0
            } else if tag_lower.starts_with("macos") {
                1
            } else if tag_lower.starts_with("windows") {
                2
            } else {
                3
            };
            (api, browser, os)
        }
        sort_key(a).cmp(&sort_key(b))
    });

    let mut context = Context::new();
    context.insert("runs".to_string(), &runs);
    context.insert("all_examples".to_string(), &all_examples);
    context.insert("all_mobile_platforms".to_string(), &all_mobile_platforms);
    context.insert("all_wasm_platforms".to_string(), &all_wasm_platforms);

    let mut tera = Tera::default();
    tera.add_raw_template(
        "icons.html",
        &std::fs::read_to_string("./templates/icons.html").unwrap(),
    )
    .unwrap();
    tera.add_raw_template(
        "macros.html",
        &std::fs::read_to_string("./templates/macros.html").unwrap(),
    )
    .unwrap();
    tera.add_raw_template(
        "index.html",
        &std::fs::read_to_string("./templates/index.html").unwrap(),
    )
    .unwrap();
    tera.add_raw_template(
        "about.html",
        &std::fs::read_to_string("./templates/about.html").unwrap(),
    )
    .unwrap();

    let rendered = tera.render("index.html", &context).unwrap();
    std::fs::write("./site/index.html", &rendered).unwrap();

    let rendered = tera.render("about.html", &context).unwrap();
    std::fs::write("./site/about.html", &rendered).unwrap();
}
