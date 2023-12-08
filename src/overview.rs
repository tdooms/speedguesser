use yew::{Callback, function_component, Html, html};
use cobul::*;
use cobul::icons::Solid;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Overview)]
pub fn overview() -> Html {
    let nav = use_navigator().unwrap();

    let c = move |name: &'static str| {
        let nav = nav.clone();
        Callback::from(move |_| nav.push(&Route::Play { id: name.into() }))
    };

    let left = html! {
        <Box>
        <Title> {"Easy"} </Title>
        <Buttons>
            <Button icon={Solid::Play} text="Fruits" color={Color::Success} fullwidth=true click={c("fruits")} />
        </Buttons>
        </Box>
    };

    let middle = html! {
        <Box>
        <Title> {"Medium"} </Title>
        <Buttons>
            <Button icon={Solid::Play} text="Christmas" color={Color::Success} fullwidth=true click={c("christmas1")}/>
            <Button icon={Solid::Play} text="Animals" color={Color::Danger} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Celebrities" color={Color::Warning} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Foods" color={Color::Link} fullwidth=true disabled=true />
        </Buttons>
        </Box>
    };

    let right = html! {
        <Box>
        <Title> {"Hard"} </Title>
        <Buttons>
            <Button icon={Solid::Play} text="Christmas" color={Color::Success} fullwidth=true click={c("christmas2")}/>
            <Button icon={Solid::Play} text="Movies" color={Color::Danger} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Monuments" color={Color::Warning} fullwidth=true disabled=true />
            <Button icon={Solid::Play} text="Bands" color={Color::Link} fullwidth=true disabled=true />
        </Buttons>
        </Box>
    };

    let rules = html! {
        <Notification light=true>
        <Content>
        <h3> {"Rules"} </h3>
        <ul>
            <li> {"No talking and writing during the round"} </li>
            <li> {"Each correct answer is worth 1 point"} </li>
            <li> {"The (fully) correct order is worth 3 bonus points"} </li>
        </ul>
        </Content>
        </Notification>
    };

    html! {
        <Container>
        <Columns class="mt-3 mb-6">
            <Column class="has-text-centered"> <Title> {"Flash Quizzes"} </Title> </Column>
        </Columns>

        <Columns>
        <Column> {left} {rules} </Column>
        <Column> {middle} </Column>
        <Column> {right} </Column>
        </Columns>

        </Container>
    }
}