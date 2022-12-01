use actix_web::{get, App, HttpResponse, HttpRequest, HttpServer, Responder};
use actix_files::Files;
use tokio;
use uno_frontend::{ServerApp, ServerAppProps};


#[get("/{tail:.*}")]
async fn render_app(req: HttpRequest) -> impl Responder {
    let index_html_s = tokio::fs::read_to_string("./dist/index.html")
    .await
    .expect("Failed to read index.html");
 
    let url = req.uri().to_string();
    
    let renderer = yew::ServerRenderer
        ::<ServerApp>
        ::with_props(move || ServerAppProps { url: url.into(),  }
    );

    let rendered = renderer.render().await;    
    let rendered = index_html_s.replace("<body>", &format!("<body>{}", &rendered));

    let resp = HttpResponse::Ok()
       .content_type("text/html")
       .body(rendered);

    return resp;
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
