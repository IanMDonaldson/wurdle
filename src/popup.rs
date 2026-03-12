use crate::context::{GameContext, WonState};
use dioxus::hooks::use_context;
use dioxus::prelude::*;

#[component]
pub fn Popup() -> Element {
    let mut gamecx = use_context::<GameContext>();
    let mut show_popup = gamecx.show_popup;
    let won = gamecx.won.read().cloned();
    let message = gamecx.won.read().as_message();
    let action_message = gamecx.won.read().as_popup_action_message();
    rsx! {
        div {
            class: "popup-outer-container",
            onclick: move |_| show_popup.set(false),
            div {
                class: "popup-inner-container",
                onclick: move |event| event.stop_propagation(),
                h2 { "{message}"}
                button {
                    class: "popup-button",
                    onclick: move |_| {
                        show_popup.set(false);
                        if won != WonState::NA {
                            gamecx.reset();
                        }
                    },
                    "{action_message}"
                }
            }
        }
    }
}
