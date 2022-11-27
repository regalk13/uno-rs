use yew::prelude::*;
use crate::components::LoginForm;
use uno_core::user::LoginUser;

#[function_component(Login)]
pub fn login() -> Html {
    let onsubmit = {
        Callback::from(move |user: LoginUser| {
            return
        })
    }; 

    html! {
    <section class="flex items-center justify-center min-h-screen bg-gray-200">
        <LoginForm {onsubmit} />
    </section>
    }
}
