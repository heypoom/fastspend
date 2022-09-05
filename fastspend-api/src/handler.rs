use serde::{Deserialize, Serialize};
use worker::{console_log, Cors, Request, Response, RouteContext};

use crate::{
    config,
    parser::parse_command,
    sinks::{self, ynab::TransactionInput},
    HandlerContext,
};

type Res = Result<Response, worker::Error>;

#[derive(Serialize, Deserialize)]
struct CommandPayload {
    command: String,
}

fn ok(body: impl Into<String>) -> Res {
    cors(Response::ok(body))
}

fn err(body: impl Into<String>, status: u16) -> Res {
    cors(Response::error(body, status))
}

fn cors(res: Res) -> Res {
    res.unwrap().with_cors(&Cors::new().with_origins(vec!["*"]))
}

pub async fn command_handler(
    mut req: Request,
    ctx: RouteContext<HandlerContext>,
) -> Result<worker::Response, worker::Error> {
    let payload = req.json::<CommandPayload>().await;

    let config = &config::STATIC_CONFIG;

    if let Ok(payload) = payload {
        let input = payload.command.clone();
        let commands = parse_command(input);

        for command in commands {
            let modifier = command.modifier.clone();
            let explicit_payee = command.payee_name.clone();

            console_log!("Command: {:?}", command);

            let account = config::account_by_modifier(&config.accounts, command.modifier);

            if account == None {
                return err("account not found", 500);
            }

            let keyword = config.get_keyword(command.keyword);

            if keyword == None {
                return err("keyword undefined", 400);
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
                inflow: account.is_inflow(modifier),
                account_id: account.id.clone(),
                category_id: keyword.category_id.clone(),
                payee_name: payee_name,
                flag_color: None,
                memo: command.memo,
                amount: command.amount,
                payee_id: keyword.payee_id.clone(),
            };

            let budget_id = ctx.data.ynab_budget_id.clone();
            let token = ctx.data.ynab_token.clone();

            let tx = || async move {
                let _result = sinks::ynab::create_ynab_transaction(input, &budget_id, &token).await;

                ()
            };

            ctx.data.worker_context.wait_until(tx());
        }

        return ok("{\"status\": \"ok\"}");
    }

    err("error", 400)
}
