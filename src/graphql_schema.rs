use juniper::{EmptyMutation, RootNode};

#[allow(non_snake_case)]
struct Product {
    id: String,
    productName: String,
    price: i32,
}

#[juniper::object(description = "A product in the list")]
impl Product {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn productName(&self) -> &str {
        self.productName.as_str()
    }

    pub fn price(&self) -> i32 {
        self.price
    }
}

struct CartItem {
    item: Product,
    amount: i32,
    total: i32,
}

#[juniper::object(description = "An item in the cart")]
impl CartItem {
    pub fn item(&self) -> &Product {
        &self.item
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn total(&self) -> i32 {
        self.total
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn products() -> Vec<Product> {
        vec![
            Product {
                id: "bea0c670-5c17-420b-a682-db3fdbc609ba".to_owned(),
                productName: "Soup - Cream Of Broccoli, Dry".to_owned(),
                price: 19484,
            },
            Product {
                id: "c70a90ff-823b-4d34-8d12-6ba32e9f685f".to_owned(),
                productName: "Juice - Lemon".to_owned(),
                price: 38631,
            },
            Product {
                id: "153dc45d-6adf-4bfc-995c-82f6d113ce93".to_owned(),
                productName: "Paper Towel Touchless".to_owned(),
                price: 80904,
            },
            Product {
                id: "7abb0be7-4a56-4398-95cc-bd7309458866".to_owned(),
                productName: "Bread - Sour Sticks With Onion".to_owned(),
                price: 35891,
            },
            Product {
                id: "125a25a2-b604-449a-8e2f-09e0743657a6".to_owned(),
                productName: "Flour - Strong".to_owned(),
                price: 80869,
            },
        ]
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
