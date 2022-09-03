use handler::command_handler;
use utils::log_request;
use worker::{event, Env, Request, Response, Result, Router};

#[macro_use]
extern crate pest_derive;

pub mod config;
pub mod handler;
pub mod parser;
pub mod sinks;
pub mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("{}"))
        .get("/status", |_, _| Response::ok("ready"))
        .post_async("/command", command_handler)
        .run(req, env)
        .await
}
