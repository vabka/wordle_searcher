use yew::{html, Component, Properties};

pub struct Game {
    wordle: NaiveSolver<5, 6>,
}
#[derive(Properties, PartialEq)]
pub struct GameProps {
    corpus: Vec<String>,
}
impl Component for Game {
    type Message = ();

    type Properties = GameProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Game {
            // TODO excessive clone
            wordle: NaiveSolver::new(ctx.props().corpus.clone()),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div>
             {"TODO"}
            </div>
        }
    }
}
