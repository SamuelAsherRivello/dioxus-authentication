//! Shared UI crate for the web and desktop packages.

pub mod client;

pub use client::{
    default_locale, App, AppErrorFallback, DeveloperTools, HomePage, Page, PageFooter, PageHeader,
    Route, Theme,
};
