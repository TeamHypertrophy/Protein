use shadow_rs::ShadowBuilder;

fn main() {
    ShadowBuilder::builder()
        .deny_const(Default::default())
        .build()
        .unwrap();
    println!("cargo:rerun-if-changed=migrations");
}
