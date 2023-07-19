use crate::api_layer::{business_to_customer, generate_auth_token};

use crate::{
    models::{
        B2CFailedData, B2CResultData, BeneficiaryData, BusinessToCustomerInputDetails,
        MixedTypeValue, PostTransactionData, ProjectData, ResponseStatus, TransactionData,
    },
    persistence::{
        create_b2c_result, create_b2c_timeout, create_beneficiary, create_project,
        create_transaction, get_beneficiary_data, get_mpesa_access_token, get_post_transaction,
        get_project_data, get_settings_details, get_transaction_data,
    },
};

use actix_web::{get, post, web, Responder};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use mysql::*;
use std::str;

const TRANSACTION_COMMAND_ID: &str = "BusinessPayment"; //SalaryPayment, BusinessPayment, PromotionPayment

const TRANSACTION_REMARKS: &str = "Performance payment fees";

const TRANSACTION_OCCASSION: &str = "Performance payment fees";
const AUTHORISATION_BEARER: &str = "Bearer";

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    format!("")
}

#[post("/addproject")]
pub(crate) async fn add_project(
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

    let response_data = create_project(
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
pub(crate) async fn add_beneficiary(
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

    let response_data = create_beneficiary(
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
pub(crate) async fn add_transaction(
    transaction_data: web::Json<TransactionData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let project_id = &transaction_data.project_id;
    let project_name = &transaction_data.project_name;
    let beneficiary_id = &transaction_data.beneficiary_id;
    let amount_paid = &transaction_data.amount_paid;
    let is_bank_payment = &transaction_data.is_bank_payment;

    let response_data = create_transaction(
        &data,
        *project_id,
        project_name.to_string(),
        *beneficiary_id,
        *amount_paid,
        *is_bank_payment,
    );

    web::Json(response_data)
}

#[post("/posttransaction")]
pub(crate) async fn post_transaction(
    transaction_data: web::Json<PostTransactionData>,
    data: web::Data<Pool>,
) -> impl Responder {
    let project_id = &transaction_data.project_id;
    let transaction_id = &transaction_data.transaction_id;
    let my_status_code: u8 = 1;
    let my_status_description: String =
        String::from("Error occured during processing, please try again.");

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    let response_data = get_post_transaction(&data, *project_id, *transaction_id);
    if response_data.status_code == 0
        && response_data.amount_paid > 0
        && response_data.mobile_no.len() > 0
    {
        let mobile_no = &response_data.mobile_no;
        let amount_paid = &response_data.amount_paid;
        let command_id = TRANSACTION_COMMAND_ID.to_string();
        let _remarks = TRANSACTION_REMARKS.to_string();
        let _occassion = TRANSACTION_OCCASSION.to_string();
        let mut access_token = AUTHORISATION_BEARER.to_string();
        let k = " "; // Separator
        let password: String = get_mpesa_access_token(&data);
        access_token.push_str(k);
        access_token.push_str(&password);
        let business_to_customer_data = get_business_to_customer_details(
            &data,
            mobile_no.to_string(),
            *amount_paid,
            command_id.to_string(),
            _remarks.to_string(),
            _occassion.to_string(),
            access_token.to_string(),
        );

        tokio::spawn(async move {
            // Process each request concurrently.
            business_to_customer(data, business_to_customer_data).await;
        });
    }

    response_status.status_code = response_data.status_code;
    response_status.status_description = response_data.status_description;

    web::Json(response_status)
}

#[get("/getproject")]
pub(crate) async fn get_project(data: web::Data<Pool>) -> impl Responder {
    let project_response_data = get_project_data(&data);

    web::Json(project_response_data)
}

#[get("/getbeneficiary")]
pub(crate) async fn get_beneficiary(data: web::Data<Pool>) -> impl Responder {
    let beneficiary_response_data = get_beneficiary_data(&data);

    web::Json(beneficiary_response_data)
}

#[get("/gettransaction")]
pub(crate) async fn get_transaction(data: web::Data<Pool>) -> impl Responder {
    let transaction_response_data = get_transaction_data(&data);

    web::Json(transaction_response_data)
}

#[post("/b2c/timeout")]
pub(crate) async fn get_b2c_timeout(
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

    create_b2c_timeout(
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
pub(crate) async fn get_b2c_result(
    result_data: web::Json<B2CResultData>,
    data: web::Data<Pool>,
) -> impl Responder {
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
        create_b2c_result(
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

    format!("")
}

#[get("/generateauth")]
pub(crate) async fn generate_auth(data: web::Data<Pool>) -> impl Responder {
    let api_key = get_api_key(&data);
    let api_url = get_settings_details(&data, String::from("authtokenurlmpesa"));

    tokio::spawn(async move {
        // Process each request concurrently.
        generate_auth_token(data, api_key, api_url).await;
    });

    format!("")
}

fn get_business_to_customer_details(
    data: &web::Data<Pool>,
    my_party_b: String,
    my_amount: u32,
    my_command_id: String,
    my_remarks: String,
    my_occassion: String,
    my_access_token: String,
) -> BusinessToCustomerInputDetails {
    let my_api_url: String = get_settings_details(&data, String::from("b2cpaymentrequesturlmpesa"));
    let my_initiator_name: String =
        get_settings_details(&data, String::from("b2cinitiatornamempesa"));
    let my_security_credential: String =
        get_settings_details(&data, String::from("b2csecuritycredentialmpesa"));
    let my_party_a: String = get_settings_details(&data, String::from("b2cpartyampesa"));
    let my_queue_time_out_url: String =
        get_settings_details(&data, String::from("b2capplicationqueuetimeouturl"));
    let my_result_url: String =
        get_settings_details(&data, String::from("b2capplicationresulturl"));

    let my_party_a: u32 = match my_party_a.parse::<u32>() {
        Ok(a) => a,
        Err(e) => 0,
    };

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

fn get_api_key(data: &web::Data<Pool>) -> String {
    let consumer_key_mpesa = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret_mpesa = get_settings_details(&data, String::from("consumersecretmpesa"));
    let mut password: String = consumer_key_mpesa;
    let k = ":"; // Separator
    password.push_str(k);
    password.push_str(&consumer_secret_mpesa);
    let encodedpassword = general_purpose::STANDARD.encode(password);

    let mut api_key = String::from("Basic");
    let k = " "; // Separator
    api_key.push_str(k);
    api_key.push_str(&encodedpassword);

    api_key
}
