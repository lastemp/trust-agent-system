CREATE TABLE `mpesa_access_token_archive` (
  `id` int NOT NULL AUTO_INCREMENT,
  `entry_id` int DEFAULT 0,
  `access_token` varchar(200) DEFAULT '',
  `expires_in` int DEFAULT 0,
  `response_code` varchar(20) DEFAULT '',
  `error_code` varchar(50) DEFAULT '',
  `error_message` varchar(200) DEFAULT '',
  `transaction_date` datetime DEFAULT CURRENT_TIMESTAMP,
  `posted_to_mpesa` tinyint(1) DEFAULT 0,
  `date_to_mpesa` datetime DEFAULT NULL,
  `date_from_mpesa` datetime DEFAULT NULL,
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  `date_updated` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb3;
