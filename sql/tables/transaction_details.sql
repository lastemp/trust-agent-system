CREATE TABLE `transaction_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `project_id` int DEFAULT 0,
  `project_name` varchar(100) DEFAULT '',
  `beneficiary_id` int DEFAULT 0,
  `amount_paid` int DEFAULT 0,
  `is_bank_payment` tinyint(1) DEFAULT 0,
  `posted_to_mpesa` tinyint(1) DEFAULT '0',
  `duplicate_entry` tinyint(1) DEFAULT '0',
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
