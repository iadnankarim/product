//handler/mod.rs
pub mod product;

pub use product::{
    create_product, get_all_products, get_product_by_id, soft_delete_product, update_product,
};
