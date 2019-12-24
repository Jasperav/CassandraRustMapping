use uuid::Uuid;
use cdrs::query::QueryValues;
use cdrs::types::prelude::*;
use crate::cdrs::types::from_cdrs::FromCDRS;
use crate::cdrs::frame::IntoBytes;

#[derive(Debug, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
struct Car {
    user_id: Uuid,
    password: String,
    salt: String,
    ts_created: i64,
    username: String,
}