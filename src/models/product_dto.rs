use serde::{Deserialize, Serialize};
#[derive( Serialize, Deserialize)]
pub struct CreateProductDto {
    pub nombre: String,
    pub precio : i32,
    pub stock :i32,

}

#[derive( Serialize, Deserialize)]
pub struct UpdateProductDto {
    pub nombre: Option<String>,
    pub precio: Option<i32>,
    pub stock: Option<i32>,

}