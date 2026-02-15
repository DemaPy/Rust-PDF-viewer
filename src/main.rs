use dioxus::prelude::*;

fn main() {
    let config = dioxus_desktop::Config::new().with_window(
        dioxus_desktop::tao::window::WindowBuilder::new()
            .with_title("PDF Viewer in Rust")
            .with_inner_size(dioxus_desktop::tao::dpi::LogicalSize::new(1200.0, 800.0)),
    );
    dioxus_desktop::launch::launch(app, vec![], config);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "High-five counter: {count}" }
            button {
                onclick: move |_| count += 1, "Increment"
            }
            button {
                onclick: move |_| count -= 1, "Decrement"
            }
        }
    }
}
