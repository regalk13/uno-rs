use cfg_if::cfg_if;
use leptos::*;
use tower_http::compression::CompressionLayer;

cfg_if! {
   if #[cfg(feature = "ssr")]  {
        use axum::{
            extract::Extension,
            Router,
        };
        use leptos_config::get_configuration;
        use leptos_axum::{LeptosRoutes,generate_route_list};
        use std::sync::Arc;
        use uno_rs::app::*;
        use uno_rs::file::file_handler;

        #[tokio::main]
        async fn main() {
            let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr.clone();
            let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
            println!("INFO: Serving at {addr}");

            let app = Router::new()
                .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
                .fallback(file_handler)
                .layer(CompressionLayer::new())
                .layer(Extension(Arc::new(leptos_options)));

            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
   }


else {
        pub fn main() {
            println!("ERROR: CSR option is not yet implemented, use SSR instead ;)");
        }
    }
}
