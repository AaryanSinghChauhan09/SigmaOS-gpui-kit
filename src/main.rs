//! SigmaOS-gpui-kit — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_gpui_kit_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-gpui-kit: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-gpui-kit: ok");
}
