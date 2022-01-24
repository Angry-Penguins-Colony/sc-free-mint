function concat ()
{
    prefix=$1
    suffix=$2

    echo "${prefix}${suffix}"
}

data=$(concat '{"scAddress":"' $1)
data=$(concat $data '", "funcName":"getNftPrice", "args":[]}')

echo $data

curl -X POST https://gateway.elrond.com/vm-values/int -H "Content-Type: application/json" --data "$data"