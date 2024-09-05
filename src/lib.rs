use dioxus::prelude::{rsx, Element, GlobalSignal};
use dioxus_fullstack::prelude::server_fn;
use dioxus_fullstack::prelude::{use_server_future, ServerFnError};

pub fn app() -> Element {
    let mut count = use_server_future(get_server_data)?;

    rsx! {"server data is {count.value():?}"}
}

#[dioxus_fullstack::prelude::server]
async fn get_server_data() -> Result<String, ServerFnError> {
    // Access a database

    Ok("Hello from the server!".to_string())
}
