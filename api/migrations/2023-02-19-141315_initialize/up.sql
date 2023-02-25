-- Your SQL goes here

-- création de la table users pour gérer les utilisateurs
CREATE TABLE IF NOT EXISTS `projet_db`.`users` (
  id INT(11) NOT NULL AUTO_INCREMENT,
  username VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  PRIMARY KEY (id)
);

-- création de la table currencies pour stocker les informations sur les différentes monnaies/actions
CREATE TABLE IF NOT EXISTS `projet_db`.`currencies` (
  id INT(11) NOT NULL AUTO_INCREMENT,
  symbol VARCHAR(20) NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL,
  PRIMARY KEY (id)
);

-- création de la table transactions pour stocker les transactions de chaque utilisateur
CREATE TABLE IF NOT EXISTS `projet_db`.`transactions` (
  id INT(11) NOT NULL AUTO_INCREMENT,
  user_id INT(11) NOT NULL,
  currency_id INT(11) NOT NULL,
  price DECIMAL(13,2) NOT NULL,
  quantity DECIMAL(13,2) NOT NULL,
  transaction_type VARCHAR(20) NOT NULL,
  timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (currency_id) REFERENCES currencies(id) ON DELETE CASCADE
);

-- création de la table prices pour stocker les prix des monnaies/actions
CREATE TABLE IF NOT EXISTS `projet_db`.`prices` (
  id INT(11) NOT NULL AUTO_INCREMENT,
  currency_id INT(11) NOT NULL,
  price DECIMAL(13,2) NOT NULL,
  timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (currency_id) REFERENCES currencies(id) ON DELETE CASCADE
);
