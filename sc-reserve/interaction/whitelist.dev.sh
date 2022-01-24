PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

SC_ADDRESS="erd1qqqqqqqqqqqqqpgqp60cl29mkxj882du8w94a6xydt3fpyjyf0tq0u4wh9"
WALLET="../../wallet/wallet-owner.pem" 

address="0x$(erdpy wallet bech32 --decode $1)"

erdpy contract call $SC_ADDRESS --recall-nonce --pem=${WALLET} --proxy=${PROXY} --chain=${CHAIN_ID}  --function="whitelistAddress" --gas-limit 10000000 --arguments $address --send