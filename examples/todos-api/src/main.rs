use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx_query_as_builder::query_as_builder;
use structural_typing::{select, structural};

#[structural]
#[derive(Debug, Serialize, Deserialize)]
struct Project<T: todo::Fields = select!(todo: all-)> {
    id: i64,
    name: String,
    description: String,
    todos: Vec<Todo<T>>,
}

#[structural]
#[derive(Debug, Serialize, Deserialize)]
struct Todo<P: project::Fields = select!(project: all-)> {
    id: i64,
    #[nested(project: id, name, description)]
    project: Project<P>,
    title: String,
    completed: bool,
}

type CreateProject = Project<select!(project: name, description?)>;
type ProjectId = Project<select!(project: id)>;
type ProjectWithTodos = Project<select!(project: description?, all), select!(todo: project-, all)>;

type ProjectBasicFields = select!(project: description?, todos-, all);
type ProjectBasic = Project<ProjectBasicFields>;

type CreateTodo = Todo<select!(todo: title, project), select!(project: id)>;
type UpdateTodo = Todo<select!(todo: title?, completed?)>;
type TodoId = Todo<select!(todo: id)>;
type TodoWithProject = Todo<select!(todo: all), ProjectBasicFields>;

async fn create_project(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProject>,
) -> Result<(StatusCode, Json<ProjectBasic>), StatusCode> {
    let project = query_as_builder!(
        project::empty(),
        "INSERT INTO projects (name, description) VALUES (?, ?) RETURNING *",
        payload.name,
        payload.description
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(project)))
}

#[derive(Serialize)]
struct ListProjects {
    projects: Vec<ProjectBasic>,
}

async fn list_projects(State(pool): State<SqlitePool>) -> Result<Json<ListProjects>, StatusCode> {
    let projects = query_as_builder!(project::empty(), "SELECT * FROM projects")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ListProjects { projects }))
}

async fn get_project(
    State(pool): State<SqlitePool>,
    Path(Project { id, .. }): Path<ProjectId>,
) -> Result<Json<ProjectWithTodos>, StatusCode> {
    let project = query_as_builder!(project::empty(), "SELECT * FROM projects WHERE id = ?", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let todos = query_as_builder!(
        todo::empty(),
        "SELECT id, title, completed FROM todos WHERE project_id = ?",
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project_with_todos = project.todos(todos);

    Ok(Json(project_with_todos))
}

async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<TodoWithProject>), StatusCode> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let inserted = query_as_builder!(
        todo::empty().project(project::empty()),
        r#"INSERT INTO todos (title, project_id, completed) VALUES (?, ?, 0)
           RETURNING id as "id!", project_id, title, completed"#,
        payload.title,
        payload.project.id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project = query_as_builder!(
        project::empty(),
        r#"SELECT * FROM projects WHERE id = ?"#,
        inserted.project.id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(inserted.project(project))))
}

#[derive(Serialize)]
struct ListTodos {
    todos: Vec<TodoWithProject>,
}

async fn list_todos(State(pool): State<SqlitePool>) -> Result<Json<ListTodos>, StatusCode> {
    let todos = query_as_builder!(
        todo::empty().project(project::empty()),
        r#"SELECT
            todos.*,
            projects.name as project_name,
            projects.description as project_description
         FROM todos
         JOIN projects ON todos.project_id = projects.id"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ListTodos { todos }))
}

async fn get_todo(
    State(pool): State<SqlitePool>,
    Path(Todo { id, .. }): Path<TodoId>,
) -> Result<Json<TodoWithProject>, StatusCode> {
    let todo = query_as_builder!(
        todo::empty().project(project::empty()),
        r#"SELECT
            todos.*,
            projects.name as project_name,
            projects.description as project_description
         FROM todos
         JOIN projects ON todos.project_id = projects.id
         WHERE todos.id = ?"#,
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
) -> Result<Json<TodoWithProject>, StatusCode> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let updated = query_as_builder!(
        todo::empty().project(project::empty()),
        r#"UPDATE todos
           SET title = COALESCE(?, title),
               completed = COALESCE(?, completed)
           WHERE id = ?
           RETURNING *"#,
        payload.title,
        payload.completed,
        id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let project = query_as_builder!(
        project::empty(),
        r#"SELECT * FROM projects WHERE id = ?"#,
        updated.project.id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated.project(project)))
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
        .route("/projects", get(list_projects).post(create_project))
        .route("/projects/{id}", get(get_project))
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
