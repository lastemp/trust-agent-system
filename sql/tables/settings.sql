CREATE TABLE `settings` (
  `id` int NOT NULL AUTO_INCREMENT,
  `consumer_key_mpesa` varchar(200) DEFAULT '',
  `consumer_secret_mpesa` varchar(200) DEFAULT '',
  `b2c_initiator_name_mpesa` varchar(50) DEFAULT '',
  `b2c_security_credential_mpesa` varchar(1000) DEFAULT '',
  `b2c_party_a_mpesa` int DEFAULT 0,
  `auth_token_url_mpesa` varchar(200) DEFAULT '',
  `b2c_payment_request_url_mpesa` varchar(200) DEFAULT '',
  `b2c_application_queue_time_out_url` varchar(200) DEFAULT '',
  `b2c_application_result_url` varchar(200) DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb3;
