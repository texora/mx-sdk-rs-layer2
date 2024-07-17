use std::rc::Rc;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    builtin_func_map::BuiltinFunctionMap,
    builtin_func_role_check_wrapper::BuiltinFunctionRoleCheckWrapper,
    builtin_func_trait::BuiltinFunction,
    dct_nft::{
        DCTLocalBurn, DCTLocalMint, DCTNftAddQuantity, DCTNftAddUri, DCTNftBurn,
        DCTNftCreate, DCTNftUpdateAttributes,
    },
    general::{ChangeOwner, ClaimDeveloperRewards, SetUsername, UpgradeContract},
    transfer::{DCTMultiTransfer, DCTNftTransfer, DCTTransfer},
};

const DCT_ROLE_LOCAL_MINT: &str = "DCTRoleLocalMint";
const DCT_ROLE_LOCAL_BURN: &str = "DCTRoleLocalBurn";
const DCT_ROLE_NFT_CREATE: &str = "DCTRoleNFTCreate";
const DCT_ROLE_NFT_ADD_QUANTITY: &str = "DCTRoleNFTAddQuantity";
const DCT_ROLE_NFT_BURN: &str = "DCTRoleNFTBurn";
const DCT_ROLE_NFT_ADD_URI: &str = "DCTRoleNFTAddURI";
const DCT_ROLE_NFT_UPDATE_ATTRIBUTES: &str = "DCTRoleNFTUpdateAttributes";

fn builtin_function_impls() -> Vec<Box<dyn BuiltinFunction>> {
    vec![
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCT_ROLE_LOCAL_MINT,
            Box::new(DCTLocalMint),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCT_ROLE_LOCAL_BURN,
            Box::new(DCTLocalBurn),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCT_ROLE_NFT_CREATE,
            Box::new(DCTNftCreate),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCT_ROLE_NFT_ADD_QUANTITY,
            Box::new(DCTNftAddQuantity),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCT_ROLE_NFT_BURN,
            Box::new(DCTNftBurn),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCT_ROLE_NFT_ADD_URI,
            Box::new(DCTNftAddUri),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            DCT_ROLE_NFT_UPDATE_ATTRIBUTES,
            Box::new(DCTNftUpdateAttributes),
        )),
        Box::new(DCTMultiTransfer),
        Box::new(DCTNftTransfer),
        Box::new(DCTTransfer),
        Box::new(ChangeOwner),
        Box::new(ClaimDeveloperRewards),
        Box::new(SetUsername),
        Box::new(UpgradeContract),
    ]
}

pub fn init_builtin_functions() -> BuiltinFunctionMap {
    BuiltinFunctionMap::init(builtin_function_impls())
}

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    let builtin_functions = Rc::clone(&tx_cache.blockchain_ref().builtin_functions);
    if let Some(builtin_func) = builtin_functions.get(&tx_input.func_name) {
        builtin_func.execute(tx_input, tx_cache)
    } else {
        default_execution(tx_input, tx_cache)
    }
}
