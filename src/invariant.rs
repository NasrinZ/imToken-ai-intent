// ============================================================================
// MODULE  : Global Execution Invariants
// SUBSYSTEM: Compile-Time Safety Guardrails for Autonomous Agents
// ============================================================================

pub struct SafetyInvariantLimits {
    pub max_allowed_slippage_bps: u16, // Basis points (100 = 1%)
    pub absolute_gas_limit_wei: u64,
    pub layer_2_gas_ceiling_gwei: u32,
}

impl SafetyInvariantLimits {
    pub const fn institutional_policy() -> Self {
        Self {
            max_allowed_slippage_bps: 250, // Max 2.5% slippage rule
            absolute_gas_limit_wei: 5_000_000_000_000_000, // 0.005 ETH Cap
            layer_2_gas_ceiling_gwei: 150,
        }
    }
}
