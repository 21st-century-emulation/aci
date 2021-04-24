docker build -q -t aci .
docker run --rm --name aci -d -p 8080:8080 aci

RESULT=`curl -s --header "Content-Type: application/json" \
  --request POST \
  --data '{"id":"abcd", "opcode":128,"state":{"a":20,"b":1,"c":0,"d":5,"e":15,"h":10,"l":20,"flags":{"sign":false,"zero":false,"auxCarry":false,"parity":false,"carry":true},"programCounter":1,"stackPointer":2,"cycles":0}}' \
  http://localhost:8080/api/v1/execute?operand1=66`
EXPECTED='{"id":"abcd", "opcode":128,"state":{"a":87,"b":1,"c":0,"d":5,"e":15,"h":10,"l":20,"flags":{"sign":false,"zero":false,"auxCarry":false,"parity":false,"carry":false},"programCounter":1,"stackPointer":2,"cycles":7}}'

docker kill aci

DIFF=`diff <(jq -S . <<< "$RESULT") <(jq -S . <<< "$EXPECTED")`

if [ $? -eq 0 ]; then
    echo -e "\e[32mACI Test Pass \e[0m"
    exit 0
else
    echo -e "\e[31mACI Test Fail  \e[0m"
    echo "$RESULT"
    echo "$DIFF"
    exit -1
fi