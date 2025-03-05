/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/
use std::sync::LazyLock;

// Struct Containing Admin Information
pub struct Admin<'a> {
    // Static Vector of Admin Routes
    pub routes: LazyLock<Vec<&'a str>>,
    pub master_key: String,
    pub app_env: String,
}
