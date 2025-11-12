use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use structural_typing::{select, structural};

#[structural]
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
    completed: bool,
}

type CreateTodo = Todo<select!(todo: title)>;
type UpdateTodo = Todo<select!(todo: ?title, ?completed)>;
type TodoId = Todo<select!(todo: id)>;

async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title) VALUES (?) RETURNING *",
        payload.title
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[derive(Serialize)]
struct ListTodos {
    todos: Vec<Todo>
}

async fn list_todos(State(pool): State<SqlitePool>) -> Result<Json<ListTodos>, StatusCode> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json( ListTodos { todos } ))
}

async fn get_todo(
    State(pool): State<SqlitePool>,
    Path(Todo { id, .. }): Path<TodoId>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

async fn update_todo(
    State(pool): State<SqlitePool>,
    Path(Todo { id, .. }): Path<TodoId>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        "UPDATE todos
         SET title = COALESCE(?, title),
             completed = COALESCE(?, completed)
         WHERE id = ?
         RETURNING *",
        payload.title,
        payload.completed,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(Todo { id, .. }): Path<TodoId>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:examples/todos-api/todos.db")
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
