DELIMITER $$
CREATE PROCEDURE `insertupdatempesaaccesstoken`(
	IN myaccesstoken varchar(200),
	IN myexpiresin int,
	IN mydatetompesa varchar(30),
	IN mydatefrommpesa varchar(30)
)
BEGIN

	declare myresponsecode int default 0; 
	declare mytransactiondate datetime default Now();
	declare mypostedtompesa int default 1;
		
	
	declare mycount int default 0;

	set myaccesstoken = trim(coalesce(myaccesstoken,''));
	set myexpiresin = coalesce(myexpiresin,0);
	set mydatetompesa = trim(coalesce(mydatetompesa,''));
	set mydatefrommpesa = trim(coalesce(mydatefrommpesa,''));
    
    set mycount = 0;
	

		
		select count(id) into mycount from mpesa_access_token;
		
		set mycount = coalesce(mycount,0);
		
		if (mycount > 0) then
			insert into mpesa_access_token_archive
			(entry_id
			,access_token
			,expires_in
			,response_code
			,error_code
			,error_message
			,transaction_date
			,posted_to_mpesa
			,date_to_mpesa
			,date_from_mpesa
			,date_added
			,date_updated)

			select id
			,access_token
			,expires_in
			,response_code
			,error_code
			,error_message
			,transaction_date
			,posted_to_mpesa
			,date_to_mpesa
			,date_from_mpesa
			,date_added
			,date_updated
			from mpesa_access_token;
					
			update mpesa_access_token
			set access_token = myaccesstoken
			,expires_in = myexpiresin
			,response_code = myresponsecode
			,transaction_date = mytransactiondate
			,posted_to_mpesa = mypostedtompesa
			,date_to_mpesa = mydatetompesa
			,date_from_mpesa = mydatefrommpesa
			,date_updated = Now();
        else
			
			insert into mpesa_access_token
			(access_token
			,expires_in
			,response_code
			,transaction_date
			,posted_to_mpesa
			,date_to_mpesa
			,date_from_mpesa)
			values
			(myaccesstoken
			,myexpiresin
			,myresponsecode
			,mytransactiondate
			,mypostedtompesa
			,mydatetompesa
			,mydatefrommpesa);
		
		end if;

    
END$$
DELIMITER ;
