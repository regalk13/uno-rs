use actix_files::Files;
use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tokio;
use uno_frontend::{ServerApp, ServerAppProps};

// Function to render HttpRequests (This service is called with all requests)
#[get("/{tail:.*}")]
async fn render_app(req: HttpRequest) -> impl Responder {
    // Asynchronous filesystem manipulation operations that read the index.html template
    let index_html_s = tokio::fs::read_to_string("./dist/index.html")
        .await
        .expect("Failed to read index.html");

    // Get the url from the HttpRequest
    let url = req.uri().to_string();

    // Call yew::ServerRenderer using the ServerApp and Props { url: AttrValue <- virtual DOM definition }
    let renderer =
        yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps { url: url.into() });

    // String that return the method render()
    let rendered = renderer.render().await;
    // Replace the <body> of the index.html template with the string render()
    let rendered = index_html_s.replace("<body>", &format!("<body>{}", &rendered));

    // Response define the type and use the variable rendered as body
    let resp = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered);

    // Return the response
    return resp;
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
