use resources::models::{
    GetAccountList200ResponseAllOfDataInner, GetAccountList200ResponseAllOfDataInnerAllOfAttributes,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AccountRow {
    pub address: String,
}

impl Into<GetAccountList200ResponseAllOfDataInner> for AccountRow {
    fn into(self) -> GetAccountList200ResponseAllOfDataInner {
        GetAccountList200ResponseAllOfDataInner {
            id: self.address.clone(),
            r#type: Default::default(),
            attributes: Box::new(GetAccountList200ResponseAllOfDataInnerAllOfAttributes {
                address: self.address,
            }),
        }
    }
}
