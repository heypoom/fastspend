use serde::{Deserialize, Serialize};
use worker::{Request, Response, RouteContext};

use crate::{
    config,
    sinks::{self, ynab::TransactionInput},
};

#[derive(Serialize, Deserialize)]
struct CommandPayload {
    command: String,
}

pub async fn command_handler(
    mut req: Request,
    ctx: RouteContext<()>,
) -> Result<worker::Response, worker::Error> {
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

        let _result = sinks::ynab::create_ynab_transaction(input, ynab_budget_id, ynab_token).await;

        return Response::ok(payload.command);
    }

    Response::error("error", 400)
}
