mod app;

use leptos::*;
use crate::app::App;

pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;

    leptos::mount_to_body(move |cx| {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! {cx, <App /> }
    });
}
