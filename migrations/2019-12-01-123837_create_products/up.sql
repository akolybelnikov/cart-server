CREATE TABLE products
(
    id SERIAL PRIMARY KEY,
    sku VARCHAR NOT NULL,
    productName VARCHAR NOT NULL,
    price INT NOT NULL,
    UNIQUE (sku)
);

CREATE TABLE cart_items
(
    id SERIAL PRIMARY KEY,
    total INT NOT NULL,
    amount INT NOT NULL,
    sku VARCHAR NOT NULL,
    FOREIGN KEY (sku) REFERENCES products(sku)
);

INSERT INTO products
    (sku, productName, price)
VALUES
    ('bea0c670-5c17-420b-a682-db3fdbc609ba', 'Soup - Cream Of Broccoli, Dry', 19484);
INSERT INTO products
    (sku, productName, price)
VALUES
    ('c70a90ff-823b-4d34-8d12-6ba32e9f685f', 'Juice - Lemon', 38631);
INSERT INTO products
    (sku, productName, price)
VALUES
    ('153dc45d-6adf-4bfc-995c-82f6d113ce93', 'Paper Towel Touchless', 80904);
INSERT INTO products
    (sku, productName, price)
VALUES
    ('7abb0be7-4a56-4398-95cc-bd7309458866', 'Bread - Sour Sticks With Onion', 35891);
INSERT INTO products
    (sku, productName, price)
VALUES
    ('125a25a2-b604-449a-8e2f-09e0743657a6', 'Flour - Strong', 80869);

INSERT INTO cart_items
    (total, amount, sku)
VALUES
    (19484, 1, 'bea0c670-5c17-420b-a682-db3fdbc609ba');
