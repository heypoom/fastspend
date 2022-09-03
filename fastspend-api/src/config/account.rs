use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub title: String,
    pub default: bool,
    pub inflow_modifiers: Vec<String>,
    pub outflow_modifiers: Vec<String>,
}

impl Account {
    pub fn is_inflow(&self, modifier: Option<String>) -> bool {
        match modifier {
            None => false,
            Some(modifier) if modifier.is_empty() => false,
            Some(modifier) => self.inflow_modifiers.iter().any(|m| m == &modifier),
        }
    }
}
