#[test]
fn generate_kitty_genes_go() {
    dharitri_wasm_debug::denali_go("denali/generate-kitty-genes.scen.json");
}

#[test]
fn init_go() {
    dharitri_wasm_debug::denali_go("denali/init.scen.json");
}
