pub mod router;
pub mod components;

pub use yew_router::history::{AnyHistory, History, MemoryHistory};
pub use yew_router::prelude::*;
pub use yew::prelude::*;
pub use yew::{Properties, AttrValue};

pub use router::{switch, Route};
pub use components::*;


#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url); 
    
    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }

}
