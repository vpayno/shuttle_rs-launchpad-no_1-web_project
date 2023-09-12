use axum::Router;
use std::net::SocketAddr; /* 1 */

#[tokio::main] /* 2 */
async fn main() {
    let app = Router::new(); /* 3 */

    let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000); /* 4 */

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap(); /* 5 */
}
