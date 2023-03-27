use cfg_if::cfg_if;
use leptos::*;


cfg_if {
   if #[cfg(feature = "ssr")]  {
        use axum::{
            routing::{get, post},
            extract::Extension,
            Router,
        };
        use leptos_config::get_configuration;
        use leptos_axum::{LeptosRoutes,generate_route_list};
        use std::sync::Arc;

        #[tokio::main]
        async fn main() {
            let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = letops_options.site_address.clone();


        }
   }

}

else {

}
