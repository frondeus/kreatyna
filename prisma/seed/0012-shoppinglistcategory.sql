
INSERT INTO public."ShoppingListCategory" VALUES ('seasoning', 'Seasoning', '🧂', 74);
INSERT INTO public."ShoppingListCategory" VALUES ('oriental', 'Oriental', '🍜', 75);
INSERT INTO public."ShoppingListCategory" VALUES ('bread_and_pastries', 'Bread and Pastries', '🥖', 76);
INSERT INTO public."ShoppingListCategory" VALUES ('sauces_and_condiments', 'Sauces and Condiments', '🍅', 77);
INSERT INTO public."ShoppingListCategory" VALUES ('sweets', 'Sweets', '🍬', 78);
INSERT INTO public."ShoppingListCategory" VALUES ('spices', 'Spices', '🌶️', 79);
INSERT INTO public."ShoppingListCategory" VALUES ('eggs_and_butter', 'Eggs and Butter', '🥚', 80);
INSERT INTO public."ShoppingListCategory" VALUES ('hygiene_products', 'Hygiene Products', '🧴', 81);
INSERT INTO public."ShoppingListCategory" VALUES ('vegetables_and_fruits', 'Vegetables and Fruits', '🥦', 82);
INSERT INTO public."ShoppingListCategory" VALUES ('gardening', 'Gardening', '', 83);
INSERT INTO public."ShoppingListCategory" VALUES ('deli_meats', 'Deli Meats', '🥩', 84);
INSERT INTO public."ShoppingListCategory" VALUES ('household_chemicals', 'Household Chemicals', '🧴', 85);
INSERT INTO public."ShoppingListCategory" VALUES ('home_decorations', 'Home Decorations', '🏠', 86);
INSERT INTO public."ShoppingListCategory" VALUES ('canned_goods', 'Canned Goods', '🥫', 89);
INSERT INTO public."ShoppingListCategory" VALUES ('meat', 'Meat', '🥩', 90);
INSERT INTO public."ShoppingListCategory" VALUES ('oils', 'Oils', '🛢️', 92);
INSERT INTO public."ShoppingListCategory" VALUES ('plant_based', 'Plant Based', '🌱', 93);
INSERT INTO public."ShoppingListCategory" VALUES ('cheese', 'Cheese', '🧀', 95);
INSERT INTO public."ShoppingListCategory" VALUES ('milk_and_yogurts', 'Milk and Yogurts', '🥛', 96);
INSERT INTO public."ShoppingListCategory" VALUES ('nuts', 'Nuts', '🥜', 98);
INSERT INTO public."ShoppingListCategory" VALUES ('pasta_and_rice', 'Pasta and Rice', '🍝', 99);

SELECT pg_catalog.setval('public."ShoppingListCategory_order_seq"', 99, true);

