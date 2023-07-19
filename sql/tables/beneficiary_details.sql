CREATE TABLE `beneficiary_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `beneficiary_name` varchar(200) DEFAULT '',
  `mobile_no` varchar(20) DEFAULT '',
  `alternate_mobile_no` varchar(20) DEFAULT '',
  `bank_account` varchar(30) DEFAULT '',
  `beneficiary_amount` int DEFAULT 0,
  `amount_paid` int DEFAULT 0,
  `payment_completed` tinyint(1) DEFAULT 0,
  `duplicate_entry` tinyint(1) DEFAULT '0',
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
