use yew::prelude::*;

use crate::{components::nav::Nav, app::AppStateContext};

#[function_component(Home)]
pub fn home() -> Html {
    let app_ctx = use_context::<AppStateContext>().unwrap();
    let onclick = {
        let counter = app_ctx.clone();
        Callback::from(move |_| counter.dispatch(counter.counter + 1))
    };

    html! {
        <main>
        <Nav />
        <div>
        <h1>{"This is the Home page"}</h1>
        <h2>{"Counter: "} {app_ctx.counter}</h2>
        <button {onclick} class="button button-primary">{"+"}</button>
        </div>
        </main>
    }
}