ALICE="${USERS}/alice.pem"
BOB="${USERS}/bob.pem"

ADDRESS=$(moapy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.dharitri.com

DEPLOY_GAS="80000000"
TARGET=10
DEADLINE_UNIX_TIMESTAMP=1609452000 # Fri Jan 01 2021 00:00:00 GMT+0200 (Eastern European Standard Time)
MOA_TOKEN_ID=0x4d4f41 # "MOA"

deploy() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${MOA_TOKEN_ID} \
          --outfile="deploy-testnet.interaction.json" --send --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(moapy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    moapy data store --key=address-testnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

checkDeployment() {
    moapy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']" --proxy=${PROXY}
    moapy account get --address=$ADDRESS --omit-fields="['code']" --proxy=${PROXY}
}

# BOB sends funds
sendFunds() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=10000000 \
        --function="fund" --value=5 \
        --proxy=${PROXY} --chain=T \
        --send
}

# ALICE claims
claimFunds() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="claim" \
        --proxy=${PROXY} --chain=T \
        --send
}

# 0 - Funding Period
# 1 - Successful
# 2 - Failed
status() {
    moapy --verbose contract query ${ADDRESS} --function="status" --proxy=${PROXY} --chain=T
}

getCurrentFunds() {
    moapy --verbose contract query ${ADDRESS} --function="getCurrentFunds" --proxy=${PROXY} --chain=T
}

getTarget() {
    moapy --verbose contract query ${ADDRESS} --function="getTarget" --proxy=${PROXY} --chain=T
}

getDeadline() {
    moapy --verbose contract query ${ADDRESS} --function="getDeadline" --proxy=${PROXY} --chain=T
}

# BOB's deposit
getDeposit() {
    local BOB_ADDRESS_BECH32=moa1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruq0yu4wk
    local BOB_ADDRESS_HEX=0x$(moapy wallet bech32 --decode ${BOB_ADDRESS_BECH32})

    moapy --verbose contract query ${ADDRESS} --function="getDeposit" --arguments ${BOB_ADDRESS_HEX} --proxy=${PROXY} --chain=T
}

getCrowdfundingTokenName() {
    moapy --verbose contract query ${ADDRESS} --function="getCrowdfundingTokenName" --proxy=${PROXY} --chain=T
}
