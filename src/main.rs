use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/pessoas", get(|| async { "Busca Pessoas" }))
        .route("/pessoas/:id", get(|| async { "Pessoa por id" }))
        .route("/contagem-pessoas", get(|| async { "Contar Pessoas" }))
        .route("/pessoas", post(|| async { "Criar pessoas" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
