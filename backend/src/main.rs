use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

/// MySQL の tasks テーブルのレコードに対応する構造体。
/// sqlx::FromRow を derive することで、SQL の結果を自動で Task にマッピングできます。
#[derive(Serialize, Debug, sqlx::FromRow)]
struct Task {
    id: i32,
    description: String,
    is_completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// タスクの作成・更新時に受け取るデータ用構造体
#[derive(Deserialize)]
struct TaskInput {
    description: String,
    is_completed: bool,
}

/// GET /tasks
/// MySQL の tasks テーブルから全タスクを取得し、JSON として返す
#[get("/tasks")]
async fn read_tasks(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>(
        "SELECT id, description, is_completed, created_at, updated_at FROM tasks",
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(err) => {
            eprintln!("Error fetching tasks: {}", err);
            HttpResponse::InternalServerError().body("Error fetching tasks")
        }
    }
}

/// GET /tasks/{id}
/// 指定された ID のタスクを取得
#[get("/tasks/{id}")]
async fn read_task(pool: web::Data<MySqlPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query_as::<_, Task>(
        "SELECT id, description, is_completed, created_at, updated_at FROM tasks WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(err) => {
            eprintln!("Error fetching task {}: {}", id, err);
            HttpResponse::NotFound().body("Task not found")
        }
    }
}

/// POST /tasks
/// タスクを作成
#[post("/tasks")]
async fn create_task(pool: web::Data<MySqlPool>, new_task: web::Json<TaskInput>) -> impl Responder {
    let result = sqlx::query("INSERT INTO tasks (description, is_completed) VALUES (?, ?)")
        .bind(&new_task.description)
        .bind(new_task.is_completed)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(res) => {
            HttpResponse::Ok().body(format!("Task created with id: {}", res.last_insert_id()))
        }
        Err(err) => {
            eprintln!("Error creating task: {}", err);
            HttpResponse::InternalServerError().body("Error creating task")
        }
    }
}

/// PUT /tasks/{id}
/// タスクを更新
#[put("/tasks/{id}")]
async fn update_task(
    pool: web::Data<MySqlPool>,
    path: web::Path<i32>,
    update: web::Json<TaskInput>,
) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query("UPDATE tasks SET description = ?, is_completed = ? WHERE id = ?")
        .bind(&update.description)
        .bind(update.is_completed)
        .bind(id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Task updated for id: {}", id)),
        Err(err) => {
            eprintln!("Error updating task {}: {}", id, err);
            HttpResponse::InternalServerError().body("Error updating task")
        }
    }
}

/// DELETE /tasks/{id}
/// タスクを削除
#[delete("/tasks/{id}")]
async fn delete_task(pool: web::Data<MySqlPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Task deleted for id: {}", id)),
        Err(err) => {
            eprintln!("Error deleting task {}: {}", id, err);
            HttpResponse::InternalServerError().body("Error deleting task")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // docker-compose で起動した MySQL コンテナの接続情報に合わせてください
    let database_url = "mysql://myuser:password@127.0.0.1:3306/mydb";
    let pool = MySqlPool::connect(database_url)
        .await
        .expect("Failed to create database pool");

    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            // DB プールを各ハンドラに渡すために app_data として登録
            .app_data(web::Data::new(pool.clone()))
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
