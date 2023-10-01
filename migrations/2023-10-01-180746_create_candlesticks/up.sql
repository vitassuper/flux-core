-- Your SQL goes here
CREATE TABLE candlesticks (
   id SERIAL PRIMARY KEY,
   open_time TIMESTAMP NOT NULL,
   open_price DECIMAL(16, 8) NOT NULL,
   high_price DECIMAL(16, 8) NOT NULL,
   low_price DECIMAL(16, 8) NOT NULL,
   close_price DECIMAL(16, 8) NOT NULL,
   close_time TIMESTAMP NOT NULL
);
