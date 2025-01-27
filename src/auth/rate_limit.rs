/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};

use crate::constants::RATE_LIMIT_PER;

pub type RateLimit<'a> = RocketGovernor<'a, RateLimitGuard>;

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(RATE_LIMIT_PER))
    }
}
