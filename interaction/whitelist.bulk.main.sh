array=( erd16fmwvt78d74ky7htwchjkgenke09qefvvnzrznmetkx04ktahmtq4saemk erd1my2pw4r7sl66d8lwvulwesd08skncvwpg5dy7q5yk5l0jn67x59sh6042k 
erd1v8x6j4du9ep4md2le8hhqqlxu949sk5jmju6hedx42yhfqhj3ssscu0fdu
erd1ar02cwjlzltk46ghp7tgg3lvpq4jfk3dll2hv0ns60ldkmy697cs24lhs4
erd1r8ktxe5z723u5ps4rc203n8xajk03ag5fp5xyklf0c2qhc62k6ksuvsuz4
erd1djaulckxkuqlkcxyx7w90ht23hggkkmf8ttnm93atekn5glhsvmqpns58y
erd1e6aprf50uf68m6amykedq2rdu5kvcqs8p34swr24g33at3jx4zvss7rxdj)

for i in "${array[@]}"
do
    :
    ./whitelist.main.sh $i
done