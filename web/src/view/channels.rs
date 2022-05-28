use super::Data;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onselect: Callback<u32>,
}

#[function_component(Channels)]
pub fn channels(props: &Props) -> Html {
    use std::iter::zip;

    let data: Data = use_context().expect("context");

    let from_index = |index| {
        if data.current_channel == index {
            Callback::noop()
        } else {
            let onselect = props.onselect.clone();
            Callback::from(move |_: MouseEvent| onselect.emit(index))
        }
    };

    html! {
        <div class="channels">
            <div>
            {
                for zip(0.., data.state.channels()).map(|(index, chan)| {
                    let class = classes![
                        "channel",
                        (data.current_channel == index).then(|| "current"),
                    ];

                    html! {
                        <div { class } onclick={ from_index(index) }>
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
                                <div class="last">{ chan.last_message() }</div>
                            </div>
                        </div>
                    }
                })
            }
            </div>
        </div>
    }
}
