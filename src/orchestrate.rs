use std::rc::Rc;
use web_sys::HtmlImageElement;
use yew::*;
use yew_router::hooks::use_navigator;

use crate::{Data, Route};
use crate::start::Start;
use crate::play::Play;
use crate::wait::Wait;
use crate::summary::Summary;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

enum Stage {
    Start,
    Play,
    Wait,
    Summary,
}

#[function_component(Orchestrate)]
pub fn orchestrate(props: &Props) -> Html {
    let stage = use_state(|| Stage::Start);
    let data = use_context::<Data>().unwrap();

    let convert = |(_, v): (&&str, &&str)| {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(*v);
        image
    };

    let images: Vec<_> = data.get(props.id.as_str()).unwrap().iter().map(convert).collect();
    let images = Rc::new(images);

    let names = data.get(props.id.as_str()).unwrap().iter().map(|(a, b)| (a.clone(), b.clone())).collect::<Vec<_>>();

    let play = {
        let cloned = stage.clone();
        Callback::from(move |_| cloned.set(Stage::Play))
    };
    let wait = {
        let cloned = stage.clone();
        Callback::from(move |_| cloned.set(Stage::Wait))
    };
    let summary = {
        let cloned = stage.clone();
        Callback::from(move |_| cloned.set(Stage::Summary))
    };
    let leave = {
        let nav = use_navigator().unwrap();
        Callback::from(move |_| nav.push(&Route::Overview))
    };

    match *stage {
        Stage::Start => html! { <Start done={play} /> },
        Stage::Play => html! { <Play {images} done={wait} /> },
        Stage::Wait => html! { <Wait done={summary} /> },
        Stage::Summary => html! { <Summary done={leave} images={names} />},
    }
}