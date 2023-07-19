# Projects API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Project

```sh
curl -d '{"project_name": "Kisumu performance - July 2023","total_budget": 50000,"funds_deposited": 50000,"mpesa_transaction_reference": "NBSQDM7W2B0","bank_transaction_reference": "","is_bank_payment": false,"is_active": true,"is_closed": false}' -H 'Content-Type: application/json' http://localhost:8080/addproject

http POST :8080/addproject project_name="Kisumu performance - July 2023" total_budget=50000 funds_deposited=50000 mpesa_transaction_reference="NBSQDM7W2B0" bank_transaction_reference="" is_bank_payment=false is_active=true is_closed=false
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Projects

```sh
curl http://localhost:8080/getproject

http :8080/getproject
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "project_data": [
        {
            "project_name": "KISUMU PERFORMANCE - JULY 2023",
            "total_budget": 50000,
            "funds_deposited": 50000,
            "mpesa_transaction_reference": "NBSQDM7W2B0",
            "bank_transaction_reference": "",
            "is_bank_payment": false,
            "is_active": true,
            "is_closed": false
        }
    ]
}
```
