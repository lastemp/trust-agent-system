DELIMITER $$
CREATE PROCEDURE `inserttransactiondetails`(
	IN myprojectid int,
	IN myprojectname varchar(100),
	IN mybeneficiaryid int,
	IN myamountpaid int,
	IN myisbankpayment tinyint(1),
	OUT mytransactionid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

	set myprojectid = coalesce(myprojectid,0);
	set myprojectname = trim(coalesce(myprojectname,''));
	set mybeneficiaryid = coalesce(mybeneficiaryid,0);
	set myamountpaid = coalesce(myamountpaid,0);
	set myisbankpayment = coalesce(myisbankpayment,0);
    
    set mytransactionid = 0;
    set mycount = 0;
	
	select count(id) into mycount from transaction_details 
	where coalesce(project_id,0) = myprojectid and
    coalesce(beneficiary_id,0) = mybeneficiaryid and
    coalesce(amount_paid,0) = myamountpaid;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into transaction_details
	(project_id,
	project_name,
	beneficiary_id,
	amount_paid,
	is_bank_payment,
	duplicate_entry,
	date_added)
	VALUES
	(myprojectid
	,myprojectname
	,mybeneficiaryid
	,myamountpaid
	,myisbankpayment
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set mytransactionid = last_insert_id();
		set mytransactionid = coalesce(mytransactionid,0);
	else
		set mytransactionid = 0;
	end if;
    
END$$
DELIMITER ;
