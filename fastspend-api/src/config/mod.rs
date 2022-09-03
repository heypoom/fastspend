mod keyword;

use keyword::{Keyword, KeywordType};
use std::collections::HashMap;

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

        let keyword_f = config.get_keyword("f".into()).unwrap();
        assert_eq!(keyword_f.r#type, KeywordType::CATEGORY);
        assert_eq!(keyword_f.key, MOCK_CATEGORY);

        let keyword_sb = config.get_keyword("sb".into()).unwrap();
        assert_eq!(keyword_sb.r#type, KeywordType::PAYEE);
        assert_eq!(keyword_sb.key, "Starbucks");
    }
}
