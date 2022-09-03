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

pub fn default_account(accounts: &Vec<Account>) -> Option<&Account> {
    accounts.iter().find(|a| a.default == true)
}

pub fn account_by_modifier(accounts: &Vec<Account>, modifier: Option<String>) -> Option<&Account> {
    match modifier {
        None => default_account(accounts),
        Some(modifier) if modifier.is_empty() => default_account(accounts),
        Some(modifier) => {
            let account = accounts.iter().find(|a| {
                a.inflow_modifiers.iter().any(|m| m == &modifier)
                    || a.outflow_modifiers.iter().any(|m| m == &modifier)
            });

            if account == None {
                return default_account(accounts);
            }

            account
        }
    }
}
