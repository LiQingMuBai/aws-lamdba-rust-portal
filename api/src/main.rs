#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

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

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(user_handler);

    Ok(())
}

fn user_handler(e: GetUserEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.user_name == "" {
        error!("Empty user name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty username"));
    }
    if e.password == "" {
        error!("Empty user name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty username"));
    }
    if e.mobile == "" {
        error!("Empty user name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty username"));
    }


    Ok(CustomOutput {
        message: format!("Hello, {}!", e.user_name),
    })
}