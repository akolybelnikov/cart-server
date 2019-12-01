table! {
    cart_items (id) {
        id -> Int4,
        total -> Int4,
        amount -> Int4,
        sku -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        sku -> Varchar,
        productname -> Varchar,
        price -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    cart_items,
    products,
);
