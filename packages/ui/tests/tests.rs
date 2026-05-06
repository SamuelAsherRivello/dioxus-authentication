use dioxus_i18n::fluent::{FluentArgs, FluentBundle, FluentResource};
use dioxus_i18n::unic_langid::LanguageIdentifier;
use ui::client::pages::home_page::origin_warning_target;
use ui::{default_locale, Theme};

const LOCALES: [(&str, &str); 4] = [
    ("en-US", include_str!("../assets/i18n/en-US.ftl")),
    ("es-MX", include_str!("../assets/i18n/es-MX.ftl")),
    ("pt-BR", include_str!("../assets/i18n/pt-BR.ftl")),
    ("fr-FR", include_str!("../assets/i18n/fr-FR.ftl")),
];

const TRANSLATION_KEYS: &[&str] = &[
    "nav-page-01",
    "nav-page-01-short",
    "view-page-01",
    "open-github-repository",
    "toggle-theme",
    "theme",
    "language-selector",
    "language-en",
    "language-es",
    "language-pt",
    "language-fr",
    "dev-tools",
    "home-hero-title",
    "home-hero-body",
    "component-section-title",
    "component-section-body",
    "usage-section-title",
    "demo-section-title",
    "demo-origin-warning-prefix",
    "unknown-origin",
    "app-error-title",
    "app-error-default-message",
    "app-error-retry",
    "footer-rights",
];

#[test]
fn theme_labels_match_display_text() {
    assert_eq!(Theme::Light.label(), "Light");
    assert_eq!(Theme::Dark.label(), "Dark");
}

#[test]
fn theme_class_names_match_shell_modifiers() {
    assert_eq!(Theme::Light.class_name(), "app-shell--light");
    assert_eq!(Theme::Dark.class_name(), "app-shell--dark");
}

#[test]
fn theme_default_is_dark() {
    assert_eq!(Theme::default(), Theme::Dark);
}

#[test]
fn theme_toggle_switches_between_light_and_dark() {
    assert_eq!(Theme::Light.toggled(), Theme::Dark);
    assert_eq!(Theme::Dark.toggled(), Theme::Light);
}

#[test]
fn language_default_is_english() {
    assert_eq!(default_locale().to_string(), "en-US");
}

#[test]
fn locale_files_are_valid_fluent() {
    for (locale, source) in LOCALES {
        FluentResource::try_new(source.to_string())
            .unwrap_or_else(|errors| panic!("{locale} has invalid Fluent syntax: {errors:#?}"));
    }
}

#[test]
fn locale_files_cover_all_ui_translation_keys() {
    for (locale, source) in LOCALES {
        for key in TRANSLATION_KEYS {
            let has_key = source
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{key} =")));

            assert!(has_key, "{locale} is missing translation key `{key}`");
        }
    }
}

#[test]
fn locale_files_format_all_ui_translation_keys() {
    let args = FluentArgs::new();

    for (locale, source) in LOCALES {
        let language_id = LanguageIdentifier::from_bytes(locale.as_bytes())
            .unwrap_or_else(|error| panic!("{locale} has an invalid language id: {error}"));
        let resource = FluentResource::try_new(source.to_string())
            .unwrap_or_else(|errors| panic!("{locale} has invalid Fluent syntax: {errors:#?}"));
        let mut bundle = FluentBundle::new(vec![language_id]);

        bundle
            .add_resource(resource)
            .unwrap_or_else(|errors| panic!("{locale} has invalid Fluent resources: {errors:#?}"));

        for key in TRANSLATION_KEYS {
            let message = bundle
                .get_message(key)
                .unwrap_or_else(|| panic!("{locale} is missing translation key `{key}`"));
            let pattern = message
                .value()
                .unwrap_or_else(|| panic!("{locale} translation key `{key}` has no value"));
            let mut errors = Vec::new();
            let formatted = bundle.format_pattern(pattern, Some(&args), &mut errors);

            assert!(
                errors.is_empty(),
                "{locale} translation key `{key}` failed to format: {errors:#?}"
            );
            assert!(
                !formatted.trim().is_empty(),
                "{locale} translation key `{key}` formatted to empty text"
            );
        }
    }
}

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
