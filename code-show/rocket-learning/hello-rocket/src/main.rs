use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::{catch, catchers, delete, get, post, put, routes, State};
use std::collections::HashMap;

// https://rocket.rs/guide/v0.5/getting-started/#getting-started

// #################################################
// state
// #################################################
// map -> mutex -> state
type UserItems = Mutex<HashMap<usize, User>>;
type Messages<'r> = &'r State<UserItems>;

// #################################################
// struct: req, resp; model, db
// #################################################
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct User {
    id: usize,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserVO {
    id: usize,
    username: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserDTO {
    username: String,
    password: String,
    check_password: String,
}

// #################################################
// catch: error + record
// #################################################
#[catch(404)]
fn not_found() -> Value {
    json!({
        "res":"404 Not Found!"
    })
}

// #################################################
// health
// >> (health) GET /health/check
// >> (health_json) GET /health/json/<id>
// #################################################
#[get("/check")]
async fn health() -> String {
    "Health OK!".to_string()
}

#[get("/json/<id>")]
async fn health_json(id: usize) -> Value {
    json!({
        "id": id,
        "res": "Health JSON OK!",
    })
}

// #################################################
// user
// >> (get_users) GET /user/list
// >> (get_user) GET /user/current/<id>
// >> (post_user) POST /user/register
// >> (put_user) PUT /user/edit/<id>
// >> (delete_user) DELETE /user/delete/<_id>
// #################################################
#[get("/list")]
async fn get_users() -> Json<Vec<UserVO>> {
    // db: select * from user where ...
    let users = vec![
        UserVO {
            id: 1,
            username: "oswin".to_string(),
        },
        UserVO {
            id: 2,
            username: "yuanzi".to_string(),
        },
    ];
    Json(users)
}

#[get("/current/<id>")]
async fn get_user(id: usize, messages: Messages<'_>) -> Json<UserVO> {
    let user_map = messages.lock().await; // unlock

    // id违法
    if id < 1 {
        return Json(UserVO {
            id: 0,
            username: "_".to_string(),
        });
    }

    match user_map.get(&id) {
        // 没找到指定id
        None => Json(UserVO {
            id: 0,
            username: "".to_string(),
        }),
        // 找到了正常返回
        Some(user) => Json(user.to_owned()),
    }

    // db: select * from user where id = id
    // Json(UserVO {
    //     id: id,
    //     username: "oswin".to_string(),
    // })
}

#[post("/register", format = "json", data = "<user>")]
async fn post_user(user: Json<UserDTO>) -> Option<Json<UserVO>> {
    let user = user.into_inner();
    println!(
        "user: {}, {}, {}.",
        user.username, user.password, user.check_password
    );

    // verify
    // db

    Some(Json(UserVO {
        id: 12,
        username: user.username,
    }))
}

#[put("/edit/<id>")]
async fn put_user(id: usize) -> Value {
    json!({
        "res":format!("User {} Put OK!", id)
    })
}

#[delete("/delete/<_id>")]
async fn delete_user(_id: usize) -> Value {
    json!({
        "res":"User _id Delete OK!"
    })
}

// #################################################
// rocket main
// #################################################
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rocket = rocket::build()
        // routes
        .mount("/health", routes![health, health_json])
        .mount(
            "/user",
            routes![get_user, get_users, post_user, put_user, delete_user],
        )
        // catch
        .register("/", catchers![not_found])
        // state
        .manage(UserItems::new(HashMap::new()));

    rocket.launch().await?;
    Ok(())
}
