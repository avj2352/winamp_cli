# Music folder configuration

Set `MUSIC_FOLDER` in the environment or in a `.env` file. For example, add the following line to `.env`.

```dotenv
MUSIC_FOLDER="/path/to/My Music"
```

At startup, the CLI uses dotenvy to find the nearest `.env` in the working directory or its parents. An existing environment variable takes precedence, including an empty value. A missing `.env` is allowed. If `MUSIC_FOLDER` is missing or contains invalid Unicode, the CLI uses the existing default of `/home/pramod/Music`. A malformed or unreadable `.env` causes startup to fail with an error.

The lookup returns the folder setting as a string. The lookup accepts the configured path without checking the filesystem.
