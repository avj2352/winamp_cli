// help handler

use colored::Colorize;

const HELP_TEXT: &str = r#"
    ✴️ Winamp CLI v{version} ✴️

     __    __ _                                 ___   __   _____ 
    / / /\ \ (_)_ __   __ _ _ __ ___  _ __     / __\ / /   \_   \
    \ \/  \/ / | '_ \ / _` | '_ ` _ \| '_ \   / /   / /     / /\/
     \  /\  /| | | | | (_| | | | | | | |_) | / /___/ /___/\/ /_  
      \/  \/ |_|_| |_|\__,_|_| |_| |_| .__/  \____/\____/\____/  
                                     |_|                         

                                                                                                                                                                                                  
    📖 Usage:

    * winamp                                        - (default) winamp help - lists available commands
    * winamp help                                   - lists available commands
    * winamp --version                              - show the installed version

"#;

// display custom help message
pub fn handle_help() -> String {
    HELP_TEXT
        .replace("{version}", crate::VERSION)
        .cyan()
        .to_string()
}
