//use axum::{response::Html, routing::get, Router};
//use tower_http::services::ServeDir;
use auth_service::Application;
use auth_service::app_state::AppState;
use auth_service::services::*;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    /*let app = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/hello", get(hello_handler));

     */

    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);
    let app = Application::build(app_state ,"0.0.0.0:3000")
        .await
        .expect("Failed to build app");
    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
    // This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
 /*     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
 */
    app.run().await.expect("Failed to run app");   
}

/*async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    Html("<h1>Hello, new Rustacean</h1>")
}
*/
