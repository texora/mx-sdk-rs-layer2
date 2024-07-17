use dharitri_wasm_debug::{dharitri_wasm::types::Address, denali::value_interpreter::keccak256};

#[cfg(test)]
use dharitri_wasm_debug::bech32;

fn get_initial_dns_address() -> Address {
    Address::from_slice(&[1u8; 32])
}

fn compute_smart_contract_address(owner_address: Address, owner_nonce: u64) -> Address {
    // 8 bytes of zero + 2 bytes for VM type + 20 bytes of hash(owner) + 2 bytes of shard(owner)
    let owner_bytes = owner_address.as_bytes();
    let nonce_bytes = owner_nonce.to_le_bytes();
    let bytes_to_hash = [owner_bytes, &nonce_bytes].concat();
    let initial_padding = [0u8; 8];
    let vm_type: [u8; 2] = [5, 0];
    let address = keccak256(&bytes_to_hash);
    let address = [
        initial_padding.as_slice(),
        vm_type.as_slice(),
        &address[10..30],
        &owner_bytes[30..],
    ]
    .concat();
    Address::from_slice(&address)
}

fn compute_dns_address_for_shard_id(shard_id: u8) -> Address {
    let initial_dns_address = get_initial_dns_address();
    let initial_dns_address_slice = initial_dns_address.as_array();
    let shard_identifier = &[0u8, shard_id];
    let deployer_pubkey_prefix =
        &initial_dns_address_slice[0..initial_dns_address_slice.len() - shard_identifier.len()];

    let deployer_pubkey = [deployer_pubkey_prefix, shard_identifier].concat();
    let deployer_address = Address::from_slice(&deployer_pubkey);
    let deployer_nonce = 0;
    compute_smart_contract_address(deployer_address, deployer_nonce)
}

fn shard_id_from_name(name: &str) -> u8 {
    let name_hash = keccak256(name.as_bytes());
    name_hash[31]
}

pub fn dns_address_for_name(name: &str) -> Address {
    let shard_id = shard_id_from_name(name);
    compute_dns_address_for_shard_id(shard_id)
}

#[test]
fn test_compute_dns_address() {
    assert_eq!(
        bech32::encode(&compute_dns_address_for_shard_id(0)),
        "moa1qqqqqqqqqqqqqpgqnhvsujzd95jz6fyv3ldmynlf97tscs9nqqqqcalux7"
    );
    assert_eq!(
        bech32::encode(&compute_dns_address_for_shard_id(1)),
        "moa1qqqqqqqqqqqqqpgqysmcsfkqed279x6jvs694th4e4v50p4pqqqstkzp8l"
    );
    assert_eq!(
        bech32::encode(&compute_dns_address_for_shard_id(2)),
        "moa1qqqqqqqqqqqqqpgqnk5fq8sgg4vc63ffzf7qez550xe2l5jgqqpq5vhzys"
    );
}

#[test]
fn test_dns_for_name() {
    assert_eq!(
        bech32::encode(&dns_address_for_name("test.dharitri")),
        "moa1qqqqqqqqqqqqqpgqp64e3pqxwwyy93t5wp2w2jnlf4lfx3ljqqgs6lxpsj"
    );
    assert_eq!(
        bech32::encode(&dns_address_for_name("helloworld.dharitri")),
        "moa1qqqqqqqqqqqqqpgqrx5fz9lsd0nz526wmcjnj5cv6as3y2qkqzhqe5tved"
    );
}
