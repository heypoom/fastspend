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
