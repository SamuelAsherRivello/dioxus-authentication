use dioxus::prelude::*;

const CODE_BLOCK_CLASS: &str = "home-code";

#[component]
pub fn DemoSnippet() -> Element {
    rsx! {
        pre { class: CODE_BLOCK_CLASS,
            code {
                span { class: "home-code__keyword", "use" }
                " dioxus::prelude::*;\n"
                span { class: "home-code__keyword", "use" }
                " authentication::prelude::*;\n\n"
                span { class: "home-code__attribute", "#[component]" }
                "\n"
                span { class: "home-code__keyword", "fn" }
                " "
                span { class: "home-code__function", "Home" }
                "() -> "
                span { class: "home-code__type", "Element" }
                " {{\n\n\n    "
                span { class: "home-code__comment", "// Configuration" }
                "\n    "
                span { class: "home-code__keyword", "let" }
                " app_id = \"my_app_id\"; "
                span { class: "home-code__comment", "// Unique key for your app" }
                "\n    "
                span { class: "home-code__keyword", "let" }
                " username = \"my_email@my_email.com\";"
                "\n    "
                span { class: "home-code__keyword", "let" }
                " session_config = "
                span { class: "home-code__type", "AuthenticationSessionConfig" }
                "::"
                span { class: "home-code__function", "new" }
                "(\n        "
                span { class: "home-code__number", "48" }
                ", "
                span { class: "home-code__comment", "// Hours till expiration" }
                "\n        app_id,"
                "\n    );\n    "
                span { class: "home-code__keyword", "let" }
                " passkey_config = "
                span { class: "home-code__type", "AuthenticationPasskeyConfig" }
                "::"
                span { class: "home-code__function", "new" }
                "(\n        app_id,"
                "\n        \"My Dioxus Authentication\", "
                span { class: "home-code__comment", "// App label" }
                "\n        \"my_email@my_email.com\", "
                span { class: "home-code__comment", "// User identifier, any format" }
                "\n        \"My Demo User\", "
                span { class: "home-code__comment", "// User label" }
                "\n    );\n\n\n    "
                span { class: "home-code__comment", "// Callbacks" }
                "\n    "
                span { class: "home-code__keyword", "let" }
                " on_login = EventHandler::new(move |_provider_id: String| {{\n        println!(\"You are logged in as user {{username}}\");\n    }});\n    "
                span { class: "home-code__keyword", "let" }
                " on_logout = EventHandler::new(move |_| {{\n        println!(\"You are logged out\");\n    }});\n\n\n    "
                span { class: "home-code__comment", "// Component" }
                "\n    "
                span { class: "home-code__keyword", "let" }
                " config = "
                span { class: "home-code__type", "AuthenticationViewConfig" }
                "::"
                span { class: "home-code__function", "new" }
                "(\n        session_config, "
                span { class: "home-code__comment", "// Session settings" }
                "\n        vec![passkey_provider(\"PassKey\")], "
                span { class: "home-code__comment", "// Offer list of providers. User picks" }
                "\n        auth_status, "
                span { class: "home-code__comment", "// Current state" }
                "\n        auth_is_busy, "
                span { class: "home-code__comment", "// Loading state" }
                "\n        on_login, "
                span { class: "home-code__comment", "// Sign in" }
                "\n        on_logout, "
                span { class: "home-code__comment", "// Sign out" }
                "\n    );\n\n    "
                span { class: "home-code__macro", "rsx!" }
                " {{\n        "
                span { class: "home-code__type", "AuthenticationView" }
                " {{\n            "
                span { class: "home-code__property", "config" }
                "\n        "
                "}}\n    "
                "}}\n"
            }
        }
    }
}
