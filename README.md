# videoupscaler-rs — Official Rust SDK for VideoUpscaler

[![Version](https://img.shields.io/badge/crate-v0.1.0-orange.svg)](https://github.com/arkyu2077/videoupscaler-rs/releases)
[![Rust Edition](https://img.shields.io/badge/edition-2021-blue.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Powered by VideoUpscaler](https://img.shields.io/badge/Powered%20by-VideoUpscaler.co-6366f1)](https://videoupscaler.co)

**`videoupscaler`** is the official Rust SDK for **[VideoUpscaler](https://videoupscaler.co)** — the cloud AI video super-resolution platform for upscaling video footage to **1080p, 2K, and 4K (60fps)** via REST API or browser workflow.

Designed for high-throughput Rust media servers, serverless workers, and video transcoding pipelines, `videoupscaler` provides strongly-typed request/response models and endpoint builders with `serde` serialization.

---

## Supported AI Video Pipelines

- **[4K AI Video Upscaler](https://videoupscaler.co/4k-video-upscaler)** — Multi-frame AI super-resolution from SD/HD up to 4K UHD (3840×2160).
- **[Batch Video Upscaler](https://videoupscaler.co/batch-video-upscaler)** — Automated high-concurrency batch processing for media catalogs and object storage buckets.
- **[AI Video Denoising](https://videoupscaler.co/denoise-video)** — Temporal noise reduction for low-light sensor grain and compression artifacts.
- **[60fps Frame Interpolation](https://videoupscaler.co/frame-interpolation)** — Motion-compensated frame rate doubling from 24/30fps to smooth 60fps.
- **[AI Video Stabilization](https://videoupscaler.co/stabilize-video)** — Trajectory smoothing for handheld, drone, and action-camera footage.

---

## Installation

Add `videoupscaler` to your `Cargo.toml`:

```toml
[dependencies]
videoupscaler = { git = "https://github.com/arkyu2077/videoupscaler-rs", tag = "v0.1.0" }
```

---

## Quick Start

```rust
use videoupscaler::{Resolution, UpscaleRequest, VideoUpscalerConfig};

fn main() {
    let config = VideoUpscalerConfig::new(
        std::env::var("VIDEOUPSCALER_API_KEY").expect("Missing VIDEOUPSCALER_API_KEY"),
    );

    let payload = UpscaleRequest::new("https://cdn.example.com/source-1080p.mp4")
        .with_resolution(Resolution::Uhd4k)
        .with_fps(60)
        .with_denoise(true)
        .with_webhook_url("https://worker.example.com/webhook/videoupscaler");

    let json_body = serde_json::to_string_pretty(&payload).unwrap();
    println!("POST {}\n{}", config.upscale_endpoint(), json_body);
}
```

---

## Documentation & Workflow Guides

- **[VideoUpscaler Platform](https://videoupscaler.co)**
- **[API & Webhook Workflow Docs](https://videoupscaler.co/docs/workflows)**
- **[Developer Integrations Hub](https://videoupscaler.co/integrations)**

---

## Official Ecosystem & Sibling SDKs

- **JavaScript / TypeScript SDK (npm)**: [`videoupscaler`](https://www.npmjs.com/package/videoupscaler) ([GitHub](https://github.com/arkyu2077/videoupscaler-js))
- **Python SDK & CLI (PyPI)**: [`videoupscaler-cli`](https://pypi.org/project/videoupscaler-cli/) ([GitHub](https://github.com/arkyu2077/videoupscaler-cli))
- **Go SDK**: [`github.com/arkyu2077/videoupscaler-go`](https://github.com/arkyu2077/videoupscaler-go)
- **Rust SDK**: [`github.com/arkyu2077/videoupscaler-rs`](https://github.com/arkyu2077/videoupscaler-rs)
- **PHP SDK (Composer)**: [`github.com/arkyu2077/videoupscaler-php`](https://github.com/arkyu2077/videoupscaler-php)
- **GitHub Action**: [`arkyu2077/videoupscaler-action`](https://github.com/arkyu2077/videoupscaler-action)
- **n8n Community Node**: [`n8n-nodes-videoupscaler`](https://www.npmjs.com/package/n8n-nodes-videoupscaler) ([GitHub](https://github.com/arkyu2077/n8n-nodes-videoupscaler))
- **Cloudflare R2 / S3 Webhook Worker**: [`arkyu2077/videoupscaler-webhook-worker`](https://github.com/arkyu2077/videoupscaler-webhook-worker)
- **Awesome AI Video Upscaling**: [`arkyu2077/awesome-ai-video-upscaling`](https://github.com/arkyu2077/awesome-ai-video-upscaling)

---

## License

Released under the [MIT License](./LICENSE). © VideoUpscaler ([https://videoupscaler.co](https://videoupscaler.co)).
