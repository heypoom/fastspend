use serde::{Deserialize, Serialize};
use sinks::ynab::TransactionInput;
use utils::log_request;
use worker::{event, wasm_bindgen::UnwrapThrowExt, Env, Request, Response, Result, Router};

mod sinks;
mod utils;

#[derive(Serialize, Deserialize)]
struct CommandPayload {
    command: String,
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

            let ynab_budget_id = ctx.secret("YNAB_BUDGET_ID")?.to_string();
            let ynab_token = ctx.secret("YNAB_TOKEN")?.to_string();

            if let Ok(payload) = payload {
                let input = TransactionInput {
                    account_id: "TODO".to_owned(),
                    category_id: "TODO".to_owned(),
                    flag_color: "red".to_owned(),
                    payee_name: "Automation Test".to_owned(),
                    memo: "From Automation Test".to_owned(),
                    amount: 10.20,
                };

                let _result =
                    sinks::ynab::create_ynab_transaction(input, ynab_budget_id, ynab_token).await;

                return Response::ok(payload.command);
            }

            Response::error("error", 400)
        })
        .run(req, env)
        .await
}
