#[macro_use]
extern crate serde_json;

pub mod backtrace;
mod sender;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UserDefined {
    pub annotations: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SubmissionTarget {
    token: String,
    url: String,
}

#[derive(Debug, Clone)]
pub struct Report {
    user_defined: UserDefined,
}

pub fn register_error_handler<T>(url: &str, token: &str, func: T)
where
    T: Fn() -> UserDefined,
{
    unsafe {
        SUBMISSION_TARGET = Some(SubmissionTarget {
            token: String::from(token),
            url: String::from(url),
        });
        REPORT = Some(Report {
            user_defined: func(),
        });
    }

    std::panic::set_hook(Box::new(|panic_info| unsafe {
        sender::submit(&SUBMISSION_TARGET, &REPORT, panic_info);
    }));
}

static mut SUBMISSION_TARGET: Option<SubmissionTarget> = None;
static mut REPORT: Option<Report> = None;
