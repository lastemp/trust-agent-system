mod api_layer;
mod db_layer;

use crate::api_layer::BusinessToCustomerInputDetails;
use actix_web::{get, post, web, App, HttpServer, Responder};
use dotenv::dotenv;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::str;

#[derive(Deserialize)]
struct ProjectData {
    project_name: String,
    total_budget: u32,
    funds_deposited: u32,
    mpesa_transaction_reference: String,
    bank_transaction_reference: String,
    is_bank_payment: bool,
    is_active: bool,
    is_closed: bool,
}

#[derive(Deserialize)]
struct BeneficiaryData {
    beneficiary_name: String,
    mobile_no: String,
    alternate_mobile_no: String,
    bank_account: String,
    beneficiary_amount: u32,
    amount_paid: u32,
    payment_completed: bool,
}

#[derive(Deserialize)]
struct TransactionData {
    project_id: u32,
    project_name: String,
    beneficiary_id: u32,
    amount_paid: u32,
    is_bank_payment: bool,
}

#[derive(Deserialize, Debug)]
pub struct ReferenceItemDetails {
    pub Key: String,
    pub Value: String,
}

#[derive(Deserialize, Debug)]
pub struct ReferenceItem {
    pub ReferenceItem: ReferenceItemDetails,
}

#[derive(Deserialize, Debug)]
pub struct ResultParameterDetails {
    pub Key: String,
    pub Value: MixedTypeValue,
}

#[derive(Deserialize, Debug)]
pub struct ResultParameter {
    pub ResultParameter: Vec<ResultParameterDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MixedTypeValue {
    StringValue(String),
    IntegerValue(i32),
    FloatValue(f32),
}

#[derive(Deserialize, Debug)]
pub struct B2CResultDetails {
    pub ResultType: u8,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ResultParameters: ResultParameter,
    pub ReferenceData: ReferenceItem,
}

#[derive(Deserialize, Debug)]
struct B2CResultData {
    Result: B2CResultDetails,
}

#[derive(Deserialize, Debug)]
pub struct B2CFailedDetails {
    pub ResultType: u8,
    pub ResultCode: u32,
    pub ResultDesc: String,
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub TransactionID: String,
    pub ReferenceData: ReferenceItem,
}

#[derive(Deserialize, Debug)]
struct B2CFailedData {
    Result: B2CFailedDetails,
}

// output
#[derive(Serialize)]
pub struct ResponseStatus {
    pub status_code: u8,
    pub status_description: String,
}

#[derive(Serialize)]
pub struct ProjectDetails {
    pub project_name: String,
    pub total_budget: u32,
    pub funds_deposited: u32,
    pub mpesa_transaction_reference: String,
    pub bank_transaction_reference: String,
    pub is_bank_payment: bool,
    pub is_active: bool,
    pub is_closed: bool,
}

#[derive(Serialize)]
pub struct ProjectResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub project_data: Vec<ProjectDetails>,
}

#[derive(Serialize)]
pub struct BeneficiaryDetails {
    pub beneficiary_name: String,
    pub mobile_no: String,
    pub alternate_mobile_no: String,
    pub bank_account: String,
    pub beneficiary_amount: u32,
    pub amount_paid: u32,
    pub payment_completed: bool,
}

#[derive(Serialize)]
pub struct BeneficiaryResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub beneficiary_data: Vec<BeneficiaryDetails>,
}

#[derive(Serialize)]
pub struct TransactionDetails {
    pub project_id: u32,
    pub project_name: String,
    pub beneficiary_name: String,
    pub amount_paid: u32,
    pub is_bank_payment: bool,
}

#[derive(Serialize)]
pub struct TransactionResponseData {
    pub status_code: u8,
    pub status_description: String,
    pub transaction_data: Vec<TransactionDetails>,
}

#[get("/")]
async fn index() -> impl Responder {
    format!("")
}

