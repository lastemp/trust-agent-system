DELIMITER $$
CREATE PROCEDURE `insertprojectdetails`(
	IN myprojectname varchar(100),
	IN mytotalbudget int,
	IN myfundsdeposited int,
	IN mympesatransactionreference varchar(30),
	IN mybanktransactionreference varchar(30),
	IN myisbankpayment tinyint(1),
	IN myisactive tinyint(1),
	IN myisclosed tinyint(1),
	OUT myprojectid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

	set myprojectname = trim(coalesce(myprojectname,''));
	set mytotalbudget = coalesce(mytotalbudget,0);
	set myfundsdeposited = coalesce(myfundsdeposited,0);
	set mympesatransactionreference = trim(coalesce(mympesatransactionreference,''));
	set mybanktransactionreference = trim(coalesce(mybanktransactionreference,''));
	set myisbankpayment = coalesce(myisbankpayment,0);
	set myisactive = coalesce(myisactive,0);
	set myisclosed = coalesce(myisclosed,0);
    
    set myprojectid = 0;
    set mycount = 0;
	
	select count(id) into mycount from project_details 
	where trim(coalesce(project_name,'')) = myprojectname;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into project_details
	(project_name,
	total_budget,
	funds_deposited,
	mpesa_transaction_reference,
	bank_transaction_reference,
	is_bank_payment,
	is_active,
	is_closed,
	duplicate_entry,
	date_added)
	VALUES
	(myprojectname
	,mytotalbudget
	,myfundsdeposited
	,mympesatransactionreference
	,mybanktransactionreference
	,myisbankpayment
	,myisactive
	,myisclosed
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set myprojectid = last_insert_id();
		set myprojectid = coalesce(myprojectid,0);
	else
		set myprojectid = 0;
	end if;
    
END$$
DELIMITER ;
