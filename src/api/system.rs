/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

use rocket::{
    get,
    serde::json::{json, Value},
    State,
};

use sysinfo::System;

use crate::build;

#[get("/", format = "application/json")]
pub async fn system(sys: &State<System>) -> Value {
    json!({
        "KERNEL_VERSION": System::kernel_version(),
        "OS_VERSION": System::long_os_version(),
        "UPTIME": System::uptime(),
        "BOOT_TIME": System::boot_time(),
        "CPU_CORE_COUNT": sys.physical_core_count(),
        "TOTAL_MEM": sys.total_memory(),
    })
}

#[get("/rust", format = "application/json")]
pub async fn rust() -> Value {
    json!({
        "RUST_VERSION": build::RUST_VERSION,
        "RUST_CHANNEL": build::RUST_CHANNEL,
        "CARGO_VERSION": build::CARGO_VERSION,
    })
}

#[get("/package", format = "application/json")]
pub async fn package() -> Value {
    json!({
        "BUILD_OS": build::BUILD_OS,
        "PROJECT_NAME": build::PROJECT_NAME,
        "BUILD_TIME": build::BUILD_TIME,
        "BUILD_RUST_CHANNEL": build::BUILD_RUST_CHANNEL,
        "BRANCH": build::BRANCH,
        "PKG_VERSION": build::PKG_VERSION,
    })
}

#[get("/git", format = "application/json")]
pub async fn git() -> Value {
    json!({
        "GIT_BRANCH": shadow_rs::branch(),
        "GIT_TAG": shadow_rs::tag(),
        "GIT_CLEAN": shadow_rs::git_clean(),
        "GIT_COMMIT": build::SHORT_COMMIT,
        "GIT_COMMIT_DATE": build::COMMIT_DATE_2822,
    })
}
