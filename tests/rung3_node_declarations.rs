//! Scanner-effect gate for `dec.rung-three-node-declarations`: the dogfood
//! blueprint's hotspot owner nodes attribute their files, including the
//! self-declaration case where `cairn.blueprint` is owned by a node that the
//! same file declares. The predicate below mirrors the reconciler's
//! `eligible_owners` and `most_specific_owner` semantics (sorted
//! most-specific path first, component-boundary containment); the absence of
//! new `CAIRN_RECONCILE_ORPHANED_FILE` findings is asserted by the
//! `cairn scan --strict` gate, not here.

use std::path::Path;

fn collect(node: &cairn::blueprint::Node, owners: &mut Vec<(String, String)>) {
    let is_internal = !node.children.is_empty();
    if !is_internal || node.owns_files {
        for path in &node.paths {
            owners.push((node.id.clone(), path.trim_start_matches("./").to_owned()));
        }
    }
    for child in &node.children {
        collect(child, owners);
    }
}

fn dogfood_owners() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let ast = cairn::blueprint::parse_file(root.join("cairn.blueprint"))
        .expect("dogfood blueprint parses");
    let mut owners = Vec::new();
    for node in &ast.nodes {
        collect(node, &mut owners);
    }
    owners.sort_by_key(|(_, path)| std::cmp::Reverse(path.len()));
    owners
}

fn most_specific_owner(owners: &[(String, String)], file: &str) -> Option<String> {
    for (id, path) in owners {
        if path.is_empty()
            || path == "."
            || file == path
            || (file.starts_with(path.as_str()) && file.as_bytes().get(path.len()) == Some(&b'/'))
        {
            return Some(id.clone());
        }
    }
    None
}

#[test]
fn registries_node_owns_error_codes_registry() {
    let owners = dogfood_owners();
    assert_eq!(
        most_specific_owner(&owners, "docs/registries/error-codes.md").as_deref(),
        Some("cairn.registries")
    );
}

#[test]
fn blueprint_source_node_owns_the_blueprint_itself() {
    let owners = dogfood_owners();
    assert_eq!(
        most_specific_owner(&owners, "cairn.blueprint").as_deref(),
        Some("cairn.blueprint-source")
    );
}

#[test]
fn design_copy_node_owns_the_copy_table() {
    let owners = dogfood_owners();
    assert_eq!(
        most_specific_owner(&owners, "docs/design-system/copy.toml").as_deref(),
        Some("cairn.design-copy")
    );
}

#[test]
fn hotspot_ownership_does_not_leak_to_siblings() {
    let owners = dogfood_owners();
    assert_eq!(
        most_specific_owner(&owners, "docs/design-system/tokens.css"),
        None,
        "only the copy table is hotspot-owned, not the whole design system"
    );
}
