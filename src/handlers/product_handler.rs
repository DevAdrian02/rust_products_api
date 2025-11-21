use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, DatabaseConnection};
use crate::entity::producto; // Contiene producto::ActiveModel
use crate::entity::prelude::Producto; // Contiene Producto::find(), Producto::find_by_id(), etc.
use crate::models::product_dto::{CreateProductDto, UpdateProductDto};
use log;

// POST /producto (Crear producto)
#[post("/producto")] // Usar /producto
async fn create_product(
    db: web::Data<DatabaseConnection>,
    item: web::Json<CreateProductDto>,
) -> impl Responder {
    // Validaciones
    if item.nombre.is_empty() {
        return HttpResponse::BadRequest().body("El nombre no puede estar vacío");
    }
    if item.precio <= 0 {
        return HttpResponse::BadRequest().body("El precio debe ser mayor a 0");
    }
    if item.stock < 0 {
        return HttpResponse::BadRequest().body("El stock debe ser mayor o igual a 0");
    }

    // ActiveModel para insertar
    let new_product = producto::ActiveModel {
        nombre: Set(item.nombre.clone()),
        precio: Set(item.precio),
        stock: Set(item.stock),
        ..Default::default()
    };

    match new_product.insert(db.get_ref()).await {
        Ok(inserted) => HttpResponse::Created().json(serde_json::json!(inserted)),
        Err(e) => {
            log::error!("Error de serialización final: {:?}", e);
            HttpResponse::InternalServerError().body("Error interno al crear producto")
        },
    }
}

// GET /productos (Listado de productos)
#[get("/productos")]
async fn get_products(db: web::Data<DatabaseConnection>) -> impl Responder {
    match Producto::find().all(db.get_ref()).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// GET /producto/{id} (Obtener por ID)
#[get("/producto/{id}")]
async fn get_product_by_id(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match Producto::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(p)) => HttpResponse::Ok().json(p),
        Ok(None) => HttpResponse::NotFound().body("ID inexistente (404)"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// PUT /producto/{id} (Actualizar producto)
#[put("/producto/{id}")]
async fn update_product(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    item: web::Json<UpdateProductDto>,
) -> impl Responder {
    let id = path.into_inner();

    match Producto::find_by_id(id).one(db.get_ref()).await {
        Ok(Some(p)) => {
            let mut active_model: producto::ActiveModel = p.into();
            //Validaciones
            if let Some(nombre) = item.nombre.clone() {
                if nombre.is_empty() { return HttpResponse::BadRequest().body("Nombre no puede estar vacío"); }
                active_model.nombre = Set(nombre);
            }

            if let Some(precio) = item.precio {
                if precio <= 0 { return HttpResponse::BadRequest().body("Precio debe ser mayor a 0"); }
                active_model.precio = Set(precio);
            }
            if let Some(stock) = item.stock {
                if stock < 0 { return HttpResponse::BadRequest().body("Stock debe ser mayor o igual a 0"); }
                active_model.stock = Set(stock);
            }

            // Actualizar en la base de datos
            match active_model.update(db.get_ref()).await {
                Ok(updated) => HttpResponse::Ok().json(updated),
                Err(_) => HttpResponse::InternalServerError().body("Error interno al actualizar"),
            }
        },
        Ok(None) => HttpResponse::NotFound().body("Producto no encontrado para actualizar (404)"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// DELETE /producto/{id} (Eliminar producto)
#[delete("/producto/{id}")]
async fn delete_product(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match Producto::delete_by_id(id).exec(db.get_ref()).await {
        Ok(res) => {
            if res.rows_affected == 0 {
                HttpResponse::NotFound().body("Producto no encontrado para eliminar (404)")
            } else{
                HttpResponse::Ok().body("Producto eliminado")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Error interno al eliminar"),
    }
}