#[post("/addproject")]
async fn add_project(
    project_data: web::Json<ProjectData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let project_name = &project_data.project_name;
    let total_budget = &project_data.total_budget;
    let funds_deposited = &project_data.funds_deposited;
    let mpesa_transaction_reference = &project_data.mpesa_transaction_reference;
    let bank_transaction_reference = &project_data.bank_transaction_reference;
    let is_bank_payment = &project_data.is_bank_payment;
    let is_active = &project_data.is_active;
    let is_closed = &project_data.is_closed;

    let response_data = db_layer::create_project(
        &data,
        project_name.to_string(),
        *total_budget,
        *funds_deposited,
        mpesa_transaction_reference.to_string(),
        bank_transaction_reference.to_string(),
        *is_bank_payment,
        *is_active,
        *is_closed,
    );

    web::Json(response_data)
}

#[post("/addbeneficiary")]
async fn add_beneficiary(
    beneficiary_data: web::Json<BeneficiaryData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let beneficiary_name = &beneficiary_data.beneficiary_name;
    let mobile_no = &beneficiary_data.mobile_no;
    let alternate_mobile_no = &beneficiary_data.alternate_mobile_no;
    let bank_account = &beneficiary_data.bank_account;
    let beneficiary_amount = &beneficiary_data.beneficiary_amount;
    let amount_paid = &beneficiary_data.amount_paid;
    let payment_completed = &beneficiary_data.payment_completed;

    let response_data = db_layer::create_beneficiary(
        &data,
        beneficiary_name.to_string(),
        mobile_no.to_string(),
        alternate_mobile_no.to_string(),
        bank_account.to_string(),
        *beneficiary_amount,
        *amount_paid,
        *payment_completed,
    );

    web::Json(response_data)
}

#[post("/addtransaction")]
async fn add_transaction(
    transaction_data: web::Json<TransactionData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let project_id = &transaction_data.project_id;
    let project_name = &transaction_data.project_name;
    let beneficiary_id = &transaction_data.beneficiary_id;
    let amount_paid = &transaction_data.amount_paid;
    let is_bank_payment = &transaction_data.is_bank_payment;

    let response_data = db_layer::create_transaction(
        &data,
        *project_id,
        project_name.to_string(),
        *beneficiary_id,
        *amount_paid,
        *is_bank_payment,
    );

    web::Json(response_data)
}

#[get("/initiatebusinesstocustomer")]
async fn initiate_business_to_customer(data: web::Data<Pool>) -> impl Responder {
    let business_to_customer_data = get_business_to_customer_details(&data);

    tokio::spawn(async move {
        // Process each request concurrently.
        api_layer::business_to_customer(data, business_to_customer_data).await;
    });

    format!("")
}

#[get("/getproject")]
async fn get_project(data: web::Data<Pool>) -> impl Responder {
    let project_response_data = db_layer::get_project_data(&data);

    web::Json(project_response_data)
}

#[get("/getbeneficiary")]
async fn get_beneficiary(data: web::Data<Pool>) -> impl Responder {
    let beneficiary_response_data = db_layer::get_beneficiary_data(&data);

    web::Json(beneficiary_response_data)
}

#[get("/gettransaction")]
async fn get_transaction(data: web::Data<Pool>) -> impl Responder {
    let transaction_response_data = db_layer::get_transaction_data(&data);

    web::Json(transaction_response_data)
}

#[post("/b2c/timeout")]
async fn get_b2c_timeout(
    result_data: web::Json<B2CFailedData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let reference_item = &result_data.Result.ReferenceData.ReferenceItem;
    let queue_timeout_url = &reference_item.Value;
    /*
    println!("result_type: {:?}", &result_type);
    println!("result_code: {:?}", &result_code);
    println!("result_desc: {:?}", &result_desc);
    println!(
        "originator_conversation_id: {:?}",
        &originator_conversation_id
    );
    println!("conversation_id: {:?}", &conversation_id);
    println!("transaction_id: {:?}", &transaction_id);
    println!("queue_timeout_url: {:?}", &queue_timeout_url);
    */
    db_layer::create_b2c_timeout(
        &data,
        *result_type,
        *result_code,
        result_desc.to_string(),
        originator_conversation_id.to_string(),
        conversation_id.to_string(),
        transaction_id.to_string(),
        queue_timeout_url.to_string(),
    );
    format!("")
}

