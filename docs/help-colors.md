# Custom help colors

The custom help in `src/handlers/help_handler.rs` uses the existing `colored` crate to render the banner, usage, and command descriptions in cyan. The version placeholder is replaced before styling.

Color selection follows the crate’s terminal and environment detection; redirected output normally remains plain text.

Version 0.1.4 updates the package version, application version constant, README badge, and changelog.
