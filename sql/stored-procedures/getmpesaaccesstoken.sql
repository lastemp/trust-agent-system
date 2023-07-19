DELIMITER $$
CREATE PROCEDURE `getmpesaaccesstoken`(
    OUT myaccesstoken varchar(200)
)
BEGIN
	set myaccesstoken = '';
    
	SELECT 
    access_token
INTO myaccesstoken FROM
    mpesa_access_token
WHERE
    TRIM(COALESCE(response_code, '')) = '0'
        AND COALESCE(posted_to_mpesa, 0) = 1
LIMIT 1;
    
    set myaccesstoken = trim(coalesce(myaccesstoken,''));
END$$
DELIMITER ;
