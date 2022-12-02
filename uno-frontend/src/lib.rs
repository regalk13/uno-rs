pub mod router;
pub mod components;

pub use yew_router::history::{AnyHistory, History, MemoryHistory};
pub use yew_router::prelude::*;
pub use yew::prelude::*;
pub use yew::{Properties, AttrValue};

pub use router::{switch, Route};
pub use components::*;

// ServerAppProps this is used on backend for ServerRenderer props
#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
}

// ServerApp contains the Router compatible with ServerSideRendering
#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    // A History that provides a universial API to the underlying history type.
    let history = AnyHistory::from(MemoryHistory::new());
    // Pushes a route entry with None being the state.
    history.push(&*props.url); 
    
    // Return the router
    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}
