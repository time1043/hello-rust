# rust-web

- 网站开发框架

  [`Rocket0.5`](https://rocket.rs/)：大型项目，易学；更新太慢

  [`Actix`](https://actix.rs/)：大型项目，性能超好，现在少unsafe且支持Tokio

  [`Warp`](https://github.com/seanmonstar/warp)：微型项目

  [`Axum`](https://github.com/tokio-rs/axum)：

  [`Salvo`](https://salvo.rs/)： 

  

---

- 关系型数据库

  `PostgreSQL`：开源，灵活，语法好，稳定

  `MySQL`：

  `SQLite`：嵌入式项目，本地存储需求

- 非关系型数据库

  `Redis`：缓存数据，减少关系型数据库的读开销 (键值型数据库)

  `MongoDB`：持久化数据，对标关系型数据库 (文档型数据库)

- 数据库工具包

  `SQLx`：纯异步，性能极高；无ORM，需要手写SQL  ✔

  > **ORM**适合业务逻辑比较重的项目，*性能消耗*主要在数据库，此时没必要选择Rust
  >
  > 解决：把业务重的服务拆分掉，用Rust的非ORM方案，获得性能上的提升

  

---

- RPC

  `gPRC`框架：

  `Tonic`驱动：

- HttpClient

  `Reqwest`：方便，易学

- 消息队列

  `RabbitMQ`：老牌，稳定

  `Kafka`：粗粒度，应用场景主要为日志



























































































































