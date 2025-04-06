/*
______          _       _
| ___ \        | |     (_)
| |_/ / __ ___ | |_ ___ _ _ __
|  __/ '__/ _ \| __/ _ \ | '_ \
| |  | | | (_) | ||  __/ | | | |
\_|  |_|  \___/ \__\___|_|_| |_|

        Made with ❤️
*/

// These Are The HTML Templates Being Used In The Email Service
// They Are Rendered Using The Askama Template Engine
// https://docs.rs/askama/0.12.1/askama/

use askama::Template;

#[derive(Template)]
#[template(path = "signup.html")]
pub struct Signup<'a> {
    pub name: &'a str,
    pub verification_link: &'a str,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login<'a> {
    pub name: &'a str,
    pub time: &'a str,
    pub ip_address: &'a str,
    pub device: &'a str,
}

#[derive(Template)]
#[template(path = "password/update.html")]
pub struct UpdatedPassword<'a> {
    pub name: &'a str,
    pub ip_address: &'a str,
}

#[derive(Template)]
#[template(path = "password/reset.html")]
pub struct PasswordReset<'a> {
    pub name: &'a str,
    pub new_password: &'a str,
    pub ip_address: &'a str,
    pub device: &'a str,
}

#[derive(Template)]
#[template(path = "mfa/code.html")]
pub struct MFACode<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Template)]
#[template(path = "mfa/verification.html")]
pub struct MFAVerification<'a> {
    pub name: &'a str,
    pub verification_link: &'a str,
}

#[derive(Template)]
#[template(path = "mfa/disabled.html")]
pub struct MFADisabled<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "email/verified.html")]
pub struct EmailVerified<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "account/deleted.html")]
pub struct AccountDeleted<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "password/request.html")]
pub struct RequestPasswordReset<'a> {
    pub username: &'a str,
    pub code: &'a str,
}

#[derive(Template)]
#[template(path = "success/success.html")]
pub struct Success<'a> {
    pub name: &'a str,
}
