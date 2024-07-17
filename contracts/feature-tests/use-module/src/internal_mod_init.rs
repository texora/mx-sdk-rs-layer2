dharitri_wasm::imports!();

/// Example of a module that contains the constructor.
#[dharitri_wasm::module]
pub trait InternalModuleInit {
    /// The constructor can reside in a module.
    /// The method can have any name.
    #[init]
    fn constructor_in_a_module(&self) {}
}
