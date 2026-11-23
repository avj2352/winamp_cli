# List output formatting (0.2.1)

`winamp ls` now adds a blank line before and after a nonempty list and prefixes each filename with two spaces and a cyan bullet. Filenames retain their cyan coloring and sorted order. Empty-folder messages are unchanged.

Example:

```text

  • First song.mp3
  • Second song.wav

```

The same spacing and bullets appear in redirected output; `colored` continues to handle color detection. The application version, Cargo package version, README badge and changelog are updated to 0.2.1.
