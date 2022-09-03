use serde::Serialize;

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub title: String,
    pub default: bool,
    pub inflow_modifiers: Vec<String>,
    pub outflow_modifiers: Vec<String>,
}
