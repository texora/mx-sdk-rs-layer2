ALICE="${USERS}/alice.pem"
BOB="${USERS}/bob.pem"

ADDRESS=$(moapy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-devnet)

DEPLOY_GAS="80000000"
TARGET=10
DEADLINE_UNIX_TIMESTAMP=1609452000 # Fri Jan 01 2021 00:00:00 GMT+0200 (Eastern European Standard Time)
MOA_TOKEN_ID=0x4d4f41 # "MOA"

deploy() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${MOA_TOKEN_ID} \
          --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    moapy data store --key=address-devnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deploySimulate() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${MOA_TOKEN_ID} \
          --outfile="simulate-devnet.interaction.json" --simulate || return

    TRANSACTION=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['hash']")
    ADDRESS=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['contractAddress']")
    RETCODE=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnCode']")
    RETMSG=$(moapy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnMessage']")

    echo ""
    echo "Simulated transaction: ${TRANSACTION}"
    echo "Smart contract address: ${ADDRESS}"
    echo "Deployment return code: ${RETCODE}"
    echo "Deployment return message: ${RETMSG}"
}

checkDeployment() {
    moapy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    moapy account get --address=$ADDRESS --omit-fields="['code']"
}

# BOB sends funds
sendFunds() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=10000000 \
        --function="fund" --value=5 \
        --send
}

# ALICE claims
claimFunds() {
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="claim" \
        --send
}

# 0 - Funding Period
# 1 - Successful
# 2 - Failed
status() {
    moapy --verbose contract query ${ADDRESS} --function="status"
}

getCurrentFunds() {
    moapy --verbose contract query ${ADDRESS} --function="getCurrentFunds"
}

getTarget() {
    moapy --verbose contract query ${ADDRESS} --function="getTarget"
}

getDeadline() {
    moapy --verbose contract query ${ADDRESS} --function="getDeadline"
}

# BOB's deposit
getDeposit() {
    local BOB_ADDRESS_BECH32=moa1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruq0yu4wk
    local BOB_ADDRESS_HEX=0x$(moapy wallet bech32 --decode ${BOB_ADDRESS_BECH32})

    moapy --verbose contract query ${ADDRESS} --function="getDeposit" --arguments ${BOB_ADDRESS_HEX}
}

getCrowdfundingTokenName() {
    moapy --verbose contract query ${ADDRESS} --function="getCrowdfundingTokenName"
}
