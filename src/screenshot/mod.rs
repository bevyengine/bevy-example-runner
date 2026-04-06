use serde::{Deserialize, Serialize};

use crate::{ImageUrl, SnapshotViewerUrl};

pub mod pixeleagle;

#[derive(Debug)]
pub struct ScreenshotData {
    /// category / name
    pub example: String,
    /// URL to the screenshot
    pub screenshot: ImageUrl,
    pub changed: ScreenshotState,
    /// Tag used to differentiate for mobile
    pub tag: Option<String>,
    pub diff_ratio: f32,
    pub snapshot_url: SnapshotViewerUrl,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum ScreenshotState {
    Similar,
    Changed,
}
