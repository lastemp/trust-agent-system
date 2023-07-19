# MySQL

This RESTful Actix Web API that connects to MySQL database.
It is meant to enable project owners to make payments from a single point to different beneficiaries.
This system is useful for artists and other creatives who need surety that they will be paid since the amount was credited to a trusted agent. 

You'll need to have a MySQL (or compatible) server running on your machine to test this example.

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../trust-agent-system
```

1. Create database and tables:

   The `sql` directory contains the SQL files used for database setup:
   
   Database
   ```sh
   mysql -u root -p < sql/0_create_database.sql
   ```
   
   Tables
   ```sh
   mysql -u root -p ebusiness_payments < sql/tables/b2c_acknowledgement_details.sql
   mysql -u root -p ebusiness_payments < sql/tables/b2c_result_details.sql
   mysql -u root -p ebusiness_payments < sql/tables/b2c_timeout_details.sql
   mysql -u root -p ebusiness_payments < sql/tables/beneficiary_details.sql
   mysql -u root -p ebusiness_payments < sql/tables/mpesa_access_token.sql
   mysql -u root -p ebusiness_payments < sql/tables/mpesa_access_token_archive.sql
   mysql -u root -p ebusiness_payments < sql/tables/project_details.sql
   mysql -u root -p ebusiness_payments < sql/tables/settings.sql
   mysql -u root -p ebusiness_payments < sql/tables/transaction_details.sql
   ```
   
   Stored procedures
   ```sh
   mysql -u root -p ebusiness_payments < sql/stored-procedures/getmpesaaccesstoken.sql
   mysql -u root -p ebusiness_payments < sql/stored-procedures/getposttransactiondetails.sql
   mysql -u root -p ebusiness_payments < sql/stored-procedures/getsettings.sql
   mysql -u root -p ebusiness_payments < sql/stored-procedures/insertbeneficiarydetails.sql
   mysql -u root -p ebusiness_payments < sql/stored-procedures/insertprojectdetails.sql
   mysql -u root -p ebusiness_payments < sql/stored-procedures/inserttransactiondetails.sql
   mysql -u root -p ebusiness_payments < sql/stored-procedures/insertupdatempesaaccesstoken.sql
   ```

   For each step you will be prompted for the root user's password. If there's no password set on the root use, just hit enter again.

1. Create a `.env` file in this this directory:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=root
   MYSQL_PASSWORD=<password>
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=ebusiness_payments
   ```

   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.

1. Run the server:

   ```sh
   cargo run
   ```

1. Using a different terminal send requests to the running server. For example, using [HTTPie]:

   ```sh
   http POST :8080/posttransaction project_id=1 transaction_id=1
   ```

   See [the API documentation pages](./apis/) for more info.

[HTTPie]: https://httpie.io/cli
