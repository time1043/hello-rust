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
