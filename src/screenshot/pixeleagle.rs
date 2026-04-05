use pixeleagle_cli::types::{ComparisonResult, Difference};
use serde::Deserialize;

use crate::{ImageUrl, SnapshotViewerUrl};

use super::{ScreenshotData, ScreenshotState};

#[derive(Deserialize)]
struct ComparisonTarget {
    project_id: String,
    from: u32,
    to: u32,
}

pub fn read_results(results: String) -> Vec<ScreenshotData> {
    let Ok(target) = serde_json::from_str::<ComparisonTarget>(&results) else {
        return vec![];
    };

    let project = pixeleagle_cli::blocking::Project::new(
        &format!("https://pixel-eagle.com/{}/", target.project_id),
        std::env::var("PIXEL_EAGLE_TOKEN").unwrap_or_default(),
    );

    let comparison = project.get_comparison(target.from, target.to);

    comparison_to_screenshot_data(comparison)
}

fn comparison_to_screenshot_data(comparison: ComparisonResult) -> Vec<ScreenshotData> {
    let project_id = comparison.project_id.to_string();
    // NOTE: screenshot/diff/viewer URL construction has no CLI library equivalent
    let mut result = vec![];

    for screenshot in comparison.new {
        result.push(ScreenshotData {
            example: screenshot.name.clone(),
            screenshot: ImageUrl(format!(
                "https://pixel-eagle.com/files/{}/screenshot/{}",
                project_id, screenshot.hash
            )),
            changed: ScreenshotState::Changed,
            tag: None,
            diff_ratio: 0.0,
            snapshot_url: SnapshotViewerUrl(format!(
                "https://pixel-eagle.com/project/{}/run/{}/compare/{}?screenshot={}",
                project_id, comparison.from, comparison.to, screenshot.name
            )),
        });
    }

    for screenshot in comparison.unchanged {
        result.push(ScreenshotData {
            example: screenshot.name.clone(),
            screenshot: ImageUrl(format!(
                "https://pixel-eagle.com/{}/screenshot/{}",
                project_id, screenshot.hash
            )),
            changed: ScreenshotState::Similar,
            tag: None,
            diff_ratio: 0.0,
            snapshot_url: SnapshotViewerUrl(format!(
                "https://pixel-eagle.com/project/{}/run/{}/compare/{}?screenshot={}",
                project_id, comparison.from, comparison.to, screenshot.name
            )),
        });
    }

    for screenshot in comparison.diff {
        result.push(ScreenshotData {
            example: screenshot.name.clone(),
            screenshot: ImageUrl(format!(
                "https://pixel-eagle.com/{}/screenshot/{}",
                project_id, screenshot.hash
            )),
            changed: ScreenshotState::Changed,
            tag: None,
            diff_ratio: match screenshot.diff {
                Difference::Done(ratio) => ratio,
                _ => 1.0,
            },
            snapshot_url: SnapshotViewerUrl(format!(
                "https://pixel-eagle.com/project/{}/run/{}/compare/{}?screenshot={}",
                project_id, comparison.from, comparison.to, screenshot.name
            )),
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn read_file_native() {
        let file = fs::read_to_string("src/screenshot/test-pixeleagle.json").unwrap();
        let read = serde_json::from_str::<ComparisonResult>(&file).unwrap();
        dbg!(comparison_to_screenshot_data(read));
    }
}
