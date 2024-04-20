use askama::Template;
use axum::{response::Html, routing::get, Router}; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "main.html")] // using the template in this path, relative
                                // to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    title: &'a str, // the field name should match the variable name
                    // in your template
}
#[tokio::main]
async fn main() {
    // let hello = HelloTemplate { name: "world" }; // instantiate your struct
    // println!("{}", hello.render().unwrap()); // then render it.
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    let hello = HelloTemplate {
        title: "Name Of Page",
    };
    // instantiate your struct
    Html(match hello.render() {
        Ok(v) => v,
        Err(e) => "error".to_owned(),
    })
}
