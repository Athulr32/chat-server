use std::time::{SystemTime, UNIX_EPOCH};

use axum::{Json, response::{IntoResponse,Response}, http::StatusCode};
use serde::{Serialize,Deserialize};
use secp256k1::{Secp256k1, ecdsa::Signature,PublicKey,Message};
use sha2::{Sha256,Digest};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use std::collections::BTreeMap;

//User login Details
#[derive(Serialize, Deserialize)]
pub struct LoginData {
    signature: Vec<u8>,
    recid: u8,
    message: String,
    pub_key: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct LoginError {
    msg: String,
}



#[derive(Serialize)]
pub struct JWT {
    token: String,
}

//To generate JWT TOKEN
pub async fn get_token(pub_key: &str) -> Json<JWT> {
    let system_time = SystemTime::now();
    println!("{:?}",pub_key);
    println!("{:?}", system_time);
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"wtsefhkjvsfvshkn").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("key", pub_key);

    let token_str = claims.sign_with_key(&key).unwrap();

    Json(JWT { token: token_str })
}


#[axum_macros::debug_handler]
//User login handler
pub async fn login(Json(data): Json<LoginData>) -> Result<Json<JWT>, MyError> {
    //Get digital Signature from user and verify
    //If correct issue token containing the public key and time
    print!("{:?}", data.pub_key);

    //Check if time is correct
    let time = SystemTime::now();
    let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    let secp256k1 = Secp256k1::new();

    let mut hasher = Sha256::new();
    hasher.update(&data.message);
    let result = hasher.finalize();

    let message = Message::from_slice(&result).unwrap();
    let signature = Signature::from_compact(&data.signature[..]).unwrap();
    let public_key = PublicKey::from_slice(&data.pub_key).unwrap();

    let res = secp256k1
        .verify_ecdsa(&message, &signature, &public_key)
        .is_ok();

    if res {
        println!("Correct");
        Ok(get_token(&hex::encode(&data.pub_key)).await)
    } else {
        println!("Incorrect");
        Err(MyError::SomethingElseWentWrong)
    }

    // if data.public_key.len() != 2{

    //         return Err(MyError::SomethingElseWentWrong);
    // }
    // else{

    //     Ok(Json(LoginError{
    //         msg:data.public_key
    //     }))
    // }

    // let v = data.as_object();
    // let foo = data.as_object().unwrap();
    //     println!("{:?}",foo);
    // let iv = foo
    //     .get("iv")
    //     .unwrap()
    //     .get("data")
    //     .unwrap()
    //     .as_array()
    //     .unwrap().to_owned();

    // let ephemPublicKey = foo
    //     .get("ephemPublicKey")
    //     .unwrap()
    //     .get("data")
    //     .unwrap()
    //     .as_array()
    //     .unwrap().to_owned();

    // let ciphertext = foo
    //     .get("ciphertext")
    //     .unwrap()
    //     .get("data")
    //     .unwrap()
    //     .as_array()
    //     .unwrap().to_owned();
    // let mac = foo
    //     .get("mac")
    //     .unwrap()
    //     .get("data")
    //     .unwrap()
    //     .as_array()
    //     .unwrap().to_owned();
}

pub enum MyError {
    SomethingWentWrong,
    SomethingElseWentWrong,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::SomethingWentWrong => "something went wrong",
            MyError::SomethingElseWentWrong => "something else went wrong",
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}