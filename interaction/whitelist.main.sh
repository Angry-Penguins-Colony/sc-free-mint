PROXY="https://gateway.elrond.com"
CHAIN_ID="1"


SC_ADDRESS="erd1qqqqqqqqqqqqqpgq2tywwckyz6ynw6zglymvyae4029yxwmwf0tqe6ahc4"
WALLET="../../../wallet/wallet-owner.pem" 

address="0x$(erdpy wallet bech32 --decode $1)"

erdpy contract call $SC_ADDRESS --recall-nonce --proxy=${PROXY} --chain=${CHAIN_ID} --pem=${WALLET} --function="whitelistAddress" --gas-limit 10000000 --arguments $address --send

echo "address: $1 add to whitelist"