#[macro_export]
macro_rules! wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        pub use dharitri_wasm_output;

        #[no_mangle]
        fn init() {
            $mod_name::endpoints::init::<dharitri_wasm_node::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<dharitri_wasm_node::VmApiImpl>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        pub use dharitri_wasm_output;

        #[no_mangle]
        fn init() {
            dharitri_wasm_node::dharitri_wasm::external_view_contract::external_view_contract_constructor::<dharitri_wasm_node::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<dharitri_wasm_node::dharitri_wasm::api::ExternalViewApi<dharitri_wasm_node::VmApiImpl>>();
            }
        )*
    };
}

#[macro_export]
macro_rules! wasm_empty_callback {
    () => {
        #[allow(non_snake_case)]
        #[no_mangle]
        fn callBack() {}
    };
}
