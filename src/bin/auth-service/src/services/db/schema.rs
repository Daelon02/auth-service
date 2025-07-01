diesel::table!(users {
    id -> Serial,
    auth_id -> Varchar,
    username -> Varchar,
    email -> Varchar,
    is_email_activate -> Bool,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
});
