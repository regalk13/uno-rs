use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use std::rc::Rc;

use uno_core::{
    chat::{Chat as CoreChat, Message as CoreMessage},
    user::{Level as CoreLevel, User as CoreUser},
};

#[derive(PartialEq, Properties)]
pub struct ChatProps {
    pub user: CoreUser,

    #[prop_or_default]
    pub chat: Rc<CoreChat>,

    #[prop_or_default]
    pub group: bool,

    #[prop_or_default]
    pub submit: Callback<CoreMessage>,
}

#[derive(PartialEq, Properties)]
pub struct ChatViewProps {
    pub user: CoreUser,

    #[prop_or_default]
    pub chat: Rc<CoreChat>,

    #[prop_or_default]
    pub group: bool,
}

#[derive(PartialEq, Properties)]
struct MessageProps {
    pub message: CoreMessage,
    pub own: bool,
    pub group: bool,
}

#[function_component(Chat)]
pub fn chat(
    ChatProps {
        user,
        chat,
        group,
        submit,
    }: &ChatProps,
) -> Html {
    let typing = use_state(String::new);

    let on_change = {
        let typing = typing.clone();
        Callback::from(move |event: Event| {
            typing.set(
                event
                    .target()
                    .unwrap()
                    .unchecked_into::<HtmlTextAreaElement>()
                    .value(),
            );
        })
    };

    let on_input = {
        let typing = typing.clone();
        Callback::from(move |event: InputEvent| {
            typing.set(
                event
                    .target()
                    .unwrap()
                    .unchecked_into::<HtmlTextAreaElement>()
                    .value(),
            );
        })
    };

    let on_key = {
        let user = user.clone();
        let submit = submit.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.code() != "Enter" || event.shift_key() {
                return;
            }

            if typing.is_empty() {
                return;
            }

            event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .set_value("");

            event.prevent_default();

            submit.emit(CoreMessage::new(user.clone(), (*typing).clone()));
            typing.set(String::new());
        })
    };

    html! {
        <div class={"chat"}>
            <span class={"chat-title"}>
                { if *group { "[Group Chat]" } else { "[Chat]" } }
            </span>

            <ChatView
                user={(*user).clone()}
                chat={Rc::clone(chat)}
                group={*group}
            />

            <textarea
                class="chat-typing visible-scrollbar"
                placeholder={"Message"}
                onchange={on_change}
                oninput={on_input}
                onkeydown={on_key}
            />
        </div>
    }
}

#[function_component(ChatView)]
pub fn chat_view(ChatViewProps { user, chat, group }: &ChatViewProps) -> Html {
    html! {
        <div class={"chat-view visible-scrollbar"}>
            if chat.is_empty() {
                <Message
                    message={CoreMessage::new(CoreUser::new_system(), "Chat is empty, be the first one to send a message!".into())}
                    own={false}
                    group={false}
                />
            } else {
                {
                    for chat.iter().map(|message| html_nested! {
                        <Message
                            message={message.clone()}
                            own={user == message.sender()}
                            group={*group}
                        />
                    })
                }
            }
        </div>
    }
}

#[function_component(Message)]
fn message(
    MessageProps {
        message,
        own,
        group,
    }: &MessageProps,
) -> Html {
    let class = match (own, message.sender().level()) {
        (false, CoreLevel::Guess) => "chat-message guess",
        (false, CoreLevel::User) => "chat-message user",
        (true, CoreLevel::Guess) => "chat-message guess own",
        (true, CoreLevel::User) => "chat-message user own",
        (_, CoreLevel::System) => "chat-message system",
    };

    let name = message.sender().name();

    let (date, hour) = {
        use chrono::{DateTime, Local, LocalResult, TimeZone, Utc};

        let timestamp = message.id().timestamp() as i64;
        match Utc.timestamp_millis_opt(timestamp) {
            LocalResult::Single(utc) => {
                let local: DateTime<Local> = DateTime::from(utc);
                (
                    format!("{}", local.format("%Y/%m/%d")),
                    format!("{}", local.format("%H:%M")),
                )
            }
            _ => (String::new(), String::new()),
        }
    };

    html! {
        <div {class}>
            if *group && !own {
                <a
                    class={"sender"}
                    target={"_blank"}
                    href={format!("/user/{}", name)}
                >{
                    name
                }</a>
            }
            <span>{ message.content() }</span>
            <sub class={"time"} title={date}>{ hour }</sub>
        </div>
    }
}
