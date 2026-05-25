//! OwnerSignal contract for privileged `persona-spirit` lifecycle.
//!
//! Ordinary psyche and intent vocabulary lives in `signal-persona-spirit`.
//! This crate carries supervisor-issued lifecycle and policy orders only.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::{emit_schema, signal_channel};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Generation(u64);

impl Generation {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct IdentityName(String);

impl IdentityName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Start {
    pub generation: Generation,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Drain {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPolicy {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub name: IdentityName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Retirement {
    pub name: IdentityName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Started {
    pub generation: Generation,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DrainedAndStopped {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPolicyReloaded {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct IdentityRegistered {
    pub name: IdentityName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct IdentityRetired {
    pub name: IdentityName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel Owner {
        operation Start(Start),
        operation Drain(Drain),
        operation Reload(BootstrapPolicy),
        operation Register(Registration),
        operation Retire(Retirement),
    }
    reply Reply {
        Started(Started),
        DrainedAndStopped(DrainedAndStopped),
        BootstrapPolicyReloaded(BootstrapPolicyReloaded),
        IdentityRegistered(IdentityRegistered),
        IdentityRetired(IdentityRetired),
        RequestUnimplemented(RequestUnimplemented),
    }
}

// Schema-driven dual emission per psyche 2026-05-26 + intent records
// 709, 710 (the three-language POC). The owner wire schema
// (`owner-spirit.schema`) IS the WIRE LANGUAGE for the permissioned
// socket — the SECOND emission of the first language per /345's
// "one channel = one contract = one schema" discipline. The emitted
// module lands at `owner_signal_persona_spirit::owner_spirit::*`
// alongside the legacy `signal_channel!{...}` emission at crate root.
//
// Downstream consumers may reach for either path:
//
//   Legacy:  owner_signal_persona_spirit::Operation
//   Schema:  owner_signal_persona_spirit::owner_spirit::Operation
//
// The schema engine's extended universal-Unknown carrier check
// injects `Unknown(String)` into the schema-driven `Reply` enum — the
// wire-forward-compat floor matching the actor RESPONSE floor on the
// internal-channel side.
emit_schema!("owner-spirit.schema");
