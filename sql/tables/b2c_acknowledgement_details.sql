CREATE TABLE `b2c_acknowledgement_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `originator_conversation_id` varchar(200) DEFAULT '',
  `conversation_id` varchar(200) DEFAULT '',
  `response_code` varchar(50) DEFAULT '',
  `response_description` varchar(200) DEFAULT '',
  `command_id` varchar(20) DEFAULT '',
  `party_a` int DEFAULT 0,
  `party_b` varchar(20) DEFAULT '',
  `amount` int DEFAULT 0,
  `request_id` varchar(50) DEFAULT '',
  `error_code` varchar(20) DEFAULT '',
  `error_message` varchar(200) DEFAULT '',
  `posted_to_mpesa` tinyint(1) DEFAULT '0',
  `date_to_mpesa` datetime DEFAULT NULL,
  `date_from_mpesa` datetime DEFAULT NULL,
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  `date_updated` datetime DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb3;