# Compilation fixes

Version 0.1.5 repairs the CLI command builder chain and calls the synchronous play and list handlers without `.await`. Music folder configuration is initialized at runtime with errors propagated to the caller; its existing default remains in the configuration helper.

The unfinished play and list handlers now return explicit not-implemented errors instead of panicking. Unused imports, unreachable code, and the unused `indicatif`, `reqwest`, `console`, `serde`, and `tokio` dependencies were removed. Cargo lockfile, version constant, package version, README badge, and changelog are updated together.
