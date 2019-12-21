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

use rand::Rng;
use rand::distributions::{Bernoulli, Normal, Uniform};

use std::ops::Range;

#[derive(Deserialize, Clone)]
struct UserEvent {
    #[serde(rename = "userName")]
    user_name: String,
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "code")]
    code: u32,
    #[serde(rename = "mobile")]
    mobile: String,
    #[serde(rename = "eventType")]
    event_type: u32,
}


#[derive(Serialize, Clone)]
struct CustomOutput {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    #[serde(rename = "statusCode")]
    status_code: u16,
    message: String,
}

// Just a static method to help us build the `CustomOutput`.
impl CustomOutput {
    fn new(message: String) -> Self {
        CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            message,
        }
    }
}
//https://robertohuertas.com/2018/12/02/aws-lambda-rust/
static API_BASE_URL: &str = "http://localhost:8080/home/";
static API_BASE_USER_URL: &str = "http://localhost:8080/user/";
static API_BASE_USERCODE_URL: &str = "http://localhost:8080/usercode/";
static API_BASE_REGISTER_URL: &str = "http://localhost:8080/user/register/";

#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", rename_all = "lowercase")]
enum RngRequest {
    Register {
        user_name: String,
        password: String,
        code: u32,
        mobile: String,
    },
    Login {
        password: String,
        code: u32,
        mobile: String,
    },
    Default {
        mobile: String,
    },
}


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(user_handler);

    Ok(())
}

fn user_handler(event: UserEvent, context: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if event.event_type == 0 {
        return home_handler(event, context);
    }
    else if event.event_type == 1 {
        return get_user_handler(event, context);
    }
    else if event.event_type == 2 {
        return user_handler_for_send_user_code(event, context);
    }
    else {
        return user_handler_registe_user(event, context);
    }
}


fn home_handler(event: UserEvent, context: lambda::Context) -> Result<CustomOutput, HandlerError> {
    let mut response=reqwest::Client::new()
    .post(API_BASE_USER_URL)
    .send()
    .unwrap();
    // copy the response body directly to stdout
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf).unwrap();
    let result = std::str::from_utf8(&buf).unwrap();

    Ok(CustomOutput {
        is_base64_encoded: false,
        status_code: 200,
        message: format!("{}!", result.to_string()),
    })
}




fn get_user_handler(event: UserEvent, context: lambda::Context) -> Result<CustomOutput, HandlerError> {

    if event.mobile == "" && event.user_name == "" {
        error!("Empty user mobile and user name  in request {}", context.aws_request_id);
        return Err(context.new_error("both mobile and username are empty"));
    }
    // if e.user_name == "" {
    //     error!("Empty user name in request {}", c.aws_request_id);
    //     return Err(c.new_error("Empty username"));
    // }
    if event.password == "" {
        error!("Empty user password in request {}", context.aws_request_id);
        return Err(context.new_error("Empty password"));
    }
    let mut map = HashMap::new();
    map.insert("userName", event.user_name);
    map.insert("code", event.code.to_string());
    map.insert("password", event.password);
    map.insert("mobile", event.mobile);
    let mut response=reqwest::Client::new()
    .post(API_BASE_USER_URL)
    .json(&map)
    .send()
    .unwrap();
    // copy the response body directly to stdout
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf).unwrap();
    let result = std::str::from_utf8(&buf).unwrap();

    Ok(CustomOutput {
        is_base64_encoded: false,
        status_code: 200,
        message: format!("{}!", result.to_string()),
    })
}

fn user_handler_for_send_user_code(event: UserEvent, context: lambda::Context) -> Result<CustomOutput, HandlerError> {

    if event.mobile == "" {
        error!("Empty user mobile in request {}", context.aws_request_id);
        return Err(context.new_error("Empty mobile"));
    }

    let mut map = HashMap::new();
    map.insert("code", event.code.to_string());
    let mut response=reqwest::Client::new()
    .post(API_BASE_USERCODE_URL)
    .json(&map)
    .send()
    .unwrap();
    // copy the response body directly to stdout
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf).unwrap();
    let result = std::str::from_utf8(&buf).unwrap();
    Ok(CustomOutput {
        is_base64_encoded: false,
        status_code: 200,
        message: format!("{}!", result.to_string()),
    })
}

fn user_handler_registe_user(event: UserEvent, context: lambda::Context) -> Result<CustomOutput, HandlerError> {

    if event.mobile == "" {
        error!("Empty user mobile in request {}", context.aws_request_id);
        return Err(context.new_error("Empty mobile"));
    }

    if event.user_name == "" {
        error!("Empty user name in request {}", context.aws_request_id);
        return Err(context.new_error("Empty username"));
    }
    if event.password == "" {
        error!("Empty user password in request {}", context.aws_request_id);
        return Err(context.new_error("Empty password"));
    }
    let mut map = HashMap::new();
    map.insert("userName", event.user_name);
    map.insert("code", event.code.to_string());
    map.insert("password", event.password);
    map.insert("mobile", event.mobile);
    let mut response=reqwest::Client::new()
    .post(API_BASE_REGISTER_URL)
    .json(&map)
    .send()
    .unwrap();
    // copy the response body directly to stdout
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf).unwrap();
    let result = std::str::from_utf8(&buf).unwrap();
    Ok(CustomOutput {
        is_base64_encoded: false,
        status_code: 200,
        message: format!("{}!", result.to_string()),
    })
}