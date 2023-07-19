DELIMITER $$
CREATE PROCEDURE `insertbeneficiarydetails`(
	IN mybeneficiaryname varchar(200),
	IN mymobileno varchar(20),
	IN myalternatemobileno varchar(20),
	IN mybankaccount varchar(30),
	IN mybeneficiaryamount int,
	IN myamountpaid int,
	IN mypaymentcompleted tinyint(1),
	OUT mybeneficiaryid int
	
)
BEGIN

	declare myentrydate datetime default Now();
	declare myduplicateentry bit default 0;
	declare mycount int default 0;

	set mybeneficiaryname = trim(coalesce(mybeneficiaryname,''));
	set mymobileno = trim(coalesce(mymobileno,''));
	set myalternatemobileno = trim(coalesce(myalternatemobileno,''));
    set mybankaccount = trim(coalesce(mybankaccount,''));
	set mybeneficiaryamount = coalesce(mybeneficiaryamount,0);
	set myamountpaid = coalesce(myamountpaid,0);
	set mypaymentcompleted = coalesce(mypaymentcompleted,0);
    
    set mybeneficiaryid = 0;
    set mycount = 0;
	
	select count(id) into mycount from beneficiary_details 
	where trim(coalesce(beneficiary_name,'')) = mybeneficiaryname and
    trim(coalesce(mobile_no,'')) = mymobileno;
	
	set mycount = coalesce(mycount,0);
	
	if (mycount > 0) then
		set myduplicateentry = 1;
	else
		set myduplicateentry = 0;
	end if;
		
	insert into beneficiary_details
	(beneficiary_name,
	mobile_no,
	alternate_mobile_no,
	bank_account,
	beneficiary_amount,
	amount_paid,
	payment_completed,
	duplicate_entry,
	date_added)
	VALUES
	(mybeneficiaryname
	,mymobileno
	,myalternatemobileno
	,mybankaccount
	,mybeneficiaryamount
	,myamountpaid
	,mypaymentcompleted
	,myduplicateentry
	,myentrydate
	);
    
    if (myduplicateentry = 0) then
		set mybeneficiaryid = last_insert_id();
		set mybeneficiaryid = coalesce(mybeneficiaryid,0);
	else
		set mybeneficiaryid = 0;
	end if;
    
END$$
DELIMITER ;
