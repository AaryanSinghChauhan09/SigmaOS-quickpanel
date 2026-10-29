//! SigmaOS QuickPanel — applet and tray widget management subsystem.
//!
//! Principles: Command-Query Separation, Single Responsibility, Encapsulation,
//! Interface Segregation, Composition over Inheritance, Defensive Programming.

pub mod widget;
pub mod panel;

pub use widget::{Widget, WidgetId, WidgetKind, SimpleWidget};
pub use panel::{QuickPanel, PanelPosition, PanelConfig};
