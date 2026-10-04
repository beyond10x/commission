// generated from commission v1
// model digest a3b6896a96a91b19581506d7622aa6bba27b271020f12d78673c828a61542674
// contract digest 376f5be8445e9cc0e3df87c1f9b7e389768aa141b5e21a3d0cb87fac737a44cd
// do not edit: regenerate with `ess synthesize --layout crate`

//! Semantic types synthesised from the `commission` specification, v1.
//!
//! The responsibility model for governed autonomous workers: an agent revision commissioned to a durable case under a governor, the frontier the governor returns, and the runs that act on it.
//!
//! Generated, not written: the specification is the source of truth, and the door to changing
//! anything here is `ess synthesize`. What is deliberately absent — behaviour, queries,
//! escalations — is listed with reasons in the `PLAN.md` beside this workspace, and every entry
//! there is owed through a typed seam in an `obligations` module here.

// `deny`, not the source workspace's lint set: this crate must hold on its own, and an undocumented
// public item here is an emitter defect worth failing the gate over.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod json;
pub mod primitives;
pub mod responsibility;

