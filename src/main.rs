mod vec2;
mod play;
mod summary;
mod wait;
mod overview;
mod start;
mod orchestrate;

use std::collections::HashMap;
use std::rc::Rc;
use web_sys::HtmlImageElement;
use yew::*;
use yew_router::*;

use orchestrate::Orchestrate;
use overview::Overview;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Overview,
    #[at("/play/:id")]
    Play { id: String },
}

pub type Quiz = Rc<HashMap<&'static str, &'static str>>;
pub type Data = Rc<HashMap<&'static str, Quiz>>;

pub type Images = Rc<Vec<HtmlImageElement>>;

fn switch(route: Route) -> Html {
    match route {
        Route::Overview => html! { <Overview /> },
        Route::Play { id } => html! { <Orchestrate {id} />},
    }
}

#[function_component(App)]
fn app() -> Html {
    let str = include_str!("../data.json");
    let data: Data = serde_json::from_str(str).unwrap();

    html! {
        <BrowserRouter>
        <ContextProvider<Data> context={data}>
            <Switch<Route> render={switch} />
        </ContextProvider<Data>>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    Renderer::<App>::new().render();
}
