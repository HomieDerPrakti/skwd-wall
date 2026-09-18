use std::path::Path;
use std::process::{Command, Stdio};

use crate::contracts::preview::{PREVIEW_HEIGHT, PREVIEW_WIDTH};

use super::decode::{DecodePool, Job};
use super::live_stream::scanner_bin;

pub fn tall_thumb_path(thumb: &str) -> String {
    let stem = thumb.strip_suffix(".webp").unwrap_or(thumb);
    format!("{stem}.tall.webp")
}

pub fn ensure_tall(store: usize, source: String, thumb: String, video: bool, pool: DecodePool) {
    std::thread::spawn(move || {
        let tall = tall_thumb_path(&thumb);
        if !Path::new(&tall).exists() {
            let mut command = Command::new(scanner_bin());
            command.arg("--tall").arg(&source).arg(&thumb);
            if video {
                command.arg("--video");
            }
            let status =
                command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();
            log::info!("tall thumb for {source}: {status:?}");
        }
        pool.enqueue(
            Job {
                store_idx: store,
                path: tall,
                fallback: None,
                tier: 3,
                layer: 0,
                x: 0,
                y: 0,
                w: PREVIEW_WIDTH,
                h: PREVIEW_HEIGHT,
                compressed: false,
            },
            true,
        );
    });
}
