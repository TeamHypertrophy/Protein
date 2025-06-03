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
pub const CACHE_TCP_CONFIG_DELAY: bool = true;
pub const PONG: &str = "PONG"; // Redis PONG Response

// PostgreSQL
pub const POSTGRES_POOL_SIZE: usize = 10; // 10 Concurrent Postgres Connections

// API
pub const ENV_PATH: &str = ".env"; // Environment File Path
pub const API_QUOTA_LIMIT: i32 = 100000; // 100,000 Requests per API Key
pub const API_VERSION: &str = "1.0"; // API Version
pub const REDIS_VERSION: &str = "6.0"; // Redis Version
pub const POSTGRES_VERSION: &str = "17.4"; // Postgres Version

// Error Messages
pub const DEFAULT_ERROR_MESSAGE: &str = "[!!] Error in Protein Service";
pub const NOT_FOUND_ERROR_MESSAGE: &str = "[!!] Requested Path was Not Found";
pub const INTERNAL_SERVER_ERROR_MESSAGE: &str = "[!!] There was an Internal Server Issue!";
pub const UNAUTHORIZED_ERROR_MESSAGE: &str = "[!!] You are Not Authorized to Access This Resource";
pub const UNPROCESSABLE_ENTITY_MESSAGE: &str = "[!!] This Request Has An Unprocessable Entity";
pub const UNPROCESSABLE_ENTITY_NOTE: &str =
    "The request was well-formed but was unable to be followed due to semantic/parsing errors.";

// Logging
pub const TERMINAL_FILTER: &str = "warn,info,protein=debug";
pub const LOG_FILE: &str = "[Protein].log";
pub const LOG_PATH: &str = "./logs/";

// Jobs
pub const API_KEY_JOB_INTERVAL: &str = "every day";
pub const ASSET_CLEANER_JOB_INTERVAL: &str = "every day";
pub const EXERCISE_CACHE_JOB_INTERVAL: &str = "every day";

// Assets
pub const ASSETS_AVATARS_PATH: &str = "assets/avatars";
pub const ASSETS_RETENTION_PERIOD: i64 = 90; // 90 days
pub const MAX_AVATAR_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

// User Agent Regexer
pub const USER_AGENT_PARSER_PATH: &str = "regexes.yaml";

// Embeds
pub const EMBED_COLOR: u32 = 0x5865F2;
pub const EMBED_TITLE: &str = "Hypertrophy - Protein";
pub const EMBED_THUMBNAIL: &str = "https://files.catbox.moe/025e3m.png";
pub const EMBED_FOOTER: &str = "Hypertrophy - Made with ❤️";
