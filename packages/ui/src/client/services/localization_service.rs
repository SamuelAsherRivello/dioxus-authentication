use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};

const EN_US: &str = concat!(
    include_str!("../../../assets/i18n/en-US.ftl"),
    "\n",
    include_str!("../../../../dioxus-authentication-component/assets/i18n/en-US.ftl")
);
const ES_MX: &str = concat!(
    include_str!("../../../assets/i18n/es-MX.ftl"),
    "\n",
    include_str!("../../../../dioxus-authentication-component/assets/i18n/es-MX.ftl")
);
const PT_BR: &str = concat!(
    include_str!("../../../assets/i18n/pt-BR.ftl"),
    "\n",
    include_str!("../../../../dioxus-authentication-component/assets/i18n/pt-BR.ftl")
);
const FR_FR: &str = concat!(
    include_str!("../../../assets/i18n/fr-FR.ftl"),
    "\n",
    include_str!("../../../../dioxus-authentication-component/assets/i18n/fr-FR.ftl")
);

#[derive(Clone, Copy, Debug)]
pub struct LocaleSpec {
    tag: &'static str,
    short_code: &'static str,
    label_key: &'static str,
    resource: &'static str,
}

pub const LOCALE_OPTIONS: [LocaleSpec; 4] = [
    LocaleSpec {
        tag: "en-US",
        short_code: "en",
        label_key: "language-en",
        resource: EN_US,
    },
    LocaleSpec {
        tag: "es-MX",
        short_code: "es",
        label_key: "language-es",
        resource: ES_MX,
    },
    LocaleSpec {
        tag: "pt-BR",
        short_code: "pt",
        label_key: "language-pt",
        resource: PT_BR,
    },
    LocaleSpec {
        tag: "fr-FR",
        short_code: "fr",
        label_key: "language-fr",
        resource: FR_FR,
    },
];

impl LocaleSpec {
    pub fn id(self) -> LanguageIdentifier {
        self.tag
            .parse::<LanguageIdentifier>()
            .expect("locale tags in LOCALE_OPTIONS must be valid identifiers")
    }

    pub fn language_id(self) -> LanguageIdentifier {
        self.id()
    }

    pub fn flag_asset(self) -> Asset {
        match self.short_code {
            "en" => asset!("/assets/images/flags/en-us.svg"),
            "es" => asset!("/assets/images/flags/es-mx.svg"),
            "pt" => asset!("/assets/images/flags/pt-br.svg"),
            "fr" => asset!("/assets/images/flags/fr-fr.svg"),
            _ => asset!("/assets/images/flags/en-us.svg"),
        }
    }

    pub fn label_key(self) -> &'static str {
        self.label_key
    }

    pub fn resource(self) -> &'static str {
        self.resource
    }
}

pub fn supported_locales() -> &'static [LocaleSpec] {
    &LOCALE_OPTIONS
}

pub fn locale_identifier(locale: &LocaleSpec) -> LanguageIdentifier {
    locale.id()
}

pub fn default_locale() -> LanguageIdentifier {
    langid!("en-US")
}

pub fn parse_locale(input: &str) -> Option<LanguageIdentifier> {
    if input.trim().is_empty() {
        return None;
    }

    let normalized = input.trim().replace('_', "-");
    let locale = LOCALE_OPTIONS
        .iter()
        .find(|candidate| {
            candidate.short_code == normalized.to_ascii_lowercase()
                || candidate.tag.eq_ignore_ascii_case(&normalized)
        })
        .copied();

    locale
        .map(LocaleSpec::id)
        .or_else(|| normalized.parse::<LanguageIdentifier>().ok())
}

fn locale_for(language: &LanguageIdentifier) -> Option<&'static LocaleSpec> {
    let normalized = language.to_string().to_ascii_lowercase();
    LOCALE_OPTIONS.iter().find(|candidate| {
        candidate.tag.eq_ignore_ascii_case(&normalized) || candidate.short_code == normalized
    })
}

pub fn language_code(language: &LanguageIdentifier) -> &'static str {
    locale_for(language).map_or("en", |locale| locale.short_code)
}

pub fn locale_flag_asset(language: &LanguageIdentifier) -> Asset {
    locale_for(language).map_or_else(
        || asset!("/assets/images/flags/en-us.svg"),
        |locale| locale.flag_asset(),
    )
}

pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
    let safe_language = parse_locale(&initial_language.to_string()).unwrap_or_else(default_locale);

    let mut config = I18nConfig::new(safe_language).with_fallback(default_locale());
    for locale in LOCALE_OPTIONS.iter() {
        config = config.with_locale((locale.id(), locale.resource()));
    }

    config
}
