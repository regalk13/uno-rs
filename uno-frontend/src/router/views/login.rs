use yew::prelude::*;

// Component login
#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <section>
        <h1>{"Login"}</h1>
        <form>
            <label>{"Name"}</label>
            <input type="text" name="name" placeholder="regalk"/>
            <label>{"Password"}</label>
            <input type="password" name="password" placeholder="password"/>

            <input type="submit" name="login" value="login"/>
        </form>
        </section>
    }
}
