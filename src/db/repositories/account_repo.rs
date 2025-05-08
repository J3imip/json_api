use crate::db::models::account_row::AccountRow;

pub struct AccountRepo {}

impl AccountRepo {
    pub fn new() -> Self {
        AccountRepo {}
    }

    pub async fn get_many(&self) -> Result<Vec<AccountRow>, Box<dyn std::error::Error>> {
        let account_row = AccountRow {
            address: "address".to_string(),
        };

        let account_rows = vec![account_row];

        Ok(account_rows)
    }

    pub async fn get_one(&self) -> Result<AccountRow, Box<dyn std::error::Error>> {
        let account_row = AccountRow {
            address: "address".to_string(),
        };

        Ok(account_row)
    }
}
