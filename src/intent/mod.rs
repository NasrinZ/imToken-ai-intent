// ============================================================================
// MODULE  : Intent Registry Lifecycle Coordinate
// SUBSYSTEM: Reactive State Machine Specifications
// ============================================================================

pub mod lexer;
pub mod state_machine;
pub mod router;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedAssetType {
    NativeEth,
    StableUsdt,
    StableUsdc,
    WrappedBtc,
}

#[derive(Debug, Clone, Copy)]
pub struct TransactionIntentPayload {
    pub target_action: u8; // 1 = Swap, 2 = Bridge, 3 = Stake
    pub asset_source: ParsedAssetType,
    pub asset_destination: ParsedAssetType,
    pub numeric_volume: u64,
    pub target_recipient_address: [u8; 20],
}
