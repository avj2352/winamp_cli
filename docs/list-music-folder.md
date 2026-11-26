# Music folder in list output (1.1.1)

`winamp ls` displays the configured `MUSIC_FOLDER` path before listing songs:

```text
MUSIC_FOLDER: /path/to/music

  • First song.mp3
  • Second song.wav

```

The path is bold and yellow when terminal colors are enabled, and comes from the
configuration initialized at startup. Empty folders also
show the path, followed by the existing no-supported-audio-files message. Song
sorting, colors, and spacing remain unchanged. The `ls` command help describes
the path display. Output uses the existing writer to preserve broken-pipe handling.

The package and application versions, Cargo lockfile, and README badge are updated
to 1.1.1, with the change recorded in the changelog.
