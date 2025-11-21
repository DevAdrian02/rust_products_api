# Rust Products API

API REST desarrollada en Rust para la gestión de inventario de productos. Este proyecto implementa operaciones CRUD completas, validaciones de negocio y autenticación mediante Bearer Token, utilizando **Actix-Web** y **SeaORM** con **PostgreSQL**.

## 🚀 Tecnologías Utilizadas

* **Lenguaje:** Rust
* **Framework Web:** Actix-Web
* **ORM:** SeaORM
* **Base de Datos:** PostgreSQL
* **Serialización:** Serde / Serde JSON
* **Runtime:** Tokio

## ⚙️ Configuración del Proyecto

### Prerrequisitos
* Rust y Cargo instalados.
* PostgreSQL (local o vía Docker).
* SeaORM CLI (`cargo install sea-orm-cli`).

### Variables de Entorno
Crear un archivo `.env` en la raíz del proyecto con la siguientes configuraciones:

```env
# Configuración de Base de Datos
DATABASE_URL=postgres://usuario:password@localhost:5432/nombre_db

# Configuración del Servidor
HOST=127.0.0.1
PORT=8080
RUST_LOG=debug

# Seguridad
AUTH_TOKEN=mi_token_secreto_super_seguro
