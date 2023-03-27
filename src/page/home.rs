use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;

use crate::route::Route;

#[styled_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={
            css!("display: flex; .navbar-item { margin-right: 10px; }")
        }>
            <Link<Route> classes={classes!("navbar-item")} to={Route::Counter}>
                { "Counter" }
            </Link<Route>>
        </div>
    }
}
