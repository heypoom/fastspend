use serde::{Deserialize, Serialize};
use worker::{console_log, Request, Response, RouteContext};

use crate::{
    config,
    parser::parse_command,
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

    let ynab_budget_id = ctx.secret("YNAB_BUDGET_ID")?.to_string();
    let ynab_token = ctx.secret("YNAB_TOKEN")?.to_string();

    let mut config = config::create_mock_config();

    if let Ok(payload) = payload {
        let input = payload.command.clone();
        let commands = parse_command(input);

        for command in commands {
            let explicit_payee = command.payee_name.clone();

            console_log!("Command: {:?}", command);

            if let Some(payee_name) = command.payee_name {
                config.register_payee(command.payee_key.unwrap(), payee_name);
            }

            let account = config::account_by_modifier(&config.accounts, command.modifier);

            if account == None {
                return Response::error("account not found", 500);
            }

            let keyword = config.get_keyword(command.keyword);

            if keyword == None {
                return Response::error("keyword undefined", 400);
            }

            let keyword = keyword.unwrap();
            let account = account.unwrap();

            let payee_name = if explicit_payee != None {
                explicit_payee
            } else if keyword.payee_name != None {
                keyword.payee_name.clone()
            } else {
                None
            };

            let input = TransactionInput {
                inflow: account.is_inflow(None),
                account_id: account.id.clone(),
                category_id: keyword.category_id.clone(),
                payee_name: payee_name,
                flag_color: None,
                memo: None,
                amount: command.amount,
            };

            let _result = sinks::ynab::create_ynab_transaction(
                input,
                ynab_budget_id.clone(),
                ynab_token.clone(),
            )
            .await;
        }

        return Response::ok(payload.command);
    }

    Response::error("error", 400)
}
