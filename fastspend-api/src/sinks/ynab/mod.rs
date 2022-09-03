use std::fmt::Debug;

use chrono::Utc;
use reqwest::{self};
use worker::console_log;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YnabPayload {
    pub transaction: YnabTransaction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YnabTransaction {
    pub account_id: String,
    pub amount: i64,
    pub payee_name: Option<String>,
    pub category_id: Option<String>,
    pub memo: Option<String>,
    pub cleared: String,
    pub approved: bool,
    pub flag_color: Option<String>,
    pub date: String,
}

pub struct TransactionInput {
    pub account_id: String,
    pub category_id: Option<String>,
    pub flag_color: Option<String>,
    pub payee_name: Option<String>,
    pub memo: Option<String>,
    pub amount: f64,
}

pub async fn create_ynab_transaction(
    input: TransactionInput,
    budget_id: String,
    authorization_token: String,
) -> std::result::Result<bool, reqwest::Error> {
    let endpoint = format!(
        "https://api.youneedabudget.com/v1/budgets/{}/transactions",
        budget_id
    );

    let client = reqwest::Client::new();

    let TransactionInput {
        amount,
        account_id,
        category_id,
        payee_name,
        flag_color,
        memo,
    } = input;

    let payload = YnabPayload {
        transaction: YnabTransaction {
            account_id: account_id,
            amount: (amount * 1000.0) as i64,
            payee_name: payee_name,
            category_id: category_id,
            memo: memo,
            flag_color: flag_color,
            approved: true,
            cleared: "cleared".into(),
            date: Utc::today().format("%Y-%m-%d").to_string(),
        },
    };

    let response = client
        .post(endpoint)
        .json(&payload)
        .header("Authorization", format!("Bearer {}", authorization_token))
        .send()
        .await?
        .text()
        .await?;

    console_log!("YNAB Response: {}", response);

    Ok(true)
}
