#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use http::StatusCode;
use lambda_http::{Body, IntoResponse, lambda, Request, Response};
use lambda_runtime::{Context, error::HandlerError};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, PutItemError, PutItemInput, ScanInput,
};
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
    code: i32,
    mobile: String,
}

impl From<&HashMap<String, AttributeValue>> for User {
    fn from(attr_map: &HashMap<String, AttributeValue>) -> Self {
        let code = attr_map
            .get("code")
            .and_then(|v| v.n.clone())
            .unwrap_or_default();

        let username = attr_map
            .get("username")
            .and_then(|v| v.s.clone())
            .unwrap_or_default();

        let password = attr_map
            .get("password")
            .and_then(|v| v.s.clone())
            .unwrap_or_default();

        let mobile = attr_map
            .get("mobile")
            .and_then(|v| v.s.clone())
            .unwrap_or_default();
        User {
            username,
            password,
            code: code.parse::<i32>().unwrap_or_else(|_| 0),
            mobile,

        }
    }
}

impl From<HashMap<String, AttributeValue>> for User {
    fn from(attr_map: HashMap<String, AttributeValue>) -> Self {
        User::from(&attr_map)
    }
}

impl User {
    fn to_attr_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert(
            "username".to_string(),
            AttributeValue {
                s: Some(self.username.clone()),
                ..Default::default()
            },
        );
        map.insert(
            "password".to_string(),
            AttributeValue {
                s: Some(self.password.clone()),
                ..Default::default()
            },
        );
        map.insert(
            "code".to_string(),
            AttributeValue {
                n: Some(self.code.to_string().clone()),
                ..Default::default()
            },
        );
        map.insert(
            "mobile".to_string(),
            AttributeValue {
                s: Some(self.mobile.clone()),
                ..Default::default()
            },
        );
        map
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(router);
    Ok(())
}

// Call handler functionb based on request method
fn router(req: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {


    match req.method().as_str() {
        "POST" => create_user(req, c),
        "GET" => get_user(req, c),
        // "PUT" => send_user_code(req,c),
        "DELETE" => logout(req,c),
        _ => {
            let mut resp = Response::default();
            *resp.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
            Ok(resp)
        }
    }
}


// fn request_handler_for_send_user_code(_: hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> {

//     let mut rt = Runtime::new().unwrap();
//     let httper_client = HttperClient::new();

//     let mut response=reqwest::Client::new()
//     .post("http://localhost:8080/employees/")
//     .json(&map)
//     .send()
//     .unwrap();
//     let mut buf: Vec<u8> = vec![];
//     response.copy_to(&mut buf)?;
//     let body = std::str::from_utf8(&buf).unwrap();

//     println!("{:?}", body.to_string());

//     hyper::Response::builder()
//         .header(CONTENT_LENGTH, body.len() as u64)
//         .header(CONTENT_TYPE, "text/plain")
//         .body(hyper::Body::from(body))
//         .expect("Failed to construct the response")


// }


// fn request_handler_for_logout(_: hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> {
//     let mut rt = Runtime::new().unwrap();
//     let httper_client = HttperClient::new();
//     let response = rt.block_on(httper_client.get("https://www.rust-lang.org/en-US/").send());
//     // let body = format!("{:?}", result);
//     // println!("{}", body.replace("Ok(Response ","").replace(")",""));
//     // copy the response body directly to stdout
//     let mut buf: Vec<u8> = vec![];
//     response.copy_to(&mut buf)?;
//     let body = std::str::from_utf8(&buf)?;

//     println!("{:?}", body.to_string());

//     hyper::Response::builder()
//         .header(CONTENT_LENGTH, body.len() as u64)
//         .header(CONTENT_TYPE, "text/plain")
//         .body(hyper::Body::from(body))
//         .expect("Failed to construct the response")
// }

fn logout(_req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    let client = DynamoDbClient::new(Region::default());
    match client
        .scan(ScanInput {
            table_name: "users".to_owned(),
            ..Default::default()
        })
        .sync()
        {
            Ok(output) => {
                let users: Vec<User> = output
                    .items
                    .unwrap_or_default()
                    .iter()
                    // HashMap -> User
                    .map(|u| u.into())
                    .collect();

                Ok(serde_json::json!(users).into_response())
            }
            Err(e) => {
                error!("Internal {}", e);
                Ok(build_resp(
                    "internal error".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
}


// fn send_user_code(_req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
//     let client = DynamoDbClient::new(Region::default());
//     match client
//         .scan(ScanInput {
//             table_name: "users".to_owned(),
//             ..Default::default()
//         })
//         .sync()
//         {
//             Ok(output) => {
//                 let users: Vec<User> = output
//                     .items
//                     .unwrap_or_default()
//                     .iter()
//                     // HashMap -> User
//                     .map(|u| u.into())
//                     .collect();

//                 Ok(serde_json::json!(users).into_response())
//             }
//             Err(e) => {
//                 error!("Internal {}", e);
//                 Ok(build_resp(
//                     "internal error".to_owned(),
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                 ))
//             }
//         }
// }



// GET /users
fn get_user(_req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    let client = DynamoDbClient::new(Region::default());
    match client
        .scan(ScanInput {
            table_name: "users".to_owned(),
            ..Default::default()
        })
        .sync()
        {
            Ok(output) => {
                let users: Vec<User> = output
                    .items
                    .unwrap_or_default()
                    .iter()
                    // HashMap -> User
                    .map(|u| u.into())
                    .collect();

                Ok(serde_json::json!(users).into_response())
            }
            Err(e) => {
                error!("Internal {}", e);
                Ok(build_resp(
                    "internal error".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
}

// POST /users
fn create_user(req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    // validate user in body
    match serde_json::from_slice::<User>(req.body().as_ref()) {
        Ok(user) => {
            let client = DynamoDbClient::new(Region::UsEast1);
            let input = PutItemInput {
                condition_expression: Some("attribute_not_exists(username)".to_string()),
                table_name: "users".to_string(),
                item: user.clone().to_attr_map(),
                ..Default::default()
            };
            // try adding user
            match client.put_item(input).sync() {
                Ok(_) => {
                    let mut resp = serde_json::json!(user).into_response();
                    *resp.status_mut() = StatusCode::CREATED;
                    Ok(resp)
                }
                Err(e) => match e {
                    PutItemError::ConditionalCheckFailed(_) => Ok(Response::builder()
                        .status(StatusCode::CONFLICT)
                        .body(format!("conflict, {} already exists", user.username).into())
                        .expect("error")),
                    e => {
                        error!("{}", e);
                        Ok(build_resp(
                            "internal error".to_owned(),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ))
                    }
                },
            }
        }
        // couldn't deserialize body
        Err(e) => {
            error!("error: {}", e);
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("bad request".into())
                .expect("err creating response"))
        }
    }
}

// simple response builder that uses a str msessage
fn build_resp(msg: String, status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(msg.into())
        .expect("err creating response")
}
