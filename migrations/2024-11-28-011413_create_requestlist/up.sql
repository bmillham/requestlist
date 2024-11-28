CREATE TABLE IF NOT EXISTS `requestlist` (
  `ID` int(11) NOT NULL AUTO_INCREMENT,
  `songID` int(11) NOT NULL DEFAULT 0,
  `t_stamp` datetime NOT NULL DEFAULT '0000-00-00 00:00:00',
  `host` varchar(255) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  `msg` text CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  `name` varchar(255) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  `code` int(11) NOT NULL DEFAULT 0,
  `ETA` datetime NOT NULL DEFAULT '0000-00-00 00:00:00',
  `status` enum('played','ignored','pending','new') CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  PRIMARY KEY (`ID`),
  KEY `t_stamp` (`t_stamp`),
  KEY `songID` (`songID`),
  KEY `status` (`status`)
) ENGINE=MyISAM AUTO_INCREMENT=5636 DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;
