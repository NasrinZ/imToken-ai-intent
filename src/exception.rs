// ============================================================================
// MODULE  : Bitmapped Exception Registry for AI Engine
// SUBSYSTEM: High-Performance Diagnostic Low-Level Registers
// ============================================================================

pub type ErrorBitmask = u32;

pub const EXCEPTION_NONE: ErrorBitmask             = 0b000000;
pub const EXCEPTION_LEXER_TIMEOUT: ErrorBitmask    = 0b000001;
pub const EXCEPTION_INVALID_TOKEN_HASH: ErrorBitmask = 0b000010;
pub const EXCEPTION_GUARDRAIL_PHISHING: ErrorBitmask = 0b000100;
pub const EXCEPTION_MEV_SLIPPAGE_HIGH: ErrorBitmask  = 0b001000;
pub const EXCEPTION_ROUTER_ROUTE_NOT_FOUND: ErrorBitmask = 0b010000;
pub const EXCEPTION_CRITICAL_STATE_PANIC: ErrorBitmask   = 0b100000;

#[derive(Debug, Clone, Copy)]
pub struct DiagnosticReport {
    pub active_errors: ErrorBitmask,
    pub instruction_pointer: usize,
}

impl DiagnosticReport {
    pub const fn initialize_clean() -> Self {
        Self { active_errors: EXCEPTION_NONE, instruction_pointer: 0 }
    }

    pub fn raise_exception(&mut self, exception: ErrorBitmask, ip: usize) {
        self.active_errors |= exception;
        self.instruction_pointer = ip;
    }

    pub const fn is_compromised(&self) -> bool {
        self.active_errors != EXCEPTION_NONE
    }
}
