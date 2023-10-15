mod step_function;

#[macro_use]
extern crate log;

use lambda_http::{
    run, service_fn,
    Body, Error, Request,
    Response, http::StatusCode
};
use step_function::Serverless;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    let str_body = std::str::from_utf8(body.as_ref()).expect("Not a UTF-8 string");
    print!("BODY: {:?}", &str_body);

    match serde_yaml::from_str::<Serverless>(&str_body) {
        Ok(serverless) => {
            info!("yml file parsed correctly");

            if let Some(step_function) = serverless.step_functions {
                match step_function.state_machines.iter().next() {
                    Some((_, state_machine)) => {
                        info!("state machine found");
                            let resp = Response::builder()
                            .status(StatusCode::OK)
                            .header("content-type", "text/html")
                            .header("Access-Control-Allow-Headers", "Content-Type,X-Amz-Date,Authorization,X-Api-Key,X-Amz-Security-Token")
                            .header("Access-Control-Allow-Methods", "OPTIONS,POST")
                            .header("Access-Control-Allow-Origin", "*")
                            .body(serde_json::to_string(&state_machine.definition)?.into())?;

                        Ok(resp)
                    },
                    None => {
                        let message = "No state machines found!";
                        error!("{}", &message);
                            let resp = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .header("content-type", "text/html")
                            .body(message.into())?;
                        Ok(resp)
                    },
                }
            } else {
                let message = "No state machines found!";
                error!("{}", &message);
                let resp = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("content-type", "text/html")
                    .body(message.into())?;

                Ok(resp)
            }
        },
        Err(_) => {
            let message = "The string is not a valid yml string!";
            error!("{}", &message);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(message.into())?;

            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .with_target(false)
        .without_time()
        .init();

    info!("🚀 Starting up");
    run(service_fn(function_handler)).await
}
