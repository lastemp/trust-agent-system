CREATE TABLE `project_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `project_name` varchar(100) DEFAULT '',
  `total_budget` int DEFAULT 0,
  `funds_deposited` int DEFAULT 0,
  `mpesa_transaction_reference` varchar(30) DEFAULT '',
  `bank_transaction_reference` varchar(30) DEFAULT '',
  `is_bank_payment` tinyint(1) DEFAULT 0,
  `is_active` tinyint(1) DEFAULT 1,
  `is_closed` tinyint(1) DEFAULT 0,
  `duplicate_entry` tinyint(1) DEFAULT '0',
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
