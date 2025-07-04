use dioxus::{prelude::*};

#[derive(Clone, Copy)]
struct GameState {
    player: Signal<String>,
}

#[component]
pub fn TicTacToe() -> Element {
    let mut state = use_signal(|| "X".to_string());
    use_context_provider(|| GameState{ player: state });

    let mut reset = use_signal(|| false);

    rsx! {
        h1 { "Tic Tac Toe" }
        div { id: "board",
            div { id: "game",
                for _ in 0..9 { GameButton {reset_signal: reset} }
            }
            button { 
                onclick: move |_| {
                    reset.set(true);
                    state.set("X".to_string());
                },
                "Reset"
            }
        }
    }
}

#[component]
pub fn GameButton(reset_signal: Signal<bool>) -> Element {
    let mut state = use_signal(|| "".to_string());
    if reset_signal.read().to_owned() { state.set("".to_string()); }
    let play = move |_| {
        if state.read().to_string() != "".to_string() { return; }
        else { reset_signal.set(false); }
        state.set(consume_context::<GameState>().player.read().to_string());

        let next = if state.read().to_string() == "X".to_string() { "O".to_string() } else { "X".to_string() };
        consume_context::<GameState>().player.set(next);
    };
    rsx! {
        button { id: "gameButton",
            onclick: play,
            "{state}"
        }
    }
}