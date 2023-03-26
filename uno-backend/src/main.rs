use actix_files::Files;
use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tokio;
use leptos::*;


// Function to render HttpRequests (This service is called with all requests)
#[get("/{tail:.*}")]
async fn render_app(req: HttpRequest) -> impl Responder {

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            // Add compress to reduce payloads size
            .wrap(middleware::Compress::default())
            // Assets/files and root directory of the render
            .service(
                Files::new("/dist", "./dist")
                )
            .service(
                Files::new("/assets", "./dist/assets/")
                )
            // Start the service of the render app
            .service(render_app)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
