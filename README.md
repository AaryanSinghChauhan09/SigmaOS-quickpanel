# SigmaOS QuickPanel

A Rust data model for a panel configuration and an ordered collection of
widgets. Widgets expose identity, visibility, display text, and a click handler;
the panel supports registration, removal, rendering visible widget text, and
dispatching clicks by ID.

## Library surface

- `Widget` defines the interface implemented by panel widgets.
- `SimpleWidget` stores a label, kind, visibility, display text, and click
  count.
- `QuickPanel` rejects duplicate IDs and stores widgets by `WidgetId`.
- `render_bar()` returns visible widgets as strings, ordered by ID.
- `dispatch_click()` invokes the selected widget's handler and returns its
  result.

```rust
use sigmaos_quickpanel::{PanelConfig, QuickPanel, SimpleWidget, WidgetKind};

let mut panel = QuickPanel::new(PanelConfig::default());
panel.add_widget(Box::new(SimpleWidget::new(
    1,
    "Clock",
    WidgetKind::Clock,
    "14:30",
))).unwrap();
let labels = panel.render_bar();
# let _ = labels;
```

## Integration limits

This crate does not create a graphical surface, draw pixels, position a panel
on a display, read system status, or connect to a Wayland or X11 session. The
rendered strings are a presentation model for a separate desktop integration
layer. Widget handlers are synchronous library callbacks.

## Development

This crate has no third party dependencies. Run `cargo check` to compile the
library and `cargo test` to run its unit tests.
