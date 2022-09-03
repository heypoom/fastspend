use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum KeywordType {
    CATEGORY,
    PAYEE,
}

#[derive(PartialEq, Debug)]
pub struct Keyword {
    pub r#type: KeywordType,
    pub key: String,
    pub title: Option<String>,
}

impl Keyword {
    pub fn into_transaction(&self) -> TransactionInfo {
        match self.r#type {
            KeywordType::CATEGORY => TransactionInfo {
                payee: None,
                category_id: Some(self.key.clone()),
            },
            KeywordType::PAYEE => TransactionInfo {
                payee: Some(self.key.clone()),
                category_id: None,
            },
        }
    }
}

pub struct TransactionInfo {
    pub payee: Option<String>,
    pub category_id: Option<String>,
}

pub struct Config {
    pub keywords: HashMap<String, Keyword>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            keywords: HashMap::new(),
        }
    }

    pub fn add_category(&mut self, alias: String, id: String, title: Option<String>) -> () {
        self.keywords.insert(
            alias,
            Keyword {
                r#type: KeywordType::CATEGORY,
                key: id,
                title: title,
            },
        );
    }

    pub fn add_payee(&mut self, alias: String, name: String) -> () {
        self.keywords.insert(
            alias,
            Keyword {
                r#type: KeywordType::PAYEE,
                key: name,
                title: None,
            },
        );
    }

    pub fn get_keyword(&self, alias: String) -> Option<&Keyword> {
        self.keywords.get(&alias)
    }

    pub fn get_id_by_keyword(&self, alias: String) -> Option<String> {
        self.get_keyword(alias).and_then(|k| Some(k.key.clone()))
    }
}

static MOCK_CATEGORY: &'static str = "d3d92867-779b-453f-bf5b-0adf9859af96";

pub fn create_mock_config() -> Config {
    let mut config = Config::new();
    config.add_category("f".into(), MOCK_CATEGORY.into(), None);
    config.add_payee("sb".into(), "Starbucks".into());

    config
}

#[cfg(test)]
mod tests {
    use crate::config::{create_mock_config, KeywordType, MOCK_CATEGORY};

    #[test]
    fn it_gets_the_keyword() {
        let config = create_mock_config();

        assert_eq!(
            config.get_keyword("f".into()).unwrap().r#type,
            KeywordType::CATEGORY
        );

        assert_eq!(
            config.get_keyword("sb".into()).unwrap().r#type,
            KeywordType::PAYEE
        );

        assert_eq!(config.get_id_by_keyword("f".into()).unwrap(), MOCK_CATEGORY);
        assert_eq!(config.get_id_by_keyword("sb".into()).unwrap(), "Starbucks");
    }
}
