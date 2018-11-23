extern crate lambda_runtime as lambda;
extern crate serde_derive;
extern crate log;
extern crate simple_logger;
extern crate tokio;

use lambda::{lambda, error::HandlerError};
use tokio::runtime::Runtime;
use serde_derive::{Serialize, Deserialize};
use log::error;
use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;

    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(my_handler, rt);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty first name"));
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}