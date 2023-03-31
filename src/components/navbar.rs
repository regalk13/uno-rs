use leptos::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <nav class="p-5 bg-white shadow md:flex md:items-center md:justify-between">
            <div class="md:flex md:items-center">
                <img class="mr-5" src="assets/logo.png" alt="logo" />
                <span class="text-3xl font-bold cursor-pointer">
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
                <button class="px-8 py-2 mx-3 text-lg font-bold bg-[#b1b2b5] text-[#3d3a3a] rounded-lg hover:bg-[#3d3a3a] hover:text-white duration-500">
                    { "Sign up" }
                </button>
                <button class="px-8 py-2 mx-3 text-white text-lg font-bold bg-blue-600 rounded-lg hover:bg-blue-500 duration-500">
                    { "Log In" }
                </button>
            </ul>
        </nav>
    }
}