#[post("/b2c/result")]
async fn get_b2c_result(
    result_data: web::Json<B2CResultData>,
    data: web::Data<Pool>,
) -> impl Responder {
    //let project_response_data = db_layer::get_project_data(&data);
    //println!("result_data: {:?}", &result_data);
    let result_type = &result_data.Result.ResultType;
    let result_code = &result_data.Result.ResultCode;
    let result_desc = &result_data.Result.ResultDesc;
    let originator_conversation_id = &result_data.Result.OriginatorConversationID;
    let conversation_id = &result_data.Result.ConversationID;
    let transaction_id = &result_data.Result.TransactionID;
    let result_parameters = &result_data.Result.ResultParameters;
    let mut transaction_amount: f32 = 0.0;
    let mut transaction_receipt = String::from("");
    let mut b2c_recipient_is_registered_customer = String::from("");
    let mut b2c_charges_paid_account_available_funds: f32 = 0.0;
    let mut receiver_party_public_name = String::from("");
    let mut transaction_completed_date_time = String::from("");
    let mut b2c_utility_account_available_funds: f32 = 0.0;
    let mut b2c_working_account_available_funds: f32 = 0.0;
    let reference_item = &result_data.Result.ReferenceData.ReferenceItem;
    let queue_timeout_url = &reference_item.Value;

    for result_parameter in result_parameters.ResultParameter.iter() {
        let _key = &result_parameter.Key;
        let _value = &result_parameter.Value;

        //TransactionAmount
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("TransactionAmount"))
        {
            transaction_amount = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }

        //TransactionReceipt
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("TransactionReceipt"))
        {
            transaction_receipt = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //B2CRecipientIsRegisteredCustomer
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CRecipientIsRegisteredCustomer"))
        {
            b2c_recipient_is_registered_customer = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //B2CChargesPaidAccountAvailableFunds
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CChargesPaidAccountAvailableFunds"))
        {
            b2c_charges_paid_account_available_funds = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }

        //ReceiverPartyPublicName
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("ReceiverPartyPublicName"))
        {
            receiver_party_public_name = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //TransactionCompletedDateTime
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("TransactionCompletedDateTime"))
        {
            transaction_completed_date_time = match _value {
                MixedTypeValue::StringValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        //B2CUtilityAccountAvailableFunds
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CUtilityAccountAvailableFunds"))
        {
            b2c_utility_account_available_funds = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }

        //B2CWorkingAccountAvailableFunds
        if _key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("B2CWorkingAccountAvailableFunds"))
        {
            b2c_working_account_available_funds = match _value {
                MixedTypeValue::StringValue(s) => 0.0,
                MixedTypeValue::IntegerValue(i) => *i as f32,
                MixedTypeValue::FloatValue(f) => *f,
                _ => 0.0,
            }
        }
    }

    if transaction_id.replace(" ", "").trim().len() > 0
        && transaction_receipt.replace(" ", "").trim().len() > 0
    {
        // Lets insert each entry
        db_layer::create_b2c_result(
            &data,
            *result_type,
            *result_code,
            result_desc.to_string(),
            originator_conversation_id.to_string(),
            conversation_id.to_string(),
            transaction_id.to_string(),
            transaction_amount,
            transaction_receipt.to_string(),
            b2c_recipient_is_registered_customer.to_string(),
            b2c_charges_paid_account_available_funds,
            receiver_party_public_name.to_string(),
            transaction_completed_date_time.to_string(),
            b2c_utility_account_available_funds,
            b2c_working_account_available_funds,
            queue_timeout_url.to_string(),
        );
    }
    /*
    println!("transaction_amount: {:?}", &transaction_amount);
    println!("transaction_receipt: {:?}", &transaction_receipt);
    println!(
        "b2c_recipient_is_registered_customer: {:?}",
        &b2c_recipient_is_registered_customer
    );
    println!(
        "b2c_charges_paid_account_available_funds: {:?}",
        &b2c_charges_paid_account_available_funds
    );
    println!(
        "receiver_party_public_name: {:?}",
        &receiver_party_public_name
    );
    println!(
        "transaction_completed_date_time: {:?}",
        &transaction_completed_date_time
    );
    println!(
        "b2c_utility_account_available_funds: {:?}",
        &b2c_utility_account_available_funds
    );
    println!(
        "b2c_working_account_available_funds: {:?}",
        &b2c_working_account_available_funds
    );
    println!("queue_timeout_url: {:?}", &queue_timeout_url);
    */
    format!("")
}

