-- This file should undo anything in `up.sql`

-- Drop the 'orders' table (dependent on 'products')
DROP TABLE IF EXISTS "orders";

-- Drop the 'products' table (dependent on 'category')
DROP TABLE IF EXISTS "products";

-- Drop the 'admins' table
DROP TABLE IF EXISTS "admins";

-- Drop the 'category' table
DROP TABLE IF EXISTS "category";