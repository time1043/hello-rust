# tera

- Reference - dev

  [tera](https://docs.rs/tera/latest/tera/), [tera](https://keats.github.io/tera/), [tera github](https://github.com/Keats/tera), 

  [jinja2](https://docs.jinkan.org/docs/jinja2/), 

  



- Tera

  Rust模板引擎 编写服务端渲染的网页

  受到Jinja2和Django模板语言的启发

  vscode-plugin: tera

  



## 新建项目

- 新建项目

  ```bash
  cargo new hello-tera  # hello_tera
  mkdir templates && touch templates/index.html
  
  # cargo run
  cargo watch -w templates/index.html -x r  # cargo install cargo-watch
  
  ```

  Cargo.toml

  ```toml
  [package]
  name = "hello-tera"
  version = "0.1.0"
  edition = "2021"
  
  [dependencies]
  anyhow = "1.0.66"
  salvo = { version = "0.37.7", features = ["affix"] }
  tera = "1.17.1"
  tokio = { version = "1.22.0", features = ["full"] }
  tracing = "0.1.37"
  tracing-subscriber = "0.3.16"
  
  ```

  src/main.rs

  ```rust
  use salvo::affix;
  use salvo::handler;
  use salvo::prelude::*;
  use tera::Context;
  use tera::Tera;
  
  #[tokio::main]
  async fn main() {
      tracing_subscriber::fmt::init();
  
      tracing::info!("Listening on http://127.0.0.1:7878"); // logging
      Server::new(TcpListener::bind("127.0.0.1:7878"))
          .serve(route())
          .await;
  }
  
  fn route() -> Router {
      let mut tera = Tera::new("templates/**/*").unwrap();
      tera.full_reload().unwrap(); // 热更新
  
      Router::new().hoop(affix::inject(tera)).get(hello_world)
  }
  
  #[handler]
  async fn hello_world(depot: &mut Depot, res: &mut Response) {
      let tera: &Tera = depot.obtain::<Tera>().unwrap();
      let mut context = Context::new();
      context.insert("message", "Hello, Rust Tera!");
  
      let page: String = tera.render("index.html", &context).unwrap();
      res.render(Text::Html(page));
  }
  
  ```

  templates/index.html

  ```html
  <!DOCTYPE html>
  <html lang="en">
    <head>
      <meta charset="UTF-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <title>Document</title>
    </head>
    <body>
      <h1>{{ message }}</h1>
    </body>
  </html>
  
  ```

  



## Tera语法

- Tera语法

  ```rust
  // expressions
  {{ expressions }}
  // in  判断右边内容(数组 字符串 对象字面量 变量)包含左边内容(数字 字符串 bool)
  {{ "rust" in "trust" }}  // true
  
  // statements
  {% statements %}
  // row  插值表达式不会取值渲染
  {% raw %} {{ message }} {% endraw %}
  // 变量设置
  {% set name = "oswin" %}
  // 空白控制
  {%- set name = "oswin" -%}
  {{ message~name }}  // 连接符
  // if and or not  未定义变量为false
  {% if name == "oswin" %}
  <p>Hello {{ name }} !</p>
  {% endif %}
  
  
  // 注释 (F12不会看到)  // html注释 (F12会看到)
  {# comments #}
  
  ```

  | Snippet | Description                        |
  | :------ | :--------------------------------- |
  | xx      | `{{ }}`                            |
  | block   | `{% block %} {% endblock %}`       |
  | inblock | Same as above but on a single line |
  | if      | `{% if %} {% endif %}`             |
  | ifi     | Same as above but on a single line |
  | ifelse  | `{% if %} {% elif %} {% endif %}`  |
  | else    | `{% else %}`                       |
  | filter  | `{% filter %} {% endfilter %}`     |
  | forloop | `{% for in %} {% endfor %}`        |
  | extend  | `{% extends "" %}`                 |
  | include | `{% include "" %}`                 |
  | import  | `{% import "" %}`                  |
  | macro   | `{% macro %} {% endmacro %}`       |

- Jinja2

  





























