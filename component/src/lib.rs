wit_bindgen::generate!({
    path: "wit",
    world: "page",
});

struct HelloComponent;

impl Guest for HelloComponent {
    fn definition() -> String {
        include_str!("../../runtime/pages.json").to_owned()
    }
}

export!(HelloComponent);
