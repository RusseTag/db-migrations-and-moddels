// @generated automatically by Diesel CLI.

diesel::table! {
    admins (Id) {
        Id -> Int4,
        #[max_length = 45]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
    }
}

diesel::table! {
    category (id) {
        id -> Int4,
        #[max_length = 255]
        category_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        #[max_length = 500]
        adress -> Nullable<Varchar>,
        postal_number -> Nullable<Int4>,
        phone -> Nullable<Int4>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        price -> Nullable<Int4>,
    }
}

diesel::table! {
    post (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        aliexpress_link -> Nullable<Varchar>,
        cost_price -> Nullable<Int4>,
        sell_price -> Nullable<Int4>,
        date_added -> Nullable<Date>,
        category_id -> Nullable<Int4>,
    }
}

diesel::joinable!(orders -> products (product_id));
diesel::joinable!(products -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    category,
    orders,
    post,
    products,
);
