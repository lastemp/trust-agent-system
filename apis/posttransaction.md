# PostTransaction API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A PostTransaction

```sh
curl -d '{"project_id": 1,"transaction_id": 1}' -H 'Content-Type: application/json' http://localhost:8080/posttransaction

http POST :8080/posttransaction project_id=1 transaction_id=1
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```
