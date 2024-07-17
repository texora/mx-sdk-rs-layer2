#[test]
fn external_pure_go() {
    dharitri_wasm_debug::denali_go("denali/external-pure.scen.json");
}

#[test]
fn external_get_go() {
    dharitri_wasm_debug::denali_go("denali/external-get.scen.json");
}
