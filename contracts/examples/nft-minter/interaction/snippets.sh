ALICE="" # PEM path
ADDRESS=$(moapy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-devnet)
PROXY=https://devnet-gateway.dharitri.com
CHAIN_ID=D

deploy() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 \
    --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    moapy data store --key=address-devnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

issueNft() {
    local TOKEN_DISPLAY_NAME=0x4d79546573744e667464  # "MyTestNft"
    local TOKEN_TICKER=0x544553544e4654  # "TESTNFT"

    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 --value=50000000000000000 --function="issueToken" \
    --arguments ${TOKEN_DISPLAY_NAME} ${TOKEN_TICKER} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRoles() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=100000000 --function="setLocalRoles" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

createNft() {
    local TOKEN_NAME=0x4e616d65 # "Name"
    local ROYALTIES=1000 # 10%
    local URI=0x72616e647572692e636f6d # randuri.com
    local SELLING_PRICE=0

    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=50000000 --function="createNft" \
    --arguments ${TOKEN_NAME} ${ROYALTIES} ${URI} ${SELLING_PRICE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

buyNft() {
    local NFT_NONCE=1

    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} \
    --gas-limit=10000000 --function="buyNft" \
    --arguments ${NFT_NONCE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}
