use dioxus_test_app::app;

#[cfg(feature = "web")]
fn main() {
    dioxus::launch(app);
}

#[cfg(not(feature = "web"))]
fn main() {}
