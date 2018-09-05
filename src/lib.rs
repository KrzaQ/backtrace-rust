#[macro_use]
extern crate serde_json;

pub mod backtrace;
mod sender;

use std::collections::HashMap;
use std::panic::PanicInfo;

#[derive(Debug, Clone)]
pub struct SubmissionTarget {
    token: String,
    url: String,
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub annotations: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
}

pub fn register_error_handler<T>(url: &str, token: &str, user_handler: T)
where
    T: 'static + Fn(&mut Report, &PanicInfo) -> Report + Send + Sync,
{
    let submission_target = SubmissionTarget {
        token: String::from(token),
        url: String::from(url),
    };

    std::panic::set_hook(Box::new(move |panic_info| {
        sender::submit(&submission_target, panic_info, &user_handler);
    }));
}
