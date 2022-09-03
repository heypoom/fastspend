use reqwest;
use serde::{Deserialize, Serialize};
use worker::{console_log, event, Date, Env, Request, Response, Result, Router};

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[derive(Serialize, Deserialize)]
struct CommandPayload {
    command: String,
}

pub async fn log_to_ynab() -> std::result::Result<bool, reqwest::Error> {
    let res = reqwest::get("http://icanhazip.com").await?.text().await?;
    console_log!("My IP is {}", res);

    Ok(true)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("{}"))
        .get("/status", |_, _| Response::ok("ready"))
        .post_async("/command", |mut req, ctx| async move {
            let payload = req.json::<CommandPayload>().await;

            if let Ok(payload) = payload {
                let _result = log_to_ynab().await;

                return Response::ok(payload.command);
            }

            Response::error("error", 400)
        })
        .run(req, env)
        .await
}
