use yew::prelude::*;

use crate::{components::nav::Nav, app::AppStateContext};

#[function_component(About)]
pub fn about() -> Html {
    let app_ctx = use_context::<AppStateContext>().unwrap();
    
    html! {
        <main>
        <Nav />
        <div>
        <h1>{"This is the About page"}</h1>
        <h2>{"Counter: "} {app_ctx.counter}</h2>
        </div>
        </main>
    }
}