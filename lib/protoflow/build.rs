// This is free and unencumbered software released into the public domain.

use std::collections::BTreeSet;

fn main() -> std::io::Result<()> {
    // See: https://github.com/baoyachi/shadow-rs
    // Omit all nonpublic and/or sensitive information:
    let mut omit = BTreeSet::new();
    omit.insert(shadow_rs::CARGO_TREE);
    omit.insert(shadow_rs::CARGO_MANIFEST_DIR);
    omit.insert(shadow_rs::COMMIT_AUTHOR);
    omit.insert(shadow_rs::COMMIT_EMAIL);
    omit.insert(shadow_rs::GIT_STATUS_FILE);
    shadow_rs::new_deny(omit).unwrap();

    Ok(())
}
