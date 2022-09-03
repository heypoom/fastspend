use std::fmt::Debug;

use chrono::Utc;
use reqwest::{self};
use worker::{console_log, Date};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    pub transaction: Transaction,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub account_id: String,
    pub amount: i64,
    pub payee_name: String,
    pub category_id: String,
    pub memo: String,
    pub cleared: String,
    pub approved: bool,
    pub flag_color: String,
    pub date: String,
}

pub async fn create_ynab_transaction(
    budget_id: String,
    account_id: String,
    category_id: String,
    flag_color: String,
    payee_name: String,
    memo: String,
    amount: f64,
    authorization_token: String,
) -> std::result::Result<bool, reqwest::Error> {
    let endpoint = format!(
        "https://api.youneedabudget.com/v1/budgets/{}/transactions",
        budget_id
    );

    let client = reqwest::Client::new();

    let payload = Payload {
        transaction: Transaction {
            account_id: account_id,
            amount: (amount * 1000.0) as i64,
            payee_name: payee_name,
            category_id: category_id,
            memo: memo,
            flag_color: flag_color,
            approved: true,
            cleared: "cleared".to_owned(),
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
