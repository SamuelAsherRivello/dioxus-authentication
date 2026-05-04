use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::client::components::prompt::MessagePrompt;
use authentication::prelude::*;

const PAGE_CLASS: &str = "home-page";
const HERO_CLASS: &str = "home-hero";
const TITLE_CLASS: &str = "home-hero__title";
const BODY_TEXT_CLASS: &str = "home-page__body";
const SECTION_CLASS: &str = "home-section";
const SECTION_TITLE_CLASS: &str = "home-section__title";
const DEMO_WARNING_CLASS: &str = "home-demo-warning";
const AUTH_EMBED_CLASS: &str = "home-auth-embed";
const CODE_BLOCK_CLASS: &str = "home-code";

#[derive(Clone, Debug, PartialEq, Eq)]
struct DemoOriginWarning {
    prefix: String,
    target_origin: String,
    target_url: String,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Debug, PartialEq, Eq)]
struct DemoOriginWarningTarget {
    current_origin: String,
    target_origin: String,
    target_url: String,
}

#[component]
pub fn HomePage() -> Element {


    // Configuration
    let mut auth_status = use_signal(|| None::<Result<AuthenticationStatus, String>>);
    let mut auth_is_busy = use_signal(|| true);
    let auth_prompt_message = use_signal(|| None::<String>);
    let mut is_logout_confirmation_open = use_signal(|| false);
    let session_config = AuthenticationSessionConfig::new(
        48,
        "my_app_id",
    );
    let passkey_config = AuthenticationPasskeyConfig::new(
        "my_app_id".to_string(),
        "My Dioxus Authentication".to_string(),
        "my_email@my_email.com".to_string(),
        "My Demo User".to_string(),
    );
    let session_config_for_status = session_config.clone();
    use_future(move || {
        let session_config = session_config_for_status.clone();
        async move {
            auth_status.set(Some(AuthenticationService::status(session_config).await));
            auth_is_busy.set(false);
        }
    });


    // Callbacks
    let session_config_for_login = session_config.clone();
    let on_login = EventHandler::new(move |provider_id: String| {
        auth_is_busy.set(true);
        let auth_is_busy = auth_is_busy;
        let auth_prompt_message = auth_prompt_message;
        let session_config = session_config_for_login.clone();
        let passkey_config = passkey_config.clone();
        let logged_in_prompt_for_task = t!("auth-prompt.logged-in");
        let operation_failed_prompt_for_task = t!("auth-prompt.operation-failed");
        let prior_status = auth_status();

        let mut auth_status_for_task = auth_status.clone();
        let mut auth_is_busy_for_task = auth_is_busy.clone();
        let mut auth_prompt_message_for_task = auth_prompt_message.clone();
        let session_config_for_refresh = session_config.clone();
        spawn(async move {
            match AuthenticationService::login(session_config, provider_id, passkey_config).await {
                Ok(next_status) => {
                    auth_status_for_task.set(Some(Ok(next_status)));
                    auth_prompt_message_for_task.set(Some(logged_in_prompt_for_task));
                }
                Err(_) => {
                    if let Ok(next_status) =
                        AuthenticationService::status(session_config_for_refresh).await
                    {
                        auth_status_for_task.set(Some(Ok(next_status)));
                    } else {
                        auth_status_for_task.set(prior_status.clone());
                    }
                    auth_prompt_message_for_task.set(Some(operation_failed_prompt_for_task));
                }
            }
            auth_is_busy_for_task.set(false);
        });
    });

    let on_logout = EventHandler::new(move |_| {
        is_logout_confirmation_open.set(true);
    });

    let session_config_for_logout = session_config.clone();
    let on_confirm_logout = EventHandler::new(move |confirmed: bool| {
        if !confirmed {
            return;
        }

        let auth_status = auth_status;
        let mut auth_is_busy = auth_is_busy;
        let auth_prompt_message = auth_prompt_message;
        let session_config = session_config_for_logout.clone();
        let logged_out_prompt_for_task = t!("auth-prompt.logged-out");
        let operation_failed_prompt_for_task = t!("auth-prompt.operation-failed");

        auth_is_busy.set(true);
        let mut auth_status_for_task = auth_status.clone();
        let mut auth_is_busy_for_task = auth_is_busy.clone();
        let mut auth_prompt_message_for_task = auth_prompt_message.clone();
        spawn(async move {
            match AuthenticationService::logout(session_config).await {
                Ok(next_status) => {
                    auth_status_for_task.set(Some(Ok(next_status)));
                    auth_prompt_message_for_task.set(Some(logged_out_prompt_for_task));
                }
                Err(error) => {
                    auth_status_for_task.set(Some(Err(error)));
                    auth_prompt_message_for_task.set(Some(operation_failed_prompt_for_task));
                }
            }
            auth_is_busy_for_task.set(false);
        });
    });


    // Component
    let auth_view_config = AuthenticationViewConfig::new(
        session_config,
        vec![passkey_provider(t!("auth-provider.passkey"))],
        auth_status,
        auth_is_busy,
        on_login,
        on_logout,
    );

    rsx! {
        main { class: PAGE_CLASS,
            section { class: HERO_CLASS,
                h1 { class: TITLE_CLASS, {t!("home-hero-title")} }
                p { class: BODY_TEXT_CLASS,
                    {t!("home-hero-body")}
                }
            }

            section { class: SECTION_CLASS,
                h2 { class: SECTION_TITLE_CLASS, {t!("component-section-title")} }
                p { class: BODY_TEXT_CLASS,
                    {t!("component-section-body")}
                }
            }

            section { class: SECTION_CLASS,
                h2 { class: SECTION_TITLE_CLASS, {t!("usage-section-title")} }
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
                        " session_config = "
                        span { class: "home-code__type", "AuthenticationSessionConfig" }
                        "::"
                        span { class: "home-code__function", "new" }
                        "(\n        "
                        span { class: "home-code__number", "48" }
                        ",\n        \"my_app_id\", "
                        span { class: "home-code__comment", "// Reuse this key" }
                        "\n    );\n    "
                        span { class: "home-code__keyword", "let" }
                        " passkey_config = "
                        span { class: "home-code__type", "AuthenticationPasskeyConfig" }
                        "::"
                        span { class: "home-code__function", "new" }
                        "(\n        \"my_app_id\", "
                        span { class: "home-code__comment", "// Same app key" }
                        "\n        \"My Dioxus Authentication\", "
                        span { class: "home-code__comment", "// Prompt app name" }
                        "\n        \"my_email@my_email.com\", "
                        span { class: "home-code__comment", "// Account name" }
                        "\n        \"My Demo User\", "
                        span { class: "home-code__comment", "// Friendly label" }
                        "\n    );\n\n\n    "
                        span { class: "home-code__comment", "// Callbacks" }
                        "\n    "
                        span { class: "home-code__keyword", "let" }
                        " on_login = EventHandler::new(move |_provider_id: String| {{\n        println!(\"You are logged in\");\n    }});\n    "
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
                        span { class: "home-code__comment", "// Pick one provider" }
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

            section { class: SECTION_CLASS,
                h2 { class: SECTION_TITLE_CLASS, {t!("demo-section-title")} }
                if let Some(warning) = current_demo_origin_warning() {
                    p { class: DEMO_WARNING_CLASS, role: "alert",
                        "{warning.prefix} "
                        a { href: warning.target_url, "{warning.target_origin}" }
                        "."
                    }
                }
                div { class: AUTH_EMBED_CLASS,
                    AuthenticationView {
                        config: auth_view_config,
                    }
                }
                AuthenticationConfirmationPrompt {
                    open: is_logout_confirmation_open,
                    on_answer: move |confirmed| {
                        is_logout_confirmation_open.set(false);
                        if confirmed {
                            on_confirm_logout.call(confirmed);
                        }
                    },
                }
                MessagePrompt {
                    message: auth_prompt_message,
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn origin_warning(
    protocol: &str,
    hostname: &str,
    port: &str,
    path: &str,
    search: &str,
    hash: &str,
) -> Option<DemoOriginWarning> {
    let target = origin_warning_target(protocol, hostname, port, path, search, hash)?;
    let prefix =
        t!("demo-origin-warning-prefix").replace("__current_origin__", &target.current_origin);

    Some(DemoOriginWarning {
        prefix,
        target_origin: target.target_origin,
        target_url: target.target_url,
    })
}

#[cfg(any(target_arch = "wasm32", test))]
fn origin_warning_target(
    protocol: &str,
    hostname: &str,
    port: &str,
    path: &str,
    search: &str,
    hash: &str,
) -> Option<DemoOriginWarningTarget> {
    if protocol == "https:" || hostname.eq_ignore_ascii_case("localhost") {
        return None;
    }

    if protocol != "http:" || !is_local_testing_host(hostname) {
        return None;
    }

    let current_origin = format_origin(protocol, hostname, port);
    let target_origin = format_origin("http:", "localhost", port);
    let target_url = format!("{target_origin}{path}{search}{hash}");

    Some(DemoOriginWarningTarget {
        current_origin,
        target_origin,
        target_url,
    })
}

#[cfg(any(target_arch = "wasm32", test))]
fn format_origin(protocol: &str, hostname: &str, port: &str) -> String {
    if port.is_empty() {
        format!("{protocol}//{hostname}")
    } else {
        format!("{protocol}//{hostname}:{port}")
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn is_local_testing_host(hostname: &str) -> bool {
    if hostname.eq_ignore_ascii_case("localhost") {
        return true;
    }

    if hostname == "::1" || hostname == "0.0.0.0" || hostname.starts_with("127.") {
        return true;
    }

    let Some(octets) = parse_ipv4_octets(hostname) else {
        return false;
    };

    octets[0] == 10
        || (octets[0] == 172 && (16..=31).contains(&octets[1]))
        || (octets[0] == 192 && octets[1] == 168)
}

#[cfg(any(target_arch = "wasm32", test))]
fn parse_ipv4_octets(hostname: &str) -> Option<[u8; 4]> {
    let mut octets = [0_u8; 4];
    let mut count = 0;

    for part in hostname.split('.') {
        if count == octets.len() {
            return None;
        }

        octets[count] = part.parse().ok()?;
        count += 1;
    }

    if count == octets.len() {
        Some(octets)
    } else {
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn current_demo_origin_warning() -> Option<DemoOriginWarning> {
    let location = web_sys::window()?.location();
    let protocol = location.protocol().ok()?;
    let hostname = location.hostname().ok()?;
    let port = location.port().ok()?;
    let path = location.pathname().ok().unwrap_or_default();
    let search = location.search().ok().unwrap_or_default();
    let hash = location.hash().ok().unwrap_or_default();

    origin_warning(&protocol, &hostname, &port, &path, &search, &hash)
}

#[cfg(not(target_arch = "wasm32"))]
fn current_demo_origin_warning() -> Option<DemoOriginWarning> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_warning_links_loopback_http_to_localhost() {
        let warning = origin_warning_target("http:", "127.0.0.1", "8080", "/demo", "?a=1", "#top")
            .expect("loopback HTTP should warn");

        assert_eq!(warning.current_origin, "http://127.0.0.1:8080");
        assert_eq!(warning.target_origin, "http://localhost:8080");
        assert_eq!(warning.target_url, "http://localhost:8080/demo?a=1#top");
    }

    #[test]
    fn origin_warning_accepts_https_hosts() {
        assert_eq!(
            origin_warning_target("https:", "samuelasherivello.github.io", "", "/", "", ""),
            None
        );
    }

    #[test]
    fn origin_warning_accepts_localhost_http() {
        assert_eq!(
            origin_warning_target("http:", "localhost", "8080", "/", "", ""),
            None
        );
    }

    #[test]
    fn origin_warning_ignores_non_local_http_hosts() {
        assert_eq!(
            origin_warning_target("http:", "example.com", "", "/", "", ""),
            None
        );
    }
}
