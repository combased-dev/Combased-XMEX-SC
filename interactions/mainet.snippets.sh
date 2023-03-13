PROXY=https://gateway.multiversx.com
CHAIN_ID="1"
WALLET="./wallets/wallet.pem"
ADDRESS=$(mxpy data load --key=address-testnet)
######################################################################

DEPLOY_GAS_LIMIT=450000000

######################################################################
TOKEN_ID="XMEX-fda355"
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

TOKEN_NONCE="2e"
TOKEN_NONCE_HEX="0x${TOKEN_NONCE}"

deploy() {
    mxpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_NONCE_HEX} \
    --outfile="deploy-testnet.interaction.json" \
    --gas-limit=${DEPLOY_GAS_LIMIT}

    ADDRESS=$(mxpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")
    mxpy data store --key=address-testnet --value=${ADDRESS}
}

upgrade() {
    mxpy --verbose contract upgrade "erd1qqqqqqqqqqqqqpgqhpauarfmx75nf4pwxh2fuy520ym03p8e8jcqt466up" --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="upgrade.json" \
    --gas-limit=${DEPLOY_GAS_LIMIT}
}