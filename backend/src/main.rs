use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[get("/tasks")]
async fn read_tasks() -> impl Responder {
    HttpResponse::Ok().body("All tasks")
}

#[get("/tasks/{id}")]
async fn read_task(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Task detail for id: {}", id))
}

#[post("/tasks")]
async fn create_task() -> impl Responder {
    HttpResponse::Ok().body("Task created")
}

#[put("/tasks/{id}")]
async fn update_task(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Task updated for id: {}", id))
}

#[delete("/tasks/{id}")]
async fn delete_task(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Task deleted for id: {}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(read_tasks)
            .service(read_task)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
