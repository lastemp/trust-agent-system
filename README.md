# trust-agent-system

This is a RESTful Actix Web API that connects to MySQL database. 
It is meant to enable project owners to make payments from a single point to different beneficiaries.
This system is useful for artists and other creatives who need surety that they will be paid since the amount was credited to a trusted agent. 

It integrates with Safaricom M-Pesa Mobile Money Payment Gateway (i.e exposed API endpoints for accessing M-Pesa services by Kenyan Telco called "Safaricom")
and enables customers to transfer money and pay for utilities like water, PayTv, electricity from their phone wallets. 
The Kenyan Telco "Safaricom" has provided M-Pesa API endpoints for B2C, C2B and B2B (https://developer.safaricom.co.ke/Documentation). 

Currently this RESTful API supports: 
- add project
- add beneficiary
- add transaction
- get project
- get beneficiary
- get transaction
- initiate business to customer
- handle timeout
- handle result

The RESTful Actix Web API has below listed dependencies:
- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [MySQL](https://github.com/mysql/mysql-server) MySQL database server
- [mysql](https://github.com/blackbeam/rust-mysql-simple) MySql database driver