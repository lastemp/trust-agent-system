CREATE TABLE `b2c_timeout_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `result_type` int DEFAULT 0,
  `result_code` int DEFAULT 0,
  `result_description` varchar(200) DEFAULT '',
  `originator_conversation_id` varchar(200) DEFAULT '',
  `conversation_id` varchar(200) DEFAULT '',
  `transaction_id` varchar(200) DEFAULT '',
  `queue_timeout_url` varchar(200) DEFAULT '',
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  `date_updated` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb3;
