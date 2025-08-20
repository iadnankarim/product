// use crate::dto::{CreateProductDto, SoftDeleteProductDto, SoftDeleteProductDto, UpdateProductDto};
use crate::dto::{CreateProductDto, SoftDeleteProductDto, UpdateProductDto};
use crate::models::Product;
use actix_web::{HttpResponse, delete, get, post, put, web};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[post("/product")]
pub async fn create_product(
    pool: web::Data<PgPool>,
    dto: web::Json<CreateProductDto>,
) -> HttpResponse {
    // Parse UUIDs from DTO
    let event_id = match Uuid::parse_str(&dto.event_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid event_id"),
    };
    let vendor_id = match Uuid::parse_str(&dto.vendor_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid vendor_id"),
    };
    let created_by = match &dto.created_by {
        Some(id_str) => match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return HttpResponse::BadRequest().body("Invalid created_by"),
        },
        None => Uuid::nil(),
    };

    let now = Utc::now().naive_utc();

    // Fetch default status_id for "PUBLISHED"
    let status_id = match sqlx::query_scalar::<_, Uuid>("SELECT id FROM status WHERE name = $1")
        .bind("PUBLISHED")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get default status"),
    };

    // Insert product
    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO product
        (event_id, vendor_id, name, description, points, initial_quantity, quantity_limit, status_id, created_date, created_by)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
        RETURNING *
        "#
    )
    .bind(event_id)
    .bind(vendor_id)
    .bind(&dto.name)
    .bind(&dto.description)
    .bind(dto.points)
    .bind(dto.initial_quantity)
    .bind(dto.quantity_limit)
    .bind(status_id) // valid status_id
    .bind(now)
    .bind(created_by)
    .fetch_one(pool.get_ref())
    .await;

    match product {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

#[get("/products")]
pub async fn get_all_products(pool: web::Data<PgPool>) -> HttpResponse {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM product")
        .fetch_all(pool.get_ref())
        .await;

    match products {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

#[get("/product/{id}")]
pub async fn get_product_by_id(pool: web::Data<PgPool>, path: web::Path<String>) -> HttpResponse {
    let id = match uuid::Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid product id"),
    };

    let product = sqlx::query_as::<_, Product>("SELECT * FROM product WHERE id = $1")
        .bind(id)
        .fetch_one(pool.get_ref())
        .await;

    match product {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Product not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

// use actix_web::{put, web, HttpResponse};
// use chrono::Utc;
// use sqlx::PgPool;
// use uuid::Uuid;
// use crate::dto::UpdateProductDto;
// use crate::models::Product;

#[put("/product/{id}")]
pub async fn update_product(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    dto: web::Json<UpdateProductDto>,
) -> HttpResponse {
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid product id"),
    };

    let modified_by = match &dto.modified_by {
        Some(id_str) => match Uuid::parse_str(id_str) {
            Ok(id) => Some(id),
            Err(_) => return HttpResponse::BadRequest().body("Invalid modified_by"),
        },
        None => None,
    };

    let now = Utc::now().naive_utc();

    let updated = sqlx::query_as::<_, Product>(
        r#"
        UPDATE product
        SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            points = COALESCE($3, points),
            initial_quantity = COALESCE($4, initial_quantity),
            quantity_limit = COALESCE($5, quantity_limit),
            modified_date = $6,
            modified_by = $7
        WHERE id = $8 AND deleted_date IS NULL
        RETURNING *
        "#,
    )
    .bind(&dto.name)
    .bind(&dto.description)
    .bind(dto.points)
    .bind(dto.initial_quantity)
    .bind(dto.quantity_limit)
    .bind(now)
    .bind(modified_by)
    .bind(id)
    .fetch_one(pool.get_ref())
    .await;

    match updated {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().body("Product not found or already deleted")
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

#[delete("/product/{id}")]
pub async fn soft_delete_product(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    dto: web::Json<SoftDeleteProductDto>,
) -> HttpResponse {
    // Parse product ID
    let id = match Uuid::parse_str(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid product id"),
    };

    // Parse deleted_by UUID if provided
    let deleted_by = match &dto.deleted_by {
        Some(id_str) => match Uuid::parse_str(id_str) {
            Ok(id) => Some(id),
            Err(_) => return HttpResponse::BadRequest().body("Invalid deleted_by"),
        },
        None => None,
    };

    let now = Utc::now().naive_utc();

    // First, check if product exists and is not already deleted
    let existing = sqlx::query_as::<_, Product>(
        "SELECT * FROM product WHERE id = $1 AND deleted_date IS NULL",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match existing {
        Ok(Some(_)) => {
            // Soft delete the product
            let deleted = sqlx::query_as::<_, Product>(
                r#"
                UPDATE product
                SET deleted_date = $1, deleted_by = $2
                WHERE id = $3
                RETURNING *
                "#,
            )
            .bind(now)
            .bind(deleted_by)
            .bind(id)
            .fetch_one(pool.get_ref())
            .await;

            match deleted {
                Ok(p) => HttpResponse::Ok().json(p),
                Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Product not found or already deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}
