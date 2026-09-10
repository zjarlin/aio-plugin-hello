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
        let event = serde_json::from_str::<serde_json::Value>(&request).unwrap_or_default();
        let (status, body) = match event["kind"].as_str() {
            Some("page_action") => reduce_page_action(&event),
            Some("service_request") => (
                200,
                serde_json::json!({
                    "message": "Hello from Wasm Component",
                    "request": event,
                }),
            ),
            _ => (400, serde_json::json!({ "error": "不支持的插件请求类型" })),
        };
        component_response(status, body)
    }
}

fn reduce_page_action(event: &serde_json::Value) -> (u16, serde_json::Value) {
    if event["page_id"] != "hello" || event["action_id"] != "increment" {
        return (400, serde_json::json!({ "error": "不支持的页面动作" }));
    }
    let Some(count) = event
        .pointer("/body/state/count")
        .and_then(serde_json::Value::as_u64)
        .and_then(|count| count.checked_add(1))
    else {
        return (400, serde_json::json!({ "error": "页面计数状态无效" }));
    };
    (
        200,
        serde_json::json!({
            "body": {
                "kind": "actions",
                "title": "Hello Runtime",
                "content": format!("计数：{count}"),
                "state": { "count": count },
                "actions": [{ "id": "increment", "label": "+1" }],
            }
        }),
    )
}

fn component_response(status: u16, body: serde_json::Value) -> String {
    serde_json::json!({
        "status": status,
        "content_type": "application/json",
        "body": body.to_string(),
    })
    .to_string()
}

export!(HelloComponent);

#[cfg(test)]
mod tests {
    use super::reduce_page_action;

    #[test]
    fn increments_the_host_supplied_page_state() {
        let event = serde_json::json!({
            "kind": "page_action",
            "page_id": "hello",
            "action_id": "increment",
            "tenant_id": "tenant-a",
            "user_id": "user-a",
            "body": {
                "kind": "actions",
                "title": "Hello Runtime",
                "content": "计数：4",
                "state": { "count": 4 },
                "actions": [{ "id": "increment", "label": "+1" }],
            }
        });
        let (status, result) = reduce_page_action(&event);

        assert_eq!(status, 200);
        assert_eq!(result["body"]["state"]["count"], 5);
        assert_eq!(result["body"]["content"], "计数：5");
    }
}
