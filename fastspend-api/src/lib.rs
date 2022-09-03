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
            let modifier = Some("c".to_owned());

            let ynab_budget_id = ctx.secret("YNAB_BUDGET_ID")?.to_string();
            let ynab_token = ctx.secret("YNAB_TOKEN")?.to_string();

            let config = config::create_mock_config();
            let account = config::account_by_modifier(&config.accounts, modifier);

            if account == None {
                return Response::error("account not found", 500);
            }

            if let Ok(payload) = payload {
                let keyword = config.get_keyword(payload.command.clone());

                if keyword == None {
                    return Response::error("keyword undefined", 400);
                }

                let keyword = keyword.unwrap();
                let account = account.unwrap();

                let amount = 6969.69;

                let input = TransactionInput {
                    inflow: account.is_inflow(None),
                    account_id: account.id.clone(),
                    category_id: keyword.category_id.clone(),
                    payee_name: keyword.payee_name.clone(),
                    flag_color: None,
                    memo: None,
                    amount: amount,
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
