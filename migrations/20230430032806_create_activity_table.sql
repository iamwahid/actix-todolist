-- Add migration script here

CREATE TABLE activities (
  id int(11) NOT NULL AUTO_INCREMENT,
  title varchar(255) NOT NULL,
  email varchar(255) DEFAULT NULL,
  created_at datetime NOT NULL,
  updated_at datetime DEFAULT NULL,
  deleted_at datetime DEFAULT NULL,
  PRIMARY KEY (id)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;