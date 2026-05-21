imtoken-ai-intent/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── exception.rs         # Completely different error model using bitmaps
    ├── invariant.rs         # Global security rules enforced at compile-time
    ├── intent/
    │   ├── mod.rs
    │   ├── state_machine.rs # FSM tracking user prompt parsing states
    │   ├── lexer.rs         # Low-level byte-scanning of natural language tokens
    │   └── router.rs        # Cross-chain multi-hop AI intent router
    ├── guardrail/
    │   ├── mod.rs
    │   ├── whitelist.rs     # Blazing fast binary-search address validator
    │   └── risk_oracle.rs   # AI slippage and front-running protection shield
    └── main.rs              # Replit interactive CLI loop
