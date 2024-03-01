// @generated automatically by Diesel CLI.

diesel::table! {
    calendars (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        company_type -> Varchar,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    holidays (id) {
        id -> Int4,
        name -> Varchar,
        date -> Date,
        uuid -> Uuid,
        calendar_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quotes (id) {
        id -> Int4,
        ticker_id -> Int4,
        date -> Date,
        close -> Numeric,
        open -> Nullable<Numeric>,
        high -> Nullable<Numeric>,
        low -> Nullable<Numeric>,
        average -> Nullable<Numeric>,
        ask -> Nullable<Numeric>,
        bid -> Nullable<Numeric>,
        adjust -> Nullable<Numeric>,
        volume -> Nullable<Numeric>,
        trades -> Nullable<Numeric>,
        change_24hrs -> Numeric,
        change_5days -> Numeric,
        change_10days -> Numeric,
        change_1week -> Numeric,
        change_1month -> Numeric,
        change_1year -> Numeric,
        change_2year -> Numeric,
        change_5year -> Numeric,
        change_month -> Numeric,
        change_year -> Numeric,
        daily_factor -> Numeric,
        accumulated_factor -> Numeric,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    theory_portfolio_transactions (id) {
        id -> Int4,
        date -> Date,
        quantity -> Numeric,
        uuid -> Uuid,
        ticker_id -> Int4,
        theory_portfolio_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    theory_portfolios (id) {
        id -> Int4,
        name -> Varchar,
        uuid -> Uuid,
        index_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tickers (id) {
        id -> Int4,
        symbol -> Varchar,
        security_type -> Varchar,
        uuid -> Uuid,
        company_id -> Int4,
        calendar_id -> Int4,
        currency_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(holidays -> calendars (calendar_id));
diesel::joinable!(quotes -> tickers (ticker_id));
diesel::joinable!(theory_portfolio_transactions -> theory_portfolios (theory_portfolio_id));
diesel::joinable!(theory_portfolio_transactions -> tickers (ticker_id));
diesel::joinable!(theory_portfolios -> tickers (index_id));
diesel::joinable!(tickers -> calendars (calendar_id));
diesel::joinable!(tickers -> companies (company_id));
diesel::joinable!(tickers -> currencies (calendar_id));

diesel::allow_tables_to_appear_in_same_query!(
    calendars,
    companies,
    currencies,
    holidays,
    quotes,
    theory_portfolio_transactions,
    theory_portfolios,
    tickers,
);
