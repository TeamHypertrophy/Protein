/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

pub const CACHE_EXPIRATION_TIME: i64 = 1800; // 30 minutes
pub const RATE_LIMIT_PER: u32 = 30u32; // 30 requests per minnute
pub const POSTGRES_POOL_SIZE: usize = 10; // 10 Concurrent Postgres Connections
pub const PONG: &str = "PONG"; // Redis PONG Response
