use actix_web::web;
use chrono::prelude::*;
use mysql::Pool;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct BusinessToCustomerData {
    InitiatorName: String,
    SecurityCredential: String,
    CommandID: String,
    Amount: u32,
    PartyA: u32,
    PartyB: String,
    Remarks: String,
    QueueTimeOutURL: String,
    ResultURL: String,
    Occassion: String,
}

#[derive(Deserialize, Debug)]
struct AuthTokenResponseData {
    access_token: Option<String>,
    expires_in: Option<String>,
}

#[derive(Deserialize, Debug)]
struct BusinessToCustomerResponseData {
    OriginatorConversationID: Option<String>,
    ConversationID: Option<String>,
    ResponseCode: Option<String>,
    ResponseDescription: Option<String>,
}

#[derive(Deserialize, Debug)]
struct BusinessToCustomerErrorResponseData {
    requestId: Option<String>,
    errorCode: Option<String>,
    errorMessage: Option<String>,
}

// This struct holds  Business To Customer processing data
pub struct BusinessToCustomerInputDetails {
    pub access_token: String,
    pub api_url: String,
    pub initiator_name: String,
    pub security_credential: String,
    pub command_id: String,
    pub amount: u32,
    pub party_a: u32,
    pub party_b: String,
    pub _remarks: String,
    pub queue_time_out_url: String,
    pub result_url: String,
    pub _occassion: String,
}

pub async fn generate_auth_token(
    data: web::Data<Pool>,
    api_key: String,
    api_url: String,
) -> std::result::Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .get(api_url)
        .header(CONTENT_TYPE, "text/plain")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_key)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            println!("server not responding");
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<AuthTokenResponseData>().await?;

                    let access_token = &my_output.access_token.as_ref().unwrap_or(&k);
                    let expires_in = &my_output.expires_in.as_ref().unwrap_or(&k);

                    let expires_in: u32 = match expires_in.parse::<u32>() {
                        Ok(a) => a,
                        Err(e) => 0,
                    };
                    /*
                    crate::db_layer::create_mpesa_access_token(
                        &data,
                        access_token.to_string(),
                        expires_in,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => println!("Received response status: {:?}", s),
            }
        }
    };

    Ok(())
}

pub async fn business_to_customer(
    data: web::Data<Pool>,
    business_to_customer_details: BusinessToCustomerInputDetails,
) -> std::result::Result<(), reqwest::Error> {
    let access_token: String = business_to_customer_details.access_token;
    let api_url: String = business_to_customer_details.api_url;
    let initiator_name: String = business_to_customer_details.initiator_name;
    let security_credential: String = business_to_customer_details.security_credential;
    let command_id: String = business_to_customer_details.command_id;
    let amount: u32 = business_to_customer_details.amount;
    let party_a: u32 = business_to_customer_details.party_a;
    let party_b: String = business_to_customer_details.party_b;
    let _remarks: String = business_to_customer_details._remarks;
    let queue_time_out_url: String = business_to_customer_details.queue_time_out_url;
    let result_url: String = business_to_customer_details.result_url;
    let _occassion: String = business_to_customer_details._occassion;

    let business_to_customer_data = BusinessToCustomerData {
        InitiatorName: initiator_name,
        SecurityCredential: security_credential,
        CommandID: command_id,
        Amount: amount,
        PartyA: party_a,
        PartyB: party_b,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
        Occassion: _occassion,
    };
    /*
    println!("access_token: {:?}", &access_token);
    println!(
        "business_to_customer_data: {:?}",
        &business_to_customer_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        .json(&business_to_customer_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<BusinessToCustomerResponseData>().await?;

                    let originator_conversation_id =
                        &my_output.OriginatorConversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);
                    let request_id = String::from("");
                    let error_code = String::from("");
                    let error_message = String::from("");
                    /*
                    println!(
                        "originator_conversation_id: {:?}",
                        originator_conversation_id
                    );
                    println!("conversation_id: {:?}", conversation_id);
                    println!("response_code: {:?}", response_code);
                    println!("response_description: {:?}", response_description);
                    */
                    crate::db_layer::create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                }
                s => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let my_output = response
                        .json::<BusinessToCustomerErrorResponseData>()
                        .await?;
                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);
                    let originator_conversation_id = String::from("");
                    let conversation_id = String::from("");
                    let response_code = String::from("");
                    let response_description = String::from("");
                    /*
                    println!("Received response status: {:?}", s);
                    println!("request_id: {:?}", request_id);
                    println!("error_code: {:?}", error_code);
                    println!("error_message: {:?}", error_message);
                    */
                    crate::db_layer::create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                }
            }
        }
    };

    Ok(())
}
