mod account;
mod keyword;

pub use account::Account;
pub use keyword::{Keyword, KeywordType};

use std::collections::HashMap;

pub struct Config {
    pub keywords: HashMap<String, Keyword>,
    pub accounts: Vec<Account>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            keywords: HashMap::new(),
            accounts: vec![],
        }
    }

    pub fn register_category(&mut self, alias: String, id: String, title: Option<String>) -> () {
        self.keywords.insert(
            alias,
            Keyword {
                r#type: KeywordType::CATEGORY,
                category_id: Some(id),
                category_name: title,
                payee_name: None,
            },
        );
    }

    pub fn register_payee(&mut self, alias: String, name: String) -> () {
        self.keywords.insert(
            alias,
            Keyword {
                r#type: KeywordType::PAYEE,
                category_id: None,
                category_name: None,
                payee_name: name.into(),
            },
        );
    }

    pub fn create_payee_with_category_alias(
        &mut self,
        alias: String,
        name: String,
        category_alias: String,
    ) -> () {
        if let Some(category) = self.get_keyword(category_alias) {
            // If the category alias does not resolve to a category, fail the operation.
            if category.r#type != KeywordType::CATEGORY {
                return;
            }

            self.keywords.insert(
                alias,
                Keyword {
                    r#type: KeywordType::PAYEE,
                    payee_name: name.into(),
                    category_id: category.category_id.clone(),
                    category_name: category.category_name.clone(),
                },
            );
        }
    }

    pub fn get_keyword(&self, alias: String) -> Option<&Keyword> {
        self.keywords.get(&alias)
    }

    pub fn set_default_expense_account(
        &mut self,
        name: String,
        modifier: String,
        id: String,
    ) -> () {
        if self.default_account() != None {
            return;
        }

        self.accounts.push(Account {
            id: id,
            title: name,
            default: true,
            inflow_modifiers: vec![],
            outflow_modifiers: vec![modifier],
        });
    }

    pub fn default_account(&self) -> Option<&Account> {
        self.accounts.iter().find(|a| a.default == true)
    }

    pub fn is_inflow(&self, modifier: Option<String>) -> bool {
        match modifier {
            None => false,
            Some(modifier) if modifier.is_empty() => false,
            Some(modifier) => self
                .accounts
                .iter()
                .any(|a| a.inflow_modifiers.iter().any(|m| m.contains(&modifier))),
        }
    }
}

static MOCK_CATEGORY: &'static str = "d3d92867-779b-453f-bf5b-0adf9859af96";
static MOCK_ACCOUNT: &'static str = "f076943a-aa68-46d5-a78f-00880fa8f067";
static MOCK_PAYEE: &'static str = "Starbucks Coffee";

pub fn create_mock_config() -> Config {
    let mut config = Config::new();

    config.register_category("f".into(), MOCK_CATEGORY.into(), None);

    // Register with the command "100f@sb : Starbucks Coffee"
    config.create_payee_with_category_alias("sb".into(), MOCK_PAYEE.into(), "f".into());

    // Set a default expense account, in this case a credit card.
    config.set_default_expense_account("Credit Card".into(), "c".into(), MOCK_ACCOUNT.into());

    config
}

#[cfg(test)]
mod tests {
    use crate::config::{create_mock_config, KeywordType, MOCK_CATEGORY, MOCK_PAYEE};

    #[test]
    fn it_gets_the_keyword() {
        let config = create_mock_config();

        let keyword_f = config.get_keyword("f".into()).unwrap();
        assert_eq!(keyword_f.r#type, KeywordType::CATEGORY);
        assert_eq!(keyword_f.category_id, Some(MOCK_CATEGORY.into()));

        let keyword_sb = config.get_keyword("sb".into()).unwrap();
        assert_eq!(keyword_sb.r#type, KeywordType::PAYEE);
        assert_eq!(keyword_sb.payee_name, Some(MOCK_PAYEE.into()));
    }
}
