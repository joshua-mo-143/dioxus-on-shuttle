use dioxus::prelude::{rsx, Element, GlobalSignal};
use dioxus_fullstack::prelude::server_fn;
use dioxus_fullstack::prelude::{use_server_future, ServerFnError};

use dioxus::prelude::LaunchBuilder;
use dioxus::prelude::*;

use dioxus_test_app::app;

#[cfg(feature = "web")]
fn main() {
    dioxus::launch(app);
}

#[cfg(not(feature = "web"))]
fn main() {}
