use std::string;
use std::collections::HashMap;

use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32, 
    email: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}
#[tokio::main]
async fn main() {
    let user = User {
        name: "Sarah".to_string(),
        age: 30,
        email: "sarahdorothy@gmail.com".to_string()
    };

    let firebase = Firebase::new("https://console.firebase.google.com/u/0/project/crud-uno-42271/database/crud-uno-42271-default-rtdb/data/~2F").unwrap();

    let response = set_user(&firebase, &user).await;

    let mut users = get_users(&firebase, &response.name).await;
    println!("{:?}", user);

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    user.email = "updateuserdata@gmail.com".to_string();
    let update_user = update_user(&firebase, &response.name, &user);

    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("user deleted");


}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}
 async fn get_users() -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
    return users.unwrap();  
}
async fn get_user() -> User {
    let firebase = firebase_client.at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}
async fn update_user() -> User {

}
async fn delete_user() {

}

// convert a string to a response
fn string_to_response(s: &str) -> Response {

}