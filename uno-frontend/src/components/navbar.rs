use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="p-5 bg-white shadow md:flex md:items-center md:justify-between">
            <div>
                <span class="text-2xl font-bold 2cursor-pointer">
                    { "Uno Rust" }
                </span>
            </div>

            <ul class="md:flex md:items-center">
                <li class="mx-3">
                    <a href="/" class="text-x1 font-bold text-lg hover:text-blue-900 duration-500"> { "Home" } </a>
                </li>
                <li class="mx-3">
                    <a href="/" class="text-x1 font-bold text-lg hover:text-blue-900 duration-500"> { "About" } </a>
                </li>
                <button class="px-8 py-2 mx-3 text-white text-lg font-bold bg-[#b1b2b5] text-[#3d3a3a] rounded-lg hover:bg-[#3d3a3a] hover:text-[#b1b2b5] duration-500">
                    { "Sign up" }
                </button>
                <button class="px-8 py-2 mx-3 text-white text-lg font-bold bg-blue-600 rounded-lg hover:bg-blue-300 hover:text-blue-600 duration-500">
                    { "Log In" }
                </button>
            </ul>
        </nav>
    }
}
