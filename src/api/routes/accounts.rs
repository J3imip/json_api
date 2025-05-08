use crate::services::account::AccountService;
use actix_web::Responder;
use actix_web::get;
use actix_web::scope;

#[scope("/accounts")]
pub mod accounts {
    use super::*;
    use crate::db::repositories::account_repo::AccountRepo;

    #[get("")]
    pub async fn get_account_list() -> impl Responder {
        let resp = AccountService::new(AccountRepo {})
            .get_many()
            .await
            .unwrap();

        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&resp).unwrap())
    }

    #[get("/{id}")]
    pub async fn get_account() -> impl Responder {
        let resp = AccountService::new(AccountRepo {}).get_one().await.unwrap();

        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&resp).unwrap())
    }
}
