use dioxus_i18n::fluent::{FluentArgs, FluentBundle, FluentResource};
use dioxus_i18n::unic_langid::LanguageIdentifier;

const LOCALES: [(&str, &str); 4] = [
    ("en-US", include_str!("../assets/i18n/en-US.ftl")),
    ("es-MX", include_str!("../assets/i18n/es-MX.ftl")),
    ("pt-BR", include_str!("../assets/i18n/pt-BR.ftl")),
    ("fr-FR", include_str!("../assets/i18n/fr-FR.ftl")),
];

const TRANSLATION_KEYS: &[&str] = &[
    "auth.title",
    "auth-status.label",
    "auth-status.checking",
    "auth-status.error",
    "auth-status.logged-in-at",
    "auth-status.logged-in",
    "auth-status.unavailable",
    "auth-status.logged-out",
    "auth-detail.checking",
    "auth-detail.error",
    "auth-detail.logged-in-method-prefix",
    "auth-detail.timestamp-label",
    "auth-detail.expiration-label",
    "auth-detail.hours-unit",
    "auth-detail.user-key-label",
    "auth-detail.logged-in-session",
    "auth-detail.database-key",
    "auth-detail.session-expires",
    "auth-detail.web-passkey",
    "auth-detail.web-unavailable",
    "auth-detail.windows-passkey",
    "auth-detail.windows-unavailable",
    "auth-detail.native-unavailable",
    "auth-button.login",
    "auth-button.logout",
    "auth-method.passkey",
    "auth-method.windows-passkey",
    "auth-prompt.logged-in",
    "auth-prompt.logged-out",
    "auth-prompt.operation-failed",
    "auth-confirmation.title",
    "auth-confirmation.description",
    "auth-confirmation.confirm",
    "auth-confirmation.cancel",
    "auth-message.title",
    "auth-message.ok",
];

#[test]
fn authentication_locale_files_are_valid_fluent() {
    for (locale, source) in LOCALES {
        FluentResource::try_new(source.to_string())
            .unwrap_or_else(|errors| panic!("{locale} has invalid Fluent syntax: {errors:#?}"));
    }
}

#[test]
fn authentication_locale_files_cover_all_auth_translation_keys() {
    for (locale, source) in LOCALES {
        for key in TRANSLATION_KEYS {
            let (message_id, attribute_id) = key
                .split_once('.')
                .unwrap_or_else(|| panic!("test key `{key}` must use message.attribute shape"));
            let has_message = source
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{message_id} =")));
            let has_attribute = source
                .lines()
                .any(|line| line.trim_start().starts_with(&format!(".{attribute_id} =")));

            assert!(
                has_message && has_attribute,
                "{locale} is missing translation key `{key}`"
            );
        }
    }
}

#[test]
fn authentication_locale_files_format_all_auth_translation_keys() {
    let mut args = FluentArgs::new();
    args.set("method", "passkey");
    args.set("authenticated_at", "2026-05-06 10:30");
    args.set("expiration_hours", 48);
    args.set("passkey_database_key", "demo-key");

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
            let (message_id, attribute_id) = key
                .split_once('.')
                .unwrap_or_else(|| panic!("test key `{key}` must use message.attribute shape"));
            let message = bundle
                .get_message(message_id)
                .unwrap_or_else(|| panic!("{locale} is missing message `{message_id}`"));
            let attribute = message
                .get_attribute(attribute_id)
                .unwrap_or_else(|| panic!("{locale} is missing translation key `{key}`"));
            let pattern = attribute.value();
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