fn get_business_to_customer_details(data: &web::Data<Pool>) -> BusinessToCustomerInputDetails {
    let my_access_token: String = String::from("Bearer ***");
    let my_api_url: String =
        String::from("https://sandbox.safaricom.co.ke/mpesa/b2c/v1/paymentrequest");
    let my_initiator_name: String = String::from("testapi");
    let my_security_credential: String = String::from("***");
    let my_command_id: String = String::from("BusinessPayment"); //SalaryPayment, BusinessPayment, PromotionPayment
    let my_amount: u32 = 150;
    let my_party_a: u32 = ***;
    let my_party_b: String = String::from("2547***");
    let my_remarks: String = String::from("Performance payment fees");
    let my_queue_time_out_url: String =
        String::from("https://ef67-154-159-237-160.ngrok.io/b2c/timeout");
    let my_result_url: String = String::from("https://ef67-154-159-237-160.ngrok.io/b2c/result");
    let my_occassion: String = String::from("performance event");

    let business_to_customer_data = BusinessToCustomerInputDetails {
        access_token: my_access_token,
        api_url: my_api_url,
        initiator_name: my_initiator_name,
        security_credential: my_security_credential,
        command_id: my_command_id,
        amount: my_amount,
        party_a: my_party_a,
        party_b: my_party_b,
        _remarks: my_remarks,
        queue_time_out_url: my_queue_time_out_url,
        result_url: my_result_url,
        _occassion: my_occassion,
    };

    business_to_customer_data
}

fn get_conn_builder(
    db_user: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
) -> OptsBuilder {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password));
    builder
}

#[actix_web::main]
async fn main() {
    // get env vars
    dotenv().ok();
    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");
    let db_user = env::var("MYSQL_USER").expect("MYSQL_USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD is not set in .env file");
    let db_host = env::var("MYSQL_HOST").expect("MYSQL_HOST is not set in .env file");
    let my_db_port = env::var("MYSQL_PORT").expect("MYSQL_PORT is not set in .env file");
    let db_name = env::var("MYSQL_DBNAME").expect("MYSQL_DBNAME is not set in .env file");
    let mut http_server_status = String::from("[info] ActixWebHttpServer - Listening for HTTP on ");
    let db_port: u16 = match my_db_port.parse::<u16>() {
        Ok(a) => a,
        Err(e) => 3306, // default mysql server port
    };

    http_server_status.push_str(&server_addr);

    let builder: OptsBuilder = get_conn_builder(db_user, db_password, db_host, db_port, db_name);
    let pool = match Pool::new(builder) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e);
            return;
        }
    };

    let shared_data = web::Data::new(pool);

    let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(index)
            .service(add_project)
            .service(add_beneficiary)
            .service(add_transaction)
            .service(initiate_business_to_customer)
            .service(get_project)
            .service(get_beneficiary)
            .service(get_transaction)
            .service(get_b2c_timeout)
            .service(get_b2c_result)
    })
    .bind(server_addr)
    {
        Ok(s) => {
            println!("{:?}", http_server_status);
            s
        }
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };

    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}
