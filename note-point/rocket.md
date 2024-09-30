# rocket

- Reference

  [Rocket](https://rocket.rs/)

  



## 背景介绍

### 版本介绍

- Rocket0.5之前

  支持nightly 不支持stable

  不支持async / await

- Rocket0.5

  实现：hyper + tokio

  支持：stable + async / await

  



- 新建项目

  ```bash
  cd /opt/code/rust-code/hello-rust/code-show/rocket-learning
  cargo new hello-rocket
  code hello-rocket/
  
  cargo run
  
  ```

- 引入依赖

  https://rocket.rs/guide/v0.5/getting-started/#getting-started

  ```toml
  [package]
  name = "hello-rocket"
  version = "0.1.0"
  edition = "2021"
  
  [dependencies]
  # rocket = "0.5.0-rc.1"
  rocket = { version = "0.5.0-rc.1", features = ["secrets", "tls", "json"] }
  
  ```

  



### 两种模式

- launch (官方推荐??)

  ```rust
  use rocket::{get, launch, routes};
  // https://rocket.rs/guide/v0.5/getting-started/#getting-started
  // 127.0.0.1:8000/health
  
  #[get("/health")]
  async fn health() -> String {
      "Health OK!".to_string()
  }
  
  #[launch]
  fn rocket() -> _ {
      rocket::build().mount("/", routes![health])
  }
  ```

- rocket::build ✔

  ```rust
  use rocket::{get, routes};
  // https://rocket.rs/guide/v0.5/getting-started/#getting-started
  // 127.0.0.1:8000/health
  
  #[get("/health")]
  async fn health() -> String {
      "Health OK!".to_string()
  }
  
  #[rocket::main]
  async fn main() -> Result<(), Box<dyn std::error::Error>> {
      rocket::build().mount("/", routes![health]).launch().await?;
      Ok(())
  }
  
  ```

  



## Rocket Demo

### Restful api

- Model 1

  Get 获取 (list, single)

  Post 创建未存在 

  Put 修改已存在 (修改不存在则报错)

  Delete 删除已存在 (删除不存在则报错)

- Model 2

  Get 

  Post (Put Delete) 

  



### url参数的获取

- 接收url参数

  ```rust
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
  ```

  



### JSON模块

- 引入

  ```rust
  use rocket::serde::json::{Json, Value};
  use rocket::serde::{Deserialize, Serialize};
  ```

  返回JSON

  ```rust
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
  ```

  接收JSON

  ```rust
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
  ```

  



### catchers捕获错误

- 函数

  ```rust
  // #################################################
  // catch: error + record
  // #################################################
  #[catch(404)]
  fn not_found() -> Value {
      json!({
          "res":"404 Not Found!"
      })
  }
  ```

- 注册

  ```rust
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
          .register("/", catchers![not_found]);
  
      rocket.launch().await?;
      Ok(())
  }
  ```

  



### state状态维护

- 应用

  数据库连接、socket连接、令牌

- 代码

  ```rust
  
  ```

  



### 代码汇总

- main.ts

  ```rust
  use rocket::serde::json::serde_json::json;
  use rocket::serde::json::{Json, Value};
  use rocket::serde::{Deserialize, Serialize};
  use rocket::{catch, catchers, delete, get, post, put, routes};
  // https://rocket.rs/guide/v0.5/getting-started/#getting-started
  
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
  // health
  // >> (health) GET /health/check
  // >> (health_json) GET /health/json
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
  async fn get_user(id: usize) -> Option<Json<UserVO>> {
      // db: select * from user where id = id
      Some(Json(UserVO {
          id: id,
          username: "oswin".to_string(),
      }))
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
  // catch: error + record
  // #################################################
  #[catch(404)]
  fn not_found() -> Value {
      json!({
          "res":"404 Not Found!"
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
          .register("/", catchers![not_found]);
  
      rocket.launch().await?;
      Ok(())
  }
  
  ```

  



















