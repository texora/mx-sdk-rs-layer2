ALICE="/home/dharitri/dharitri-sdk/moapy/testnet/wallets/users/alice.pem"
BOB="/home/dharitri/dharitri-sdk/moapy/testnet/wallets/users/bob.pem"
ADDRESS=$(moapy data load --key=address-testnet-moa-dct-swap)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-testnet)
PROXY=https://testnet-gateway.dharitri.com
CHAIN_ID=T

DCT_SYSTEM_SC_ADDRESS=moa1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls29jpxv

deploy() {
    ######################################################################
    ############################ Update after issue ######################
    ######################################################################
    local WRAPPED_MOA_TOKEN_ID=0x

    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 \
    --arguments ${WRAPPED_MOA_TOKEN_ID} \
    --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    moapy data store --key=address-testnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-testnet-moa-dct-swap --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    moapy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --arguments ${WRAPPED_MOA_TOKEN_ID} --gas-limit=100000000 --outfile="upgrade.json" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

issueWrappedMoa() {
    local TOKEN_DISPLAY_NAME=0x577261707065644d6f61  # "WrappedMoa"
    local TOKEN_TICKER=0x574d4f41  # "WMOA"
    local INITIAL_SUPPLY=0x01 # 1
    local NR_DECIMALS=0x12 # 18
    local CAN_ADD_SPECIAL_ROLES=0x63616e4164645370656369616c526f6c6573 # "canAddSpecialRoles"
    local TRUE=0x74727565 # "true"

    moapy --verbose contract call ${DCT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=60000000 --value=5000000000000000000 --function="issue" \
    --arguments ${TOKEN_DISPLAY_NAME} ${TOKEN_TICKER} ${INITIAL_SUPPLY} ${NR_DECIMALS} ${CAN_ADD_SPECIAL_ROLES} ${TRUE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRoles() {
    local LOCAL_MINT_ROLE=0x444354526f6c654c6f63616c4d696e74 # "DCTRoleLocalMint"
    local LOCAL_BURN_ROLE=0x444354526f6c654c6f63616c4275726e # "DCTRoleLocalBurn"
    local ADDRESS_HEX = $(moapy wallet bech32 --decode ${ADDRESS})

    moapy --verbose contract call ${DCT_SYSTEM_SC_ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments ${WRAPPED_MOA_TOKEN_ID} ${ADDRESS_HEX} ${LOCAL_MINT_ROLE} ${LOCAL_BURN_ROLE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

wrapMoaBob() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} \
    --gas-limit=10000000 --value=1000 --function="wrapMoa" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unwrapMoaBob() {
    local UNWRAP_MOA_ENDPOINT=0x756e777261704d6f61 # "unwrapMoa"
    local UNWRAP_AMOUNT=0x05

    getWrappedMoaTokenIdentifier
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} \
    --gas-limit=10000000 --function="DCTTransfer" \
    --arguments ${TOKEN_IDENTIFIER} ${UNWRAP_AMOUNT} ${UNWRAP_MOA_ENDPOINT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# views

getWrappedMoaTokenIdentifier() {
    local QUERY_OUTPUT=$(moapy --verbose contract query ${ADDRESS} --function="getWrappedMoaTokenId" --proxy=${PROXY})
    TOKEN_IDENTIFIER=0x$(jq -r '.[0] .hex' <<< "${QUERY_OUTPUT}")
    echo "Wrapped MOA token identifier: ${TOKEN_IDENTIFIER}"
}

getLockedMoaBalance() {
    moapy --verbose contract query ${ADDRESS} --function="getLockedMoaBalance" --proxy=${PROXY}
}
