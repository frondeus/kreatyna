
INSERT INTO public."Tag" VALUES (5, 'breakfast', 5);
INSERT INTO public."Tag" VALUES (6, 'lunch', 5);
INSERT INTO public."Tag" VALUES (7, 'dinner', 5);
INSERT INTO public."Tag" VALUES (9, 'snack', 5);
INSERT INTO public."Tag" VALUES (10, 'desert', 5);

SELECT pg_catalog.setval('public."Tag_id_seq"', 10, true);

