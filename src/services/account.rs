use crate::db::repositories::account_repo::AccountRepo;
use resources::models::{
    GetAccount200Response, GetAccountList200Response, GetAccountList200ResponseAllOfDataInner,
};
use serde::Serialize;

pub struct AccountService {
    repo: AccountRepo,
}

// TODO: Move pagination-related structs into a codegen module
#[derive(Serialize)]
struct PaginationMeta {
    current_page: u32,
    total_pages: u32,
}

#[derive(Serialize)]
struct PaginationLinks {
    #[serde(rename = "self")]
    param_self: String,
    prev: Option<String>,
    next: Option<String>,
}

impl AccountService {
    pub(crate) fn new(repo: AccountRepo) -> Self {
        AccountService { repo }
    }

    pub(crate) async fn get_one(
        &self,
    ) -> Result<GetAccount200Response, Box<dyn std::error::Error>> {
        let row = self.repo.get_one().await?;

        Ok(GetAccount200Response {
            data: Box::new(row.into()),
            meta: None,
        })
    }

    pub(crate) async fn get_many(
        &self,
    ) -> Result<GetAccountList200Response, Box<dyn std::error::Error>> {
        let rows = self.repo.get_many().await?;

        let data: Vec<GetAccountList200ResponseAllOfDataInner> =
            rows.into_iter().map(|row| row.into()).collect();

        let meta = PaginationMeta {
            current_page: 1,
            total_pages: 1,
        };

        let links = PaginationLinks {
            param_self: "self_link".to_string(),
            prev: Some("prev_link".to_string()),
            next: Some("next_link".to_string()),
        };

        Ok(GetAccountList200Response {
            data,
            meta: serde_json::to_value(meta).ok(),
            links: serde_json::to_value(links).ok(),
        })
    }
}
