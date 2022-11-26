use yew::prelude::*;

// Component login
#[function_component(Login)]
pub fn login() -> Html {
    html! {
    <section class="flex items-center justify-center min-h-screen bg-gray-300">
        <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg">
            <h1 class="text-2x1 font-bold text-center">{"Login"}</h1>
            <form action="">
                    <div class="mt-4">
                        <div>
                            <label class="block">{"Username"}</label>
                            <input class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-ringblue-600"
                                   type="text" name="name" placeholder="Username"/>
                        </div>
                        <div class="mt-4">
                            <label class="block">{"Password"}</label>
                            <input class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-ringblue-600"
                                   type="password" name="password" placeholder="password"/>
                        </div>
                         <div class="flex items-baseline justify-between">
                            <input type="submit" class="px-6 py-2 mt-4 text-white bg-blue-600 rounded-lg hover:bg-blue-900" value="Login"/>
                            <a href="#" class="text-sm text-blue-600 hover:underline">{"Forgot password?"}</a>
                        </div>
                    </div>
            </form>
        </div>
    </section>
    }
}
