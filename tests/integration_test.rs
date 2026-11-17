// Integration test for SigmaOS-gpui-kit
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use gpui_kit as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
