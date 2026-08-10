//! Parity gate for `cairn onboard decisions`: the onboard owner resolver agrees
//! with the reconciler's most-specific-prefix ownership.
//!
//! `src/reconcile/generic.rs` keeps `eligible_owners` and `most_specific_owner`
//! private, so `brownfield::decisions::OwnerResolver` reimplements the rule
//! (`dec.brownfield-extraction-mechanism` clause 1). The expectations below are
//! the same dogfood-blueprint fixture expectations `tests/rung3_node_declarations.rs`
//! pins for the reconciler side, so the two implementations cannot drift apart
//! silently.

use std::{collections::BTreeMap, path::Path};

use cairn::{
    artefacts::contract::ContractSet, blueprint, brownfield::decisions::OwnerResolver, map,
};

fn dogfood_graph() -> map::Graph {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ast =
        blueprint::parse_file(root.join("cairn.blueprint")).expect("dogfood blueprint parses");
    let contracts = ContractSet::default();
    let mut claimed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    map::build_graph(&ast, root, &contracts, &mut claimed, Vec::new())
}

#[test]
fn resolver_binds_the_registries_registry_like_the_reconciler() {
    let graph = dogfood_graph();
    let resolver = OwnerResolver::new(&graph);
    assert_eq!(
        resolver.owner_of("docs/registries/error-codes.md"),
        Some("cairn.registries")
    );
}

#[test]
fn resolver_binds_the_blueprint_itself_like_the_reconciler() {
    let graph = dogfood_graph();
    let resolver = OwnerResolver::new(&graph);
    assert_eq!(
        resolver.owner_of("cairn.blueprint"),
        Some("cairn.blueprint-source")
    );
}

#[test]
fn resolver_prefers_the_most_specific_declared_path() {
    let graph = dogfood_graph();
    let resolver = OwnerResolver::new(&graph);
    assert_eq!(
        resolver.owner_of("docs/design-system/copy.toml"),
        Some("cairn.design-copy"),
        "the hotspot copy table outranks any shorter ancestor path"
    );
}

#[test]
fn resolver_reports_unowned_paths_rather_than_inventing_a_binding() {
    let graph = dogfood_graph();
    let resolver = OwnerResolver::new(&graph);
    assert_eq!(
        resolver.owner_of("docs/design-system/tokens.css"),
        None,
        "only the copy table is hotspot-owned, not the whole design system"
    );
}
