use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateProductDto {
    pub event_id: String,
    pub vendor_id: String,
    pub name: String,
    pub description: String,
    pub points: i32,
    pub initial_quantity: Option<i32>,
    pub quantity_limit: Option<bool>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub points: Option<i32>,
    pub initial_quantity: Option<i32>,
    pub quantity_limit: Option<bool>,
    pub modified_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SoftDeleteProductDto {
    pub deleted_by: Option<String>,
}
