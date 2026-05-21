// ============================================================================
// MODULE  : Reactive Finite State Machine (FSM)
// SUBSYSTEM: Lifecycle State Transition Manager
// ============================================================================

use crate::intent::TransactionIntentPayload;
use crate::exception::{DiagnosticReport, EXCEPTION_CRITICAL_STATE_PANIC};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineExecutionState {
    IsolatedPromptIngestion,
    InvariantsAuditing,
    DispatchedToMemPool,
}

pub struct AgentLifecycleState {
    pub current_state: EngineExecutionState,
    pub payload_checkpoint: Option<TransactionIntentPayload>,
}

impl AgentLifecycleState {
    pub const fn initialize_fsm() -> Self {
        Self {
            current_state: EngineExecutionState::IsolatedPromptIngestion,
            payload_checkpoint: None,
        }
    }

    /// Validates and advances the system state atomically
    pub fn transition_next(
        &mut self, 
        next_stage: EngineExecutionState, 
        report: &mut DiagnosticReport
    ) {
        match (self.current_state, next_stage) {
            (EngineExecutionState::IsolatedPromptIngestion, EngineExecutionState::InvariantsAuditing) => {
                self.current_state = EngineExecutionState::InvariantsAuditing;
            }
            (EngineExecutionState::InvariantsAuditing, EngineExecutionState::DispatchedToMemPool) => {
                if self.payload_checkpoint.is_none() {
                    report.raise_exception(EXCEPTION_CRITICAL_STATE_PANIC, 0x0B2);
                    return;
                }
                self.current_state = EngineExecutionState::DispatchedToMemPool;
            }
            _ => {
                report.raise_exception(EXCEPTION_CRITICAL_STATE_PANIC, 0x0B3);
            }
        }
    }
}
