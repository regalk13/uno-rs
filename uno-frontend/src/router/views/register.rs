use crate::components::RegisterForm;
use uno_core::user::RegisterUser;
use yew::prelude::*;

use log::info;
use wasm_bindgen::JsValue;

#[function_component(Register)]
pub fn register() -> Html {
    let onsubmit = {
        Callback::from(move |user: RegisterUser| {
            let email = user.email;
            let username = user.username;
            let password = user.password;
            let object_email = JsValue::from(email);
            let object_user = JsValue::from(username);
            let object_password = JsValue::from(password);
            info!("Email: {}", object_email.as_string().unwrap());
            info!("Username: {}", object_user.as_string().unwrap());
            info!("Password: {}", object_password.as_string().unwrap());
        })
    };

    html! {
    <section class="flex items-center justify-center min-h-screen bg-gray-200">
        <RegisterForm {onsubmit} />
    </section>
    }
}
