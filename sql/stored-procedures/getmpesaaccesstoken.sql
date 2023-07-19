DELIMITER $$
CREATE PROCEDURE `getmpesaaccesstoken`(
    OUT myaccesstoken varchar(200)
)
BEGIN
	set myaccesstoken = '';
    
	select access_token into myaccesstoken 
    from mpesa_access_token where trim(coalesce(response_code,'')) = '0' and 
	coalesce(posted_to_mpesa,0) = 1
    limit 1;
    
    set myaccesstoken = trim(coalesce(myaccesstoken,''));
END$$
DELIMITER ;
