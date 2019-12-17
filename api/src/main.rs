#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate core;
extern crate reqwest;
use std::collections::HashMap;
use reqwest::Client;
use reqwest::StatusCode;
use lambda::error::HandlerError;
use std::error::Error;

#[derive(Deserialize, Clone)]
struct GetUserEvent {
    #[serde(rename = "username")]
    user_name: String,
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "code")]
    code: i32,
    #[serde(rename = "mobile")]
    mobile: String,

}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

static API_BASE_URL: &str = "http://localhost:8080/user/";


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(user_handler);

    Ok(())
}

fn user_handler(e: GetUserEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.mobile == "" {
        error!("Empty user mobile in request {}", c.aws_request_id);
        return Err(c.new_error("Empty mobile"));
    }

    if e.code == "" {
        error!("Empty user code in request {}", c.aws_request_id);
        return Err(c.new_error("Empty code"));
    }

    if e.user_name == "" {
        error!("Empty user name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty username"));
    }
    if e.password == "" {
        error!("Empty user password in request {}", c.aws_request_id);
        return Err(c.new_error("Empty password"));
    }
    let mut map = HashMap::new();
    map.insert("userName", e.user_name);
    map.insert("code", e.code);
    map.insert("password", e.password);
    map.insert("mobile", e.mobile);
    let mut response=reqwest::Client::new()
    .post(API_BASE_URL)
    .json(&map)
    .send()
    .unwrap();
    // copy the response body directly to stdout
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf).unwrap();
    let result = std::str::from_utf8(&buf).unwrap();
    Ok(CustomOutput {
        message: format!("{}!", result.to_string()),
    })
}