use crate::BeneficiaryDetails;
use crate::BeneficiaryResponseData;
use crate::ProjectDetails;
use crate::ProjectResponseData;
use crate::ResponseStatus;
use crate::TransactionDetails;
use crate::TransactionResponseData;
use actix_web::web;
use mysql::prelude::*;
use mysql::*;

const ERROR_MESSAGE: &str = "Error occured during processing, please try again.";

pub fn create_project(
    data: &web::Data<Pool>,
    project_name: String,
    total_budget: u32,
    funds_deposited: u32,
    mpesa_transaction_reference: String,
    bank_transaction_reference: String,
    is_bank_payment: bool,
    is_active: bool,
    is_closed: bool,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if project_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Project name is empty!");
        return response_status;
    }

    if total_budget == 0 {
        response_status.status_description =
            String::from("Total budget must be greater than zero!");
        return response_status;
    }

    if funds_deposited == 0 {
        response_status.status_description =
            String::from("Funds deposited must be greater than zero!");
        return response_status;
    }

    if !is_bank_payment && mpesa_transaction_reference.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Mpesa transaction reference is empty!");
        return response_status;
    }

    if is_bank_payment && bank_transaction_reference.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Bank transaction reference is empty!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_project_data(
            &mut conn,
            project_name.to_uppercase(),
            total_budget,
            funds_deposited,
            mpesa_transaction_reference.to_uppercase(),
            bank_transaction_reference.to_uppercase(),
            is_bank_payment,
            is_active,
            is_closed,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_bank {:?}", e),
    }

    response_status
}

pub fn create_beneficiary(
    data: &web::Data<Pool>,
    beneficiary_name: String,
    mobile_no: String,
    alternate_mobile_no: String,
    bank_account: String,
    beneficiary_amount: u32,
    amount_paid: u32,
    payment_completed: bool,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if beneficiary_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Beneficiary name is empty!");
        return response_status;
    }

    if mobile_no.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Mobile no is empty!");
        return response_status;
    }

    if beneficiary_amount == 0 {
        response_status.status_description =
            String::from("Beneficiary amount must be greater than zero!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_beneficiary_data(
            &mut conn,
            beneficiary_name.to_uppercase(),
            mobile_no,
            alternate_mobile_no,
            bank_account,
            beneficiary_amount,
            amount_paid,
            payment_completed,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_branch {:?}", e),
    }

    response_status
}

