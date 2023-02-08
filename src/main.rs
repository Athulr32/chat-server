use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use futures_util::{lock::Mutex, Sink, SinkExt, StreamExt, stream::SplitSink};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use redis::Commands;
use sha2::Sha256;
use std::time::{Duration, SystemTime};
use std::{
    collections::{BTreeMap, HashMap},
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::broadcast;
use tokio::time::error::Error;

use serde::{Deserialize, Serialize};


struct Sock{
    socket:SplitSink<WebSocket,Message>
}


#[derive(Serialize)]
struct JWT {
    key: String,
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    pk: String,
}

#[derive(Deserialize, Debug,Serialize)]
struct SocketAuth {
    jwt: String,
}

async fn handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>>,
) -> Response {
    //upgrade the websocket connection
    ws.on_failed_upgrade(|_| {})
        .on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(
    socket: WebSocket,
    state: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
) {
    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = broadcast::channel(100);


    //Send message to other user
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let _ = sender.send(Message::Text(msg)).await;
        }
    });


    //Wait for message from user
    tokio::spawn(async move {
        let mut auth = false;
        let mut pk = String::from("");

        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            if !auth {
                let foo: Result<SocketAuth, serde_json::Error> = serde_json::from_str(&msg);

                if let Ok(auth_details) = foo {
                    //Check if details are correct
                    //If yes Add to authenticated pool
                    //Add the public key and channel


                    pk = auth_details.jwt.to_string();

                    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
                    let mut con = client.get_connection().unwrap();
                  A
                    let ser = serde_json::to_string(&auth_details).unwrap();

    

             

                    let mut muttex = state.lock().await;
                    muttex.insert(auth_details.jwt, tx.clone());
                    auth = true;
                } else {
                    tx.send("Invalid".to_string()).unwrap();
                }
            } else {
                let foo = state.lock().await;
                println!("{:?}", foo);

                let tr = foo.get(msg.trim()).unwrap();

                let _ = tr.send("You got it".to_string()).unwrap();
            }
        }

        let mut foo = state.lock().await;

        foo.remove(&pk[..]);

        println!("Disss");
    });
}

async fn get_token(Json(data): Json<UserInfo>) -> Json<JWT> {

    let system_time = SystemTime::now();

    println!("{:?}", system_time);
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"wtsefhkjvsfvshkn").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");

    let token_str = claims.sign_with_key(&key).unwrap();

    Json(JWT { key: token_str })
}

// impl Deref for Details {
//     type Target = HashMap<String, broadcast::Sender<String>>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for Details {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

#[tokio::main]
async fn main() {
    let state: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/ws", get(handler))
        .route("/token", post(get_token))
        // .route("/check1", get(sample1))
        // .route("/check2", get(sample2))
        .layer(Extension(state));

    axum::Server::bind(&"127.0.0.1:3010".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
