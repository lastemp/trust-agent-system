DELIMITER $$
CREATE PROCEDURE `getsettings`(
	IN paramkey varchar(50),
    OUT paramvalue varchar(1000)
)
BEGIN
	set paramvalue = '';
    set paramkey = trim(coalesce(paramkey,''));
    
    if (length(paramkey) > 0) then
		set paramkey = lower(paramkey);	
        
		select (case when paramkey = 'consumerkeympesa' then coalesce(consumer_key_mpesa,'')
		when paramkey = 'consumersecretmpesa' then coalesce(consumer_secret_mpesa,'')
        when paramkey = 'b2cinitiatornamempesa' then coalesce(b2c_initiator_name_mpesa,'')
        when paramkey = 'b2csecuritycredentialmpesa' then coalesce(b2c_security_credential_mpesa,'')
        when paramkey = 'b2cpartyampesa' then coalesce(b2c_party_a_mpesa,'')
		when paramkey = 'authtokenurlmpesa' then coalesce(auth_token_url_mpesa,'')
		when paramkey = 'b2cpaymentrequesturlmpesa' then coalesce(b2c_payment_request_url_mpesa,'')
		when paramkey = 'b2capplicationqueuetimeouturl' then coalesce(b2c_application_queue_time_out_url,'')
		when paramkey = 'b2capplicationresulturl' then coalesce(b2c_application_result_url,'')
		else '' end)
		into paramvalue from settings limit 1;
    end if;
    
    set paramvalue = trim(coalesce(paramvalue,''));
END$$
DELIMITER ;
