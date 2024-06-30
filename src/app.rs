mod navbar;

use yew::prelude::*;
// componentes
use navbar::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    let title = "Rust WASM";
    let content = "Probando pacos";
    html! {
      <>
        <Navbar title={ title.to_string() } />
        <main>{ content }</main>
      </>
    }
}
