//! # videoupscaler — Official Rust SDK for VideoUpscaler
//!
//! `videoupscaler` provides strongly-typed Rust bindings and request builders for the
//! **[VideoUpscaler](https://videoupscaler.co)** REST API (`https://videoupscaler.co/api/v1`),
//! enabling automated cloud AI video enhancement, 4K 60fps super-resolution, temporal denoising,
//! optical-flow frame interpolation, and camera stabilization.
//!
//! ## Key Platform Links
//!
//! - **VideoUpscaler Home**: <https://videoupscaler.co>
//! - **4K AI Video Upscaler**: <https://videoupscaler.co/4k-video-upscaler>
//! - **Batch Video Upscaler**: <https://videoupscaler.co/batch-video-upscaler>
//! - **AI Video Denoising**: <https://videoupscaler.co/denoise-video>
//! - **60fps Frame Interpolation**: <https://videoupscaler.co/frame-interpolation>
//! - **AI Video Stabilization**: <https://videoupscaler.co/stabilize-video>
//! - **Developer Integrations**: <https://videoupscaler.co/integrations>
//! - **API & Workflow Docs**: <https://videoupscaler.co/docs/workflows>
//!
//! ## Example
//!
//! ```rust
//! use videoupscaler::{UpscaleRequest, Resolution, VideoUpscalerConfig};
//!
//! let config = VideoUpscalerConfig::new("vu_live_api_key");
//! let request = UpscaleRequest::new("https://cdn.example.com/clip-720p.mp4")
//!     .with_resolution(Resolution::Uhd4k)
//!     .with_fps(60)
//!     .with_denoise(true);
//!
//! assert_eq!(config.upscale_endpoint(), "https://videoupscaler.co/api/v1/upscale");
//! assert_eq!(request.resolution.as_str(), "4k");
//! ```

use serde::{Deserialize, Serialize};

/// Default base URL for the [VideoUpscaler API](https://videoupscaler.co/docs/workflows).
pub const DEFAULT_BASE_URL: &str = "https://videoupscaler.co/api/v1";

/// Target output resolution supported by [VideoUpscaler 4K AI Upscaler](https://videoupscaler.co/4k-video-upscaler).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Resolution {
    /// Full HD 1080p (1920×1080)
    #[serde(rename = "1080p")]
    Fhd1080p,
    /// Quad HD 2K (2560×1440)
    #[serde(rename = "2k")]
    Qhd2k,
    /// Ultra HD 4K (3840×2160)
    #[serde(rename = "4k")]
    Uhd4k,
}

impl Resolution {
    /// Returns the canonical API string value (`"1080p"`, `"2k"`, or `"4k"`).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fhd1080p => "1080p",
            Self::Qhd2k => "2k",
            Self::Uhd4k => "4k",
        }
    }
}

/// Configuration for authenticating with the [VideoUpscaler REST API](https://videoupscaler.co/integrations).
#[derive(Debug, Clone)]
pub struct VideoUpscalerConfig {
    pub api_key: String,
    pub base_url: String,
}

impl VideoUpscalerConfig {
    /// Create a new configuration with the given API key and default base URL (`https://videoupscaler.co/api/v1`).
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Override the base URL for custom enterprise or proxy endpoints.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_string();
        self
    }

    /// Returns the full URL for `POST /upscale`.
    pub fn upscale_endpoint(&self) -> String {
        format!("{}/upscale", self.base_url)
    }

    /// Returns the full URL for `GET /tasks/{id}`.
    pub fn task_endpoint(&self, task_id: &str) -> String {
        format!("{}/tasks/{}", self.base_url, task_id)
    }

    /// Returns the full URL for `GET /account`.
    pub fn account_endpoint(&self) -> String {
        format!("{}/account", self.base_url)
    }
}

/// Request payload for submitting a video enhancement job to `POST /api/v1/upscale`.
///
/// See <https://videoupscaler.co/docs/workflows> for full parameter documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpscaleRequest {
    pub video_url: String,
    pub scale: u8,
    pub resolution: Resolution,
    pub fps: u16,
    pub denoise: bool,
    pub stabilize: bool,
    pub face_enhance: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl UpscaleRequest {
    /// Construct a default 4K 60fps upscaling request for the given input `video_url`.
    pub fn new(video_url: impl Into<String>) -> Self {
        Self {
            video_url: video_url.into(),
            scale: 2,
            resolution: Resolution::Uhd4k,
            fps: 60,
            denoise: true,
            stabilize: false,
            face_enhance: false,
            webhook_url: None,
        }
    }

    /// Set target output resolution (`1080p`, `2k`, or `4k`).
    pub fn with_resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Set target frame rate (`30`, `60`, or `120` fps).
    pub fn with_fps(mut self, fps: u16) -> Self {
        self.fps = fps;
        self
    }

    /// Enable or disable temporal AI denoising (<https://videoupscaler.co/denoise-video>).
    pub fn with_denoise(mut self, denoise: bool) -> Self {
        self.denoise = denoise;
        self
    }

    /// Enable or disable optical stabilization (<https://videoupscaler.co/stabilize-video>).
    pub fn with_stabilize(mut self, stabilize: bool) -> Self {
        self.stabilize = stabilize;
        self
    }

    /// Configure an asynchronous webhook callback URL.
    pub fn with_webhook_url(mut self, webhook_url: impl Into<String>) -> Self {
        self.webhook_url = Some(webhook_url.into());
        self
    }
}

/// Lifecycle status of a VideoUpscaler task (`GET /api/v1/tasks/{id}`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Canceled,
}

/// Response returned when creating or querying a VideoUpscaler task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpscaleTask {
    pub id: String,
    pub status: TaskStatus,
    pub resolution: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
