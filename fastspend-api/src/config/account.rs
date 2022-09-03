use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum ModifierType {
    INFLOW,
    OUTFLOW,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Modifier {
    pub r#type: ModifierType,
    pub alias: String,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub title: String,
    pub default: bool,
    pub modifiers: Vec<Modifier>,
}

impl Account {
    pub fn is_inflow(&self, modifier: Option<String>) -> bool {
        match modifier {
            None => false,
            Some(modifier) if modifier.is_empty() => false,
            Some(modifier) => self
                .modifiers
                .iter()
                .any(|m| &m.alias == &modifier && m.r#type == ModifierType::INFLOW),
        }
    }
}

pub fn default_account(accounts: &Vec<Account>) -> Option<&Account> {
    accounts.iter().find(|a| a.default == true)
}

pub fn account_by_modifier(accounts: &Vec<Account>, modifier: Option<String>) -> Option<&Account> {
    match modifier {
        None => default_account(accounts),
        Some(modifier) if modifier.is_empty() => default_account(accounts),
        Some(modifier) => {
            let account = accounts
                .iter()
                .find(|a| a.modifiers.iter().any(|m| &m.alias == &modifier));

            if account == None {
                return default_account(accounts);
            }

            account
        }
    }
}
