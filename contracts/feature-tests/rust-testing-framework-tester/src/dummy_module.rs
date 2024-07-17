dharitri_wasm::imports!();

#[dharitri_wasm::module]
pub trait DummyModule {
    fn some_function(&self) -> BigUint {
        BigUint::zero()
    }
}
