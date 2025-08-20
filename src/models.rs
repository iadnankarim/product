use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub event_id: Uuid,
    pub vendor_id: Uuid,
    pub name: String,
    pub description: String,
    pub points: i32,
    pub initial_quantity: Option<i32>,
    pub quantity_limit: Option<bool>,
    pub status_id: Uuid,
    pub created_date: NaiveDateTime,
    pub created_by: Uuid,
    pub modified_date: Option<NaiveDateTime>,
    pub modified_by: Option<Uuid>,
    pub deleted_date: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}
