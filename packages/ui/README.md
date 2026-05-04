# UI Package

Shared Dioxus demo UI, routes, models, assets, client services, and tests for the web and desktop apps. Reusable auth code lives in `../authentication`.

## Key Areas

```text
src/
  client/
    components/
    pages/
    services/
    app.rs
    mod.rs
    models.rs
assets/
  i18n/
  images/flags/
  styling/
tests/
```

The native SQLite example creates its first schema and seed row through `create_database_if_missing()` in `src/client/services/database_service.rs`. Browser builds use localStorage snapshots through `storage_service.rs`.
