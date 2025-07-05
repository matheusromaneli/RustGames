use dioxus::{logger::tracing, prelude::*};

#[derive(Clone)]
struct GameState {
    player: char,
    reset: bool
}


#[component]
pub fn TicTacToe() -> Element {
    use_context_provider(|| Signal::new(GameState{ player: 'X', reset: false }));
    tracing::info!("tic tac toe");
    rsx! {
        h1 { "Tic Tac Toe" }
        div { id: "board",
            div { id: "game",
                for _ in 0..9 { GameButton {} }
            }
            ResetButton {}
        }
    }
}

#[component]
pub fn ResetButton() -> Element {
    let mut state = use_context::<Signal<GameState>>();
    tracing::info!("reset button");
    return rsx! {
        button { 
            onclick: move |_| {
                state.set(GameState{ player: 'X', reset: true });
            },
            "Reset"
        }
    }
}

#[component]
pub fn GameButton() -> Element {
    tracing::info!("game button");
    let mut state = use_signal(|| ' ');
    let mut game_state = consume_context::<Signal<GameState>>();
    
    let play = move |_| {
        if state() != ' ' { return; }
        
        state.set(game_state().player);
        
        let next = if state() == 'X' { 'O' } else { 'X' };
        game_state.set(GameState {player: next, reset: false});
    };

    use_effect(move ||{
        if game_state().reset { state.set(' '); }
    });
    
    rsx! {
        button { id: "gameButton",
            onclick: play,
            "{state}"
        }
    }
}