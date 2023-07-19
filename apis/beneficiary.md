# Beneficiaries API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Beneficiary

```sh
curl -d '{"beneficiary_name": "JOHN DOE","mobile_no": "25472*****04","alternate_mobile_no": "","bank_account": "","beneficiary_amount": 50000,"amount_paid": 0,"payment_completed": false}' -H 'Content-Type: application/json' http://localhost:8080/addbeneficiary

http POST :8080/addbeneficiary beneficiary_name="JOHN DOE" mobile_no="25472*****04" alternate_mobile_no="" bank_account="" beneficiary_amount=50000 amount_paid=0 payment_completed=false
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing Beneficiaries

```sh
curl http://localhost:8080/getbeneficiary

http :8080/getbeneficiary
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "beneficiary_data": [
        {
            "beneficiary_name": "JOHN DOE",
            "mobile_no": "25472*****04",
            "alternate_mobile_no": "",
            "bank_account": "",
            "beneficiary_amount": 50000,
            "amount_paid": 0,
            "payment_completed": false
        }
    ]
}
```
