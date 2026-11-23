# Changelog

## 0.2.1 (2026-11-22)

- Add blank lines, indentation and cyan bullets to the audio file listing.

## 0.2.0 (2026-11-22)

- Add colorful audio file listing and interactive playback with a live audio-driven waveform and pause/stop controls.

## 0.1.6 (2026-11-22)

- Initialize a global `MUSIC_FOLDER` once at startup from configuration, propagating load errors.

## 0.1.5 (2026-11-22)

- Fixed command construction, runtime configuration initialization, and synchronous handler calls.
- Replaced unfinished handler panics with errors and removed unused imports and dependencies.

## 0.1.4 (2026-11-22)

- Colorized custom help output in cyan using the `colored` crate.

## 0.1.3 (2026-11-22)

- Refactored music folder lookup to load `.env` with dotenvy before reading `MUSIC_FOLDER`.

## 0.1.2 (2026-11-22)

- Load the optional `MUSIC_FOLDER` setting at CLI startup.

## 0.1.1 (2026-11-22)

- Fixed CLI startup and made the custom help available by default and via the help command.
