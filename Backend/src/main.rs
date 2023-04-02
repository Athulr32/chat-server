use axum::{
    routing::{get, post},
    Extension, Router,
};
use futures_util::{lock::Mutex};

use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast;

use chat_server::login::{login};


use mongodb::{
    bson::{doc, to_document, Document},
    options::{ClientOptions, FindOptions},
    Client, Database,
};
use chat_server::get_message::get_message;
use chat_server::websocket::ws_handler;




#[tokio::main]
async fn main() {
    let state: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let  client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();


    let db = client.database("ism");


    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/login", post(login))
        .route("/getMessage", get(get_message))
        .layer(Extension(state))
        .with_state(db);

    axum::Server::bind(&"127.0.0.1:3011".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
