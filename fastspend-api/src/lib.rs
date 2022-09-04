use handler::command_handler;
use utils::log_request;
use worker::{event, Env, Request, Response, Result, Router};

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod handler;
pub mod parser;
pub mod sinks;
pub mod utils;

pub struct HandlerContext {
    pub worker_context: worker::Context,

    pub ynab_token: String,
    pub ynab_budget_id: String,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, worker_context: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::with_data(HandlerContext {
        worker_context,
        ynab_token: env.secret("YNAB_TOKEN")?.to_string(),
        ynab_budget_id: env.secret("YNAB_BUDGET_ID")?.to_string(),
    });

    router
        .get("/", |_, _| Response::ok("{}"))
        .get("/status", |_, _| Response::ok("ready"))
        .post_async("/command", command_handler)
        .run(req, env)
        .await
}
