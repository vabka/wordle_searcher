mod corpus_loader;
mod game;
use corpus_loader::*;
use game::*;
use yew::{function_component, html, use_state, Callback};
// load corpus
// - input
// - button
// show corpus
// - list of words
// - button
// game screen
// - list of guesses
// -- each guess is discrete component with state
// --- text input
// --- result input
// --- done
// - predictions (10 items with ellipsis)
// -- each item is clickable
// --- on click - item got filled in guess component

#[function_component(App)]
fn app() -> Html {
    let loaded = use_state(|| false);
    let corpus = use_state(|| Option::None as Option<Vec<String>>);

    let callback = {
        let loaded = loaded.clone();
        let corpus = corpus.clone();
        Callback::from(move |data| {
            corpus.set(Some(data));
            loaded.set(true)
        })
    };

    html! {
        <div>
            if !*loaded {
                <CorpusLoader
                    default_url={"https://raw.githubusercontent.com/Harrix/Russian-Nouns/main/dist/russian_nouns.txt".to_string()}
                    on_loaded={callback}
                />
            } else {
                // <Game/>
            }
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
