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
struct UserEvent {
    #[serde(rename = "username")]
    user_name: String,
    #[serde(rename = "password")]
    password: String,
    #[serde(rename = "code")]
    code: i32,
    #[serde(rename = "mobile")]
    mobile: String,
    #[serde(rename = "type")]
    type: u32,
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
    fn new(body: String) -> Self {
        CustomOutput {
            is_base64_encoded: false,
            status_code: 200,
            body,
        }
    }
}
//https://robertohuertas.com/2018/12/02/aws-lambda-rust/

static API_BASE_URL: &str = "http://localhost:8080/user/";
static API_BASE_USERCODE_URL: &str = "http://localhost:8080/usercode/";
static API_BASE_REGISTER_URL: &str = "http://localhost:8080/user/register/";


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(user_handler);

    Ok(())
}

fn user_handler(e: UserEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.type == 1{
        return get_user_handler(e, c);
    }
    else if e.type == 2{
        return user_handler_for_send_user_code(e, c);
    }
    else {
        return user_handler_registe_user(e, c);
    }
}



fn get_user_handler(e: UserEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {

    if e.mobile == "" {
        error!("Empty user mobile in request {}", c.aws_request_id);
        return Err(c.new_error("Empty mobile"));
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
    map.insert("code", e.code.to_string());
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
        is_base64_encoded: false,
        status_code: 200,
        message: format!("{}!", result.to_string()),
    })
}

fn user_handler_for_send_user_code(e: UserEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {

    if e.mobile == "" {
        error!("Empty user mobile in request {}", c.aws_request_id);
        return Err(c.new_error("Empty mobile"));
    }

    let mut map = HashMap::new();
    map.insert("code", e.code.to_string());
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

fn user_handler_registe_user(e: UserEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {

    if e.mobile == "" {
        error!("Empty user mobile in request {}", c.aws_request_id);
        return Err(c.new_error("Empty mobile"));
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
    map.insert("code", e.code.to_string());
    map.insert("password", e.password);
    map.insert("mobile", e.mobile);
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