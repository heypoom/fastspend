use pest::Parser;

#[derive(Parser)]
#[grammar = "./parser/spending.pest"]
pub struct SpendingParser;

#[derive(Debug, Default)]
pub struct LogCommand {
    pub amount: f64,
    pub keyword: String,

    // Explicit payee alias with (@)
    pub payee_key: Option<String>,

    // Payee name registered with (:)
    pub payee_name: Option<String>,

    // Account and inflow/outflow modifier with (!)
    pub modifier: Option<String>,

    // Tags with ($)
    pub tags: Option<String>,
}

pub fn parse_command(input: String) -> Vec<LogCommand> {
    let result = SpendingParser::parse(Rule::expression, &input)
        .expect("fail")
        .next()
        .expect("woah");

    result
        .into_inner()
        .map(|pair| -> LogCommand {
            let mut command = LogCommand::default();

            pair.into_inner().for_each(|inner| {
                let value = inner.as_str();

                match inner.as_rule() {
                    Rule::amount => command.amount = value.parse().unwrap(),
                    Rule::keyword => command.keyword = value.into(),
                    Rule::payee_key => command.payee_key = Some(value.into()),
                    Rule::payee_name => command.payee_name = Some(value.into()),
                    Rule::modifier => command.modifier = Some(value.into()),
                    Rule::tags => command.tags = Some(value.into()),
                    _ => {}
                }
            });

            command
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_command;

    #[test]
    fn parse_multiple_transactions() {
        let commands = parse_command("500f, 302.58f".into());
        assert_eq!(commands[0].amount, 500.0);
        assert_eq!(commands[1].amount, 302.58);
    }

    #[test]
    fn parse_complex_transaction() {
        let commands = parse_command("402.98f@sb!d$ok : Starbucks".into());
        let cmd = &commands[0];
        assert_eq!(cmd.amount, 402.98);
        assert_eq!(cmd.payee_key, Some("sb".into()));
        assert_eq!(cmd.payee_name, Some("Starbucks".into()));
        assert_eq!(cmd.modifier, Some("d".into()));
        assert_eq!(cmd.tags, Some("ok".into()));
    }
}
