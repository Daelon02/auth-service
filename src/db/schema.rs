diesel::table!(users {
    id -> Uuid,
    username -> Varchar,
    email -> Varchar,
    is_email_activate -> Bool,
    created_at -> Timestamptz,
    updated_at -> Nullable<Timestamptz>
});
