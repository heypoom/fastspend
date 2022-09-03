use serde::{Deserialize, Serialize};
use sinks::ynab::TransactionInput;
use utils::log_request;
use worker::{event, Env, Request, Response, Result, Router};

mod config;
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

            // TODO: update account id
            let account_id = "f076943a-aa68-46d5-a78f-00880fa8f067".into();

            let ynab_budget_id = ctx.secret("YNAB_BUDGET_ID")?.to_string();
            let ynab_token = ctx.secret("YNAB_TOKEN")?.to_string();

            let config = config::create_mock_config();

            if let Ok(payload) = payload {
                let keyword = config.get_keyword(payload.command.clone());

                if keyword == None {
                    return Response::error("keyword undefined", 400);
                }

                let tx_info = keyword.unwrap().into_transaction();

                let input = TransactionInput {
                    account_id: account_id,
                    category_id: tx_info.category_id,
                    payee_name: tx_info.payee,
                    flag_color: None,
                    memo: None,
                    amount: 1999.20,
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