pub fn create_transaction(
    data: &web::Data<Pool>,
    project_id: u32,
    project_name: String,
    beneficiary_id: u32,
    amount_paid: u32,
    is_bank_payment: bool,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    if project_id == 0 {
        response_status.status_description = String::from("Project id must be greater than zero!");
        return response_status;
    }

    if project_name.replace(" ", "").trim().len() == 0 {
        response_status.status_description = String::from("Project name is empty!");
        return response_status;
    }

    if beneficiary_id == 0 {
        response_status.status_description =
            String::from("Beneficiary id must be greater than zero!");
        return response_status;
    }

    if amount_paid == 0 {
        response_status.status_description = String::from("Amount paid must be greater than zero!");
        return response_status;
    }

    match data.get_conn().and_then(|mut conn| {
        insert_transaction_data(
            &mut conn,
            project_id,
            project_name,
            beneficiary_id,
            amount_paid,
            is_bank_payment,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_teller {:?}", e),
    }

    response_status
}

pub fn create_b2c_acknowledgement(
    data: &web::Data<Pool>,
    originator_conversation_id: String,
    conversation_id: String,
    response_code: String,
    response_description: String,
    command_id: String,
    party_a: u32,
    party_b: String,
    amount: u32,
    request_id: String,
    error_code: String,
    error_message: String,
    date_to_mpesa: String,
    date_from_mpesa: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    match data.get_conn().and_then(|mut conn| {
        insert_b2c_acknowledgement_data(
            &mut conn,
            originator_conversation_id,
            conversation_id,
            response_code,
            response_description,
            command_id,
            party_a,
            party_b,
            amount,
            request_id,
            error_code,
            error_message,
            date_to_mpesa,
            date_from_mpesa,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_teller {:?}", e),
    }

    response_status
}

pub fn create_b2c_timeout(
    data: &web::Data<Pool>,
    result_type: u8,
    result_code: u32,
    result_description: String,
    originator_conversation_id: String,
    conversation_id: String,
    transaction_id: String,
    queue_timeout_url: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    match data.get_conn().and_then(|mut conn| {
        insert_b2c_timeout_data(
            &mut conn,
            result_type,
            result_code,
            result_description,
            originator_conversation_id,
            conversation_id,
            transaction_id,
            queue_timeout_url,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_teller {:?}", e),
    }

    response_status
}

pub fn create_b2c_result(
    data: &web::Data<Pool>,
    result_type: u8,
    result_code: u32,
    result_description: String,
    originator_conversation_id: String,
    conversation_id: String,
    transaction_id: String,
    transaction_amount: f32,
    transaction_receipt: String,
    b2c_recipient_is_registered_customer: String,
    b2c_charges_paid_account_available_funds: f32,
    receiver_party_public_name: String,
    transaction_completed_date_time: String,
    b2c_utility_account_available_funds: f32,
    b2c_working_account_available_funds: f32,
    queue_timeout_url: String,
) -> ResponseStatus {
    let my_status_code: u8 = 1;
    let my_status_description: String = ERROR_MESSAGE.to_string();

    let mut response_status = ResponseStatus {
        status_code: my_status_code,
        status_description: my_status_description,
    };

    match data.get_conn().and_then(|mut conn| {
        insert_b2c_result_data(
            &mut conn,
            result_type,
            result_code,
            result_description,
            originator_conversation_id,
            conversation_id,
            transaction_id,
            transaction_amount,
            transaction_receipt,
            b2c_recipient_is_registered_customer,
            b2c_charges_paid_account_available_funds,
            receiver_party_public_name,
            transaction_completed_date_time,
            b2c_utility_account_available_funds,
            b2c_working_account_available_funds,
            queue_timeout_url,
        )
    }) {
        Ok(x) => {
            if x > 0 {
                response_status.status_code = 0;
                response_status.status_description = String::from("Successful");
            }
        }
        Err(e) => println!("Failed to open DB connection. create_teller {:?}", e),
    }

    response_status
}

pub fn get_project_data(data: &web::Data<Pool>) -> ProjectResponseData {
    let mut vec_project_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_project_details(&mut conn))
    {
        Ok(s) => {
            vec_project_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_project_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = ProjectResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        project_data: vec_project_data,
    };

    output_data
}

pub fn get_beneficiary_data(data: &web::Data<Pool>) -> BeneficiaryResponseData {
    let mut vec_beneficiary_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_beneficiary_details(&mut conn))
    {
        Ok(s) => {
            vec_beneficiary_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_beneficiary_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = BeneficiaryResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        beneficiary_data: vec_beneficiary_data,
    };

    output_data
}

pub fn get_transaction_data(data: &web::Data<Pool>) -> TransactionResponseData {
    let mut vec_transaction_data = Vec::new();
    let mut my_status_code: u8 = 1;
    let mut my_status_description: String = String::from("Record not found");

    match data
        .get_conn()
        .and_then(|mut conn| select_transaction_details(&mut conn))
    {
        Ok(s) => {
            vec_transaction_data = s;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    if vec_transaction_data.len() > 0 {
        my_status_code = 0;
        my_status_description = String::from("Successful");
    }

    //Assign values to struct variable
    let output_data = TransactionResponseData {
        status_code: my_status_code,
        status_description: my_status_description,
        transaction_data: vec_transaction_data,
    };

    output_data
}

fn insert_project_data(
    conn: &mut PooledConn,
    project_name: String,
    total_budget: u32,
    funds_deposited: u32,
    mpesa_transaction_reference: String,
    bank_transaction_reference: String,
    is_bank_payment: bool,
    is_active: bool,
    is_closed: bool,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table bank_details
    let mut project_id: u64 = 0;
    conn.exec_map(
        "call insertprojectdetails (:myprojectname, :mytotalbudget, :myfundsdeposited, :mympesatransactionreference, :mybanktransactionreference, :myisbankpayment, :myisactive, :myisclosed, :myprojectid);",
		params! {
            "myprojectname" => project_name,
            "mytotalbudget" => total_budget,
            "myfundsdeposited" => funds_deposited,
            "mympesatransactionreference" => mpesa_transaction_reference,
            "mybanktransactionreference" => bank_transaction_reference,
            "myisbankpayment" => is_bank_payment,
            "myisactive" => is_active,
            "myisclosed" => is_closed,
            "myprojectid" => 0, // output param
        },
        |myprojectid| {
            project_id = myprojectid;
        },
        )
	.and_then(|_| Ok(project_id))
}

fn insert_beneficiary_data(
    conn: &mut PooledConn,
    beneficiary_name: String,
    mobile_no: String,
    alternate_mobile_no: String,
    bank_account: String,
    beneficiary_amount: u32,
    amount_paid: u32,
    payment_completed: bool,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table bank_details
    let mut beneficiary_id: u64 = 0;
    conn.exec_map(
        "call insertbeneficiarydetails (:mybeneficiaryname, :mymobileno, :myalternatemobileno, :mybankaccount, :mybeneficiaryamount, :myamountpaid, :mypaymentcompleted, :mybeneficiaryid);",
		params! {
            "mybeneficiaryname" => beneficiary_name,
            "mymobileno" => mobile_no,
            "myalternatemobileno" => alternate_mobile_no,
            "mybankaccount" => bank_account,
            "mybeneficiaryamount" => beneficiary_amount,
            "myamountpaid" => amount_paid,
            "mypaymentcompleted" => payment_completed,
            "mybeneficiaryid" => 0, // output param
        },
        |mybeneficiaryid| {
            beneficiary_id = mybeneficiaryid;
        },
        )
	.and_then(|_| Ok(beneficiary_id))
}

fn insert_transaction_data(
    conn: &mut PooledConn,
    project_id: u32,
    project_name: String,
    beneficiary_id: u32,
    amount_paid: u32,
    is_bank_payment: bool,
) -> std::result::Result<u64, mysql::error::Error> {
    // Insert data into the database table bank_details
    let mut transaction_id: u64 = 0;
    conn.exec_map(
        "call inserttransactiondetails (:myprojectid, :myprojectname, :mybeneficiaryid, :myamountpaid, :myisbankpayment, :mytransactionid);",
		params! {
            "myprojectid" => project_id,
            "myprojectname" => project_name,
            "mybeneficiaryid" => beneficiary_id,
            "myamountpaid" => amount_paid,
            "myisbankpayment" => is_bank_payment,
            "mytransactionid" => 0, // output param
        },
        |mytransactionid| {
            transaction_id = mytransactionid;
        },
        )
	.and_then(|_| Ok(transaction_id))
}

fn insert_b2c_acknowledgement_data(
    conn: &mut PooledConn,
    my_originator_conversation_id: String,
    my_conversation_id: String,
    my_response_code: String,
    my_response_description: String,
    my_command_id: String,
    my_party_a: u32,
    my_party_b: String,
    my_amount: u32,
    my_request_id: String,
    my_error_code: String,
    my_error_message: String,
    my_date_to_mpesa: String,
    my_date_from_mpesa: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Now let's insert data to the database
    conn.exec_drop(
        "insert into b2c_acknowledgement_details (originator_conversation_id, conversation_id, response_code, response_description, command_id, party_a, party_b, amount, request_id, error_code, error_message, date_to_mpesa, date_from_mpesa) values (:originator_conversation_id, :conversation_id, :response_code, :response_description, :command_id, :party_a, :party_b, :amount, :request_id, :error_code, :error_message, :date_to_mpesa, :date_from_mpesa);",
        params! {
            "originator_conversation_id" => my_originator_conversation_id,
            "conversation_id" => my_conversation_id,
            "response_code" => my_response_code,
            "response_description" => my_response_description,
            "command_id" => my_command_id,
            "party_a" => my_party_a,
            "party_b" => my_party_b,
            "amount" => my_amount,
            "request_id" => my_request_id,
            "error_code" => my_error_code,
            "error_message" => my_error_message,
            "date_to_mpesa" => my_date_to_mpesa,
            "date_from_mpesa" => my_date_from_mpesa,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_b2c_timeout_data(
    conn: &mut PooledConn,
    my_result_type: u8,
    my_result_code: u32,
    my_result_description: String,
    my_originator_conversation_id: String,
    my_conversation_id: String,
    my_transaction_id: String,
    my_queue_timeout_url: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Now let's insert data to the database
    conn.exec_drop(
        "insert into b2c_timeout_details (result_type, result_code, result_description, originator_conversation_id, conversation_id, transaction_id, queue_timeout_url) values (:result_type, :result_code, :result_description, :originator_conversation_id, :conversation_id, :transaction_id, :queue_timeout_url);",
        params! {
            "result_type" => my_result_type,
            "result_code" => my_result_code,
            "result_description" => my_result_description,
            "originator_conversation_id" => my_originator_conversation_id,
            "conversation_id" => my_conversation_id,
            "transaction_id" => my_transaction_id,
            "queue_timeout_url" => my_queue_timeout_url,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn insert_b2c_result_data(
    conn: &mut PooledConn,
    my_result_type: u8,
    my_result_code: u32,
    my_result_description: String,
    my_originator_conversation_id: String,
    my_conversation_id: String,
    my_transaction_id: String,
    my_transaction_amount: f32,
    my_transaction_receipt: String,
    my_b2c_recipient_is_registered_customer: String,
    my_b2c_charges_paid_account_available_funds: f32,
    my_receiver_party_public_name: String,
    my_transaction_completed_date_time: String,
    my_b2c_utility_account_available_funds: f32,
    my_b2c_working_account_available_funds: f32,
    my_queue_timeout_url: String,
) -> std::result::Result<u64, mysql::error::Error> {
    // Now let's insert data to the database
    conn.exec_drop(
        "insert into b2c_result_details (result_type, result_code, result_description, originator_conversation_id, conversation_id, transaction_id, transaction_amount, transaction_receipt, b2c_recipient_is_registered_customer, b2c_charges_paid_account_available_funds, receiver_party_public_name, transaction_completed_date_time, b2c_utility_account_available_funds, b2c_working_account_available_funds, queue_timeout_url) values (:result_type, :result_code, :result_description, :originator_conversation_id, :conversation_id, :transaction_id, :transaction_amount, :transaction_receipt, :b2c_recipient_is_registered_customer, :b2c_charges_paid_account_available_funds, :receiver_party_public_name, :transaction_completed_date_time, :b2c_utility_account_available_funds, :b2c_working_account_available_funds, :queue_timeout_url);",
        params! {
            "result_type" => my_result_type,
            "result_code" => my_result_code,
            "result_description" => my_result_description,
            "originator_conversation_id" => my_originator_conversation_id,
            "conversation_id" => my_conversation_id,
            "transaction_id" => my_transaction_id,
            "transaction_amount" => my_transaction_amount,
            "transaction_receipt" => my_transaction_receipt,
            "b2c_recipient_is_registered_customer" => my_b2c_recipient_is_registered_customer,
            "b2c_charges_paid_account_available_funds" => my_b2c_charges_paid_account_available_funds,
            "receiver_party_public_name" => my_receiver_party_public_name,
            "transaction_completed_date_time" => my_transaction_completed_date_time,
            "b2c_utility_account_available_funds" => my_b2c_utility_account_available_funds,
            "b2c_working_account_available_funds" => my_b2c_working_account_available_funds,
            "queue_timeout_url" => my_queue_timeout_url,
        },
    )
	.and_then(|_| Ok(conn.last_insert_id()))
}

fn select_project_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<ProjectDetails>, mysql::error::Error> {
    let mut project_data = Vec::new();

    conn.query_map(
        "select project_name, total_budget, funds_deposited, mpesa_transaction_reference, bank_transaction_reference, is_bank_payment, is_active, is_closed from project_details where length(trim(coalesce(project_name,''))) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_project_name, my_total_budget, my_funds_deposited, my_mpesa_transaction_reference, my_bank_transaction_reference, my_is_bank_payment, my_is_active, my_is_closed)| {
                let project_details = ProjectDetails { project_name: my_project_name, total_budget: my_total_budget, funds_deposited: my_funds_deposited, mpesa_transaction_reference: my_mpesa_transaction_reference, bank_transaction_reference: my_bank_transaction_reference, is_bank_payment: my_is_bank_payment, is_active: my_is_active, is_closed: my_is_closed, };
                project_data.push(project_details);
            },
    )
	.and_then(|_| Ok(project_data))
}

fn select_beneficiary_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<BeneficiaryDetails>, mysql::error::Error> {
    let mut beneficiary_data = Vec::new();

    conn.query_map(
        "select beneficiary_name, mobile_no, alternate_mobile_no, bank_account, beneficiary_amount, amount_paid, payment_completed from beneficiary_details where length(trim(coalesce(beneficiary_name,''))) > 0 and coalesce(duplicate_entry,0) = 0 order by id asc;",
            |(my_beneficiary_name, my_mobile_no, my_alternate_mobile_no, my_bank_account, my_beneficiary_amount, my_amount_paid, my_payment_completed)| {
                let beneficiary_details = BeneficiaryDetails { beneficiary_name: my_beneficiary_name, mobile_no: my_mobile_no, alternate_mobile_no: my_alternate_mobile_no, bank_account: my_bank_account, beneficiary_amount: my_beneficiary_amount, amount_paid: my_amount_paid, payment_completed: my_payment_completed, };
                beneficiary_data.push(beneficiary_details);
            },
    )
	.and_then(|_| Ok(beneficiary_data))
}

fn select_transaction_details(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<TransactionDetails>, mysql::error::Error> {
    let mut transaction_data = Vec::new();

    conn.query_map(
        "select a.project_id, a.project_name, b.beneficiary_name, a.amount_paid, a.is_bank_payment from transaction_details a inner join beneficiary_details b on a.beneficiary_id = b.id where length(trim(coalesce(a.project_name,''))) > 0 and coalesce(a.duplicate_entry,0) = 0 order by a.id asc;",
            |(my_project_id, my_project_name, my_beneficiary_name, my_amount_paid, my_is_bank_payment)| {
                let transaction_details = TransactionDetails { project_id: my_project_id, project_name: my_project_name, beneficiary_name: my_beneficiary_name, amount_paid: my_amount_paid, is_bank_payment: my_is_bank_payment, };
                transaction_data.push(transaction_details);
            },
    )
	.and_then(|_| Ok(transaction_data))
}
