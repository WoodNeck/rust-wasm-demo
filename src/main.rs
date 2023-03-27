use yew::prelude::*;
use yew_router::prelude::*;

mod route;
mod page;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<route::Route> render={switch} />
        </BrowserRouter>
    }
}

pub fn switch(route: route::Route) -> Html {
    match route {
        route::Route::Home => html! { <page::home::Home /> },
        route::Route::Counter => html! { <page::counter::Counter /> },
        route::Route::NotFound => html! { <h1>{ "404" }</h1> }
    }
}


fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
    yew::Renderer::<App>::new().render();
}
