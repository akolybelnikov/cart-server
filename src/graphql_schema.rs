extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::{EmptyMutation, RootNode};

#[allow(non_snake_case)]
#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub sku: String,
    pub productName: String,
    pub price: i32,
}

#[juniper::object(description = "A product in the list")]
impl Product {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn sku(&self) -> &str {
        self.sku.as_str()
    }

    pub fn productName(&self) -> &str {
        self.productName.as_str()
    }

    pub fn price(&self) -> i32 {
        self.price
    }
}

#[derive(Queryable)]
struct CartItem {
    pub id: i32,
    pub total: i32,
    pub amount: i32,
    pub sku: String,
}

#[juniper::object(description = "An item in the cart")]
impl CartItem {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn sku(&self) -> &str {
        self.sku.as_str()
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn total(&self) -> i32 {
        self.total
    }
}

pub struct QueryRoot;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABAE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[juniper::object]
impl QueryRoot {
    fn products() -> Vec<Product> {
        use crate::schema::products::dsl::*;
        let connection = establish_connection();
        products
            .load::<Product>(&connection)
            .expect("Error loading products")
    }

    fn cart_items() -> Vec<CartItem> {
        use crate::schema::cart_items::dsl::*;
        let connection = establish_connection();
        cart_items
            .load::<CartItem>(&connection)
            .expect("Error loading cart items")
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
