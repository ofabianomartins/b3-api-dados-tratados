// @generated automatically by Diesel CLI.

diesel::table! {
    asset_positions (id) {
        id -> Int4,
        company_id -> Int4,
        asset_id -> Int4,
        date -> Date,
        side -> Varchar,
        quantity -> Numeric,
        price -> Numeric,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    assets (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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
    client_positions (id) {
        id -> Int4,
        company_id -> Int4,
        client_id -> Int4,
        date -> Date,
        side -> Varchar,
        value -> Numeric,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    clients (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
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
        cnpj -> Nullable<Varchar>,
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
    events (id) {
        id -> Int4,
        ticker_id -> Int4,
        date -> Date,
        ex_date -> Date,
        liquidation_date -> Date,
        #[sql_name = "type"]
        type_ -> Varchar,
        factor -> Numeric,
        strike -> Nullable<Numeric>,
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
    indicator_values (id) {
        id -> Int4,
        indicator_id -> Int4,
        company_id -> Int4,
        date -> Date,
        close -> Numeric,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    indicators (id) {
        id -> Int4,
        name -> Varchar,
        symbol -> Varchar,
        description -> Varchar,
        indicator_type -> Varchar,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quotes (id) {
        id -> Int4,
        ticker_id -> Int4,
        date -> Date,
        adjust_close -> Numeric,
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
        change_7days -> Numeric,
        change_month -> Numeric,
        change_1month -> Numeric,
        change_year -> Numeric,
        change_12month -> Numeric,
        change_1year -> Numeric,
        change_2year -> Numeric,
        change_3year -> Numeric,
        change_4year -> Numeric,
        change_5year -> Numeric,
        change_begin -> Numeric,
        daily_factor -> Numeric,
        accumulated_factor -> Numeric,
        uuid -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sectors (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        uuid -> Uuid,
    }
}

diesel::table! {
    segments (id) {
        id -> Int4,
        name -> Varchar,
        subsector_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        uuid -> Uuid,
    }
}

diesel::table! {
    subsectors (id) {
        id -> Int4,
        name -> Varchar,
        sector_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        uuid -> Uuid,
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
        creation_date -> Date,
        uuid -> Uuid,
        company_id -> Int4,
        calendar_id -> Int4,
        currency_id -> Int4,
        ticker_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        segment_id -> Int4,
        unit -> Varchar,
    }
}

diesel::joinable!(asset_positions -> assets (asset_id));
diesel::joinable!(asset_positions -> companies (company_id));
diesel::joinable!(client_positions -> clients (client_id));
diesel::joinable!(client_positions -> companies (company_id));
diesel::joinable!(events -> tickers (ticker_id));
diesel::joinable!(holidays -> calendars (calendar_id));
diesel::joinable!(indicator_values -> companies (company_id));
diesel::joinable!(indicator_values -> indicators (indicator_id));
diesel::joinable!(quotes -> tickers (ticker_id));
diesel::joinable!(segments -> subsectors (subsector_id));
diesel::joinable!(subsectors -> sectors (sector_id));
diesel::joinable!(theory_portfolio_transactions -> theory_portfolios (theory_portfolio_id));
diesel::joinable!(theory_portfolio_transactions -> tickers (ticker_id));
diesel::joinable!(theory_portfolios -> tickers (index_id));
diesel::joinable!(tickers -> calendars (calendar_id));
diesel::joinable!(tickers -> companies (company_id));
diesel::joinable!(tickers -> currencies (currency_id));
diesel::joinable!(tickers -> segments (segment_id));

diesel::allow_tables_to_appear_in_same_query!(
    asset_positions,
    assets,
    calendars,
    client_positions,
    clients,
    companies,
    currencies,
    events,
    holidays,
    indicator_values,
    indicators,
    quotes,
    sectors,
    segments,
    subsectors,
    theory_portfolio_transactions,
    theory_portfolios,
    tickers,
);
