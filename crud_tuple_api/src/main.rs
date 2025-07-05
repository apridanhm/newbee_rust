use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{fs, sync::Mutex};

const DATA_FILE: &str = "data.json";

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
}

struct AppState {
    items: Mutex<Vec<Item>>,
}

/* ---------- helpers ---------- */

fn load_items() -> Vec<Item> {
    match fs::read_to_string(DATA_FILE) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
        Err(_) => Vec::new(), // file belum ada
    }
}

fn save_items(items: &Vec<Item>) {
    if let Ok(json) = serde_json::to_string_pretty(items) {
        let _ = fs::write(DATA_FILE, json);
    }
}

/* ---------- handlers ---------- */

#[get("/items")]
async fn list(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

#[get("/items/{id}")]
async fn get_item(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let items = data.items.lock().unwrap();
    match items.iter().find(|item| item.id == id) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body("Not found"),
    }
}

#[post("/items")]
async fn create(new_item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();

    if items.iter().any(|it| it.id == new_item.id) {
        return HttpResponse::BadRequest().body("ID already exists");
    }

    items.push(new_item.into_inner());
    save_items(&items);
    HttpResponse::Created().body("Item created")
}

#[put("/items/{id}")]
async fn update(
    path: web::Path<u32>,
    new_item: web::Json<Item>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    match items.iter_mut().find(|it| it.id == id) {
        Some(slot) => {
            *slot = new_item.into_inner();
            save_items(&items);
            HttpResponse::Ok().body("Item updated")
        }
        None => HttpResponse::NotFound().body("Not found"),
    }
}

#[delete("/items/{id}")]
async fn delete_item(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.lock().unwrap();
    let before = items.len();
    items.retain(|it| it.id != id);
    if items.len() == before {
        HttpResponse::NotFound().body("Not found")
    } else {
        save_items(&items);
        HttpResponse::NoContent().finish()
    }
}

#[post("/reload")]
async fn reload_data(data: web::Data<AppState>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    *items = load_items(); // replace dari file
    HttpResponse::Ok().body("Reloaded from data.json")
}

/* ---------- main ---------- */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                items: Mutex::new(load_items()),
            }))
            .service(list)
            .service(get_item)
            .service(create)
            .service(update)
            .service(delete_item)
            .service(reload_data)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
