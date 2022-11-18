use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub type ChangeState<T> = (UseStateHandle<T>, Callback<Event>);

pub fn use_change_conv<T, Init, Conv>(f: Init, c: Conv) -> ChangeState<T>
where
    T: 'static,
    Init: FnOnce() -> T,
    Conv: 'static + Fn(String) -> Option<T>,
{
    let state = use_state(f);

    let callback = {
        let state = state.clone();
        Callback::from(move |event: Event| {
            let target = event.target().expect("Expected target");

            let value = target.unchecked_into::<HtmlInputElement>().value();

            if let Some(value) = c(value) {
                state.set(value)
            }
        })
    };

    (state, callback)
}

pub fn use_change<Init>(f: Init) -> ChangeState<String>
where
    Init: FnOnce() -> String,
{
    use_change_conv(f, Some)
}
