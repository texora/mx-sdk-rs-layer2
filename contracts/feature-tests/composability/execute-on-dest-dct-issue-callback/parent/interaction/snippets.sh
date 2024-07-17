ALICE="/home/dharitri/dharitri-sdk/moapy/testnet/wallets/users/alice.pem"
ADDRESS=$(moapy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-testnet)
PROXY=http://localhost:7950
CHAIN_ID=local-testnet

CHILD_CODE=0x"$(xxd -p ../child/output/child.wasm | tr -d '\n')"
DCT_ISSUE_COST=5000000000000000000

TOKEN_DISPLAY_NAME=0x577261707065644d6f61  # "WrappedMoa"
TOKEN_TICKER=0x574d4f41  # "WMOA"
INITIAL_SUPPLY=0x03e8 # 1000

deployParent() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --outfile="deploy-testnet.interaction.json" --send --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    moapy data store --key=address-testnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deployChildThroughParent() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=400000000 --function="deployChildContract" --arguments ${CHILD_CODE} --send --outfile="deploy-child-sc-spam.json" --proxy=${PROXY} --chain=${CHAIN_ID}
}

executeOnDestIssueToken() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=200000000 --value=${DCT_ISSUE_COST} --function="executeOnDestIssueToken" --arguments ${TOKEN_DISPLAY_NAME} ${TOKEN_TICKER} ${INITIAL_SUPPLY} --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

getChildContractAddress() {
    local QUERY_OUTPUT=$(moapy --verbose contract query ${ADDRESS} --function="getChildContractAddress" --proxy=${PROXY})
    parseQueryOutput
    parsedAddressToBech32

    CHILD_ADDRESS=${ADDRESS_BECH32}
    echo "Child address: ${CHILD_ADDRESS}"
}

getWrappedMoaTokenIdentifier() {
    getChildContractAddress
    moapy --verbose contract call ${CHILD_ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --function="getWrappedMoaTokenIdentifier" --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

# helpers

parseQueryOutput() {
    PARSED=$(jq -r '.[0].hex' <<< "${QUERY_OUTPUT}")
}

parsedAddressToBech32() {
    ADDRESS_BECH32=$(moapy wallet bech32 --encode ${PARSED})
}
