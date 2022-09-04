#[derive(PartialEq, Debug, Clone)]
pub enum KeywordType {
    CATEGORY,
    PAYEE,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Keyword {
    pub r#type: KeywordType,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub payee_id: Option<String>,
    pub payee_name: Option<String>,
}
