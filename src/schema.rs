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
        pair_id -> Nullable<Int4>,
    }
}

diesel::table! {
    pairs (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(candlesticks -> pairs (pair_id));

diesel::allow_tables_to_appear_in_same_query!(
    candlesticks,
    pairs,
);
