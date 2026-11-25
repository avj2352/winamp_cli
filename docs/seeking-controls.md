# Playback seeking (1.1.0)

While a selected track is playing, press Left to seek backward 10 seconds or Right to seek forward 10 seconds. Space still pauses/resumes; q, Esc and Ctrl-C stop playback. Seeking also works while paused without resuming playback.

Targets are clamped to the start and, when duration is known, the end of the track. With `--loop`, the current position is calculated within the repeated track when its duration is known. If the decoder cannot seek, the playback status displays the error and the controls remain available.

`src/features/gen_play.rs` documents controls in `play --help`. `src/handlers/play_handler.rs` handles arrow keys and forwards seeks through the waveform source to the audio decoder, clearing stale waveform samples after successful seeks.

Unit tests cover backward/forward targets, boundaries, loop positions, unknown duration, and waveform seek forwarding/reset without requiring an audio device. The package, runtime version, lockfile and README badge are updated to 1.1.0 for this feature.
