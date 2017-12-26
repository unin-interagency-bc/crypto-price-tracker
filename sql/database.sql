
CREATE TABLE currency (
id SERIAL NOT NULL PRIMARY KEY,
name CHAR(3) NOT NULL UNIQUE
);

INSERT INTO currency (name) VALUES ('BTC'),('BCH'),('ETH'),('LTC'),('XRP');

CREATE TABLE price_history (
id SERIAL NOT NULL PRIMARY KEY,
currency_id INTEGER NOT NULL,
price_timestamp timestamp without time zone default (now() AT TIME ZONE 'utc'),
price DOUBLE PRECISION
);
