use worker::console_log;

pub async fn submit_transaction_to_ynab() -> std::result::Result<bool, reqwest::Error> {
    let res = reqwest::get("http://icanhazip.com").await?.text().await?;
    console_log!("My IP is {}", res);

    Ok(true)
}
