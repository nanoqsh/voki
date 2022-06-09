use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub retry: bool,
    pub onlogin: Callback<(String, String)>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let name_node = NodeRef::default();
    let pass_node = NodeRef::default();
    let send = {
        let name_node = name_node.clone();
        let pass_node = pass_node.clone();
        let onlogin = props.onlogin.clone();
        move || {
            let name: web_sys::HtmlInputElement = name_node.cast().expect_throw("cast");
            let pass: web_sys::HtmlInputElement = pass_node.cast().expect_throw("cast");
            onlogin.emit((name.value().trim().into(), pass.value().trim().into()));
        }
    };

    let onkeypress = Callback::from({
        let send = send.clone();
        move |ev: KeyboardEvent| {
            const ENTER: u32 = 13;

            if ev.key_code() == ENTER && !ev.shift_key() {
                ev.prevent_default();
                send();
            }
        }
    });

    let onclick = Callback::from(move |_: MouseEvent| send());

    let class = classes!(props.retry.then(|| "retry"));
    html! {
        <div class="login">
            if props.retry {
                <p class="note">{ "Неверный логин или пароль" }</p>
            }
            <p>{ "Логин" }</p>
            <input
                class={ class.clone() }
                type="text"
                ref={ name_node }
                onkeypress={ onkeypress.clone() }
            />
            <p>{ "Пароль" }</p>
            <input
                { class }
                type="text"
                ref={ pass_node }
                { onkeypress }
            />
            <div class="button" { onclick }>{ "Войти" }</div>
        </div>
    }
}
