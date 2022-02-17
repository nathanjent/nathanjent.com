-- Creates tables for MySql database
CREATE TABLE IF NOT EXISTS `test`.`notes` (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    text VARCHAR(2024) NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);
