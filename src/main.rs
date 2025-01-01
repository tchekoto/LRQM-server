use std::{sync::{Arc, Mutex}, thread};

use axum::Router;
use db::DbApi;
mod api_users;
mod api_events;
mod models;
mod db;
pub mod schema;


struct AppState {
    db: db::PostgresDb,
}

type SharedState = Arc<Mutex<AppState>>;


#[tokio::main]
async fn main() {
    let mut db = db::PostgresDb  {
        connection: None,
    };
        

    db = thread::spawn(move || {
        db.connect();
        db
    }).join().unwrap();

    let shared_state = Arc::new(Mutex::new(AppState {
        db : db,
    }));

    let router = Router::new()
        .merge(api_events::stage(shared_state.clone()))
        .merge(api_users::stage(shared_state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
