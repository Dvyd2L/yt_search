use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub title: String,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let title = props.title.clone();
    html! {
        <header>
            <h1>{ title }</h1>
        </header>
    }
}
