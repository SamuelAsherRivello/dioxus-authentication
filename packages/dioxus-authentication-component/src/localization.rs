use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};

pub const AUTH_EN_US: &str = include_str!("../assets/i18n/en-US.ftl");
pub const AUTH_ES_MX: &str = include_str!("../assets/i18n/es-MX.ftl");
pub const AUTH_PT_BR: &str = include_str!("../assets/i18n/pt-BR.ftl");
pub const AUTH_FR_FR: &str = include_str!("../assets/i18n/fr-FR.ftl");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthenticationLocaleResource {
    tag: &'static str,
    source: &'static str,
}

impl AuthenticationLocaleResource {
    pub fn id(self) -> LanguageIdentifier {
        self.tag
            .parse::<LanguageIdentifier>()
            .expect("authentication locale tags must be valid language identifiers")
    }

    pub fn source(self) -> &'static str {
        self.source
    }
}

pub const AUTHENTICATION_LOCALES: [AuthenticationLocaleResource; 4] = [
    AuthenticationLocaleResource {
        tag: "en-US",
        source: AUTH_EN_US,
    },
    AuthenticationLocaleResource {
        tag: "es-MX",
        source: AUTH_ES_MX,
    },
    AuthenticationLocaleResource {
        tag: "pt-BR",
        source: AUTH_PT_BR,
    },
    AuthenticationLocaleResource {
        tag: "fr-FR",
        source: AUTH_FR_FR,
    },
];

pub fn authentication_locales() -> &'static [AuthenticationLocaleResource] {
    &AUTHENTICATION_LOCALES
}

pub fn default_authentication_locale() -> LanguageIdentifier {
    langid!("en-US")
}
