# Transactions API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Transaction

```sh
curl -d '{"project_id": 1,"project_name": "Kisumu performance - July 2023","beneficiary_id": 1,"amount_paid": 50000,"is_bank_payment": false}' -H 'Content-Type: application/json' http://localhost:8080/addtransaction

http POST :8080/addtransaction project_id=1 project_name="Kisumu performance - July 2023" beneficiary_id=1 amount_paid=50000 is_bank_payment=false
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Transactions

```sh
curl http://localhost:8080/gettransaction

http :8080/gettransaction
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "transaction_data": [
        {
            "project_id": 1,
            "project_name": "Kisumu performance - July 2023",
            "beneficiary_name": "JOHN DOE",
            "amount_paid": 50000,
            "is_bank_payment": false
        }
    ]
}
```
