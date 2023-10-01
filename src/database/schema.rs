// @generated automatically by Diesel CLI.

diesel::table! {
    candlesticks (id) {
        id -> Int4,
        open_time -> Timestamp,
        open_price -> Numeric,
        high_price -> Numeric,
        low_price -> Numeric,
        close_price -> Numeric,
        close_time -> Timestamp,
    }
}
