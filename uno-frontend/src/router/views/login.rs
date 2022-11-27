use yew::prelude::*;
use crate::components::LoginForm;
use uno_core::user::LoginUser;

use log::info;
use wasm_bindgen::JsValue;

#[function_component(Login)]
pub fn login() -> Html {
    let onsubmit = {
        Callback::from(move |user: LoginUser| {
            let object = JsValue::from(user.username);
            info!("Username: {}", object.as_string().unwrap());
        })
    }; 

    html! {
    <section class="flex items-center justify-center min-h-screen bg-gray-200">
        <LoginForm {onsubmit} />
    </section>
    }
}
