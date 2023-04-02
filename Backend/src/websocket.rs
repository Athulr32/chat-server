use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    Extension,
};
use futures_util::{lock::Mutex, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast;

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;

use sha2::Sha256;
use std::collections::BTreeMap;

use mongodb::{
    bson::{doc, to_document, Document},
    Database,
};


//User message
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct UserMessage {
    cipher:String,
    public_key: String,
}



#[derive(Deserialize, Debug, Serialize)]
struct SocketAuth {
    token: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct UserMessageExtend {
    user_message: UserMessage,
    from: String,
}

pub async fn ws_handler(
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
  let handler =  tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("{msg}");
            let send_to_client = sender.send(Message::Text(msg)).await;

            if send_to_client.is_err() {

                //If sending failed Add the message to database
                
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
                        tx.send("Invalid Key Format".to_string()).unwrap();
                        continue;
                    }
                } else {

                    //Client sent invalid format

                    tx.send("Invalid".to_string()).unwrap();
                }
            } else {

                //User message
                let get_msg: Result<UserMessage, serde_json::Error> = serde_json::from_str(&msg);

                if let Err(e) = get_msg {
                    //Client sent invalid format
                    tx.send("Error in decoding".to_string()).unwrap();
                    continue;
                }

                let user_message = get_msg.unwrap();

                //Reciever public key
                let rec_pubkey =user_message.public_key.clone();

                let foo = state.lock().await;

                //Check whether reciever is online
                let tr = foo.get(&rec_pubkey);  

                //Creating a message to send to reciever
                let user_message_extended = UserMessageExtend {
                    from: pk.clone(),
                    user_message,
                };

                if let None = tr {
                    //If user is not online 
                    //Add to database
                    let message_collection = db.collection::<Document>("messages");
                    let docs = to_document(&user_message_extended).unwrap();
                    message_collection.insert_one(docs, None).await.unwrap();

                    tx.send("User offline".to_string()).unwrap();
                } else if let Some(tr) = tr {
                    //If user is online
                    //Send message to user

                    let send_message =
                        tr.send(serde_json::to_string(&user_message_extended).unwrap());

                    if let Err(e) = send_message {

                        let _ = tx.send("Error in sending try again".to_string());

                        //Save to database
                        let message_collection = db.collection::<Document>("messages");
                        let docs = to_document(&user_message_extended).unwrap();
                        message_collection.insert_one(docs, None).await.unwrap();
                    } else {
                        tx.send("Message sent".to_string()).unwrap();
                    }
                }
            }
        }

        let mut foo = state.lock().await;

        foo.remove(&pk[..]);
        handler.abort();
        println!("Disconnected");
    });
}
