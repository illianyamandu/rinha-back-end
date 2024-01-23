use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use time::Date;
use uuid::Uuid;

struct Person {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birth_date: Date,
    pub stach: Vec<String>,
}

#[tokio::main]
async fn main() {
    let people: HashMap<Uuid, Person> = HashMap::new();

    // build our application with a single route
    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/contagem-pessoas", get(count_people))
        .route("/pessoas", post(create_person));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_people() -> impl IntoResponse {
    (StatusCode::OK, "Busca Pessoas")
}

async fn find_person() -> impl IntoResponse {
    (StatusCode::OK, "Find")
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::OK, "Person")
}

async fn count_people() -> impl IntoResponse {
    (StatusCode::OK, "Count")
}
