# Global music folder

Version 0.1.6 replaces the invalid `const MUSIC_FOLDER` with a public `static MUSIC_FOLDER: OnceLock<String>`. Configuration reads files and environment variables at runtime, so it cannot be evaluated as a Rust constant.

`main` calls `get_music_folder()?` and initializes the global before dispatching commands. Configuration errors are returned instead of panicking through `unwrap()`. The existing fallback folder remains unchanged. Once initialized, the value is shared immutably for the process lifetime.

Handlers can access the folder through `crate::MUSIC_FOLDER.get()`, which returns `Option<&String>`. For a handler returning `anyhow::Result`, use:

```rust
let folder = crate::MUSIC_FOLDER
    .get()
    .ok_or_else(|| anyhow::anyhow!("Music folder is not initialized"))?;
```

Before startup initialization, `get()` returns `None`; afterward it returns the configured folder. No additional dependency is required.
