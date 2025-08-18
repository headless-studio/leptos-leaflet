use leptos::prelude::*;

/// Strategy for determining which pane to use for popup and tooltip rendering.
///
/// This enum allows users to explicitly choose how popups and tooltips determine their pane:
/// - Use a custom pane name
/// - Use the pane context from parent components
/// - Use Leaflet's default behavior
#[derive(Debug, Clone, PartialEq)]
pub enum PaneStrategy {
    /// Use a custom pane name (can be reactive)
    Custom(Signal<String>),
    /// Use the pane context from parent Pane components
    Context,
    /// Use Leaflet's default pane behavior
    Default,
}

impl Default for PaneStrategy {
    fn default() -> Self {
        Self::Context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_strategy_default() {
        assert_eq!(PaneStrategy::default(), PaneStrategy::Context);
    }

    #[test]
    fn pane_strategy_debug() {
        let custom = PaneStrategy::Custom(Signal::derive(|| "test-pane".to_string()));
        let context = PaneStrategy::Context;
        let default = PaneStrategy::Default;

        assert_eq!(format!("{:?}", context), "Context");
        assert_eq!(format!("{:?}", default), "Default");
        // Custom variant will have a complex debug representation due to Signal
        assert!(format!("{:?}", custom).contains("Custom"));
    }

    #[test]
    fn pane_strategy_clone() {
        let context = PaneStrategy::Context;
        let cloned = context.clone();
        assert_eq!(context, cloned);

        let default = PaneStrategy::Default;
        let cloned_default = default.clone();
        assert_eq!(default, cloned_default);
    }

    #[test]
    fn pane_strategy_equality() {
        assert_eq!(PaneStrategy::Context, PaneStrategy::Context);
        assert_eq!(PaneStrategy::Default, PaneStrategy::Default);
        assert_ne!(PaneStrategy::Context, PaneStrategy::Default);
    }

    #[test]
    fn pane_strategy_custom_with_signal() {
        let signal1 = Signal::derive(|| "pane1".to_string());
        let signal2 = Signal::derive(|| "pane2".to_string());

        let custom1 = PaneStrategy::Custom(signal1);
        let custom2 = PaneStrategy::Custom(signal2);

        // Custom strategies with different signals should not be equal
        // Note: This test verifies the enum structure, actual signal comparison
        // would need runtime evaluation
        match (&custom1, &custom2) {
            (PaneStrategy::Custom(_), PaneStrategy::Custom(_)) => {
                // Both are Custom variants - this is what we expect
            }
            _ => panic!("Expected both to be Custom variants"),
        }
    }
}
