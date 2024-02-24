use crate::auth::auth_key;
use axum::{
    debug_handler,
    http::StatusCode,
    http::{HeaderMap, Method},
    response::IntoResponse,
    Json,
};
use r_http::{CommandReqBody, CommandRespBody};
use remeowte_http as r_http;
use std::{
    default,
    io::{Error, Result},
    os::unix::process::ExitStatusExt,
    process::Output,
    time::SystemTime,
};
use tokio::process::Command;
use tracing::{debug, error, info, instrument, trace};

fn validate(headers: &HeaderMap) -> bool {
    let (mut ak, mut sk) = (None, None);
    for (name, val) in headers {
        let val = val.to_str().unwrap_or_default().to_string();
        match name.to_string().to_ascii_lowercase().as_str() {
            "ak" => {
                ak = Some(val);
            }
            "sk" => {
                sk = Some(val);
            }
            _ => {}
        }
    }

    if ak.is_none() || sk.is_none() {
        return false;
    }

    tracing::debug!("auth res is {}", auth_key(ak.unwrap(), sk.unwrap()));
    true // FIXME: for test, always true.
}

#[debug_handler]
#[instrument]
pub(crate) async fn cmd_exec(
    method: Method,
    headers: HeaderMap,
    // `String` consumes the request body and thus must be the last extractor
    payload: Option<Json<CommandReqBody>>,
) -> impl IntoResponse {
    assert!(method == Method::POST);

    if !validate(&headers) {
        return (
            StatusCode::FORBIDDEN,
            Json(CommandRespBody {
                stdout: String::new(),
                stderr: String::new(),
                return_code: 0,
                terminate_signal: 0,
                start_time: Some(SystemTime::now()),
                end_time: None,
                desc: "Please provide access key and secret key".to_string(),
            }),
        );
    }
    let mut resp = CommandRespBody::default();
    match payload {
        Some(Json(body)) => {
            let res = Command::new(body.command)
                .args(body.args)
                .output()
                .await;
            
            resp.end_time = Some(SystemTime::now());

            match res {
                Ok(output) => {
                    resp.stdout = String::from_utf8(output.stdout).unwrap();
                    resp.stderr = String::from_utf8(output.stderr).unwrap();
                    match output.status.code() {
                        Some(ret_code) => {
                            resp.return_code = ret_code;
                            resp.desc = "Command execute done.".to_string();
                        },
                        None => {
                            resp.terminate_signal = output.status.signal().unwrap_or_default();
                            resp.desc = "Command terminated by signal.".to_string();
                        }
                    }
                },
                Err(e) => {
                    resp.desc = "Command execute failed. Error: ".to_string() + &e.to_string();
                }
            }
        },
        None => {
            resp.desc = "Json format in request body not correct.".to_string();
        }
    }
    (StatusCode::OK, Json(resp))
}

// #[cfg(test)]
// mod test{
//     #[tokio::test]
//     async fn test_handle_request() {
//         use futures::{future::BoxFuture, FutureExt};
//         use std::{
//             io::{Error, Result}, os::unix::process::ExitStatusExt, process::Output, time::SystemTime
//         };
//         use axum::{Json, http::{Method, HeaderMap}, debug_handler};
//         use serde_json::Value;
//         use serde::{Serialize, Deserialize};
//         use tracing::{error, info, debug, trace};
//         use super::{CommandReqBody, CommandRespBody, cmd_exec};
//         use http::{Request, StatusCode};

//         // Create the test data
//         let test_data = CommandReqBody {
//             command: "echo".to_string(),
//             args: vec!["2333".to_string()],
//         };

//         // Create a request using the test data
//         let request = Request::get("/")
//             .extension(test_data)
//             .body(())
//             .unwrap();

//         // Call the request handler
//         let response = cmd_exec(Extension(test_data))
//             .await
//             .into_response();

//         // Assert the response status code, headers, and body as needed
//         assert_eq!(response.status(), StatusCode::OK);
//         // ...
//     }
// }
