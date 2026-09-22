//! Interrupt subsystem — IRQ abstraction trait and mock handler.

/// An IRQ line identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrqLine(pub u32);

/// Result of an IRQ handler invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqHandled {
    /// The handler consumed this interrupt.
    Handled,
    /// The handler did not own this interrupt; pass it on.
    NotMine,
}

/// Trait implemented by any component that can handle a hardware interrupt.
pub trait IrqHandler: Send + Sync {
    /// The IRQ line this handler is registered for.
    fn irq_line(&self) -> IrqLine;

    /// Handle an incoming interrupt.
    ///
    /// Called from an interrupt context (or a simulated equivalent in
    /// userspace). Implementations MUST be fast and MUST NOT block.
    fn handle(&self) -> IrqHandled;
}

/// A mock IRQ handler for testing.
///
/// Records whether it was invoked and always returns `Handled`.
#[derive(Debug, Default)]
pub struct MockIrqHandler {
    line: u32,
    /// Number of times `handle` was called.
    pub call_count: std::sync::atomic::AtomicU32,
}

impl MockIrqHandler {
    /// Create a new mock handler for `line`.
    pub fn new(line: u32) -> Self {
        Self { line, call_count: std::sync::atomic::AtomicU32::new(0) }
    }
}

impl IrqHandler for MockIrqHandler {
    fn irq_line(&self) -> IrqLine { IrqLine(self.line) }

    fn handle(&self) -> IrqHandled {
        self.call_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        IrqHandled::Handled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_handler_records_calls() {
        let h = MockIrqHandler::new(5);
        assert_eq!(h.irq_line(), IrqLine(5));
        assert_eq!(h.handle(), IrqHandled::Handled);
        assert_eq!(h.handle(), IrqHandled::Handled);
        assert_eq!(h.call_count.load(std::sync::atomic::Ordering::Relaxed), 2);
    }
}
