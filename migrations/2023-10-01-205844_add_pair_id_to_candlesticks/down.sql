-- This file should undo anything in `up.sql`
ALTER TABLE candlesticks
DROP COLUMN pair_id;
