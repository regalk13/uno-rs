use std::ops::Deref;

use uno_core::user::LoginUser;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<LoginUser>,
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html  {
    let user_info =  use_state(LoginUser::default);
    
    let _username_onchange = {
        let user_info = user_info.clone();
        Callback::from(move |username: String| {
            let mut user = user_info.deref().clone();
            user.username = username;
            user_info.set(user);
        })
    };
     
    let _password_onchange = { 
        let user_info = user_info.clone();
        Callback::from(move |password: String| {
            let mut user = user_info.deref().clone();
            user.password = password;
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
            <h1 class="text-2x1 font-bold text-center">{"Login"}</h1>
            <form {onsubmit} >
                    <div class="mt-4">
                        <div>
                            <label class="block">{"Username"}</label>
                            <input class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-ringblue-600"
                                   type="text" name="username"
                                   placeholder="Username" />
                        </div>
                        <div class="mt-4">
                            <label class="block">{"Password"}</label>
                            <input class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-ringblue-600"
                                   type="password" name="password" placeholder="Password" />
                        </div>
                         <div class="flex items-baseline justify-between">
                            <input type="submit" class="px-6 py-2 mt-4 text-white bg-blue-600 rounded-lg hover:bg-blue-900" value="Login"/>
                            <a href="#" class="text-sm text-blue-600 hover:underline">{"Forgot password?"}</a>
                        </div>
                    </div>
            </form>
        </div>
    }
}
