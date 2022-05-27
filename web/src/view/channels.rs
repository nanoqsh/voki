use super::Data;
use yew::prelude::*;

#[function_component(Channels)]
pub fn channels() -> Html {
    let data: Data = use_context().expect("context");

    html! {
        <div class="channels">
            <div>
            {
                for data.state.channels().enumerate().map(|(index, chan)| {
                    let class = classes![
                        "channel",
                        (data.current_channel as usize == index).then(|| "current"),
                    ];

                    html! {
                        <div { class }>
                            {
                                match chan.icon() {
                                    Some(image) => html! {
                                        <img class="avatar" src={ image.to_string() } />
                                    },
                                    None => html! {
                                        <div class="avatar"/>
                                    },
                                }
                            }
                            <div>
                                <div class="name">{ chan.name() }</div>
                                <div class="last">{ "abc.." }</div>
                            </div>
                        </div>
                    }
                })
            }
            </div>
        </div>
    }
}
