use dioxus::prelude::*;
use dioxus_fullstack::prelude::server_fn;
use dioxus_fullstack::prelude::{use_server_future, ServerFnError};

pub fn app() -> Element {
    let thing = use_server_future(get_meme)?;
    let Some(Ok(eep)) = thing() else {
        println!("Failed!");
        panic!("Meme!");
    };

    rsx! {
        h1 {
            "{eep}"
        }
    }
}

#[dioxus_fullstack::prelude::server(GetMeme)]
async fn get_meme() -> Result<String, ServerFnError> {
    // use crate::state::PG_POOL;
    use dioxus_fullstack::prelude::{extract, DioxusServerContext};

    Ok("Hello world!".into())
}

#[cfg(feature = "server")]
pub mod state {
    use axum::extract::FromRef;
    use tokio::sync::OnceCell;
}
