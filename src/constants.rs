/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

// Requests
pub const RATE_LIMIT: u32 = 30u32; // 30 requests per minnute

// Redis
pub const CACHE_EXPIRATION_TIME: i64 = 900; // 15 minutes
pub const CACHE_CONNECTION_TIMEOUT: u64 = 10;
pub const CACHE_MAX_COMMAND_ATTEMPTS: u32 = 5;
pub const CACHE_MAX_REDIRECTIONS: u32 = 5;
pub const CACHE_UNRESPONSIVE_MAX_TIMEOUT: u64 = 10;
pub const CACHE_UNRESPONSIVE_INTERVAL: u64 = 3;
pub const CACHE_POOL_SIZE: usize = 5;
pub const CACHE_RECONNECT_POLICY: (u32, u32, u32, u32) = (0, 100, 30_000, 2);
pub const PONG: &str = "PONG"; // Redis PONG Response

// PostgreSQL
pub const POSTGRES_POOL_SIZE: usize = 10; // 10 Concurrent Postgres Connections

// API
pub const API_QUOTA_LIMIT: i32 = 3000; // 5000 Requests per API Key

// Error Messages
pub const DEFAULT_ERROR_MESSAGE: &str = "[!!] Error in Protein Service";
pub const NOT_FOUND_ERROR_MESSAGE: &str = "[!!] Requested Path was Not Found";
pub const INTERNAL_SERVER_ERROR_MESSAGE: &str = "[!!] There was an Internal Server Issue!";
pub const UNAUTHORIZED_ERROR_MESSAGE: &str = "[!!] Unauthorized Request";
pub const UNPROCESSABLE_ENTITY_MESSAGE: &str = "[!!] This Request Has An Unprocessable Entity";
pub const UNPROCESSABLE_ENTITY_NOTE: &str =
    "The request was well-formed but was unable to be followed due to semantic/parsing errors.";

// Logging
pub const TERMINAL_FILTER: &str = "warn,info,protein=debug";
pub const LOG_FILE: &str = "[Protein].log";
pub const LOG_PATH: &str = "./logs/";

// Jobs
pub const API_KEY_JOB_INTERVAL: &str = "every day"; // must follow english-cron format

// Embeds
pub const EMBED_COLOR: u32 = 0x5865F2;
pub const EMBED_TITLE: &str = "Hypertrophy - Protein";
pub const EMBED_THUMBNAIL: &str = "https://files.catbox.moe/025e3m.png";
pub const EMBED_FOOTER: &str = "Hypertrophy - Made with ❤️";
