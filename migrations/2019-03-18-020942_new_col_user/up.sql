-- Your SQL goes here

ALTER TABLE staritems ADD COLUMN rate SMALLINT NOT NULL DEFAULT '0';
ALTER TABLE users ADD COLUMN nickname VARCHAR NOT NULL DEFAULT '';

DROP TABLE staritems;

CREATE TABLE staritems (
  id VARCHAR NOT NULL PRIMARY KEY,
  uname VARCHAR NOT NULL,
  item_id VARCHAR NOT NULL,
  star_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  note VARCHAR NOT NULL DEFAULT '',
  flag VARCHAR NOT NULL DEFAULT '',
  rate INTEGER  NOT NULL DEFAULT '0',
  UNIQUE (uname, item_id)
);