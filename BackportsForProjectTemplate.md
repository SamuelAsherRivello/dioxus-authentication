# Backports for Project Template

| Item | Status |
|---|---|
| `AppLanguage` refactor | ✅ Replaced with `LocaleId` (BCP-47 `LanguageIdentifier`) and standardized locale metadata in `packages/ui/src/client/services/localization_service.rs`. |
| Persistence format | ✅ Migrated stored language values from `AppLanguage` enum payloads to canonical locale tags (`en-US`, `es-MX`, `pt-BR`, `fr-FR`). |
| UI selector wiring | ✅ Updated `DeveloperTools` language selector and home page locale usage to consume `LocaleId` via `language_code(...)` and `locale_flag_asset(...)`. |
