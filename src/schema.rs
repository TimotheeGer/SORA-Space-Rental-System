// @generated automatically by Diesel CLI.

diesel::table! {
    contracts (id) {
        id -> Varchar,
        created_at -> Timestamptz,
        host_id -> Varchar,
        guest_id -> Varchar,
        office_id -> Varchar,
        num_posts -> Int4,
        monthly_amount -> Int4,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
    }
}

diesel::table! {
    guest_relations (user_id, office_id, created_at) {
        user_id -> Varchar,
        office_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    host_relations (user_id, office_id, created_at) {
        user_id -> Varchar,
        office_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    offices (id) {
        id -> Varchar,
        created_at -> Timestamptz,
        name -> Varchar,
        address_text -> Varchar,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        surface -> Float8,
        num_posts -> Nullable<Int4>,
        price_per_post -> Nullable<Int4>,
        parent_office_id -> Nullable<Varchar>,
        owner_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        created_at -> Timestamptz,
        first_name -> Varchar,
        last_name -> Varchar,
        user_role -> Varchar,
    }
}

diesel::joinable!(contracts -> offices (office_id));
diesel::joinable!(guest_relations -> offices (office_id));
diesel::joinable!(guest_relations -> users (user_id));
diesel::joinable!(host_relations -> offices (office_id));
diesel::joinable!(host_relations -> users (user_id));
diesel::joinable!(offices -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    contracts,
    guest_relations,
    host_relations,
    offices,
    users,
);
