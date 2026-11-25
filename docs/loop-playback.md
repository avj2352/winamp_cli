# Loop playback (0.3.0)

Run `winamp play --loop` (or `cargo run -- play --loop`) and select an audio file
to repeat that track until stopped. `--loop true` also enables repetition;
omitting the option or passing `--loop false` plays the track once.

Loop playback displays `Loop: ON` below the track name and in the live status
line, including while paused. The timer shows cumulative elapsed playback time
across repetitions. Normal playback retains its elapsed/total duration display.
Space pauses/resumes; q, Esc or Ctrl-C stops and restores the terminal.

The handler uses Rodio's looped decoder to rewind the selected file at EOF,
streaming decoded samples through the existing waveform collector without
buffering the whole track in memory. File and decoder errors retain contextual
messages. As with normal playback, an interactive terminal and audio output
device are required.

An automated regression test decodes a small PCM WAV through multiple repetitions
and checks default, bare flag, explicit true and explicit false argument forms
without requiring audio hardware. Package version, application version, README
badge and changelog are updated to 0.3.0 for the new feature.
