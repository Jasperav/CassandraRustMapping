use uuid::Uuid;
use cdrs::query::QueryValues;
use cdrs::types::prelude::*;
use crate::cdrs::types::from_cdrs::FromCDRS;
use crate::cdrs::frame::IntoBytes;

#[derive(Debug, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
struct User {
    user_id: Uuid,
    password: String,
    salt: String,
    ts_created: i64,
    username: String,
}

impl User {
    fn into_query_values(self) -> QueryValues {
        query_values!(
        "user_id" => self.user_id
           )
    }
}
