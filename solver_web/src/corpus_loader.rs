
use gloo_console::{console_dbg};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement};
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct CorpusLoadedProps {
    #[prop_or_default]
    pub default_url: String,
    #[prop_or_default]
    pub on_loaded: Callback<Vec<String>>,
}

pub enum CorpusLoader {
    UrlInput { input_ref: NodeRef, has_error: bool },
    Loading,
    ShowCorpus { words: Vec<String> },
    Error,
}
pub enum CorpusLoaderMsg {
    CorpusUrlSubmitted,
    ResetError,
    CorpusLoaded(Vec<String>),
    CorpusError(gloo_net::Error),
}

impl Component for CorpusLoader {
    type Message = CorpusLoaderMsg;

    type Properties = CorpusLoadedProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self::UrlInput {
            input_ref: NodeRef::default(),
            has_error: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let container_content = match self {
            CorpusLoader::UrlInput {
                input_ref,
                has_error,
            } => {
                let default_url = &ctx.props().default_url;
                let onclick = ctx.link().callback(|_| Self::Message::CorpusUrlSubmitted);
                let reset_error = ctx.link().callback(|_| Self::Message::ResetError);
                html! {
                    <form>
                    <input
                        class={if *has_error {"has-error"} else {""}}
                        type="url" name="corpus-url"
                        value={default_url.clone()}
                        placeholder="https://example.com/corpus.txt"
                        required={true}
                        ref={input_ref}
                        onchange={reset_error}
                        />
                    <input type="button" name="start-a-game" value="Load" {onclick}/>
                </form>
                }
            }
            CorpusLoader::ShowCorpus { words } => {
                let words = words
                    .iter()
                    .map(|word| html! {<li class="word-list_item">{word.clone()}</li>})
                    .collect::<Html>();
                html! {
                    <>
                        <il class="word-list">
                            {words}
                        </il>
                        <button>{"Submit"}</button>
                    </>
                }
            }
            CorpusLoader::Error => {
                let reset_error = ctx.link().callback(|_| Self::Message::ResetError);

                html! {
                    <div>
                        <title>{"!TODO! ERROR"}</title>
                        <h1 class="error">{"!TODO! ERROR"}</h1>
                        <div>
                            <button onclick={reset_error}>{"Reset"}</button>
                        </div>
                    </div>
                }
            }
            CorpusLoader::Loading => html! {<div>{"LOADING !TODO!"}</div>},
        };

        html! {
            <div class="corpus-loader">
            {container_content}
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CorpusLoaderMsg::CorpusUrlSubmitted => {
                if let Self::UrlInput {
                    input_ref,
                    has_error,
                } = self
                {
                    let url = input_ref.cast::<HtmlInputElement>().map(|e| e.value());
                    let callback_on_load = ctx
                        .link()
                        .callback_once(|text| Self::Message::CorpusLoaded(text));
                    let callback_on_error_loading = ctx
                        .link()
                        .callback_once(|error| Self::Message::CorpusError(error));
                    if let Some(url) = url {
                        *self = Self::Loading;
                        spawn_local(async move {
                            let response = gloo_net::http::Request::get(&url).send().await;
                            match response {
                                Ok(response) => {
                                    let response_text = response.text().await;
                                    match response_text {
                                        Ok(response_text) => {
                                            let words = response_text
                                                .lines()
                                                .map(|line| line.to_string())
                                                .collect::<Vec<_>>();
                                            console_dbg!(words.len());
                                            callback_on_load.emit(words);
                                        }
                                        Err(e) => callback_on_error_loading.emit(e),
                                    }
                                }
                                Err(e) => callback_on_error_loading.emit(e),
                            }
                        });
                        return true;
                    }
                    *has_error = true;
                    return true;
                }
                return false;
            }
            CorpusLoaderMsg::ResetError => {
                if let Self::UrlInput { has_error, .. } = self {
                    if *has_error {
                        *has_error = false;
                        return true;
                    }
                } else if let Self::Error = self {
                    *self = Self::UrlInput {
                        input_ref: Default::default(),
                        has_error: false,
                    };
                    return true;
                }
                false
            }
            CorpusLoaderMsg::CorpusLoaded(corpus) => {
                *self = Self::ShowCorpus { words: corpus };
                return true;
            }
            CorpusLoaderMsg::CorpusError(_) => {
                *self = Self::Error;
                return true;
            }
        }
    }
}
