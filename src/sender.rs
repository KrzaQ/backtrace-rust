extern crate error_chain;
extern crate reqwest;
extern crate rustc_version_runtime;
extern crate serde_json;
extern crate uuid;

use std::collections::HashMap;
use std::panic::PanicInfo;
use std::time;

use Report;
use SubmissionTarget;

pub fn submit<T>(st: &SubmissionTarget, _p: &PanicInfo, user_handler: &T)
where
    T: Fn(&mut Report, &PanicInfo) -> () + Send + Sync + 'static,
{
    let bt = error_chain::Backtrace::new();

    let version = rustc_version_runtime::version();
    let version = format!("{}.{}", version.major, version.minor);

    println!("{:?}", version);

    let mut r = Report {
        ..Default::default()
    };
    user_handler(&mut r, _p);

    let mut stack = Vec::new();

    for x in bt.frames() {
        for y in x.symbols() {
            let line = match y.lineno() {
                Some(x) => x.to_string(),
                None => String::new(),
            };

            let filename = match y.filename() {
                Some(x) => String::from(match x.to_str() {
                    Some(w) => w,
                    None => "",
                }),
                None => String::new(),
            };

            let addr = match y.addr() {
                Some(x) => format!("{:p}", x),
                None => String::new(),
            };

            let name = match y.name() {
                Some(x) => x.to_string(),
                None => String::new(),
            };

            let mut elem = HashMap::new();
            elem.insert(String::from("line"), line);
            elem.insert(String::from("library"), filename);
            elem.insert(String::from("address"), addr);
            elem.insert(String::from("funcName"), name);
            stack.push(elem);
        }
    }

    let payload = json!({
        "uuid": uuid::Uuid::new_v4().to_string(),
        "timestamp": get_timestamp(),
        "lang": "Rust",
        "langVersion": version,
        "agent": "backtrace-rust",
        "agentVersion": "0.0.0",
        "mainThread": "main",
        "annotations": r.annotations,
        "attributes": r.attributes,
        "threads": {
            "main": {
                "name": "main",
                "fault": true,
                "stack": stack
            }
        }
    });

    println!("{}", payload.to_string());

    let url = format!("{}/api/post?format=json&token={}", st.url, st.token);

    let client = reqwest::Client::new();

    let resp = client.post(&url).json(&payload).send();

    match resp {
        Ok(x) => println!("{:?}", x),
        Err(error) => println!("{:?}", error),
    }
}

fn get_timestamp() -> u64 {
    let now = time::SystemTime::now();
    now.duration_since(time::UNIX_EPOCH).expect("").as_secs()
}
