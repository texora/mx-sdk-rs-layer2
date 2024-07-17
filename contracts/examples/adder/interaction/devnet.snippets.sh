ALICE="${USERS}/alice.pem"
ADDRESS=$(moapy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(moapy data load --key=deployTransaction-devnet)

deploy() {
    moapy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --arguments 0 --send --outfile="deploy-devnet.interaction.json" || return

    TRANSACTION=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(moapy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    moapy data store --key=address-devnet --value=${ADDRESS}
    moapy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

add() {
    read -p "Enter number: " NUMBER
    moapy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --function="add" --arguments ${NUMBER} --send
}

getSum() {
    moapy --verbose contract query ${ADDRESS} --function="getSum"
}
