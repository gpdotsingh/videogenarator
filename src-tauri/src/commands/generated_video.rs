use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::json;
use tauri::State;

use crate::state::AppState;

const GENERATED_VIDEO_DIRECTORY: &str = "/Users/gauravsingh/study/video/newproject/generatedvideo";

fn safe_relative_path(value: &str) -> Result<PathBuf, String> {
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err("Invalid ComfyUI output path".to_string());
    }
    Ok(path.to_path_buf())
}

fn is_video(filename: &str) -> bool {
    Path::new(filename)
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "mp4" | "webm" | "mov" | "mkv"
            )
        })
        .unwrap_or(false)
}

#[tauri::command]
pub fn archive_generated_video(
    filename: String,
    subfolder: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    if !is_video(&filename) {
        return Err("Only completed video outputs can be archived".to_string());
    }

    let filename_path = safe_relative_path(&filename)?;
    let subfolder_path = if subfolder.trim().is_empty() {
        PathBuf::new()
    } else {
        safe_relative_path(&subfolder)?
    };
    let comfy_root = state
        .comfy_path
        .lock()
        .map_err(|_| "ComfyUI path lock unavailable".to_string())?
        .clone()
        .ok_or_else(|| "ComfyUI path is not configured".to_string())?;

    let source = PathBuf::from(comfy_root)
        .join("output")
        .join(subfolder_path)
        .join(&filename_path);
    if !source.is_file() {
        return Err(format!(
            "Generated video was not found: {}",
            source.display()
        ));
    }

    let destination_root = PathBuf::from(GENERATED_VIDEO_DIRECTORY);
    fs::create_dir_all(&destination_root)
        .map_err(|error| format!("Could not create generatedvideo: {error}"))?;
    let destination = destination_root.join(
        filename_path
            .file_name()
            .ok_or_else(|| "Generated video filename is invalid".to_string())?,
    );
    fs::copy(&source, &destination)
        .map_err(|error| format!("Could not archive generated video: {error}"))?;

    Ok(json!({
        "path": destination.to_string_lossy(),
        "filename": destination.file_name().and_then(|value| value.to_str()).unwrap_or_default(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_parent_traversal() {
        assert!(safe_relative_path("../secret").is_err());
        assert!(safe_relative_path("nested/video.mp4").is_ok());
    }

    #[test]
    fn accepts_only_video_extensions() {
        assert!(is_video("result.mp4"));
        assert!(is_video("result.WEBM"));
        assert!(!is_video("result.png"));
    }
}
