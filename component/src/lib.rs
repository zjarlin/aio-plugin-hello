wit_bindgen::generate!({
    path: "wit",
    world: "page",
});

struct HelloComponent;

impl Guest for HelloComponent {
    fn definition() -> String {
        include_str!("../../runtime/pages.json").to_owned()
    }

    fn handle(request: String) -> String {
        serde_json::json!({
            "status": 200,
            "content_type": "application/json",
            "body": serde_json::json!({
                "message": "Hello from Wasm Component",
                "request": serde_json::from_str::<serde_json::Value>(&request).unwrap_or_default(),
            }).to_string(),
        })
        .to_string()
    }
}

export!(HelloComponent);
