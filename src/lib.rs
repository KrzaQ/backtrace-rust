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
    let submission_target = SubmissionTarget {
        token: String::from(token),
        url: String::from(url),
    };
    let report = Report {
        user_defined: func(),
    };

    std::panic::set_hook(Box::new(move |panic_info| {
        sender::submit(&submission_target, &report, panic_info);
    }));
}
