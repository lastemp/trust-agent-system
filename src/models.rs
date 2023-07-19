use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProjectData {
    pub project_name: String,
    pub total_budget: u32,
    pub funds_deposited: u32,
    pub mpesa_transaction_reference: String,
    pub bank_transaction_reference: String,
    pub is_bank_payment: bool,
    pub is_active: bool,
    pub is_closed: bool,
}

#[derive(Deserialize)]
pub struct BeneficiaryData {
    pub beneficiary_name: String,
    pub mobile_no: String,
    pub alternate_mobile_no: String,
    pub bank_account: String,
    pub beneficiary_amount: u32,
    pub amount_paid: u32,
    pub payment_completed: bool,
}

#[derive(Deserialize)]
pub struct TransactionData {
    pub project_id: u32,
    pub project_name: String,
    pub beneficiary_id: u32,
    pub amount_paid: u32,
    pub is_bank_payment: bool,
}

#[derive(Deserialize)]
pub struct PostTransactionData {
    pub project_id: u32,
    pub transaction_id: u32,
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
pub struct B2CResultData {
    pub Result: B2CResultDetails,
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
pub struct B2CFailedData {
    pub Result: B2CFailedDetails,
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

#[derive(Serialize, Debug)]
pub struct PostTransactionDetails {
    pub status_code: u8,
    pub status_description: String,
    pub transaction_id: u32,
    pub beneficiary_id: u32,
    pub amount_paid: u32,
    pub mobile_no: String,
}

#[derive(Serialize, Debug)]
pub struct BusinessToCustomerData {
    pub InitiatorName: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub Amount: u32,
    pub PartyA: u32,
    pub PartyB: String,
    pub Remarks: String,
    pub QueueTimeOutURL: String,
    pub ResultURL: String,
    pub Occassion: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenResponseData {
    pub access_token: Option<String>,
    pub expires_in: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessToCustomerResponseData {
    pub OriginatorConversationID: Option<String>,
    pub ConversationID: Option<String>,
    pub ResponseCode: Option<String>,
    pub ResponseDescription: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BusinessToCustomerErrorResponseData {
    pub requestId: Option<String>,
    pub errorCode: Option<String>,
    pub errorMessage: Option<String>,
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
