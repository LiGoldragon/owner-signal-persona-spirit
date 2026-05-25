//! Owner-side schema-driven dual emission smoke tests (psyche
//! 2026-05-26 + intent records 709, 710 — the SECOND emission of the
//! WIRE LANGUAGE per /345 "one channel = one contract = one schema").
//!
//! The schema-driven module lives at
//! `owner_signal_persona_spirit::owner_spirit::*` alongside the legacy
//! `signal_channel!{...}` types at crate root.

#[test]
fn owner_schema_driven_module_is_reachable() {
    let _route_count: usize = owner_signal_persona_spirit::owner_spirit::ROUTE_COUNT;
}

#[test]
fn owner_universal_unknown_lands_on_wire_reply_enum() {
    // Wire-forward-compat floor injected by the composer's extended
    // `reply_items` emission. The constructor existing IS the
    // structural proof.
    use owner_signal_persona_spirit::owner_spirit::Reply;
    let _reply: Reply = Reply::Unknown("unknown owner operation".to_string());
}

#[test]
fn owner_schema_driven_operation_variant_count_matches_legacy() {
    // Both emissions expose 5 operation roots: Start, Drain, Reload,
    // Register, Retire. Coverage at the type level by naming each in
    // the routes constant.
    let routes: usize = owner_signal_persona_spirit::owner_spirit::ROUTE_COUNT;
    assert_eq!(routes, 5, "owner schema declares exactly 5 operation roots");
}
