/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

pub const CACHE_EXPIRATION_TIME: i64 = 900; // 15 minutes
pub const RATE_LIMIT_PER: u32 = 30u32; // 30 requests per minnute
pub const POSTGRES_POOL_SIZE: usize = 10; // 10 Concurrent Postgres Connections
pub const API_QUOTA_LIMIT: i32 = 3000; // 5000 Requests per API Key
pub const PONG: &str = "PONG"; // Redis PONG Response
pub const MASTER_API_KEY: &str = "16114308-8693-4fdf-9a98-e40065067ebe"; // This is a temporary API Key that I am creating for the Master User, will be
                                                                         // removed when prod releases.
