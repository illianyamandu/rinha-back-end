use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use time::{macros::date, Date};
use tokio::sync::RwLock;
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize)]
struct Person {
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct PersonName(String);

impl TryFrom<String> for PersonName {
    type Error = &'static str;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.len() > 100 {
            return Err("name too long");
        }

        Ok(PersonName(name))
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct Nick(String);

impl TryFrom<String> for Nick {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 32 {
            return Err("nick too long");
        }

        Ok(Self(value))
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct Tech(String);

impl TryFrom<String> for Tech {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 32 {
            return Err("tech too long");
        }

        Ok(Self(value))
    }
}

impl From<Tech> for String {
    fn from(value: Tech) -> Self {
        value.0
    }
}

#[derive(Clone, Deserialize)]
struct NewPerson {
    #[serde(rename = "nome")]
    pub name: PersonName,
    #[serde(rename = "apelido")]
    pub nick: Nick,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<Tech>>,
}

type AppState = Arc<RwLock<HashMap<Uuid, Person>>>;

#[tokio::main]
async fn main() {
    let mut people: HashMap<Uuid, Person> = HashMap::new();

    let person = Person {
        id: Uuid::now_v7(),
        name: "João".to_string(),
        nick: "Joãozinho".to_string(),
        birth_date: date!(1990 - 1 - 1),
        stack: None, //vec!["C++".to_string(), "Rust".to_string()],
    };

    println!("{}", person.id);

    people.insert(person.id, person);

    let app_state: AppState = Arc::new(RwLock::new(people));

    // build our application with a single route
    let app = Router::new()
        .route("/pessoas", get(search_people))
        .route("/pessoas/:id", get(find_person))
        .route("/contagem-pessoas", get(count_people))
        .route("/pessoas", post(create_person))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_people() -> impl IntoResponse {
    (StatusCode::OK, "Busca Pessoas")
}

async fn find_person(
    State(people): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> impl IntoResponse {
    match people.read().await.get(&person_id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse {
    let id = Uuid::now_v7();
    let person = Person {
        id,
        name: new_person.name.0,
        nick: new_person.nick.0,
        birth_date: new_person.birth_date,
        stack: new_person
            .stack
            .map(|stack| stack.into_iter().map(String::from).collect()),
    };

    people.write().await.insert(id, person.clone());

    (StatusCode::OK, Json(person))
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    let count = people.read().await.len();
    (StatusCode::OK, Json(count))
}
