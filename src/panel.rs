//! QuickPanel implementation — managing layout, visibility, and dispatch.
use crate::widget::{Widget, WidgetId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct PanelConfig {
    pub position: PanelPosition,
    pub height_px: u32,
    pub autohide: bool,
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            position: PanelPosition::Top,
            height_px: 32,
            autohide: false,
        }
    }
}

pub struct QuickPanel {
    config: PanelConfig,
    widgets: BTreeMap<WidgetId, Box<dyn Widget>>,
}

impl QuickPanel {
    pub fn new(config: PanelConfig) -> Self {
        Self {
            config,
            widgets: BTreeMap::new(),
        }
    }

    // ── Queries (Command-Query Separation) ──────────────────────────────────
    pub fn config(&self) -> &PanelConfig {
        &self.config
    }

    pub fn widget_count(&self) -> usize {
        self.widgets.len()
    }

    pub fn has_widget(&self, id: WidgetId) -> bool {
        self.widgets.contains_key(&id)
    }

    pub fn render_bar(&self) -> Vec<String> {
        self.widgets
            .values()
            .filter(|w| w.is_visible())
            .map(|w| format!("[{}: {}]", w.name(), w.render_text()))
            .collect()
    }

    // ── Commands (Command-Query Separation) ──────────────────────────────────
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) -> Result<(), String> {
        let id = widget.id();
        if self.widgets.contains_key(&id) {
            return Err(format!("Widget with ID {} already registered", id));
        }
        self.widgets.insert(id, widget);
        Ok(())
    }

    pub fn remove_widget(&mut self, id: WidgetId) -> Option<Box<dyn Widget>> {
        self.widgets.remove(&id)
    }

    pub fn dispatch_click(&mut self, id: WidgetId) -> Result<String, String> {
        match self.widgets.get_mut(&id) {
            Some(w) => w.on_click(),
            None => Err(format!("Widget with ID {} not found", id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::{SimpleWidget, WidgetKind};

    #[test]
    fn test_panel_creation_and_defaults() {
        // Arrange & Act
        let panel = QuickPanel::new(PanelConfig::default());
        // Assert
        assert_eq!(panel.config().position, PanelPosition::Top);
        assert_eq!(panel.widget_count(), 0);
    }

    #[test]
    fn test_add_and_render_widgets() {
        // Arrange
        let mut panel = QuickPanel::new(PanelConfig::default());
        let clock = Box::new(SimpleWidget::new(1, "Clock", WidgetKind::Clock, "14:30"));
        let net = Box::new(SimpleWidget::new(2, "Net", WidgetKind::StatusIndicator, "Connected"));

        // Act
        panel.add_widget(clock).unwrap();
        panel.add_widget(net).unwrap();
        let rendered = panel.render_bar();

        // Assert
        assert_eq!(panel.widget_count(), 2);
        assert_eq!(rendered.len(), 2);
        assert!(rendered[0].contains("Clock"));
        assert!(rendered[1].contains("Net"));
    }

    #[test]
    fn test_duplicate_widget_id_rejected() {
        // Arrange
        let mut panel = QuickPanel::new(PanelConfig::default());
        let w1 = Box::new(SimpleWidget::new(1, "A", WidgetKind::TrayApplet, "1"));
        let w2 = Box::new(SimpleWidget::new(1, "B", WidgetKind::TrayApplet, "2"));

        // Act
        panel.add_widget(w1).unwrap();
        let res = panel.add_widget(w2);

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn test_dispatch_click_routes_to_widget() {
        // Arrange
        let mut panel = QuickPanel::new(PanelConfig::default());
        panel.add_widget(Box::new(SimpleWidget::new(5, "Btn", WidgetKind::Launcher, "Run"))).unwrap();

        // Act
        let click_res = panel.dispatch_click(WidgetId(5)).unwrap();

        // Assert
        assert!(click_res.contains("Clicked Btn"));
    }

    #[test]
    fn test_dispatch_click_nonexistent_returns_err() {
        // Arrange
        let mut panel = QuickPanel::new(PanelConfig::default());

        // Act
        let click_res = panel.dispatch_click(WidgetId(99));

        // Assert
        assert!(click_res.is_err());
    }

    #[test]
    fn test_remove_widget() {
        // Arrange
        let mut panel = QuickPanel::new(PanelConfig::default());
        panel.add_widget(Box::new(SimpleWidget::new(1, "W", WidgetKind::Clock, "time"))).unwrap();

        // Act
        let removed = panel.remove_widget(WidgetId(1));

        // Assert
        assert!(removed.is_some());
        assert_eq!(panel.widget_count(), 0);
    }
}
