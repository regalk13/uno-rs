use std::ops::Deref;
use web_sys::HtmlInputElement;

use uno_core::user::RegisterUser;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<RegisterUser>,
}

#[function_component(RegisterForm)]
pub fn register_form(props: &Props) -> Html {
    let user_info = use_state(RegisterUser::default);
    
    let email_oninput = {
        let user_info = user_info.clone();
        Callback::from(move |email: InputEvent| {
            let input: HtmlInputElement = email.target_unchecked_into();
            let mut user = user_info.deref().clone();
            user.email = input.value();
            user_info.set(user);
        })
    };
    
    let username_oninput = {
        let user_info = user_info.clone();
        Callback::from(move |username: InputEvent| {
            let input: HtmlInputElement = username.target_unchecked_into();
            let mut user = user_info.deref().clone();
            user.username = input.value();
            user_info.set(user);
        })
    };

    let password_oninput = {
        let user_info = user_info.clone();
        Callback::from(move |password: InputEvent| {
            let input: HtmlInputElement = password.target_unchecked_into();
            let mut user = user_info.deref().clone();
            user.password = input.value();
            user_info.set(user);
        })
    };

    let onsubmit = {
        let onsubmit_prop = props.onsubmit.clone();
        let user_info = user_info;
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let user = user_info.deref().clone();
            onsubmit_prop.emit(user);
        })
    };

    html! {
        <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg">
            <h1 class="text-2x1 font-bold text-center">{"Register"}</h1>
            <form {onsubmit} >
                    <div class="mt-4">
                        <div>
                            <label class="block">{"Email"}</label>
                            <input class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-ringblue-600"
                                   type="email" name="username" placeholder="Email" oninput={email_oninput} required=true />
                        </div>
                        <div class="mt-4">
                            <label class="block">{"Username"}</label>
                            <input class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-ringblue-600"
                                   type="text" name="username" placeholder="Username" oninput={username_oninput} required=true/>
                        </div>
                        <div class="mt-4">
                            <label class="block">{"Password"}</label>
                            <input class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-ringblue-600"
                                   type="password" name="password" placeholder="At least 8 characters" oninput={password_oninput} required=true/>
                        </div>
                        <div class="flex items-baseline justify-between">
                            <input type="submit" class="px-6 py-2 mt-4 text-white bg-blue-600 rounded-lg hover:bg-blue-900" value="Sign up"/>
                            <a href="/login" class="text-sm text-blue-600 hover:underline">{"Already have an account?"}</a>
                        </div>
                    </div>
            </form>
        </div>
    }
}