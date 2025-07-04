use dioxus::{prelude::*};
mod tic_tac_toe;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: MAIN_CSS }
        Hero {}

    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header { 
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { "This is footer" }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        Header {}
        tic_tac_toe::TicTacToe {}
        Footer {}
    }
}
