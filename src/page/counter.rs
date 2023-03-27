use yew::prelude::*;
use stylist::yew::styled_component;

use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, console, HtmlElement};

#[styled_component(Counter)]
pub fn page() -> Html {
    let canvas_ref = use_node_ref();
    let counter = use_state(|| 0);

    {
        let canvas_ref = canvas_ref.clone();

        use_effect_with_deps(|canvas_ref| {
            let document = window()
                .expect_throw("window is not defined")
                .document()
                .expect_throw("document is not defined");

            let canvas = canvas_ref.cast::<HtmlElement>();

            console::log_1(&document.into());
            console::log_1(&canvas.into());
        }, canvas_ref);
    }


    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class={classes!(String::from("class-1 class-2"))}>
            <button {onclick}>{"+1"}</button>
            <p>{ *counter }</p>
            <canvas ref={canvas_ref}></canvas>
        </div>
    }
}
