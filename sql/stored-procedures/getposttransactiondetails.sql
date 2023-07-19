DELIMITER $$
CREATE PROCEDURE `getposttransactiondetails`(
	IN myprojectid int,
	IN mytransactionid int,
	OUT mytransactionid_out int,
    OUT mybeneficiaryid int,
    OUT myamountpaid int,
    OUT mymobileno varchar(20)
	
)
BEGIN

	set myprojectid = coalesce(myprojectid,0);
	set mytransactionid = coalesce(mytransactionid,0);
    
    set mytransactionid_out = 0;
    set mybeneficiaryid = 0;
    set myamountpaid = 0;
    set mymobileno = '';
    
    select coalesce(a.id,0), coalesce(a.beneficiary_id,0), coalesce(a.amount_paid,0), coalesce(b.mobile_no,'') into mytransactionid_out, mybeneficiaryid, myamountpaid, mymobileno 
    from transaction_details a 
	inner join beneficiary_details b 
	on a.beneficiary_id = b.id
	where coalesce(a.amount_paid,0) = coalesce(b.beneficiary_amount,0)
	and coalesce(a.project_id,0) = myprojectid
	and coalesce(a.id,0) = mytransactionid
	and coalesce(a.posted_to_mpesa,0) = 0
	and coalesce(b.payment_completed,0) = 0
	and coalesce(a.duplicate_entry,0) = 0
	and coalesce(b.duplicate_entry,0) = 0;
	
	set mytransactionid_out = coalesce(mytransactionid_out,0);
    set myprojectid = coalesce(myprojectid,0);
    set myamountpaid = coalesce(myamountpaid,0);
    set mymobileno = coalesce(mymobileno,'');
    
END$$
DELIMITER ;
