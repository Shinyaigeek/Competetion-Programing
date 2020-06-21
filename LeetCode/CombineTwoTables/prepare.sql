-- DROP TABLE IF EXISTS Person;
-- CREATE TABLE `Person` (
--     `PersonId` int(10) NOT NULL AUTO_INCREMENT,
--     `FirstName` varchar(100) NOT NULL,
--     `LastName` varchar(100) NOT NULL,
--     PRIMARY KEY (`PersonId`)
-- ) ENGINE = InnoDB DEFAULT CHARSET = utf8;
-- DROP TABLE IF EXISTS Address;
-- CREATE TABLE `Address` (
--     `AddressId` int(10) NOT NULL AUTO_INCREMENT,
--     `PersonId` int(10) NOT NULL,
--     `City` varchar(100) NOT NULL,
--     `Status` varchar(100) NOT NULL,
--     PRIMARY KEY (`AddressId`)
-- ) ENGINE = InnoDB DEFAULT CHARSET = utf8;

-- INSERT INTO Person (FirstName, LastName) VALUES ("Shinobu", "Hayashi");
-- INSERT INTO Person (FirstName, LastName) VALUES ("Chie", "Shiraishi");
-- INSERT INTO Person (FirstName, LastName) VALUES ("Yosuke", "Furukawa");
-- INSERT INTO Person (FirstName, LastName) VALUES ("Sho", "Miyamoto");

INSERT INTO Address (PersonId, City, Status ) VALUES (1, "Tokyo", "Sleepy");
INSERT INTO Address (PersonId, City, Status ) VALUES (4, "Chiba", "Drunk");
INSERT INTO Address (PersonId, City, Status ) VALUES (2, "Gunma", "Happy");