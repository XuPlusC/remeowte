use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandReqBody {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandRespBody {
    pub stdout: String,
    pub stderr: String,
    pub return_code: i32,
    pub terminate_signal: i32,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub desc: String,
}

impl Default for CommandRespBody {
    fn default() -> Self {
        CommandRespBody {
            stdout: String::new(),
            stderr: String::new(),
            return_code: 0,
            terminate_signal: 0,
            start_time: Some(SystemTime::now()),
            end_time: None,
            desc: String::new(),
        }
    }
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
