//! Widget definitions and traits for SigmaOS QuickPanel.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WidgetId(pub u32);

impl fmt::Display for WidgetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "widget:{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetKind {
    TrayApplet,
    StatusIndicator,
    Launcher,
    Clock,
    Separator,
}

pub trait Widget: Send + Sync {
    fn id(&self) -> WidgetId;
    fn name(&self) -> &str;
    fn kind(&self) -> WidgetKind;
    fn is_visible(&self) -> bool;
    fn render_text(&self) -> String;
    fn on_click(&mut self) -> Result<String, String>;
}

/// A standard concrete widget (Composition over inheritance).
pub struct SimpleWidget {
    id: WidgetId,
    name: String,
    kind: WidgetKind,
    visible: bool,
    text: String,
    click_count: u32,
}

impl SimpleWidget {
    pub fn new(id: u32, name: impl Into<String>, kind: WidgetKind, initial_text: impl Into<String>) -> Self {
        Self {
            id: WidgetId(id),
            name: name.into(),
            kind,
            visible: true,
            text: initial_text.into(),
            click_count: 0,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn click_count(&self) -> u32 {
        self.click_count
    }
}

impl Widget for SimpleWidget {
    fn id(&self) -> WidgetId { self.id }
    fn name(&self) -> &str { &self.name }
    fn kind(&self) -> WidgetKind { self.kind }
    fn is_visible(&self) -> bool { self.visible }
    fn render_text(&self) -> String { self.text.clone() }
    fn on_click(&mut self) -> Result<String, String> {
        self.click_count += 1;
        Ok(format!("Clicked {} (total: {})", self.name, self.click_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_initialization() {
        // Arrange & Act
        let w = SimpleWidget::new(1, "Clock", WidgetKind::Clock, "12:00");
        // Assert
        assert_eq!(w.id(), WidgetId(1));
        assert_eq!(w.name(), "Clock");
        assert_eq!(w.kind(), WidgetKind::Clock);
        assert!(w.is_visible());
        assert_eq!(w.render_text(), "12:00");
    }

    #[test]
    fn test_widget_click_increments_count() {
        // Arrange
        let mut w = SimpleWidget::new(2, "Volume", WidgetKind::TrayApplet, "100%");
        // Act
        let res = w.on_click().unwrap();
        // Assert
        assert_eq!(w.click_count(), 1);
        assert!(res.contains("total: 1"));
    }

    #[test]
    fn test_widget_visibility_toggle() {
        // Arrange
        let mut w = SimpleWidget::new(3, "Sep", WidgetKind::Separator, "|");
        // Act
        w.set_visible(false);
        // Assert
        assert!(!w.is_visible());
    }
}
