use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{home::Home, about::About, contact::Contact, faq::Faq};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/faq")]
    Faq,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppState {
    pub counter: i32,
}

impl Reducible for AppState {
    type Action = i32;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AppState { counter: action }.into()
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Contact => html! { <Contact /> },
        Route::Faq => html! { <Faq /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(
        || AppState {
            counter: 0,
        });
    log::info!("Some info");
    html! {
        <ContextProvider<AppStateContext> context={state}>
        <BrowserRouter>
            <Switch<Route> render={switch} /> 
        </BrowserRouter>
        </ContextProvider<AppStateContext>>
    }
}
