set -xe

curl -X POST http://localhost:9001/echo \
     -H "Content-Type: application/json" \
     -d '{"key1": "value1", "key2":32}'
