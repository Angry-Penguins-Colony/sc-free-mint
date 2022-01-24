str=$1
hex=$(xxd -pu <<< "$str")
hex=$(tr -d '\n' <<< "$hex")
hex="0x${hex}"
printf '\n'
echo $hex
printf "\n"

erdpy contract query erd1qqqqqqqqqqqqqpgqudt5tygshx22efvu9gg0ac3d9ntk5cq2f0tqe8r6qs  --proxy "https://devnet-api.elrond.com" --function "isWhitelisted" --arguments $hex