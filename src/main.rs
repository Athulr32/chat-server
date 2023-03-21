use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    routing::{get, post},
    Extension, Router,
};
use futures_util::{lock::Mutex, stream::SplitSink, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast;

use chat_server::login::login;

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;

use sha2::Sha256;
use std::collections::BTreeMap;

use mongodb::{
    bson::{doc, to_document, Document},
    options::ClientOptions,
    Client, Database,
};

struct Sock {
    socket: SplitSink<WebSocket, Message>,
}

//User message
#[derive(Serialize, Deserialize, Clone)]
struct UserMessage {
    iv: Vec<u8>,
    ephemPublicKey: Vec<u8>,
    ciphertext: Vec<u8>,
    mac: Vec<u8>,
    public_key: Vec<u8>,
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    pk: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct SocketAuth {
    token: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct UserMessageExtend {
    user_message: UserMessage,
    from: String,
}

async fn handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>>,
    State(db): State<Database>,
) -> Response {
    //upgrade the websocket connection
    ws.on_failed_upgrade(|_| {})
        .on_upgrade(move |socket| handle_socket(socket, state, db))
}

async fn handle_socket(
    socket: WebSocket,
    state: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
    db: Database,
) {
    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = broadcast::channel(100);

    //Send message to user itself
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("{msg}");
            let send_to_client = sender.send(Message::Text(msg)).await;

            if send_to_client.is_err() {
                break;
            }
        }
    });

    //Wait for message from user
    tokio::spawn(async move {
        let mut auth = false;
        let mut pk = String::from("");

        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            println!("{}", msg);

            if !auth {
                let foo: Result<SocketAuth, serde_json::Error> = serde_json::from_str(&msg);

                if let Ok(auth_details) = foo {
                    //Check if details are correct
                    //If yes Add to authenticated pool
                    //Add the public key and channel

                    let token = auth_details.token.to_string();

                    let key: Hmac<Sha256> = Hmac::new_from_slice(b"wtsefhkjvsfvshkn").unwrap();

                    let claims: Result<BTreeMap<String, String>, jwt::Error> =
                        token.verify_with_key(&key);

                    if let Ok(claim) = claims {
                        pk = claim["key"].to_string();
                        let mut muttex = state.lock().await;
                        muttex.insert(pk.to_string(), tx.clone());

                        auth = true;

                        tx.send("Authenticated".to_string()).unwrap();
                    } else {
                        tx.send("Invalid Key".to_string()).unwrap();
                    }
                } else {
                    tx.send("Invalid".to_string()).unwrap();
                }
            } else {
                //User message
                let get_msg: Result<UserMessage, serde_json::Error> = serde_json::from_str(&msg);

                if let Err(e) = get_msg {
                    tx.send("Error in decoding".to_string()).unwrap();
                    break;
                }
                let user_message = get_msg.unwrap();

                let rec_pubkey = hex::encode(&user_message.clone().public_key);

                let foo = state.lock().await;

                let tr = foo.get(&rec_pubkey);

                let user_message_extended = UserMessageExtend {
                    from: pk.clone(),
                    user_message,
                };

                if let None = tr {
                    
                    //Add to database
                    let message_collection = db.collection::<Document>("messages");
                    let docs = to_document(&user_message_extended).unwrap();
                    message_collection.insert_one(docs, None).await.unwrap();

                    println!("{}",serde_json::to_string(&user_message_extended).unwrap());

                    tx.send("User offline".to_string()).unwrap();
                } else if let Some(tr) = tr {
                    //Send message to user
                    let send_message = tr.send(serde_json::to_string(&user_message_extended).unwrap());

                    if let Err(e) = send_message {
                        let _ = tx.send("Error in sending try again".to_string());
                    } else {
                        tx.send("Message sent".to_string()).unwrap();
                    }
                }
            }
        }

        let mut foo = state.lock().await;

        foo.remove(&pk[..]);

        println!("Disss");
    });
}

struct DBMessage {
    messages: Vec<UserMessage>,
}

async fn get_message() {}

#[tokio::main]
async fn main() {
    let state: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();

    // for db_name in client.list_database_names(None,None).await.unwrap(){
    //     println!("{}",db_name);
    // }

    let db = client.database("ism");

    //Collection
    let user_collection = db.collection::<Document>("users");
    let message_collection = db.collection::<Document>("messages");

    let docs = doc! {"public_key":"wedjcn","name":"Athul","rfsd":"rfsd"};

    user_collection.insert_one(docs, None).await.unwrap();

    let app = Router::new()
        .route("/ws", get(handler))
        .route("/login", post(login))
        .layer(Extension(state))
        .with_state(db);

    axum::Server::bind(&"127.0.0.1:3011".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
