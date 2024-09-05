use dioxus::prelude::*;
use dioxus_fullstack::prelude::server_fn;
use dioxus_fullstack::prelude::{use_server_future, ServerFnError};

pub fn app() -> Element {
    let thing = use_server_future(get_server_data)?;
    let Some(Ok(string)) = thing() else {
        panic!("This should be always be Some(Ok(T))!");
    };

    rsx! {
        h1 {
            "{string}"
        }
    }
}

#[dioxus_fullstack::prelude::server]
async fn get_server_data() -> Result<String, ServerFnError> {
    // Access a database

    Ok("Hello from Shuttle!".to_string())
}
