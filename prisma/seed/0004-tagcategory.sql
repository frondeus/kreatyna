
INSERT INTO public."TagCategory" VALUES (1, 'cat');
INSERT INTO public."TagCategory" VALUES (2, 'foo');
INSERT INTO public."TagCategory" VALUES (3, 'is');
INSERT INTO public."TagCategory" VALUES (4, 'baz');
INSERT INTO public."TagCategory" VALUES (5, 'for');

SELECT pg_catalog.setval('public."TagCategory_id_seq"', 5, true);

