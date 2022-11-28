use crate::components::LoginForm;
use uno_core::user::LoginUser;
use yew::prelude::*;

use log::info;
use wasm_bindgen::JsValue;

#[function_component(Login)]
pub fn login() -> Html {
    let onsubmit = {
        Callback::from(move |user: LoginUser| {
            let username = user.username;
            let password = user.password;
            let object_user = JsValue::from(username);
            let object_password = JsValue::from(password);
            info!("Username: {}", object_user.as_string().unwrap());
            info!("Password: {}", object_password.as_string().unwrap());
        })
    };

    html! {
    <section class="flex items-center justify-center min-h-screen bg-gray-200">
        <LoginForm {onsubmit} />
    </section>
    }
}
