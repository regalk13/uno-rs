use actix_web::{get, App, HttpResponse, HttpRequest, Error, HttpServer};
use actix_files::Files;
use tokio;
use uno_frontend::{ServerApp, ServerAppProps};


#[get("/{tail:.*}")]
async fn render_app(req: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html_s = tokio::fs::read_to_string("./dist/index.html")
    .await
    .expect("Failed to read index.html");
 
    let server_props = ServerAppProps { url: req.uri().to_string().into(),  };
    let renderer = yew::ServerRenderer
        ::<ServerApp>
        ::with_props(server_props);

    let rendered = renderer.render().await;    
    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(index_html_s.replace("<body>", &format!("<body>{}", rendered)))
    )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
                    App::new()
                        .service(
                            Files::new("/dist", "./dist")
                        )
                        .service(render_app))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
