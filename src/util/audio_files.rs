use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Scan only the configured directory; keep paths intact for playback.
pub fn music_files() -> Result<Vec<PathBuf>> {
    let folder = crate::MUSIC_FOLDER
        .get()
        .context("Music folder is not initialized")?;
    scan_audio_files(Path::new(folder))
}

fn scan_audio_files(folder: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(folder)
        .with_context(|| format!("Could not read music folder {}", folder.display()))?
    {
        let path = entry.context("Could not read music folder entry")?.path();
        let supported = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                matches!(
                    ext.to_ascii_lowercase().as_str(),
                    "mp3" | "wav" | "flac" | "ogg" | "oga"
                )
            });
        if supported
            && fs::metadata(&path)
                .with_context(|| format!("Could not inspect {}", path.display()))?
                .is_file()
        {
            files.push(path);
        }
    }
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(files)
}

/// Escape terminal control characters in filenames without altering the real path.
pub fn display_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .chars()
        .flat_map(|c| {
            if c.is_control() {
                c.escape_default().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_supported_files_sorted_without_recursing() {
        let dir = tempfile::tempdir().unwrap();
        for name in [
            "z.mp3",
            "a.WAV",
            "b.flac",
            "c.ogg",
            "d.oga",
            "notes.txt",
            "noextension",
        ] {
            fs::write(dir.path().join(name), []).unwrap();
        }
        fs::create_dir(dir.path().join("album.mp3")).unwrap();
        fs::write(dir.path().join("album.mp3/nested.mp3"), []).unwrap();
        let names: Vec<_> = scan_audio_files(dir.path())
            .unwrap()
            .iter()
            .map(|p| display_name(p))
            .collect();
        assert_eq!(names, ["a.WAV", "b.flac", "c.ogg", "d.oga", "z.mp3"]);
    }

    #[test]
    fn empty_and_missing_folders() {
        let dir = tempfile::tempdir().unwrap();
        assert!(scan_audio_files(dir.path()).unwrap().is_empty());
        assert!(scan_audio_files(&dir.path().join("missing")).is_err());
    }

    #[test]
    fn escapes_terminal_controls() {
        assert_eq!(
            display_name(Path::new("song\n\x1b.mp3")),
            "song\\n\\u{1b}.mp3"
        );
    }
}
