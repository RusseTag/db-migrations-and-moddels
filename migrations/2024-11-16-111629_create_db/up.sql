CREATE TABLE "category" (
  "id" SERIAL,
  "category_name" VARCHAR(255),
  PRIMARY KEY ("id")
);

CREATE TABLE "products" (
  "id" SERIAL,
  "name" VARCHAR(100),
  "aliexpress_link" VARCHAR(255),
  "cost_price" INT,
  "sell_price" INT,
  "date_added" DATE,
  "category_id" INT,
  PRIMARY KEY ("id"),
  CONSTRAINT "FK_products.category_id"
    FOREIGN KEY ("category_id")
      REFERENCES "category"("id")
);

CREATE TABLE "orders" (
  "id" SERIAL,
  "product_id" INT,
  "adress" VARCHAR(500),
  "postal_number" INT,
  "phone" INT,
  "email" VARCHAR(255),
  "price" INT,
  PRIMARY KEY ("id"),
  CONSTRAINT "FK_orders.product_id"
    FOREIGN KEY ("product_id")
      REFERENCES "products"("id")
);

CREATE TABLE "admins" (
  "Id" SERIAL,
  "username" VARCHAR(45),
  "password" VARCHAR(255),
  PRIMARY KEY ("Id")
);

