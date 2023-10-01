-- Your SQL goes here
ALTER TABLE candlesticks
ADD COLUMN pair_id INTEGER REFERENCES pairs(id) ON DELETE CASCADE;
