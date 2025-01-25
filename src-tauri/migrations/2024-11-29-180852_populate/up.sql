--                               ID,      name              Container,  Protein, Cargs,  Fat, "TagId"
CREATE TEMP TABLE temp."Raw" (
    ID SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    container INTEGER NOT NULL,
    proteins FLOAT NOT NULL,
    carbs FLOAT NOT NULL,
    fat FLOAT NOT NULL,
    tag_id INTEGER
);
INSERT INTO temp."Raw"
VALUES (
        5244,
        'Strawberry, raw',
        100,
        0.659855769230769,
        8.34732371794872,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5245,
        'Apple, raw, all varieties',
        100,
        0.269357638888889,
        14.2612561177249,
        0.228571428571429,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5246,
        'Banana, raw',
        100,
        1.140625,
        22.5878194444444,
        0.205555555555556,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5247,
        'Potato, raw',
        100,
        2.02688953488372,
        17.2980286033054,
        0.290909090909091,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5248,
        'Milk, whole, konventional (not organic), 3.5 % fat',
        100,
        3.421768,
        4.41171029115393,
        3.47322033898305,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5249,
        'Parsley, raw',
        100,
        3.7125,
        7.76666666666667,
        1.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5250,
        'Pear, raw',
        100,
        0.306944444444444,
        14.3526506813417,
        0.0666666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5251, 'Rhubarb, raw', 100, 0.9, 6.8, 0.3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5252,
        'Currant, black, raw',
        100,
        1.52,
        17.1511904761905,
        1.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5253,
        'Yogurt plain, whole milk',
        100,
        3.78857731343284,
        2.77801970149254,
        3.57731343283582,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5254,
        'Celery, raw',
        100,
        0.746666666666667,
        5.51333333333333,
        0.166666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5255,
        'Cauliflower, all varieties, raw',
        100,
        2.43032786885246,
        4.70668511816053,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5256,
        'Broccoli, raw',
        100,
        3.5546875,
        6.32314144736842,
        0.24,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5257,
        'Beans, wax, raw',
        100,
        2.1,
        7.1,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5258, 'Lemon, raw', 100, 0.5, 5.075, 1.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5259,
        'Dill, raw',
        100,
        3.4625,
        7.0175,
        1.12,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5260,
        'Kale, raw',
        100,
        4.65555555555556,
        10.9366666666667,
        1.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5261,
        'Carrot, raw',
        100,
        0.687019230769231,
        8.87814903846154,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5262,
        'Garfish, raw',
        100,
        19.4791666666667,
        0,
        3.21666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5263,
        'Cabbage, white, raw',
        100,
        1.225,
        7.34506968641115,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5264,
        'Rose hip, raw',
        100,
        0.6,
        21.08,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5265,
        'Elderberry, raw',
        100,
        0.6625,
        18.3975,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5266,
        'Salmon, Atlantic, wild, raw',
        100,
        18.4,
        4.54333333333333,
        7.45666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5267,
        'Sweetcorn, on-the-cob, raw',
        100,
        3.7,
        16.3,
        1.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5268,
        'Milk, buttermilk',
        100,
        3.39969868131868,
        3.84373881868132,
        0.5015625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5269,
        'Milk, partly skimmed, konventional (not organic), 1.5 % fat',
        100,
        3.50944859375,
        4.6337768729837,
        1.59348416289593,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5270,
        'Mackerel, raw',
        100,
        17.8291666666667,
        0,
        12.8916666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5271,
        'Almond, raw',
        100,
        21.238,
        18.437,
        52.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5272,
        'Watermelon, raw',
        100,
        0.8,
        7.48333333333334,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5273,
        'Pepper, sweet, green, raw',
        100,
        0.9,
        4.84733333333333,
        0.266666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5274,
        'Pepper, sweet, red, raw',
        100,
        0.922321428571428,
        7.57239906832298,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5275, 'Radish, raw', 100, 0.8, 2.5, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5276,
        'Currant, red, raw',
        100,
        1.31578947368421,
        12.7699248120301,
        1.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5277,
        'Brussels sprouts, raw',
        100,
        4.5375,
        9.62138888888889,
        0.516666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5278,
        'Beet, red, raw',
        100,
        1.68888888888889,
        11.9095238095238,
        0.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5279,
        'Cabbage, red, raw',
        100,
        1.51111111111111,
        6.93333333333333,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5280,
        'Plaice, raw',
        100,
        17.4947916666667,
        0.101041666666666,
        1.875,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5281,
        'Celeriac, celery root, raw',
        100,
        1.76,
        8.048,
        0.32,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5282,
        'Herring, pickled, canned',
        100,
        11.9272727272727,
        19.7105594405594,
        16.2180769230769,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5283,
        'Herring, raw',
        100,
        16.5033333333333,
        0,
        13.6166666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5284,
        'Cabbage, pointed, raw',
        100,
        2.05,
        4.94166666666666,
        0.275,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5285,
        'Spinach, raw',
        100,
        2.625,
        3.16111111111111,
        0.566666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5286,
        'Gooseberry, raw',
        100,
        0.9,
        10.49,
        0.55,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5287,
        'Tomato, ripe, raw, origin unknown',
        100,
        0.782638888888889,
        4.7662093114241,
        0.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5288,
        'Water, tap, drinking, average values',
        100,
        0,
        0,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5289,
        'Grape, raw',
        100,
        0.621875,
        16.6310197368421,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5290,
        'Peas, green, raw',
        100,
        5.9421052631579,
        12.9309473684211,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5291,
        'Squash, raw',
        100,
        1.265625,
        2.810625,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5292,
        'Artichoke, raw',
        100,
        3.26875,
        10.51125,
        0.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5293,
        'Beans, mung, dried, raw',
        100,
        23.8625,
        62.1675,
        1.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5294,
        'Oats, rolled, average values',
        100,
        12.8822178571429,
        68.1899126847291,
        6.87965517241379,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5295,
        'Rye flour, dark, wholemeal',
        100,
        8.73059647058824,
        76.6336892436975,
        2.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5296,
        'Wheat, flour, wholemeal',
        100,
        10.6106,
        73.7607636363636,
        2.36363636363636,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5297,
        'Ginger root, raw',
        100,
        1.81875,
        17.77125,
        0.75,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5298, 'Okra, raw', 100, 2, 7.02995, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5299,
        'Asparagus, green, raw',
        100,
        1.8,
        4.9,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5300,
        'Fennel, bulb, raw',
        100,
        1.35842,
        4.86158,
        0.1875,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5301,
        'Saithe, raw',
        100,
        18.1283333333333,
        0,
        0.675,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5302,
        'Buckwheat, whole, raw',
        100,
        13.0625,
        68.2225,
        3.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5303,
        'Aubergine, raw',
        100,
        0.8828125,
        4.5434375,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5304,
        'Orange, raw',
        100,
        0.885795454545454,
        11.2961822169059,
        0.116666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5305,
        'Lamb, leg, unspecified , raw',
        100,
        17.8818181818182,
        0,
        12.0181818181818,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5306,
        'Cod, fillet, raw',
        100,
        17.5503309820944,
        0,
        0.578947368421053,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5307,
        'Cheese, semihard, Feta, 40 % fidm',
        100,
        18.0766666666667,
        0.0366666666666685,
        20.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5308,
        'Pork, collar with rind, raw',
        100,
        16.102,
        0,
        23.3775,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5309,
        'Rice groats, brown, raw',
        100,
        6.8,
        78.7,
        2.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5310,
        'Sugar, sucrose, white',
        100,
        0,
        99.88,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5311,
        'Lard, pork, refined, Raffinol',
        100,
        0,
        0,
        99,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5312, 'Lard, pork', 100, 0, 0, 99, NULL);
INSERT INTO temp."Raw"
VALUES (
        5313,
        'Water, bottled, carbonated',
        100,
        0,
        1,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5314,
        'Soft drink, no sugar added, average values',
        100,
        0,
        1,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5315,
        'Fish oil, hydrogenated',
        100,
        0,
        0,
        99.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5316,
        'Goose, flesh and skin, raw',
        100,
        15.78,
        0,
        33.62,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5317,
        'Nut, coco, raw',
        100,
        4.399,
        16.6743333333333,
        36.5266666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5318,
        'Snacks, expanded, potatobasis',
        100,
        4.58125,
        63.0220833333333,
        25.0666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5319,
        'Mackerel mayonnaise salad',
        100,
        8.9,
        2.8,
        39.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5320,
        'Non-alcoholic beverage (light), concentrated (1+10), no sugar added',
        100,
        0,
        0,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5321,
        'Soy sauce',
        100,
        6.8859375,
        6.9940625,
        0.04,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5322,
        'Cheese, firm, 40 % fidm., all varieties',
        100,
        25.69864,
        1.40136,
        22.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5323,
        'Ymer, low fat',
        100,
        6.29020740740741,
        3.40201481481481,
        1.7962962962963,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5324,
        'Cheese, semihard, Mozzarella, 45 % fidm.',
        100,
        24.09726,
        1.00274,
        25.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5325,
        'Cheese, processed, 45 % fidm.',
        100,
        20.6712,
        0.951152941176468,
        24.4588235294118,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5326,
        'Duck, flesh and skin, raw',
        100,
        11.46,
        0,
        39.34,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5327,
        'Rye-flour, wheat-flour, bolted',
        100,
        9.5988,
        75.8078666666667,
        1.96666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5328, 'Pudding powder', 100, 0.3, 87.5, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5329,
        'Cheese, semihard, Havarti, 30 % fidm.',
        100,
        30.10084,
        0.08916,
        18.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5330,
        'Cheese, firm, Danbo, 45 % fidm.',
        100,
        23.6864043589744,
        0.32096366515837,
        24.9601960784314,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5331,
        'Lamb, flank, spiced, cooked',
        100,
        19,
        0.1,
        31.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5332,
        'Onions, chopped and deepfried',
        100,
        6,
        37.1,
        48,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5333, 'Tea, ready-to-drink', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5334, 'Chestnut, raw', 100, 2, 42.6, 2.7, NULL);
INSERT INTO temp."Raw"
VALUES (
        5335,
        'Coffee, instant, prepared with water',
        100,
        0.0848,
        0.7452,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5336,
        'Cheese, semihard, Esrom, 45 % fidm.',
        100,
        24.19934,
        0.39066,
        25.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5337,
        'Soft drink, sugar added',
        100,
        0,
        10,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5338,
        'Cheese, firm, Elbo, 40 % fidm.',
        100,
        26.59822,
        1.00178,
        23.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5339,
        'Potato crisps, coarse',
        100,
        6.1,
        60.77,
        26.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5340,
        'Cheese, Roquefort, Danish, 50 % fidm.',
        100,
        19.36,
        0,
        29.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5341,
        'Coffee, instant, decaffeinated, prepared with water',
        100,
        0.1007,
        0.7793,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5342,
        'Cheese, Danish Blue, 50 % fidm.',
        100,
        20.5209263773769,
        0,
        29.5001462701121,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5343,
        'Cheese, Brie, 60 % fidm.',
        100,
        18.0554,
        1.6546,
        30.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5344,
        'Bacon, stewing, raw',
        100,
        15,
        1.1,
        35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5345,
        'Coffee, beverage',
        100,
        0.1007,
        0.1393,
        0.02,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5346,
        'Rye kernels, whole/cracked',
        100,
        9.04482857142857,
        76.2151714285714,
        2.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5347,
        'Sausage, liver sausage',
        100,
        13.4,
        4.3,
        35.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5348,
        'Sour cream, 50 % fat',
        100,
        1.90124,
        1.99876,
        50,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5349,
        'Cheese, firm, 45 % fidm., all varieties',
        100,
        24.7176824489796,
        1.36913387755102,
        25.6775510204082,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5350, 'Linseeds, raw', 100, 25, 36.38, 31, NULL);
INSERT INTO temp."Raw"
VALUES (
        5351,
        'Peanut, dried',
        100,
        25.80029,
        16.15971,
        49.24,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5352,
        'Ice cream, dairy, (wholemilk based)',
        100,
        4,
        27.66,
        3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5353,
        'Double Cream, 50 % fat',
        100,
        1.90124,
        2.69876,
        50,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5354,
        'Pepper, white',
        100,
        10.4,
        74.47,
        2.12,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5355,
        'Soya flour, full fat',
        100,
        40.71875,
        26.88125,
        22.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5356,
        'Rice, brown, raw',
        100,
        9.00235,
        76.0405071428571,
        2.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5357,
        'Milk, dry, skimmed, powder',
        100,
        32.1871,
        53.7129,
        1.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5358,
        'Cheese, semihard, Havarti, 45 % fidm.',
        100,
        24.4992,
        0.0108,
        26.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5359,
        'Rose hip, dried, powder',
        100,
        4.5,
        83,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5360, 'Oyster, raw', 100, 7.8, 4.1, 1.5, NULL);
INSERT INTO temp."Raw"
VALUES (5361, 'Wine, white, dry', 100, 0.1, 0.2, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5362,
        'Fish balls, canned',
        100,
        8.3125,
        3.5975,
        2.29,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5363,
        'Rye bread crumbs with brown sugar, average values',
        100,
        6.09818,
        81.40182,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5364,
        'Wine, white, average values',
        100,
        0.06875,
        2.57125,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5365,
        'Wine, white, Rhine wine',
        100,
        0.1,
        2.8,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5366,
        'Tomato, sun-dried',
        100,
        14.1125,
        55.7575,
        2.97,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5367,
        'Ymer',
        100,
        6.08578028985507,
        3.13021004830918,
        3.55777777777778,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5368,
        'Wine, red, rosé, non-alcoholic',
        100,
        0.2,
        3.6,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5369,
        'Wine, white, medium',
        100,
        0.1,
        2.3,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5370,
        'Biscuit, sweet, ''Finnish''',
        100,
        5.5974,
        59.4026,
        31.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5371,
        'Wine, white, non-alcoholic',
        100,
        0.2,
        4.5,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5372,
        'Barley groats, raw',
        100,
        9.23472,
        77.36528,
        2.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5373,
        'Beer, lager, alc. 2.6 % by vol.',
        100,
        0.2,
        2.7,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5374,
        'Rice starch',
        100,
        0.7973,
        84.9527,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5375,
        'Liquorice allsorts',
        100,
        2.3,
        82.69,
        6.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5376,
        'Danish pastry, square',
        100,
        4.7025,
        34.5293181818182,
        26.9681818181818,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5377,
        'Salad dressing, French',
        100,
        0.76875,
        15.57125,
        44.81,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5378,
        'Cheese, Danish Blue, 60 % fidm.',
        100,
        16.269,
        1.301,
        37.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5379,
        'Cheese, firm, Fynbo, 45 % fidm.',
        100,
        25.9028,
        0.9972,
        27.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5380,
        'Eggs, chicken, egg white, pasteurized',
        100,
        9,
        1.855,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5381,
        'Lumpsucker, smoked',
        100,
        15.6,
        0,
        34.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5382,
        'Wheat, kernels, whole/cracked',
        100,
        10.69222,
        75.34778,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5383,
        'Fruit juice, mixed, unsweetened, concentrated',
        100,
        0.2,
        8.5,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5384,
        'Bread-crumbs',
        100,
        10.70075,
        77.19925,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5385,
        'Apple juice, canned or bottled',
        100,
        0.0625,
        11.6775,
        0.11,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5386,
        'Beer, lager, alc. 4.4 % by vol.',
        100,
        0.25,
        2.39,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5387,
        'Pearl-sago (potato starch)',
        100,
        0.2,
        88,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5388,
        'Cheese, firm, Maribo, 45 % fidm.',
        100,
        26.36,
        0,
        28,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5389,
        'Mustard, brown, ready made',
        100,
        5,
        17.63,
        6.83,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5390, 'Garlic, raw', 100, 6.4, 33, 0.5, NULL);
INSERT INTO temp."Raw"
VALUES (5391, 'Wine, rosé', 100, 0.2, 1.4, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5392,
        'Mayonnaise, low fat',
        100,
        0,
        0,
        51.3125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5393,
        'Minced meat in gravy, average values',
        100,
        8.1,
        4.9,
        8.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5394,
        'Cheese, firm, Samsoe, 45 % fidm.',
        100,
        26.79,
        0,
        27.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5395,
        'Baby food, infant formula, low iron content, powder',
        100,
        9.5,
        58,
        27.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5396, 'Sherry, medium', 100, 0.1, 3.4, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5397, 'Wine, red', 100, 0.2, 2.5, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5398,
        'Cheese, hard, Cheshire, 50 % fidm.',
        100,
        24.00156,
        0.99844,
        31.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5399,
        'Eggs, chicken, egg white, raw',
        100,
        10.171875,
        0.851875000000001,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5400, 'Bacon, frying, raw', 100, 13, 0, 42, NULL);
INSERT INTO temp."Raw"
VALUES (5401, 'Vermouth, dry', 100, 0.1, 5.2, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5402, 'Vermouth, sweet', 100, 0, 14.5, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5403,
        'Prune, dried plum',
        100,
        2.66666666666667,
        62.1833333333333,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5404,
        'Tamarind, Indian date, raw',
        100,
        2.8,
        62.5,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5405,
        'Pastry, croissant',
        100,
        7.71381,
        44.0681066666667,
        24.1070833333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5406,
        'Beer, Danish household, low alcohol, average values',
        100,
        0.3,
        8.3,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5407,
        'Eggs, chicken, egg white, dried',
        100,
        82.3125,
        6.8375,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5408, 'Vinegar', 100, 0.4, 0.4, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5409, 'Madeira', 100, 0, 10, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5410, 'Campari', 100, 0, 10, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5411,
        'Pastry with poppy seeds',
        100,
        7.60551,
        44.2746566666667,
        25.8583333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5412,
        'Cheese, Blue and white mould, 70 % fidm.',
        100,
        12.5960240963855,
        0,
        44.0771084337349,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5413,
        'Wine, white, sweet',
        100,
        0.2,
        5.9,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5414,
        'BITTER, all types, average values',
        100,
        0,
        0.15,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5415,
        'Pepper, black',
        100,
        10.95,
        70.95,
        3.26,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5416,
        'Beer, Danish household, low alcohol',
        100,
        0.3,
        8.6,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5417,
        'BITTER, Gammel Dansk Bitter Dram',
        100,
        0,
        0,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5418, 'Brandy, cognac', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5419,
        'Wine, white, sparkling, champagne',
        100,
        0.3,
        1.4,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5420,
        'Cheese, firm, Svenbo, 45 % fidm.',
        100,
        27.59988,
        1.00012,
        29.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5421,
        'Aquavit, spiced, average values',
        100,
        0,
        10.9,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5422, 'Sherry, dry', 100, 0.2, 1.3, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5423,
        'Olives, black, without stones, in brine',
        100,
        1,
        4.69,
        17.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5424,
        'Beer, export, alc. 5.6 % by vol.',
        100,
        0.36875,
        3.64535,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5425, 'Spirits, 70 % proof', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5426, 'Vodka', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5427,
        'Plain cake, lemon cake',
        100,
        4.5999,
        55.4001,
        21.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5428,
        'BITTER, Krabask Bitter',
        100,
        0,
        1,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5429,
        'Cheese, hard, Emmentaler, 45 % fidm.',
        100,
        28.06,
        0,
        29.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5430,
        'Breakfast cereal, Rice Krispies',
        100,
        5.97975,
        84.39525,
        1.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5431,
        'Aquavit, 40 % vol., average values',
        100,
        0,
        0,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5432,
        'Ice cream, dairy, popsickle, strawberry',
        100,
        4,
        24,
        5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5433,
        'Lemon juice, fresh',
        100,
        0.38125,
        3.81875,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5434,
        'Cheese, semihard, Esrom, 60 % fidm.',
        100,
        19.40158,
        0.44842,
        38.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5435,
        'Cream yoghurt, 9 % fat, with fruit',
        100,
        2.9986,
        12.0014,
        7.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5436,
        'Tofu, soy bean curd',
        100,
        13.3125,
        1.0225,
        7.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5437, 'Port', 100, 0.1, 11.1, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5438,
        'Grapefruitjuice, canned, unswtnd.',
        100,
        0.3,
        7.9,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5439,
        'Plain cake, chocolate cake',
        100,
        5.301,
        50.599,
        23.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5440,
        'Bouillon, chicken, prepared',
        100,
        0.28125,
        0.28875,
        0.23,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5441,
        'Danish pastry, cinnamon roll',
        100,
        4.4973,
        40.7027,
        26,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5442,
        'Danish pastry, average values',
        100,
        4.4973,
        46.1027,
        27.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5443,
        'Plain cake, average values',
        100,
        4.55067272727273,
        56.6998167832168,
        20.1576923076923,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5444,
        'Pop corn, oil and salt added, traditional',
        100,
        9.625,
        65.975,
        20.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5445,
        'Bulgur, parboiled, raw',
        100,
        10.794375,
        75.235625,
        1.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5446,
        'Lentils, sprouted, raw',
        100,
        8.9625,
        22.1475,
        0.55,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5447, 'Whisky', 100, 0, 0.1, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5448,
        'Sausage, beef salami',
        100,
        20.1,
        0,
        36.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5449,
        'Cheese, semihard, Havarti, 60 % fidm.',
        100,
        19.49728,
        0.71272,
        38.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5450,
        'Beer, strong, alc. 7.6 % by vol.',
        100,
        0.4,
        4.6141,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5451,
        'Cream, cultured, 9 % fat',
        100,
        6.8266,
        4.2834,
        8.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5452,
        'Pork/beef sausage, teewurst',
        100,
        10.9,
        0,
        48.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5453,
        'Hazelnut, dried',
        100,
        13.197,
        19.453,
        60.58,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5454,
        'Cream, 9 % fat',
        100,
        3.3176,
        4.0884,
        9.024,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5455,
        'Pasta, raw',
        100,
        12.3259909090909,
        75.5740090909091,
        1.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5456,
        'Cheese, Quark, 5 % fidm.',
        100,
        9.14466666666667,
        3.942,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5457,
        'Cheese, hard, Grana, 32 % fidm.',
        100,
        33.14,
        0,
        24.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5458,
        'Cheese, hard, Parmesan, 32 % fidm.',
        100,
        33.59708,
        1.50292,
        24.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5459, 'Gin', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5460,
        'Cream, cultured, 9 % fat',
        100,
        3.43244,
        2.82556,
        9.042,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5461,
        'Gherkin, pickled, no sugar added',
        100,
        0.33125,
        2.25875,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5462,
        'Yeast, baker`s, compressed',
        100,
        7.6608,
        19.6392,
        1.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5463,
        'Biscuit, sweet, vanilla flavoured',
        100,
        4.7025,
        63.4975,
        28.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5464,
        'Cheese, hard, Cheddar, Danish',
        100,
        25.81,
        0,
        33.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5465,
        'Fish cake, raw',
        100,
        8.5,
        12.8,
        2.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5466, 'Rum', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5467,
        'Pistachio nuts, dried',
        100,
        21.5975,
        21.0925,
        50.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5468,
        'Whipped egg white and sugar with chocolate covering',
        100,
        4.4,
        63.55,
        15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5469,
        'Millet, flakes',
        100,
        12.09725,
        73.90275,
        3.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5470,
        'Cheese, unripened, smoked, 1 % fat',
        100,
        11.3564,
        3.68274893617021,
        0.520851063829787,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5471,
        'Cod, liver, canned',
        100,
        4.6,
        4.4,
        55.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5472,
        'Snacks, expanded, cornbased',
        100,
        7.375,
        62.425,
        25.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5473,
        'Pineapple juice, canned',
        100,
        0.3625,
        12.8675,
        0.12,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5474,
        'Cheese, whey, Myseost, 33 % fidm.',
        100,
        14.99938,
        32.60062,
        27,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5475,
        'Cheese, hard, Gruyere, 45 % fidm.',
        100,
        29.90306,
        1.39694,
        30.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5476,
        'Cabbage, red, canned, no sugar added',
        100,
        0.5,
        5.5,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5477,
        'Danish pastry, pretzel',
        100,
        4.2978,
        39.8022,
        27,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5478, 'Sherry, sweet', 100, 0.3, 6.5, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5479, 'Fig, dried', 100, 3.3, 72.21, 0.93, NULL);
INSERT INTO temp."Raw"
VALUES (
        5480,
        'Cranberry, raw',
        100,
        0.3875,
        12.2025,
        0.13,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5481,
        'Pastry, ''linse''',
        100,
        5.5632,
        42.4368,
        22,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5482,
        'Pop corn, oil and salt added',
        100,
        9.625,
        61.8416666666667,
        24.3333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5483,
        'Danish pastry, christmas cake',
        100,
        5.1984,
        41.1016,
        31,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5484,
        'Sausage, salami',
        100,
        14.5122727272727,
        0,
        45.1877272727273,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5485, 'Pound cake', 100, 6.1, 49.7, 25, NULL);
INSERT INTO temp."Raw"
VALUES (5486, 'Quinces, raw', 100, 0.4, 15.3, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (5487, 'Pumpkin, raw', 100, 0.6, 4.2, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5488,
        'Salad dressing, blue cheese',
        100,
        4.8,
        7.4,
        52.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5489,
        'Radishes, oriental, daikon, Japanese radish, raw',
        100,
        0.6,
        4.21952,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5490,
        'Sesame seeds, whole',
        100,
        19.5835,
        17.2515,
        53.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5491,
        'Olives, green, pickled, canned',
        100,
        1,
        7.4,
        11.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5492,
        'Seaweed, agar, dried',
        100,
        2.3,
        74.3,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5493,
        'Crispbread, knackebrot, rye, coarse',
        100,
        9.30932142857143,
        79.7378214285714,
        1.975,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5494,
        'Macaroni, spaghetti, whole meal, raw',
        100,
        13.0986,
        73.1014,
        2.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5495,
        'Rye bread, dark, wholemeal, imported',
        100,
        5.6551,
        44.7749,
        1.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5496, 'Pear, canned', 100, 0.3, 18.1, 0.2, NULL);
INSERT INTO temp."Raw"
VALUES (5497, 'Turnip, raw', 100, 0.6, 6.7, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5498,
        'BITTER, sweet, all types, average values',
        100,
        0,
        18,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5499,
        'Bean, adzuki, sprouted, raw',
        100,
        10,
        24,
        1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5500,
        'Millet, whole-grain',
        100,
        11.0187,
        72.8413,
        4.22,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5501,
        'Pop corn, microwave oven',
        100,
        9.625,
        60.3416666666667,
        26.8333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5502, 'Mussel, raw', 100, 11.9, 3.69, 2.24, NULL);
INSERT INTO temp."Raw"
VALUES (
        5503,
        'Cucumber, raw',
        100,
        0.720552884615384,
        2.57270434521719,
        0.025,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5504,
        'Loquat, raw',
        100,
        0.43125,
        12.13875,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5505,
        'Cashew nuts, oil roasted',
        100,
        16.8381,
        29.8819,
        47.77,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5506,
        'Danish pastry, butter ring',
        100,
        4.2978,
        44.7022,
        31,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5507,
        'Nut paste with cacao, spreads',
        100,
        7,
        60.5,
        29.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5508,
        'Corn flakes, average values',
        100,
        7.48684210526316,
        84.4652631578947,
        1.35368421052632,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5509,
        'Breakfast cereal, müsli, average values',
        100,
        9.42923,
        70.4194063636364,
        11.1181818181818,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5510,
        'Mock turtle ragout, average values',
        100,
        7.9,
        5.5,
        6.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5511,
        'Tomatojuice, canned',
        100,
        0.7625,
        4.2375,
        0.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5512,
        'Apricot, canned, light syrup pack',
        100,
        0.53125,
        17.14875,
        0.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5513,
        'Gherkin, pickled',
        100,
        0.33125,
        13.56875,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5514,
        'Orange juice, canned',
        100,
        0.5875,
        9.6845,
        0.14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5515,
        'Pork sausage, saveloy type, low fat',
        100,
        10,
        4.2,
        8.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5516,
        'Grape juice',
        100,
        0.5625,
        14.9475,
        0.08,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5517, 'Crayfish, raw', 100, 14.6, 1.2, 0.5, NULL);
INSERT INTO temp."Raw"
VALUES (5518, 'Corn starch', 100, 0.4, 88.8, 0.6, NULL);
INSERT INTO temp."Raw"
VALUES (
        5519,
        'Carrot, canned',
        100,
        0.6375,
        5.542,
        0.19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5520,
        'Potatoes, mashed, instant powder with dry milk',
        100,
        10.9,
        77.4,
        1.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5521,
        'Rye bread, light',
        100,
        5.44522,
        46.6836688888889,
        1.17555555555556,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5522,
        'Bitter, Jägermeister',
        100,
        0,
        14,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5523, 'Salt, table', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5524,
        'Rye bread, dark, pumpernickel',
        100,
        5.33445,
        46.11055,
        1.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5525,
        'Spring roll, frozen',
        100,
        5.8,
        17.6,
        6.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5526,
        'Apricot, canned, no sugar added',
        100,
        0.7125,
        6.3875,
        0.16,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5527,
        'Pumpkin, canned',
        100,
        0.4,
        25.6,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5528,
        'Pineapple, canned',
        100,
        0.4,
        15.5,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5529,
        'Onions, chopped, frozen',
        100,
        0.7875,
        6.82245,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5530,
        'Cream pastry, layer cake',
        100,
        3.31354166666667,
        35.139125,
        16.669,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5531,
        'Wheat rusk',
        100,
        9.3024,
        75.5976,
        5.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5532,
        'Wheat rusk',
        100,
        9.3024,
        75.5976,
        5.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5533,
        'Chayote fruit, raw',
        100,
        0.81875,
        4.51125,
        0.13,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5534,
        'Crispbread, knackebrot, rye, fine',
        100,
        10.6989,
        78.2011,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5535,
        'Mustard, yellow, ready made',
        100,
        5.7,
        21.9,
        6.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5536,
        'Peach, canned',
        100,
        0.5,
        17.35,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5537,
        'Liqueur, average values',
        100,
        0,
        29.3,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5538,
        'Corn flakes, frosted',
        100,
        5.25,
        88.9466666666667,
        0.833333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5539,
        'Sunflower seeds, decorticated, dried',
        100,
        24.1875,
        13.5875,
        54.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5540,
        'Sauce gravad lax',
        100,
        1.6,
        13.1,
        56,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5541,
        'Snacks, extruded, cornbased',
        100,
        9,
        53.7366666666667,
        32.1333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5542,
        'Oats, rolled, enriched',
        100,
        12.98076,
        68.5574218181818,
        6.49181818181818,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5543,
        'Muskmelon, raw',
        100,
        0.82,
        5.19888888888889,
        0.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5544,
        'Tomato, Danish, ripe, raw',
        100,
        0.7,
        4.5,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5545,
        'Tangerine, canned',
        100,
        0.455405405405405,
        14.5445945945946,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5546,
        'Plaice, fillet, breaded, frozen',
        100,
        11.9,
        20.15,
        1.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5547,
        'Fruit juice, mixed, sweetened, concentrated',
        100,
        0.2,
        43.4,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5548,
        'Apricot, dried',
        100,
        3.53,
        64.3533333333333,
        1.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5549,
        'Sauce, barbeque',
        100,
        0,
        36.26,
        0.29,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5550,
        'Breakfast cereal, Müsli with added sugar',
        100,
        11.99814,
        70.60186,
        6.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5551,
        'Breakfast cereal, Müsli, no sugar added',
        100,
        11.99814,
        70.60186,
        6.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5552,
        'Rye bread, fine with wheat flour, industrially produced',
        100,
        6.4713,
        49.2962,
        1.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5553, 'Lime, raw', 100, 0.7, 5.96, 0.2, NULL);
INSERT INTO temp."Raw"
VALUES (
        5554,
        'Crab apple, raw',
        100,
        0.4,
        19.94,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5555,
        'Elderberry, juice, concentrated, sugar not added',
        100,
        0.5,
        6,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5556,
        'Cream, 13 % fat (UHT)',
        100,
        2.80082,
        3.83218,
        13.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5557,
        'Frog legs, frozen',
        100,
        16.4,
        0.2,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5558,
        'Pepper, hot chili, canned',
        100,
        0.9,
        6.09996,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5559,
        'Sesame seed, decorticated',
        100,
        22.737,
        13.443,
        57.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5560,
        'Strawberry juice, concentrated, sugar not added',
        100,
        0.5,
        7,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5561,
        'Raspberry juice, concentrated, sugar not added',
        100,
        0.5,
        7,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5562, 'Burbot, raw', 100, 16.4, 0, 0.5, NULL);
INSERT INTO temp."Raw"
VALUES (5563, 'Hake, raw', 100, 16.3, 0, 0.4, NULL);
INSERT INTO temp."Raw"
VALUES (
        5564,
        'Cream pastry, strawberry cake',
        100,
        2.9,
        33,
        8.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5565,
        'Salad dressing, mayonnaise, low calorie',
        100,
        0.95,
        13.55,
        13.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5566, 'Liqueur, Curacao', 100, 0, 25.5, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5567,
        'Lettuce, iceberg (incl. crisphead types), raw',
        100,
        0.8,
        3.47378764705882,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5568,
        'Oats, rolled, not enriched',
        100,
        13.2712,
        68.2859818181818,
        6.691,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5569,
        'Currant, red, juice, concentrated, sugar not added',
        100,
        0.5,
        8,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5570,
        'Strawberries, frozen, sweetened, whole',
        100,
        0.51875,
        20.93125,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5571,
        'Cream, 13 % fat',
        100,
        2.9029,
        4.34937891156463,
        13.4727210884354,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5572, 'Saccharin', 100, 0.9375, 89.1225, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5573,
        'Sausage, salami',
        100,
        13.8826923076923,
        2.7892735042735,
        49.2011111111111,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5574,
        'Radish, white, raw',
        100,
        1,
        4.6,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5575,
        'Melon, cantaloup, raw',
        100,
        0.8375,
        8.1725,
        0.19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5576,
        'Beet, red, canned, no sugar added',
        100,
        0.9125,
        7.2075,
        0.14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5577,
        'Meat balls, canned',
        100,
        7.1,
        2.9,
        9.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5578, 'Octopus, raw', 100, 16.4, 0.7, 0.9, NULL);
INSERT INTO temp."Raw"
VALUES (
        5579,
        'Horse-radish, raw',
        100,
        7.7,
        20.5,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5580,
        'Strawberries, canned',
        100,
        0.4,
        19.5,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5581,
        'Pork in curry sauce, frozen',
        100,
        8.5,
        2,
        8.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5582,
        'Currant, black, juice, concentrated, sugar not added',
        100,
        0.5,
        11,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5583,
        'Snail, edible, raw',
        100,
        16,
        2.7,
        1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5584,
        'Carrot, frozen',
        100,
        1.08333333333333,
        6.99444444444445,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5585,
        'Pepper, sweet, canned',
        100,
        0.8,
        3.9,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5586,
        'Pastry, coco macaroon',
        100,
        6.2985,
        49.3015,
        28.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5587,
        'Ice cream, dairy, popsickle, vanilla',
        100,
        4,
        20,
        10,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5588,
        'Ice cream, dairy, nougat',
        100,
        4,
        22.9,
        10,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5589,
        'Ice cream, dairy, vanilla',
        100,
        4,
        22.9,
        10,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5590,
        'Grape, green, raw',
        100,
        0.621875,
        16.773125,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5591,
        'Papaya, raw',
        100,
        0.675,
        11.4275,
        0.2675,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5592,
        'Milk, dry, whole, powder',
        100,
        23.925,
        40.595,
        26.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5593,
        'Pizza Romana, frozen',
        100,
        7.2,
        25,
        5.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5594,
        'Cherry juice, concentrated, sugar not added',
        100,
        0.5,
        13,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5595,
        'Lingonberry, jam',
        100,
        0.2,
        39.5,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5596,
        'Pecans, dried',
        100,
        9.169,
        13.851,
        71.97,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5597,
        'Cheese, cottage, 20 % fidm.',
        100,
        13.2704,
        2.7846,
        4.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5598,
        'Shrimp, boiled, shell removed',
        100,
        15.2975,
        0,
        0.825,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5599, 'Swede, raw', 100, 1, 8.7, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5600,
        'Quark with fruit',
        100,
        6.90954,
        15.97846,
        8.214,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5601,
        'Carrot juice, canned',
        100,
        0.95,
        9.26495,
        0.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5602,
        'Cod, roe, canned',
        100,
        13.7,
        2.75,
        3.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5603,
        'Whey, sweet, fluid',
        100,
        0.84854,
        5.14146,
        0.36,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5604,
        'Shrimp, canned',
        100,
        16.5,
        0.67,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5605,
        'Blueberry, frozen, unsweetened',
        100,
        0.41875,
        12.17125,
        0.64,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5606, 'Rose hip syrup', 100, 0, 57.8, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5607, 'Date, dried', 100, 2, 77.55, 0.5, NULL);
INSERT INTO temp."Raw"
VALUES (
        5608,
        'Bouillon, beef, cube, prepared',
        100,
        1.1375,
        0.2625,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5609,
        'Ice cream, dairy, three coloured',
        100,
        4,
        22.9,
        10,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5610, 'Tea, leaves', 100, 19.6, 62.1, 2, NULL);
INSERT INTO temp."Raw"
VALUES (
        5611,
        'Rhubarb, frozen',
        100,
        0.9,
        6.8,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5612,
        'Gherkin cucumber, raw',
        100,
        1.2,
        3.4,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5613, 'Lemon sole, raw', 100, 16.3, 0, 0.7, NULL);
INSERT INTO temp."Raw"
VALUES (
        5614,
        'Cucumber, large, pickled',
        100,
        0.5,
        17.5,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5615,
        'Chicory, raw',
        100,
        1.14444444444444,
        3.72222222222222,
        0.166666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5616, 'Dab, raw', 100, 16.6, 0, 1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5617,
        'Kidney, calf, raw',
        100,
        16.4,
        3.8,
        1.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5618,
        'Cod, fillet, breaded, raw',
        100,
        13.675,
        13.415,
        0.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5619,
        'Blue whiting, fillet, raw',
        100,
        17.5,
        0,
        0.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5620,
        'Liver, pig, in cream sauce, frozen',
        100,
        11.9,
        6.9,
        5.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5621,
        'Pastry, average values',
        100,
        6.049125,
        54.1953194444444,
        26,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5622, 'Whiting, raw', 100, 18.2, 0, 0.6, NULL);
INSERT INTO temp."Raw"
VALUES (
        5623,
        'Salt, sea salt (no iodine fortification)',
        100,
        0,
        0,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5624,
        'Oat Bran, Danish',
        100,
        17.29761,
        66.23239,
        7.03,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5625, 'Carrot, raw', 100, 0.25, 8.22, 0.05, NULL);
INSERT INTO temp."Raw"
VALUES (
        5626,
        'Mandarin orange, raw',
        100,
        0.797916666666667,
        11.7590767973856,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5627, 'Perch, raw', 100, 16.4, 0, 1.3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5628,
        'Cod, fillet, boiled',
        100,
        17.5503309820944,
        0,
        0.578947368421053,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5629,
        'Wolffish (catfish), raw',
        100,
        16.1,
        2,
        2.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5630,
        'Ice cream, dairy (cream based)',
        100,
        4,
        25.1,
        10,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5631,
        'Heart, veal, raw',
        100,
        14.9,
        3.3,
        3.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5632,
        'Kidney, ox, raw',
        100,
        15.6,
        4.6,
        2.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5633, 'Torsk, tusk, raw', 100, 19, 0, 0.3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5634,
        'Cod, fillet, smoked, with skin',
        100,
        19,
        0,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5635,
        'Prickly pear, Indian fig, raw',
        100,
        0.73125,
        9.56875,
        0.51,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5636,
        'Cabbage, Chinese, raw',
        100,
        1.25789473684211,
        3.20043859649123,
        0.166666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5637, 'Kidney, pig, raw', 100, 16, 1.73, 3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5638,
        'Norway pout, flesh, raw',
        100,
        17.6,
        0,
        1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5639,
        'Pastry, frangipani pastry',
        100,
        5.7,
        52.2,
        27.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5640, 'Figs, raw', 100, 0.75, 19.18, 0.3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5641,
        'Carambola, raw',
        100,
        1.0375,
        7.9625,
        0.33,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5642,
        'Turbot, raw',
        100,
        16.4,
        2.87833333333333,
        1.42166666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5643,
        'Haddock, raw',
        100,
        17.925,
        0,
        0.575,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5644,
        'Pork, liver paste, Danish, low fat',
        100,
        10,
        12.72,
        6.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5645,
        'Flounder, raw',
        100,
        17.9105128205128,
        0,
        1.57615384615385,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5646,
        'Endive (escarole), raw',
        100,
        1.25,
        3.34967,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5647,
        'Pollack, raw',
        100,
        17.3783333333333,
        0,
        0.183333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5648,
        'Plain cake, frangipani cake',
        100,
        5.1984,
        60.4016,
        19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5649,
        'Caviar, Danish (roe, lump-sucker)',
        100,
        12.9,
        0.77,
        5.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5650,
        'Pizza Napolitana, frozen',
        100,
        8.6,
        24,
        6.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5651,
        'Lettuce, looseleaf, raw',
        100,
        1.3625,
        2.79717,
        0.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5652,
        'Split peas, dry, raw',
        100,
        22,
        63.8,
        2.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5653,
        'Persimmon, kaki fruit, raw',
        100,
        0.6609375,
        23.4004513888889,
        0.2975,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5654,
        'Ling, fillet, raw',
        100,
        19.5,
        0,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5655, 'Sole, raw', 100, 18.7, 0.1, 1.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5656,
        'Tomato, peeled, canned',
        100,
        1.2,
        3.9,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5657,
        'Meat balls, average values',
        100,
        8.8,
        6,
        9.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5658,
        'Fish balls, fried',
        100,
        13.7307692307692,
        9.84461538461538,
        6.73076923076923,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5659,
        'Endive (frisee), raw',
        100,
        1.3,
        3.29967,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5660,
        'Lettuce, Cos or romaine, raw',
        100,
        1.22366071428571,
        3.86315214285714,
        0.03,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5661,
        'Pikeperch, raw',
        100,
        19.2,
        0.9,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5662,
        'Potatoes, mashed, instant powder without dry milk',
        100,
        8.21875,
        85.51125,
        0.54,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5663,
        'Redfish, ocean perch, raw',
        100,
        18.9,
        0,
        0.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5664,
        'Spring roll, deepfried, fast food',
        100,
        6.13858695652174,
        23.8405434782609,
        9.81304347826087,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5665, 'Carrot, raw', 100, 0.375, 9.465, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5666,
        'Cod, liver, smoked',
        100,
        5.1,
        1,
        66.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5667, 'Cod, liver, raw', 100, 5.1, 1, 66.6, NULL);
INSERT INTO temp."Raw"
VALUES (
        5668,
        'Nectarine, raw',
        100,
        0.796527777777778,
        10.6238141025641,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5669, 'Pike, raw', 100, 19, 0.7, 1.2, NULL);
INSERT INTO temp."Raw"
VALUES (
        5670,
        'Salad dressing, oil and vinegar, low calorie',
        100,
        0.2625,
        8.5375,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5671,
        'Squash, all varieties, raw',
        100,
        1.7,
        2.6,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5672,
        'Wheat bread, with whole kernels',
        100,
        6.9027,
        49.1973,
        2.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5673,
        'Avocado, raw',
        100,
        1.6171875,
        5.9878125,
        15.0625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5674,
        'Wheat bran',
        100,
        16.150445,
        64.849555,
        5.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5675,
        'Non-alcoholic beverage, concentrated (1+5), sugar added',
        100,
        0,
        63,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5676,
        'Beet, red, canned',
        100,
        1,
        14.4,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5677,
        'Lentils, brown, dried, raw',
        100,
        25.1,
        60.1,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5678,
        'Tomato, imported, ripe, raw',
        100,
        1.1,
        7.2,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5679,
        'Lobster, boiled',
        100,
        18.7,
        0.3,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5680,
        'Lobster, canned',
        100,
        18.7,
        0.3,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5681,
        'Butterhead lettuce (US), cabbage lettuce (UK), raw',
        100,
        1.27777777777778,
        2.07222222222222,
        0.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5682,
        'Margarine, 70 % fat, table use, soft, vegetable fat',
        100,
        0,
        0,
        70.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5683,
        'Cabbage, red, canned',
        100,
        1,
        18.4,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5684,
        'Lychee, raw',
        100,
        0.83125,
        16.52875,
        0.44,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5685,
        'Pizza, frozen, average values',
        100,
        8.9,
        25.7,
        6.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5686,
        'Pomegranate, raw',
        100,
        0.95,
        17.17,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5687,
        'Guava, common, raw',
        100,
        0.8765625,
        11.1434375,
        0.455,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5688,
        'Spotted wolffish, fillet, raw',
        100,
        16,
        0,
        4.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5689,
        'Mussels, in water, canned',
        100,
        17,
        1.92,
        3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5690,
        'Cape gooseberries, raw',
        100,
        1.9,
        11.51,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5691,
        'Cheese, unripened, smoked, 10 % fat',
        100,
        8.63426666666667,
        3.2024,
        9.13333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5692, 'Bream, raw', 100, 16.7, 1.2, 4, NULL);
INSERT INTO temp."Raw"
VALUES (
        5693,
        'Heart, beef, raw',
        100,
        17.71875,
        0.99125,
        3.08,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5694,
        'Pak choi (bok choy), raw',
        100,
        1.5,
        2.1799,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5695,
        'Heart, pork, raw',
        100,
        16.1316666666667,
        0,
        4.26833333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5696,
        'Rye bread, dark with high fat seeds and whole kernels, industrially produced',
        100,
        5.67233652173913,
        42.490272173913,
        4.30434782608696,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5697,
        'Walnuts, dried',
        100,
        14.0715,
        17.2399285714286,
        64.6285714285714,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5698,
        'Asparagus, canned',
        100,
        1.6,
        3,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5699,
        'Garlic, dried, powder',
        100,
        16.8,
        72.6,
        0.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5700,
        'Pizza with broccoli, frozen',
        100,
        8.7,
        26,
        6.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5701,
        'Chickory, raddichio, raw',
        100,
        1.43125,
        4.47854,
        0.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5702,
        'Eggs, chicken, dried',
        100,
        50.65625,
        4.15875000000001,
        38.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5703, 'Lobster, raw', 100, 19.6, 0.7, 1.3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5704,
        'Norway lobster, raw',
        100,
        19.6,
        0.7,
        1.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5705,
        'Crispbread, knackebrot, wheat, coarse',
        100,
        13.0017,
        70.5983,
        6.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5706,
        'Curry powder',
        100,
        12.7,
        58.42,
        13.81,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5707,
        'Norway haddock, fillet, raw',
        100,
        19.1,
        0,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5708,
        'Potato crisps, fatreduced',
        100,
        5,
        66.9466666666667,
        23,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5709,
        'Sausage, tongue sausage, saveloy type',
        100,
        9.75,
        4.77,
        9.98,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5710,
        'Saithe (UK), pollock (US), fillet, frozen',
        100,
        20.5,
        0.3,
        0.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5711,
        'Cream, 18 % fat',
        100,
        2.69874,
        4.30126,
        18,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5712,
        'Beans, wax, canned',
        100,
        1.4,
        5.2,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5713,
        'Wheat bread, with wheat bran',
        100,
        9.7983,
        42.6017,
        1.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5714,
        'Pork, ham, cured, canned',
        100,
        15.7,
        0,
        5.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5715,
        'Bread, bolted rye and wheat, sour, with caraway seeds',
        100,
        7.00925,
        50.77825,
        2.82,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5716,
        'Eggs, chicken, long eggs, boiled, frozen',
        100,
        11.9583333333333,
        2.215,
        8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5717, 'Blackberry, jam', 100, 0.3, 49, 0.2, NULL);
INSERT INTO temp."Raw"
VALUES (
        5718,
        'Marmalade, mango',
        100,
        0.2,
        54,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5719,
        'Cream, cultured, 18 % fat',
        100,
        2.8246,
        2.85950714285715,
        18.5767857142857,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5720,
        'Bread, wholemeal',
        100,
        8.28284,
        50.5727963636364,
        2.643,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5721,
        'Liver, goose, raw',
        100,
        16.36875,
        6.32125,
        4.28,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5722,
        'Veal, eye of round, salted, boiled',
        100,
        19.2017391304348,
        0,
        2.59826086956522,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5723,
        'Chocolate, milk',
        100,
        7.1,
        58.1766666666667,
        31.6733333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5724,
        'Bread, white',
        100,
        7.60078125,
        50.18671875,
        2.82,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5725,
        'Wheat bread, italien type, industrially produced',
        100,
        8.16728571428571,
        52.2612857142857,
        2.47142857142857,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5726,
        'Miso, soy paste',
        100,
        10.5,
        23.5,
        4.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5727,
        'Beans, green, canned',
        100,
        1.5,
        5.15,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5728,
        'Lentils, dried',
        100,
        26.2,
        59.2,
        2.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5729,
        'Gizzard, chicken, raw',
        100,
        18.2,
        0.5,
        4.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5730, 'Kohlrabi, raw', 100, 1.7, 6.2, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5731,
        'Watercress, raw',
        100,
        1.7,
        3.09969,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5732,
        'Salad dressing, soured fresh cream, low calorie',
        100,
        1.2875,
        9.3125,
        18.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5733,
        'Eggs, chicken, yolk, dried',
        100,
        34.5,
        2.50000000000001,
        56.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5734,
        'Baguette, wheat, fine, industrially produced',
        100,
        8.19560869565217,
        54.541347826087,
        1.2304347826087,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5735,
        'Potato, canned',
        100,
        1.4125,
        13.23745,
        0.21,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5736,
        'Brazil nuts, dried, raw',
        100,
        15.00193,
        13.7347366666667,
        64.9633333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5737,
        'Pork, ham, boiled, sliced',
        100,
        17.9,
        0.29,
        4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5738, 'Charr, raw', 100, 18.5, 0, 2.7, NULL);
INSERT INTO temp."Raw"
VALUES (
        5739,
        'Meat balls, frozen',
        100,
        10.5,
        9,
        9.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5740,
        'Eggs, chicken, pasteurized',
        100,
        11.4375,
        2.015,
        8.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5741,
        'Liver, calf, raw',
        100,
        20.4375,
        3.4625,
        3.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5742,
        'Veal, lean meat, raw',
        100,
        18.1,
        0,
        2.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5743,
        'Halibut, atlantic, smoked',
        100,
        20.6,
        2.9025,
        0.9975,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5744,
        'Wheat bread, for toasting, industrially produced',
        100,
        7.89694285714286,
        51.0630571428571,
        2.82142857142857,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5745, 'Pig, fat, raw', 100, 3.16, 0, 72, NULL);
INSERT INTO temp."Raw"
VALUES (
        5746,
        'Blackberry, frozen, unsweetened',
        100,
        1.18125,
        15.66875,
        0.43,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5747,
        'Coconut meat, desiccated',
        100,
        6.466,
        26.959,
        61.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5748,
        'Sweetbread, lamb, raw',
        100,
        15.3,
        0,
        7.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5749,
        'Chicken, hen, flesh only',
        100,
        20.33125,
        0.96875,
        2.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5750,
        'Currant, red, jelly',
        100,
        0.2,
        59,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5751,
        'Halibut, atlantic, raw',
        100,
        20.6,
        2.9025,
        0.9975,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5752,
        'Liver, broiler or fryer, raw',
        100,
        19.1333333333333,
        0.370238095238095,
        4.19642857142857,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5753, 'Trout, raw', 100, 17.94, 0, 6.7, NULL);
INSERT INTO temp."Raw"
VALUES (
        5754,
        'Sweet potato, raw',
        100,
        1.303125,
        17.231875,
        0.3175,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5755,
        'Chard, swiss, raw',
        100,
        1.8,
        3.74,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5756,
        'Onion, raw',
        100,
        1.24583333333333,
        10.291484962406,
        0.0857142857142857,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5757, 'Strawberry, jam', 100, 0.3, 50, 0.3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5758,
        'Sapodilla, raw',
        100,
        0.4375,
        19.9625,
        1.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5759,
        'Coffee, instant, powder',
        100,
        16.536,
        64.564,
        1.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5760,
        'Cheese, hard, Parmesan, grated',
        100,
        38.45864,
        4.06136,
        28.61,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5761,
        'Rolls, wheat with coarse meal, industrially produced',
        100,
        8.227,
        50.1413333333333,
        2.71666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5762,
        'Pork, ham, topside (M. Semimembranosus), lean, raw',
        100,
        21.3907608695652,
        0.0567391304347891,
        2.19090909090909,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5763,
        'Cherry, canned in sugar',
        100,
        1,
        35,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5764,
        'Cod, fillet, fried',
        100,
        18.7237113402062,
        0.0684536082474182,
        5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5765,
        'Duck, flesh only, raw',
        100,
        18.28125,
        1.77375,
        5.115,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5766,
        'Turkey, breast, boiled/smoked, sliced',
        100,
        21,
        1.72,
        1.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5767,
        'Pizza with tuna, frozen',
        100,
        11,
        28,
        5.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5768,
        'Cream pastry, marzipan- and cocoa-covered',
        100,
        5.19479166666667,
        55.4752083333333,
        14.65,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5769,
        'Turkey, flesh only, raw',
        100,
        21.2675,
        0,
        2.2325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5770,
        'Pork, ham, silverside (M. Semitendinosus), lean, raw',
        100,
        19.3335774410774,
        0,
        4.44074074074074,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5771,
        'Loganberries, frozen',
        100,
        1.51875,
        13.02125,
        0.31,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5772,
        'Rye bread, with siftings (soft) and high fat seeds, industrially produced',
        100,
        6.44745,
        42.7857318181818,
        5.16818181818182,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5773,
        'Lump, lumpsucker, raw',
        100,
        9.8,
        0.3,
        14.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5774,
        'Duck, breast, flesh only, raw',
        100,
        19.4,
        0,
        4.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5775,
        'Pork, loin, lean, raw',
        100,
        22.34,
        0,
        1.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5776,
        'Onions, spring, raw',
        100,
        1.83125,
        7.3387,
        0.19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5777,
        'Liver, ox, raw',
        100,
        20.3,
        3.35676470588235,
        3.03823529411765,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5778,
        'Pork, loin fillet, smoked',
        100,
        19.4,
        0.237826086956522,
        3.76217391304348,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5779,
        'Pastry, napoleon hat',
        100,
        6.4011,
        55.4989,
        27,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5780,
        'Wheat bread, with coarse meal, for toasting, industrially produced',
        100,
        8.38406666666667,
        50.3876,
        3.18333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5781,
        'Pork, ham, silverside (M. Biceps Femoris), lean, raw',
        100,
        20.2061904761905,
        0,
        3.85238095238095,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5782,
        'Liver, duck, raw',
        100,
        18.7375,
        3.5225,
        4.64,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5783,
        'Marzipanproduct, chocolatecover, average values',
        100,
        4.9,
        61.46,
        29.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5784,
        'Pork, saddle, smoked, raw',
        100,
        18.4,
        1.1,
        4.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5785,
        'Pork, saddle, smoked, boiled',
        100,
        18.4,
        0.8,
        4.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5786,
        'Tuna, in tomato sauce, canned',
        100,
        17.2,
        4.5925,
        2.9075,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5787,
        'Cabbage, savoy, raw',
        100,
        2,
        6.0999,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5788,
        'Asparagus, all types, raw',
        100,
        1.92727272727273,
        4.04242424242424,
        0.266666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5789,
        'Chicken, breast, boiled, sliced',
        100,
        20.6,
        0.36,
        3.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5790, 'Lemon peel, raw', 100, 1.5, 16, 0.3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5791,
        'Pistachio nuts, dry roasted with salt',
        100,
        21.3484,
        27.6416,
        45.97,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5792,
        'Cherry juice, concentrated, sugar added',
        100,
        0.5,
        41,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5793,
        'Currant, red, juice, concentrated, sugar added',
        100,
        0.5,
        41,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5794,
        'Raspberry juice, concentrated, sugar added',
        100,
        0.5,
        41,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5795,
        'Strawberry juice, concentrated, sugar added',
        100,
        0.5,
        41,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5796,
        'Beans, green, raw',
        100,
        1.9,
        6.61358974358974,
        0.233333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5797,
        'Dill weed, dried',
        100,
        19.9625,
        55.8175,
        4.36,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5798,
        'Bread, white, for sausages',
        100,
        8.68775,
        47.5039166666667,
        3.825,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5799,
        'Celeriac, celery leaves, raw',
        100,
        1.76,
        8.048,
        0.32,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5800, 'Whitefish, raw', 100, 21.6, 0, 3, NULL);
INSERT INTO temp."Raw"
VALUES (
        5801,
        'Lingonberry, raw',
        100,
        0.8,
        12.8333333333333,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5802,
        'Beef, topside, cap off, raw',
        100,
        22.3020833333333,
        0.0479166666666683,
        3.13333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5803,
        'Pork, tenderloin, trimmed, raw',
        100,
        21.2282552083333,
        0.0812735069989444,
        3.29122448979592,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5804,
        'Crispbread, knackebrot, wheat, fine',
        100,
        11.343,
        71.957,
        8.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5805,
        'Hare, raw',
        100,
        21.6,
        2.0375,
        1.8625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5806,
        'Rainbow trout, raw',
        100,
        18.71,
        0,
        7.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5807,
        'Coriander seed',
        100,
        12.36875,
        54.98125,
        17.77,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5808,
        'Heart, chicken, raw',
        100,
        15.55,
        0.72,
        9.33,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5809,
        'Pork, meat, lean, approx. 3 % fat, raw',
        100,
        21.8,
        0,
        3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5810,
        'Cod, roe, raw',
        100,
        23.2403846153846,
        1.05115384615385,
        2.57692307692308,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5811,
        'Liver, pig, raw',
        100,
        21.0545454545455,
        1.920935828877,
        3.39088235294118,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5812,
        'Crab claws, meat, raw',
        100,
        22.9,
        1,
        1.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5813,
        'Crab claws, meat, canned',
        100,
        22.9,
        1,
        1.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5814,
        'Turkey, flesh and skin, raw',
        100,
        20.2,
        0,
        5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5815,
        'Bread, white, coarse grain',
        100,
        8.3391,
        51.2789,
        3.256,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5816,
        'Marzipan',
        100,
        11.00232,
        50.39768,
        26.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5817,
        'Wheat bread, white, with cracked chestnuts',
        100,
        7.95593333333333,
        49.7850666666667,
        4.104,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5818,
        'Elderberry, juice, concentrated, sugar added',
        100,
        0.5,
        53.2,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5819,
        'Fennel seed',
        100,
        15.8,
        52.3,
        14.87,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5820,
        'Marmalade, English type',
        100,
        0.1,
        69.4,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5821,
        'Chicken, flesh only, raw',
        100,
        19.2986274509804,
        0,
        5.63470588235294,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5822,
        'Lamb, leg, trimmed, raw',
        100,
        19.6,
        0,
        5.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5823, 'Whale meat, raw', 100, 22.3, 0, 2.4, NULL);
INSERT INTO temp."Raw"
VALUES (
        5824,
        'Veal, thin fat, raw',
        100,
        18.6,
        0,
        5.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5825,
        'Cauliflower, Danish, raw',
        100,
        1.703125,
        3.915625,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5826, 'Crab, boiled', 100, 20.1, 0.4, 5.2, NULL);
INSERT INTO temp."Raw"
VALUES (
        5827,
        'Potato, boiled',
        100,
        1.7125,
        20.0075,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5828,
        'Ptarmigan, flesh, raw',
        100,
        23.5,
        0.1,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5829,
        'Marmalade, apricot',
        100,
        0.4,
        55,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5830,
        'Chocolate, milk, with nuts',
        100,
        8.8,
        50.65,
        37.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5831,
        'Beef, rumpsteak, cap off, raw',
        100,
        21.305,
        0,
        4.21666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5832,
        'Pine nuts, dried',
        100,
        13.992,
        19.383,
        61.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5833,
        'Rabbit, flesh only, raw',
        100,
        20.4,
        0,
        4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5834,
        'Pepper, hot chili, raw',
        100,
        2,
        9.49996,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5835,
        'Seal meat, loin, raw',
        100,
        22.6,
        0,
        2.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5836,
        'Baguette, with rye, industrially produced',
        100,
        8.11855384615385,
        52.6260615384615,
        2.63846153846154,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5837,
        'Cashew nuts, dry roasted',
        100,
        15.3117,
        32.6883,
        46.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5838,
        'Rolls, italien type, industrially produced',
        100,
        8.8692,
        52.3538,
        1.87,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5839,
        'Beef, meat, lean (<5 % fat), raw',
        100,
        21.5,
        0,
        4.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5840,
        'Beef, thick flank, cap off, raw',
        100,
        20.8183333333333,
        0,
        7.11666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5841,
        'Caviar, made from capelin roe',
        100,
        9.1,
        5.7,
        14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5842,
        'Breakfast cereal, Guldkorn',
        100,
        6.7045,
        88.6455,
        1.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5843,
        'Leek, raw',
        100,
        1.95120192307692,
        7.83020585833086,
        0.228571428571429,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5844,
        'Tuna, in water, canned',
        100,
        23.9408333333333,
        0,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5845,
        'Cream pastry, cream puff',
        100,
        4.06625,
        25.81575,
        22.94,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5846,
        'Roastbeef, sliced',
        100,
        21.7,
        0.3,
        3.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5847,
        'Cream pastry, average values',
        100,
        2.7,
        29.6,
        20.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5848,
        'Beef, tenderloin, defatted, raw',
        100,
        20.4583333333333,
        0.0266666666666647,
        6.23333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5849,
        'Mutton, round, raw',
        100,
        20.1,
        0,
        6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5850,
        'Dried fig spread',
        100,
        2.525,
        70.185,
        4.479,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5851,
        'Coriander, leaf, dried',
        100,
        21.93125,
        51.90875,
        4.78,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5852,
        'Wheat bread, with whole kernels',
        100,
        8.96558181818182,
        50.1587038961039,
        4.18571428571429,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5853,
        'Rye bread, fine with wheat flour and high fat seeds, industrially produced',
        100,
        7.53236,
        45.08764,
        4.55,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5854,
        'Rolls, white, industrially produced',
        100,
        8.628375,
        54.172875,
        2.825,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5855,
        'Potato crisps',
        100,
        5.4,
        57.68,
        31.8666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5856,
        'Marmalade, orange',
        100,
        0.5,
        49.7,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5857,
        'Pork, hind leg without rind, raw',
        100,
        18.94,
        0,
        7.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5858,
        'Lake trout, raw',
        100,
        18.65,
        0,
        7.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5859,
        'Spinach, chopped, frozen',
        100,
        2.1,
        4.23571428571428,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5860,
        'Pheasant, flesh only, raw',
        100,
        22.24,
        0,
        3.64,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5861,
        'Yam, raw',
        100,
        1.53125,
        27.87875,
        0.17,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5862,
        'Potato, new (May to September), raw',
        100,
        1.8,
        17.1,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5863,
        'Hamburger sandwich with salad and dressing, fast food',
        100,
        9.27948113207547,
        21.6139150943396,
        11.8358490566038,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5864,
        'Egg, duck, whole, raw',
        100,
        12.8125,
        1.4475,
        13.77,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5865, 'Tongue, pig, raw', 100, 17, 6, 10, NULL);
INSERT INTO temp."Raw"
VALUES (5866, 'Marmalade, kiwi', 100, 0.4, 56, 0.2, NULL);
INSERT INTO temp."Raw"
VALUES (
        5867,
        'Tongue, pig, cured',
        100,
        17,
        5.9,
        10,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5868,
        'Vanilla-icecream, average values',
        100,
        3.93125,
        34.87775,
        13.087,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5869,
        'Dried fruits spread, average values',
        100,
        2.725,
        69.0087777777778,
        5.557,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5870,
        'Squab (pigeon), flesh, raw',
        100,
        18.5,
        0,
        7.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5871,
        'Bread, white',
        100,
        8.7742,
        52.0548404040404,
        3.88818181818182,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5872,
        'Currant, black, juice, concentrated, sugar added',
        100,
        0.5,
        48,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5873,
        'Wheat bread, white, with walnuts',
        100,
        8.941875,
        51.455625,
        5.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5874,
        'Nut, pea, oilroasted and salted',
        100,
        24.5614,
        17.2086,
        53.3333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5875, 'Sprat, raw', 100, 10.3, 0, 10.5, NULL);
INSERT INTO temp."Raw"
VALUES (
        5876,
        'Coriander, leaf, raw',
        100,
        2.13125,
        3.66875,
        0.52,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5877,
        'Bamboo shoots, canned, salted',
        100,
        2.6,
        3.7,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5878,
        'Bamboo shoots, canned, nonsalted',
        100,
        2.6,
        3.7,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5879,
        'Beef, shin + leg (shank), raw',
        100,
        21.4,
        0,
        6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5880,
        'Beef, striploin "cap off", raw',
        100,
        20.8525,
        3.1725,
        9.42,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5881,
        'Dock (US), sorrel (UK), raw',
        100,
        2,
        3.2,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5882,
        'Cheese, Quarg, 45 % fidm.',
        100,
        11.1969,
        2.8431,
        15.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5883,
        'Egg, goose, whole, raw',
        100,
        13.86875,
        1.35125,
        13.27,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5884, 'Sugar, brown', 100, 0, 95.5, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5885,
        'Coffee, instant, decaffeinated, powder',
        100,
        11.6017,
        75.9983,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5886,
        'Asparagus, white, raw',
        100,
        2.4,
        4.3,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5887,
        'Beef, outside, round, raw',
        100,
        20.5,
        0,
        7.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5888,
        'Raisin , seedless',
        100,
        3.23409090909091,
        78.3295454545455,
        1.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5889,
        'Currant, black, jam',
        100,
        0.6,
        49,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5890,
        'Chicken, leg, flesh and skin, raw',
        100,
        18.4,
        0,
        8.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5891,
        'Marmalade, black currant',
        100,
        0.5,
        49.6,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5892,
        'Chicken, drumstick, flesh and skin, raw',
        100,
        18.7,
        0,
        8.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5893,
        'Baguette, wheat with rye, high fat seeds, industrially produced',
        100,
        8.5405,
        53.8411666666667,
        3.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5894,
        'Kale, frozen',
        100,
        2.09375,
        4.3675,
        0.5875,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5895,
        'Chervil, chopped, frozen',
        100,
        2.2,
        3.38,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5896,
        'Beef, eye of round, raw',
        100,
        21.5,
        0,
        6.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5897,
        'Cooking banana, Plaintain, raw',
        100,
        1.3,
        31.88,
        0.37,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5898,
        'Chewing gum, sugar added, all types',
        100,
        0.2,
        90.6,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5899,
        'Cherimoya, raw',
        100,
        1.65,
        17.7,
        0.62,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5900,
        'Halibut, greenland, gravad',
        100,
        12.3316666666667,
        0,
        14.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5901,
        'Halibut, greenland, raw',
        100,
        12.3316666666667,
        0,
        14.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5902,
        'Crispbread, wheat',
        100,
        9.0972,
        83.8028,
        2.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5903,
        'Wheat bread, sandwich/toast type, high fat seeds, industrially produced',
        100,
        8.1225,
        48.1841666666667,
        5.61666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5904,
        'Pork, sausage, frankfurter, canned',
        100,
        10.6,
        1.6,
        17,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5905,
        'Juice, mixed vegetables, pasteurized',
        100,
        2.1125,
        10.938464,
        0.43,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5906,
        'Dogfish, piked, raw',
        100,
        19.1,
        1,
        8.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5907, 'Sweets, boiled', 100, 0, 97.49, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5908, 'Tuna, raw', 100, 26.8, 0, 1.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        5909,
        'Turkey, mince, 0-5% fat, raw',
        100,
        22.0583333333333,
        0,
        4.75,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5910,
        'Liqueur, coffee with cream',
        100,
        2.8,
        20.8,
        15.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5911,
        'Chicken, breast, flesh and skin, raw',
        100,
        21.5,
        0.1,
        6.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5912, 'Wiener nougat', 100, 8.7, 51.8, 36, NULL);
INSERT INTO temp."Raw"
VALUES (
        5913,
        'Beef, meat, 5-10 % fat, raw',
        100,
        20.4,
        0,
        7.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5914,
        'Tongue, ox, raw',
        100,
        16.2,
        2.6,
        12.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5915,
        'Tongue, calf, cured',
        100,
        13.1,
        0,
        11.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5916,
        'Broccoli, boiled',
        100,
        2.38125,
        7.18875,
        0.41,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5917,
        'Marmalade, all types',
        100,
        0.5,
        49.5,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5918,
        'Marmalade, cherry',
        100,
        0.5,
        49.5,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5919,
        'Herring, breaded, fried, in vinegar with sugar',
        100,
        12.8352272727273,
        23.6702272727273,
        14.0363636363636,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5920,
        'Ice cream, dairy, popsickle, vanilla with chocolate',
        100,
        3,
        23,
        19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5921,
        'Salmon, Atlantic, wild, canned',
        100,
        20.3,
        18.21,
        8.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5922,
        'Tomato ketchup, hot, bottled',
        100,
        2,
        24,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5923, 'Horse, meat, raw', 100, 19, 0, 10, NULL);
INSERT INTO temp."Raw"
VALUES (
        5924,
        'Sardine, in tomato sauce, canned',
        100,
        16.35,
        3.6175,
        8.0325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5925,
        'Chicken, sausage',
        100,
        12.8,
        2.07,
        15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5926,
        'Mushroom, canned',
        100,
        2.5,
        3.1,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5927,
        'Wheat bread, white, with sunflower kernels',
        100,
        9.02246666666667,
        49.3723196581197,
        5.06076923076923,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5928, 'Chives, raw', 100, 2.4, 3.85, 0.6, NULL);
INSERT INTO temp."Raw"
VALUES (
        5929,
        'Kumquats, raw',
        100,
        1.0734375,
        13.9165625,
        1.54,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5930,
        'Tongue, calf, raw',
        100,
        12.58,
        0,
        12.23,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5931,
        'Ice, popsickle, lemonade',
        100,
        0.5,
        17,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5932,
        'Pork, ham, silverside, 2 mm fat, raw',
        100,
        18.9866666666667,
        0,
        9.56666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5933,
        'Pork, ham, spiced, deboned',
        100,
        12,
        4.6,
        15.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5934, 'Parsnip, raw', 100, 2.1, 14.7, 0.5, NULL);
INSERT INTO temp."Raw"
VALUES (
        5935,
        'Potato, automn (October to January), raw',
        100,
        2.2,
        19.5,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5936,
        'Cassava, raw',
        100,
        1.3625,
        38.05624,
        0.28,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5937,
        'Beef, topside "cap on", raw',
        100,
        21.1,
        0,
        8.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5938,
        'Remoulade, fatreduced, average values',
        100,
        0.8,
        3.64,
        23.46,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5939,
        'Beef, mince, 10-15% fat, raw',
        100,
        19.3803125,
        0,
        9.65625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5940,
        'Chewing gum, no sugar added, all types',
        100,
        0.6,
        86.04,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5941, 'Beef, neck, raw', 100, 20.8, 0, 9, NULL);
INSERT INTO temp."Raw"
VALUES (
        5942,
        'Pork, mince, 5-10% fat, raw',
        100,
        18.424,
        0,
        11.16,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5943,
        'Potato, old (February to June), raw',
        100,
        2.2,
        18.2,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5944,
        'Bacalao, cod, split, salted and dried, raw',
        100,
        24.3,
        0,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5945,
        'Pork, collar, defatted, raw',
        100,
        17.427692814766,
        0,
        12.4336486486486,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5946,
        'Jerusalem artichoke, raw',
        100,
        2.1,
        17.6666666666667,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5947,
        'Liqueur, coffee',
        100,
        0.1,
        46.8,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5948,
        'Goose, flesh only, raw',
        100,
        22.75,
        0.72,
        7.13,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5949,
        'Bread, white, roll',
        100,
        9.43249411764706,
        58.1075058823529,
        1.99454545454545,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5950, 'Kale, canned', 100, 2.2, 4.97, 0.9, NULL);
INSERT INTO temp."Raw"
VALUES (
        5951,
        'Spinach, canned',
        100,
        2.7,
        2.69,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5952,
        'Chicken, flesh and skin, raw',
        100,
        18.0305785123967,
        0,
        11.3305785123967,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5953,
        'Mackerel, in tomato sauce, canned',
        100,
        11.703125,
        3.85770833333333,
        10.3416666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5954,
        'Mushroom, raw',
        100,
        1.637025,
        4.224225,
        0.125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5955,
        'Swiss roll, apricot stuffing',
        100,
        3.6024,
        62.2976,
        7.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5956,
        'Beef, rumpsteak "cap on", raw',
        100,
        19.4,
        0,
        10.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5957,
        'Biscuit, sweet',
        100,
        6.9996,
        79.1314,
        10.62,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5958,
        'Cress, garden, raw',
        100,
        2.8,
        3.5,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5959,
        'Bean sprouts, mung, raw',
        100,
        3.0375,
        5.9425,
        0.18,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5960,
        'Liquorice, sweet',
        100,
        3.6,
        78.11,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5961,
        'Marmalade, raspberry',
        100,
        0.6,
        49.3,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5962,
        'Pork, ham, silverside with rind, raw',
        100,
        18.509,
        0,
        12,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5963,
        'Herring, pickled, canned',
        100,
        12,
        0,
        16,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5964,
        'Parsley root, raw',
        100,
        2.3,
        12.3,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5965,
        'Whey, sweet, dried',
        100,
        12.93226,
        74.45774,
        1.07,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5966,
        'Bean sprouts, average values, raw',
        100,
        2.99090909090909,
        6.28741258741259,
        0.263636363636364,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5967,
        'Margarine, 30 % fat, vegetable fat',
        100,
        0,
        0,
        30.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5968,
        'Beef, shoulder, ventral part',
        100,
        19.9,
        0,
        11.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5969,
        'Cod, roe, smoked',
        100,
        26.8,
        0.12,
        4.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5970,
        'Pork, sausage, Danish, raw',
        100,
        12.2406779661017,
        3.39871401668012,
        17.3860317460317,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5971,
        'Veal, medium fat, raw',
        100,
        20,
        0.6,
        11.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5972,
        'Pork, hand with rind, raw',
        100,
        18.209,
        0,
        12.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5973,
        'Blended spread, 80% fat',
        100,
        0.2233,
        0.67124545454545,
        80.7740909090909,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5974,
        'Marshmellows, unspecified',
        100,
        5.2,
        79.14,
        0.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5975,
        'Pork, meat, approx. 10 % fat, raw',
        100,
        18.74,
        0,
        12,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5976,
        'Pork, brawn, head cheese',
        100,
        13,
        0.2,
        18.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5977,
        'Chicken, wing, flesh and skin, raw',
        100,
        19.2,
        0.1,
        12.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5978,
        'Wheat bread, high fat seeds, industrially produced',
        100,
        9.5779,
        45.3204333333333,
        6.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5979,
        'Sprat (sardine), in tomato sauce, canned',
        100,
        15.1,
        0.5,
        16,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5980,
        'Lamb, shoulder, raw',
        100,
        18.4888888888889,
        0,
        17.4111111111111,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5981,
        'Pheasant, flesh and skin, raw',
        100,
        21.67,
        0,
        9.29,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5982,
        'Marmalade, strawberry',
        100,
        0.5,
        55.5,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5983,
        'Salmon, Atlantic, wild, cold smoked',
        100,
        21.4,
        0.1,
        8.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5984,
        'Pastilles, sugarfree, unspecified',
        100,
        1,
        90.06,
        1.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5985,
        'Halibut, greenland, smoked',
        100,
        14.345,
        0,
        16.055,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5986,
        'Pork, loin, defatted (approx. 3 mm fat), raw',
        100,
        19.189,
        0,
        13.344,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5987,
        'Cauliflower, imported, raw',
        100,
        3.1,
        5.2,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5988, 'Fructose', 100, 0, 99.75, 0, NULL);
INSERT INTO temp."Raw"
VALUES (5989, 'Salt, block', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        5990,
        'Toffees, mixed',
        100,
        2.1,
        71.4,
        20.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5991,
        'Mayonnaise',
        100,
        1.1,
        1.405,
        79.995,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5992,
        'Liver pâté, pork, Danish',
        100,
        11.2666666666667,
        5.32333333333333,
        19.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5993, 'Apple jelly', 100, 0.5, 59, 0.5, NULL);
INSERT INTO temp."Raw"
VALUES (
        5994,
        'Currant, black, jelly',
        100,
        0.5,
        59,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5995,
        'Bread, white, roll, coarse grain',
        100,
        9.57703636363637,
        55.2948683982684,
        4.04809523809524,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5996,
        'Butter, salt not added',
        100,
        0.18,
        0,
        82.46,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (5997, 'Goose fat', 100, 0, 0, 99.8, NULL);
INSERT INTO temp."Raw"
VALUES (
        5998,
        'Lamb, leg with fat, raw',
        100,
        18.4,
        0,
        14.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        5999,
        'Plant margarine, 80%, fry/bake',
        100,
        0,
        0,
        80.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6000,
        'Beef, chuck, raw',
        100,
        18.8333333333333,
        0,
        13.1166666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6001,
        'Margarine, 80 % fat, table use, soft, vegetable fat',
        100,
        0,
        0,
        79.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6002,
        'Pork, ham, smoked, boiled',
        100,
        18.3,
        0,
        14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6003,
        'Chocolate, bitter',
        100,
        4.33552,
        61.93648,
        31.768,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6004,
        'Soup, tomato, ready to eat',
        100,
        1.6375,
        13.1325,
        1.53,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6005,
        'Herring, pickled, spiced, canned',
        100,
        12.4,
        10.7,
        15.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6006,
        'Pork, minced and bound with flour and egg, boiled, frozen',
        100,
        15.8,
        3.9,
        15.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6007,
        'Herring, smoked',
        100,
        20.475,
        0,
        14.9166666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6008,
        'Pork, ham, silverside (M. Biceps Femoris), 5 mm rind, raw',
        100,
        19.2,
        5.3,
        13.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6009,
        'Salmon, Atlantic, wild, gravad',
        100,
        21.2,
        2.7,
        9.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6010,
        'Margarine, 80 % fat, table use, vegetable fat',
        100,
        0,
        0,
        80.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6011,
        'Anchovy, pickled, canned',
        100,
        13.4,
        11,
        13,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6012,
        'Sprat, pickled, canned',
        100,
        13.4,
        11,
        13,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6013,
        'Margarine, 80 % fat, for frying/baking, animal and vegetable fat',
        100,
        0,
        0,
        81.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6014,
        'Chicken, hen, flesh and skin, raw',
        100,
        17.1375,
        0.6125,
        15.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6015,
        'Margarine, 80 % fat, table use, animal and vegetable fat',
        100,
        0,
        0,
        82.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6016,
        'Beef, brisket, anterior part, raw',
        100,
        18.8683333333333,
        0,
        10.2166666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6017,
        'Poularde, flesh and skin, raw',
        100,
        19,
        3.71,
        11,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6018,
        'Beef, shoulder, dorsal part, raw',
        100,
        19.2,
        0,
        14.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6019,
        'Pork, ham sausage',
        100,
        15.1,
        2.7,
        16.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6020,
        'Margarine, (more than 50 % fat), average',
        100,
        0,
        0,
        82.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6021,
        'Cream pastry, with jam, marzipan-covered',
        100,
        3.0984375,
        37.8015625,
        22,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6022,
        'Salad dressing, Italian',
        100,
        0.38125,
        10.42875,
        28.37,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6023,
        'Rolls, wheat with coarse meal, high fat seeds, industrially produced',
        100,
        9.018,
        50.5925263157895,
        5.54736842105263,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6024,
        'Milk, skimmed, organic',
        100,
        3.50262,
        4.51338,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6025,
        'Milk, skimmed',
        100,
        3.50262,
        4.51338,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6026,
        'Bamboo shoots, raw',
        100,
        3.6,
        6.7,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6027,
        'Butter, salt added',
        100,
        0.6699,
        1.16309489795918,
        81.5077551020408,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6028,
        'Beef, meat, approx. 15 % fat, raw',
        100,
        19.7,
        8.91,
        14.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6029,
        'Herring, pickled, in madeira sauce, canned',
        100,
        14,
        8,
        16,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6030,
        'Passion fruit, raw',
        100,
        2.775,
        23.1425,
        1.79,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6031,
        'Pork sausage, bierwurst',
        100,
        14.6,
        3.9,
        17.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6032,
        'Jam, fruit, average values',
        100,
        0.6,
        66.7,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6033,
        'Pork, flank, raw',
        100,
        17.91,
        0,
        16.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6034,
        'Swiss roll, raspberry stuffing',
        100,
        3.6,
        58.2914285714286,
        14.87,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6035,
        'Pork, ham, bayonne style, deboned',
        100,
        20.8,
        0.4,
        12,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6036,
        'Sugar pea (Snow pea, Mangetout) raw',
        100,
        3.775,
        8.7,
        0.295,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6037,
        'Swiss roll, unspecified',
        100,
        3.53115,
        62.85485,
        9.7425,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6038,
        'Bread, roll, danish',
        100,
        9.651525,
        57.4982369047619,
        3.55357142857143,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6039,
        'Beef, top, end of rump, raw',
        100,
        16.96875,
        1.01958333333333,
        22.1333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6040,
        'Liquorice, average values',
        100,
        3.7,
        83.26,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6041,
        'Milk, skimmed, 0.5 % fat',
        100,
        3.50262,
        4.53438,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6042,
        'Beef, mince, 15-20% fat, raw',
        100,
        17.7919047619048,
        0,
        16.2285714285714,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6043,
        'Milk, buttermilk with lemon',
        100,
        3.29846,
        7.51154,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6044,
        'Beef, T-bone steak, raw',
        100,
        20.3,
        0,
        14.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6045,
        'Veal and pork, mince, 15-20% fat, raw',
        100,
        17.5915384615385,
        0,
        16.3538461538462,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6046,
        'Cream pastry, shortbread with chocolate glaze',
        100,
        3.60208333333333,
        39.6817628205128,
        22.0261538461539,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6047,
        'Italian mayonnaise salad',
        100,
        1.9,
        8.8,
        29.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6048,
        'Pork sausage, jagdwurst',
        100,
        13.1,
        1.2,
        20,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6049,
        'Coffee bean, roasted, ground',
        100,
        12.3808,
        62.7192,
        15.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6050,
        'Pork, mince, 15-20% fat, raw',
        100,
        17.1183333333333,
        0,
        17.2388888888889,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6051,
        'Pork, thin belly, lean, approx. 2 mm rind, raw',
        100,
        17.43,
        0,
        17.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6052,
        'Herring, pickled, in curry sauce, canned',
        100,
        7.6,
        11.7,
        22.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6053,
        'Cocoa, instant, med milk, powder',
        100,
        5.5,
        82.5,
        3.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6054,
        'Plaice, fillet, battered, deepfried',
        100,
        15.2,
        18.7,
        13.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6055,
        'Rice bran',
        100,
        13.3518,
        49.6882,
        20.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6056,
        'Beef, sirloin, raw',
        100,
        19.9479166666667,
        0.350416666666668,
        14.5333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6057,
        'Pastry, garland cake',
        100,
        9.1998,
        59.8002,
        19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6058,
        'Wheat germ, raw',
        100,
        28.1006,
        51.1994,
        9.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6059,
        'Haddock, breaded, fried',
        100,
        25,
        2,
        10,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6060,
        'Tuna, in oil, canned',
        100,
        24.9,
        2.12166666666666,
        10.4183333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6061,
        'Black salsify, raw',
        100,
        3.3,
        18.6,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6062,
        'Mutton, shoulder, raw',
        100,
        18.6875,
        0.0125,
        17,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6063,
        'Chicken, flesh and skin, grilled',
        100,
        24.2,
        0,
        10.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6064,
        'Chocolate, plain, couverture',
        100,
        5.71666666666667,
        58.06,
        34.2633333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6065,
        'Frankfurt sausage, boiled, fast food',
        100,
        12.6940789473684,
        6.75486842105263,
        20.3473684210526,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6066,
        'Cream pastry, puff pastry with jam',
        100,
        2.75520833333333,
        32.9287916666667,
        24.196,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6067,
        'Dandelion, leaves, raw',
        100,
        3,
        10,
        1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6068,
        'Cinnamon stick',
        100,
        4,
        81.16,
        1.24,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6069, 'Syrup, kitchen', 100, 0.3, 76.6, 0, NULL);
INSERT INTO temp."Raw"
VALUES (6070, 'Pate, liver', 100, 13, 2.46, 21.7, NULL);
INSERT INTO temp."Raw"
VALUES (
        6071,
        'Cantherelle, raw',
        100,
        1.385175,
        5.809825,
        0.6125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6072,
        'Pork sausage, saveloy type',
        100,
        11.1955555555556,
        2.13194444444444,
        24.3525,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6073,
        'Sapotes, raw',
        100,
        2.11875,
        33.75125,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6074,
        'Bacon crisps',
        100,
        54.13,
        0,
        40.3333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6075,
        'Pork sausage, malakoff',
        100,
        9.8,
        5.3,
        23.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6076,
        'Mackerel, filet, in natural juices, canned',
        100,
        19.0347619047619,
        0,
        16.4285714285714,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6077,
        'Sandwich, rib roast',
        100,
        12.3,
        26.1,
        14.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6078,
        'Milk, skimmed, with chocolate',
        100,
        3.53735555555556,
        9.97575255255255,
        0.491891891891892,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6079,
        'Milk, skimmed, with chocolate (UHT)',
        100,
        3.59832,
        10.19168,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6080,
        'Beef, striploin "cap on", raw',
        100,
        20.0983333333333,
        0,
        14.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6081,
        'Cheese, Camembert, 30 % fidm.',
        100,
        24.99684,
        0.00316,
        11,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6082,
        'Alfalfa seeds, sprouted, raw',
        100,
        3.9875,
        2.1025,
        0.69,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6083,
        'Pork, meat, approx. 20 % fat, raw',
        100,
        17.92,
        0,
        19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6084,
        'Liquorice, salt',
        100,
        4.4,
        82.05,
        1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6085,
        'Biscuit, 20-30% wholemeal, digestive type',
        100,
        6.5151,
        73.0759,
        16.49,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6086,
        'Pork, loin with rind, raw',
        100,
        18.2557692307692,
        0.128461538461526,
        21.6557692307692,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6087, 'Fruit gums', 100, 6.6, 78.6, 0.9, NULL);
INSERT INTO temp."Raw"
VALUES (
        6088,
        'Sprat (sardine), in oil, canned',
        100,
        24.6,
        1.06117647058824,
        11.3388235294118,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6089,
        'Sardine, in oil, canned',
        100,
        24.6,
        1.06117647058824,
        11.3388235294118,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6090,
        'Cream Crackers',
        100,
        9.39075,
        71.5605,
        13.86625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6091,
        'Pork, thin belly with rind, raw',
        100,
        16.2664102564103,
        0,
        25.4461538461538,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6092,
        'Human milk, colostrum',
        100,
        2.00018,
        6.59982,
        2.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6093,
        'Breakfast cereal, Havrefras',
        100,
        13.20495,
        71.4617166666667,
        6.64333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6094,
        'Pork/beef sausage, saveloy type',
        100,
        11.1365853658537,
        4.78719486412622,
        23.7397674418605,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6095, 'Mint jelly', 100, 0.5, 65.5, 0.5, NULL);
INSERT INTO temp."Raw"
VALUES (
        6096,
        'Herring, autumn, raw',
        100,
        19.3,
        0,
        19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6097,
        'Baby food, infant formula, low iron content, prepared',
        100,
        1.2,
        7.5,
        3.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6098,
        'Vecao',
        100,
        3.26112,
        60.97888,
        33.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6099,
        'Pork, mettwurst, smoked',
        100,
        11.5549019607843,
        3.60509803921569,
        25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6100, 'Bacon, sliced, raw', 100, 0, 0, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        6101,
        'Sandwich with ham and cheese, toasted, fast food',
        100,
        11.0022727272727,
        34.4836363636364,
        13.2454545454545,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6102,
        'Alfalfa seeds',
        100,
        35.1,
        41.8,
        12.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6103,
        'Salad dressing, mayonnaise',
        100,
        1.16875,
        12.63125,
        32.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6104,
        'Rye-flour, bolted (extraction rate 70 %)',
        100,
        6.099,
        79.291,
        1.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6105,
        'Beef, entrecote "cap on", raw',
        100,
        18.2,
        0,
        21.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6106,
        'Kippers, in oil, canned',
        100,
        20.5,
        0,
        18.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6107,
        'Herring, salted, ripened',
        100,
        13.1,
        0,
        23.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6108,
        'Milk, 1.5 % fat, cultured, kefir',
        100,
        3.50944859375,
        4.8307768729837,
        1.59348416289593,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6109,
        'Milk, partly skimmed, organic, 1.5 % fat',
        100,
        3.375,
        5.075,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6110,
        'Salad dressing, oil and vinegar',
        100,
        0.5625,
        9.8375,
        34.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6111,
        'Nutmeg, whole',
        100,
        6,
        49.35,
        36.31,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6112,
        'Pork, flank, spiced, cooked',
        100,
        15.3,
        0.74818181818182,
        23.1918181818182,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6113,
        'Brussels sprouts, winter (January to April), raw',
        100,
        4.4,
        9.2,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6114, 'Chervil, raw', 100, 2.2, 2.7, 0.6, NULL);
INSERT INTO temp."Raw"
VALUES (
        6115,
        'Pork sausage, Thüringer bratwurst',
        100,
        14.4,
        1.3,
        24.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6116,
        'Peanut butter',
        100,
        22.59757,
        19.60243,
        53.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6117,
        'Pork sausage, polony type',
        100,
        15.2,
        0,
        24.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6118,
        'Sweetcorn, kernels, canned',
        100,
        3.1,
        21,
        1.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6119,
        'Apple, dried',
        100,
        0.93125,
        65.88875,
        0.32,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6120,
        'Human milk, transitional (10th day post partum)',
        100,
        1.4993,
        6.9007,
        3.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6121,
        'Yogurt, with fruit, 1.5 % fat',
        100,
        3.6047,
        11.9553,
        1.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6122, 'Veal, fat, raw', 100, 17.3, 0, 22.8, NULL);
INSERT INTO temp."Raw"
VALUES (
        6123,
        'Pork, neck, smoked, boiled',
        100,
        19.5,
        0.1,
        20.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6124,
        'Biscuit, sweet',
        100,
        5.301,
        62.9802195121951,
        26.1187804878049,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6125,
        'Cream, whipping, 38 % fat',
        100,
        2.12281693430657,
        2.31995073513888,
        38.0166516853933,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6126, 'Honey', 100, 0.3, 81.5, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        6127,
        'Beverages, soy, commercially prepared, unfortified',
        100,
        3.12826428571429,
        4.35887857142857,
        2.24285714285714,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6128,
        'Human milk, mature',
        100,
        1.30152,
        7.19848,
        4.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6129,
        'Cream, cultured, 38 % fat',
        100,
        2.11648105263158,
        1.87185228070176,
        38.3666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6130,
        'Salmon, Atlantic, wild, salted',
        100,
        8.5,
        0,
        18.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6131,
        'Black pudding, boiled',
        100,
        10.7,
        23.05,
        20.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6132,
        'Eggs, chicken, yolk, salted, pasteurized',
        100,
        14,
        1.6,
        23,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6133,
        'Charr, fillet, smoked',
        100,
        27.7,
        0,
        10.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6134,
        'Milk, with chocolate, partly skimmed, 1.5 % fat (UHT)',
        100,
        3.59832,
        9.99168,
        1.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6135, 'Gelatin', 100, 86.9, 0, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        6136,
        'Margarine, (less than 50 % fat), average',
        100,
        0,
        0,
        41.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6137,
        'Frankfurt sausage, fried, fast food',
        100,
        14.046875,
        2.383125,
        25.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6138,
        'Pecans, oil roasted',
        100,
        9.2008,
        13.0192,
        75.23,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6139,
        'Pecans, dry roasted',
        100,
        9.4976,
        13.5524,
        74.27,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6140,
        'Margarine, 40 % fat, vegetable fat',
        100,
        0,
        0,
        41.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6141,
        'Brussels sprouts, automn (October to December), raw',
        100,
        4.8,
        10.4,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6142,
        'Buckwheat flour',
        100,
        11.98065,
        70.01935,
        2.75,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6143,
        'Yeast, dried',
        100,
        32.4672,
        52.7328,
        1.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6144,
        'Squab (pigeon), raw',
        100,
        18.2,
        0,
        23.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6145,
        'Pork, thick belly, defatted, approx. 2 mm rind, raw',
        100,
        15.47,
        0,
        26.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6146,
        'Cheese, processed, 20 % fidm.',
        100,
        16.0776,
        11.8681894736842,
        12.3736842105263,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6147,
        'Cheese, firm, Danbo, 20 % fidm.',
        100,
        27.152,
        0,
        12.398,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6148,
        'Seaweed, kombu, dried',
        100,
        7.9,
        54.8,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6149,
        'Milk, with chocolate, partly skimmed, 1.5 % fat',
        100,
        3.67488,
        9.75795783783784,
        1.76540540540541,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6150,
        'Cocoa, instant, without milk, powder',
        100,
        4.01952,
        88.48048,
        3.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6151,
        'Yeast extract, Marmite',
        100,
        27.8,
        10.6,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6152,
        'Curry mayonnaise salad',
        100,
        1.7,
        9,
        37.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6153,
        'French fries, deepfried, fast food',
        100,
        3.6984375,
        42.2140625,
        14.81,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6154,
        'Egg, hen, yolk, sugar added, pasteurized, frozen',
        100,
        14.2,
        11.5,
        23.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6155,
        'Cheese, firm, 20 % fidm., all varieties',
        100,
        29.8584,
        0.761599999999999,
        11.0012765957447,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6156,
        'Cheese, Cream, 60 % fidm.',
        100,
        10.20162,
        1.95838,
        31.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6157,
        'Tomato paste, concentrated',
        100,
        4.31875,
        18.91125,
        0.47,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6158,
        'Beans, soy, dried, raw',
        100,
        35.8,
        34.1,
        18.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6159,
        'Meat extract, cube',
        100,
        15.96875,
        23.65125,
        8.89,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6160,
        'Mackerel, smoked',
        100,
        18.0183333333333,
        0,
        25.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6161,
        'Beef brisket, soaked in brine',
        100,
        19,
        0,
        22.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6162,
        'Salad dressing, thousand island',
        100,
        1.0875,
        14.6525,
        35.06,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6163,
        'Biscuit, sweet',
        100,
        5.44033333333333,
        72.1463333333333,
        17.7888888888889,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6164,
        'Rice groats, raw',
        100,
        6.4974,
        80.8026,
        1.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6165,
        'Pork, ham, smoked',
        100,
        28.4,
        0,
        13,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6166,
        'Cheese, processed, 30 % fidm.',
        100,
        26.40044,
        0.49956,
        16.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6167,
        'Cheese, Brie, 45 % fidm.',
        100,
        20.75733,
        0.82767,
        22.005,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6168,
        'Chicken, flesh and skin, battered, deepfried',
        100,
        22.5375,
        9.4225,
        17.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6169,
        'Egg, chicken, yolk, raw',
        100,
        15.62109375,
        1.44028125,
        28.383,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6170,
        'Beef, meat, approx. 25 % fat, raw',
        100,
        16.4,
        0.16,
        28.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6171,
        'Beef, brisket, posterior part, raw',
        100,
        7,
        0,
        27.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6172,
        'Lamb, meat, average values, raw',
        100,
        13.9,
        0,
        30.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6173,
        'Cheese, Camembert, 45 % fidm.',
        100,
        21.39852,
        0.15148,
        22.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6174,
        'Chicken, flesh and skin, deepfried',
        100,
        28.3,
        0.11409090909091,
        16.2859090909091,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6175,
        'Beef, brisket, middle part, raw',
        100,
        15.9,
        0.042857142857142,
        18.3571428571429,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6176,
        'Bucklingpaté (sweet cured herring, smoked)',
        100,
        11.1,
        0.85,
        33.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6177,
        'Buckwheat groats, raw',
        100,
        7.00183,
        79.79817,
        1.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6178,
        'Cheese, semihard, Feta, 50 % fidm',
        100,
        17.8002,
        1.4998,
        25.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6179,
        'Cheese, firm, skimmed milk, max. 5 % fidm.',
        100,
        43.0969,
        1.0031,
        0.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6180, 'Tomato puree', 100, 1.6, 9, 0.2, NULL);
INSERT INTO temp."Raw"
VALUES (6181, 'Eel, raw', 100, 14.903, 0, 27.7, NULL);
INSERT INTO temp."Raw"
VALUES (
        6182,
        'Pork, thick belly with rind, raw',
        100,
        14.99525,
        0,
        33.065,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6183,
        'Remoulade, average values',
        100,
        1.1,
        11.1875,
        36.8425,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6184,
        'Beef, flatribs (short ribs), raw',
        100,
        15.3,
        0,
        29.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6185,
        'Rice, polished, raw',
        100,
        8.4014,
        77.7986,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6186,
        'Bouillon, beef, concentrated, cube',
        100,
        17,
        15.8,
        3.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6187,
        'Rice flour',
        100,
        7.30065,
        80.09935,
        1.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6188,
        'Cocoa, powder',
        100,
        17.064,
        46.566,
        21.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6189,
        'Olives, black, salted, in oil',
        100,
        2.2,
        8.7,
        35.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6190,
        'Rye flour, bolted (extraction rate 85 %)',
        100,
        7.7976,
        77.4924,
        1.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6191,
        'Cheese, firm, Danbo, 30 % fidm.',
        100,
        27.7866666666667,
        0,
        15.7333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6192,
        'Beef, flank, raw',
        100,
        16.1,
        0,
        30.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6193,
        'Tomato ketchup, bottled',
        100,
        3,
        21.7,
        2.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6194,
        'Milk, whole, 3.5, (UHT), % fat',
        100,
        3.421768,
        4.41471029115393,
        3.47322033898305,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6195,
        'Milk, whole, organic, 3.5 % fat',
        100,
        3.4133,
        4.7387,
        3.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6196,
        'Lamb, loin, raw',
        100,
        15.8,
        10.7090909090909,
        20.3909090909091,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6197,
        'Pork, meat, approx. 32 % fat, raw',
        100,
        14.4,
        0,
        32,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6198,
        'Yoghurt with strawberries',
        100,
        3.29846,
        10.14154,
        3.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6199,
        'Rice, parboiled, raw',
        100,
        7.80045,
        79.3551055555556,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6200, 'Potato flour', 100, 0.2, 85.3, 0.1, NULL);
INSERT INTO temp."Raw"
VALUES (
        6201,
        'Yoghurt with mixed berries',
        100,
        3.29846,
        10.63154,
        3.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6202,
        'Cheese, firm, 30 % fidm., all varieties',
        100,
        28.69724,
        1.00276,
        16,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6203,
        'Baby food, infant formula, high iron content, powder',
        100,
        11.30217,
        56.69783,
        25.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6204,
        'Peach, dried',
        100,
        4.8875,
        83.1825,
        1.03,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6205,
        'Cheese, Camembert, 50 % fidm.',
        100,
        17.5,
        0,
        26.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6206,
        'Milk, whole, cultured',
        100,
        3.50262,
        3.55738,
        3.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6207, 'Junket', 100, 3.50262, 3.55738, 3.6, NULL);
INSERT INTO temp."Raw"
VALUES (
        6208,
        'Milk, whole, cultured (HISTORIC)',
        100,
        3.50262,
        3.55738,
        3.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6209,
        'Milk, cultured, kefir (HISTORIC)',
        100,
        3.50262,
        3.55738,
        3.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6210,
        'Yoghurt with peach melba',
        100,
        3.29915405857741,
        10.7628402190232,
        3.1978102189781,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6211,
        'Yoghurt, whole milk, with fruit',
        100,
        3.29915405857741,
        10.9928402190232,
        3.1978102189781,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6212,
        'Cheese, firm, Maribo, 30 % fidm.',
        100,
        29.09918,
        1.00082,
        16.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6213,
        'Yoghurt with pear and banana',
        100,
        3.29846,
        10.97154,
        3.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6214,
        'Yoghurt with cherries',
        100,
        3.40054,
        11.26946,
        3.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6215,
        'Milk, acidophilus cultured from whole milk, 3.5% fat',
        100,
        4.8807,
        3.4743,
        3.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6216,
        'Poppy seeds',
        100,
        18.0375,
        23.7025,
        44.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6217,
        'Barley flour, raw',
        100,
        9.19974,
        72.30026,
        3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6218,
        'Pastilles, sugar added, unspecified',
        100,
        4.4,
        84.86,
        1.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6219,
        'Pasta, boiled',
        100,
        4.9989,
        27.0011,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6220, 'Corn flour', 100, 6.8, 78.7, 2.8, NULL);
INSERT INTO temp."Raw"
VALUES (
        6221,
        'Cheese, semihard, Mozzarella, 30 % fidm.',
        100,
        28.9014,
        0.9986,
        17.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6222,
        'Wheat flour',
        100,
        9.699975,
        75.6316916666667,
        1.54583333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6223, 'Eel, smoked', 100, 19.4, 0.2, 27.8, NULL);
INSERT INTO temp."Raw"
VALUES (
        6224,
        'Cheese, Cream, 70 % fidm.',
        100,
        9.6019,
        1.8881,
        36.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6225,
        'Yoghurt with müsli',
        100,
        3.50262,
        10.29738,
        3.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6226,
        'Milk, acidophilus cultured, with lemon (HISTORIC)',
        100,
        4.8807,
        5.3793,
        3.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6227,
        'Wheat flour, breadmaking (extraction rate 75%)',
        100,
        9.9978,
        74.9922,
        1.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6228,
        'Salad dressing, soured fresh cream',
        100,
        1.55,
        9.65,
        41.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6229,
        'Semolina',
        100,
        9.1998,
        77.7002,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6230, 'Baking powder', 100, 5.2, 37.8, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        6231,
        'Cheese, Brie, 50 % fidm.',
        100,
        19.79714,
        0.40286,
        27.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6232,
        'Peas, green, canned',
        100,
        5.9,
        14.3,
        0.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6233,
        'Lamb, flank, raw',
        100,
        14.5,
        0.1,
        34.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6234,
        'Lamb, breast, raw',
        100,
        14.5,
        0.1,
        34.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6235,
        'Beansprouts, soy, raw',
        100,
        6.2,
        5.3,
        1.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6236,
        'Cheese, firm, Samsoe, 30 % fidm.',
        100,
        30.10084,
        0.99916,
        17.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6237,
        'Goat milk',
        100,
        3.56004,
        4.44996,
        4.14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6238, 'Thistle oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6239, 'Oil, corn germ', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (
        6240,
        'Rolls, sweet dough type',
        100,
        9.0948,
        51.7335333333333,
        6.56666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6241, 'Wheat germ oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6242, 'Oil, cotton seed', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6243, 'Sorbitol', 100, 0, 100, 0, NULL);
INSERT INTO temp."Raw"
VALUES (6244, 'Oil, palm', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6245, 'Oil, cocoa butter', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (
        6246,
        'Mustard powder',
        100,
        28.9,
        18.8,
        28.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6247, 'Palm kernel oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6248, 'Peanut oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6249, 'Sunflower oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6250, 'Walnut oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (
        6251,
        'Beet, red, danish, raw',
        100,
        1.109375,
        10.184375,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6252,
        'Kale, Danish, raw',
        100,
        3.4453125,
        8.2721875,
        0.8125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6253,
        'Lettuce, iceberg (incl. crisphead types), danish, raw',
        100,
        0.875,
        2.581,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6254, 'Cod liver oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (
        6255,
        'Dumplings, frozen',
        100,
        7,
        14.5,
        14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6256, 'Oil, sesame', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (
        6257,
        'Lettuce, iceberg (incl. crisphead types), imported, raw',
        100,
        0.833333333333333,
        3.2,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6258,
        'Sugar, brown cane sugar',
        100,
        0.5,
        99.3,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6259, 'Corn oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6260, 'Linseed oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (
        6261,
        'Rye bread, soft, no high fat seeds, industrially produced',
        100,
        4.99048,
        47.91752,
        1.44,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6262,
        'Wheat bread, italien type, retail bakery',
        100,
        7.9743,
        53.0157,
        1.295,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6263, 'Oil, grapeseed', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (6264, 'Oil, coconut', 100, 0, 0, 99.9, NULL);
INSERT INTO temp."Raw"
VALUES (
        6265,
        'Saithe (UK), pollock (US), liver oil',
        100,
        0,
        0.1,
        99.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6266, 'Fat, beef tallow', 100, 5, 0, 95, NULL);
INSERT INTO temp."Raw"
VALUES (
        6267,
        'Soyabean oil, refined',
        100,
        0,
        0,
        100,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6268,
        'Fish fillet, breaded, deepfried, fast food',
        100,
        13.3541666666667,
        22.8625,
        16.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6269, 'Olive oil', 100, 0, 0, 100, NULL);
INSERT INTO temp."Raw"
VALUES (
        6270,
        'Oil, rape seed (no eruca acid)',
        100,
        0,
        0,
        100,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6271,
        'Celeriac, celery root, danish, raw',
        100,
        1.38392857142857,
        7.37178571428571,
        0.157142857142857,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6272,
        'Arugula, rocket, raw',
        100,
        3.265625,
        2.473125,
        0.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6273,
        'Pumpkin seed, dried',
        100,
        30.7665,
        12.7085,
        45.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6274,
        'Durum wheat flour',
        100,
        12.44705,
        73.95795,
        1.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6275,
        'Wheat bread, white, industrially produced',
        100,
        8.42810769230769,
        50.6057384615385,
        3.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6276,
        'Bread, white, large, retail bakery',
        100,
        8.19616153846154,
        53.1749923076923,
        3.82692307692308,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6277,
        'Wheat bread, with coarse meal, industrially produced',
        100,
        8.92746666666667,
        48.4714222222222,
        4.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6278,
        'Wheat bread, large, with coarse meal, retail bakery',
        100,
        8.35651666666667,
        51.1995944444444,
        2.97777777777778,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6279,
        'Tiger prawn, aquaculture, raw, frozen',
        100,
        15.6975,
        0,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6280,
        'Prawn, giant tiger, aquaculture, boiled, frozen',
        100,
        14.82,
        0,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6281,
        'Salmon, Atlantic, aquaculture, cold smoked',
        100,
        20.8557692307692,
        1.58807692307692,
        8.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6282,
        'Salmon, atlantic, aquaculture, hot smoked',
        100,
        20.1123076923077,
        0,
        20.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6283,
        'White Wheat, whole grain flour, ready-to-bake',
        100,
        11.8932,
        68.4968,
        4.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6284,
        'Butter cookie, wheat, Danish, industrially produced',
        100,
        4.8906,
        65.0584,
        27.11,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6285,
        'American inspired cookie',
        100,
        5.4207,
        64.2103,
        26.53,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6286,
        'Cookies, oat flakes, industrially produced',
        100,
        4.54765,
        63.1965166666667,
        26.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6287,
        'Butter cookie, brown, (Danish), industrially produced',
        100,
        5.24115,
        73.1026,
        17.775,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6288,
        'Biscuit, sweet, pepper nuts, Danish tradition, industrially produced',
        100,
        5.30914285714286,
        71.588,
        19.1857142857143,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6289,
        'Cookies, Klenät, industrially produced',
        100,
        5.377,
        45.6844285714286,
        36.4285714285714,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6290,
        'Cookies, low sugar content, industrially produced',
        100,
        6.50614285714286,
        65.7195714285714,
        23.7285714285714,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6291,
        'Cookies, coconut, industrially produced',
        100,
        4.06125,
        64.33875,
        22,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6292,
        'Cookie, truffle product / unbaked, industrially produced',
        100,
        3.69296666666667,
        60.3214777777778,
        21.79,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6293,
        'Wreath cake, almond based, Danish tradition',
        100,
        6.02571428571429,
        62.9742857142857,
        19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6294,
        'Sweetcorn, baby, on-the-cob, raw',
        100,
        2.340625,
        6.829375,
        0.49,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6295,
        'Pomelo (pommelo, shaddock), raw',
        100,
        1.034375,
        13.4939583333333,
        0.67,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6296,
        'Biscuit, rye',
        100,
        6.0648,
        74.9632,
        14.88,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6297,
        'Biscuit, wheat, sweet with thick chocolate',
        100,
        6.448125,
        65.180625,
        24.8875,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6298,
        'Biscuit, double, wheat, sweet',
        100,
        5.6886,
        71.3844,
        19.71,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6299,
        'Biscuit, oat, sweet',
        100,
        6.897,
        68.429,
        19.64,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6300,
        'Biscuit, oat, sweet with chokolate',
        100,
        6.5436,
        66.3524,
        22.96,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6301,
        'Granola bar',
        100,
        6.1047,
        70.8223,
        14.96,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6302,
        'Wafer sticks',
        100,
        4.1781,
        66.1609909090909,
        25.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6303,
        'Biscuit, coarse grain with fruit and vegetables',
        100,
        7.7919,
        72.8391,
        13.69,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6304,
        'Milk, skimmed, 0.1 % fat',
        100,
        3.56571111111111,
        4.96095555555556,
        0.0944444444444445,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6305,
        'Yogurt plain, low fat, 1.5 % fat',
        100,
        4.0832,
        4.8168,
        1.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6306,
        'Yogurt low fat, 0.1 % fat',
        100,
        4.2746,
        4.9754,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6307,
        'Bread, wheat, white wheat, wholw grain, retail bakery',
        100,
        8.892,
        50.238,
        3.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6308,
        'Rolls, White Wheat, whole grain, industrially produced',
        100,
        8.208,
        48.902,
        1.97,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6309,
        'Bread, wheat, white wheat, wholw grain, industrially produced',
        100,
        7.866,
        47.944,
        1.72,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6310,
        'Herring, Baltic Sea, unspecified, raw',
        100,
        16.2092857142857,
        0,
        6.40714285714286,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6311,
        'Beef, mince, 5-10% fat, raw',
        100,
        19.454,
        0,
        9.52,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6312,
        'Pork, mince, 10-15% fat, raw',
        100,
        18.8527272727273,
        0,
        10.3909090909091,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6313,
        'Turkey, mince, 5-10% fat, raw',
        100,
        20.08,
        0,
        8.53333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6314,
        'Hamburger, small patty, salad and dressing, fast food',
        100,
        9.80208333333333,
        23.81625,
        10.4166666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6315,
        'Hamburger, medium patty, salad and dressing, fast food',
        100,
        10.40625,
        20.8220833333333,
        11,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6316,
        'Hamburger, large patty, salad and dressing, fast food',
        100,
        12.7395833333333,
        16.4120833333333,
        13.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6317,
        'Hamburger and bacon, salad and dressing, fast food',
        100,
        11.65625,
        20.76625,
        14.225,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6318,
        'Cheeseburger, salad and dressing, fast food',
        100,
        10.65625,
        23.60125,
        12.925,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6319,
        'Hamburger, bacon and cheese, salad and dressing, fast food',
        100,
        11.046875,
        19.538125,
        12.625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6320,
        'Hamburger, chicken, salad and dressing, fast food',
        100,
        9.51041666666667,
        20.7029166666667,
        9.88333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6321,
        'Sandwich with chicken, salad and dressing, fast food',
        100,
        10.5,
        30.8725,
        6.475,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6322,
        'Sandwich with chicken and bacon, salad and dressing, fast food',
        100,
        11.078125,
        25.324375,
        7.025,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6323,
        'Sandwich, ham and cheese, salad and dressing, fast food',
        100,
        9.265625,
        27.906875,
        8.875,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6324,
        'Sandwich with eggs / shrimp, salad and dressing, fast food',
        100,
        7.75,
        25.6375,
        6.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6325,
        'Sandwich with seafood, salad and dressing, fast food',
        100,
        9.5625,
        26.7275,
        6.825,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6326,
        'Sandwich with kebab, salad and dressing, fast food',
        100,
        8.53125,
        25.43875,
        9.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6327,
        'Sandwich with roast pork, red cabbage and cucumber pickles, fast food',
        100,
        14.2578125,
        21.7584375,
        12.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6328,
        'Triangle sandwich with chicken and bacon, salad and dressing, fast food',
        100,
        11.3035714285714,
        21.5564285714286,
        8.68571428571429,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6329,
        'Triangle sandwich with ham and cheese, salad and dressing, fast food',
        100,
        10.3875,
        22.9245,
        11.06,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6330,
        'Triangle sandwich with eggs, salad and dressing, fast food',
        100,
        8.65625,
        22.13375,
        9.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6331,
        'Sandwich with meatball, salad and dressing, fast food',
        100,
        8.75,
        25.6075,
        9.4375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6332,
        'Pita sandwich with kebab, salad and dressing, fast food',
        100,
        9.09375,
        18.20625,
        7.375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6333,
        'Pita sandwich with chicken, salad and dressing, fast food',
        100,
        9.78125,
        19.44375,
        5.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6334,
        'Pita sandwich with shrimp, salad and dressing, fast food',
        100,
        5.78125,
        16.51875,
        4.675,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6335,
        'Pita sandwich with falafel, salad and dressing, fast food',
        100,
        6.203125,
        28.556875,
        9.525,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6336,
        'Pita sandwich with salad and dressing, fast food',
        100,
        3.6046875,
        21.4753125,
        5.925,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6337,
        'Dürüm roll with kebab, salad and dressing, fast food',
        100,
        12.609375,
        22.428125,
        9.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6338,
        'Bacon-Wrapped Frankfurt sausage, freid, fast food',
        100,
        16.296875,
        4.235625,
        26.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6339,
        'French Hotdog, fast food',
        100,
        10.15625,
        27.50125,
        15.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6340,
        'Hotdog, fast food',
        100,
        8.5,
        22.39,
        15.125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6341,
        'Frankfurt sausage mix, fried, with frensh fries, fast food',
        100,
        7.9609375,
        29.2503125,
        19.9375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6342,
        'Kebab mix and french fries, deepfried, fast food',
        100,
        8.890625,
        26.641875,
        17.6125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6343,
        'Chicken, grilled, fast food',
        100,
        27.953,
        0,
        11.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6344,
        'Chicken nuggets, deepfried, fast food',
        100,
        14.6625,
        19.0755,
        16.14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6345,
        'Pizza with tomato and cheese, fast food',
        100,
        13.3125,
        38.085,
        13.475,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6346,
        'Pizza with seafood, tomato and cheese, fast food',
        100,
        13.75,
        28.79,
        9.975,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6347,
        'Pizza with salami, tomato and cheese, fast food',
        100,
        12.78125,
        33.60625,
        14.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6348,
        'Pizza with vegetables, fruit, tomato and cheese, fast food',
        100,
        8.078125,
        29.166875,
        8.375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6349,
        'Pizza with meat, tomato and cheese, fast food',
        100,
        11.8472222222222,
        31.445,
        9.77777777777778,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6350,
        'Pizza with meat and seafood, tomato and cheese, fast food',
        100,
        12.8125,
        24.4875,
        9.275,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6351,
        'Fish fillet, breaded, fried, delicatessens',
        100,
        12.6979166666667,
        22.8870833333333,
        15.6666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6352,
        'Tuna, yellowfin, raw',
        100,
        24.3883333333333,
        0,
        1.68333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6353,
        'Shrimp, boiled, in brine',
        100,
        15.453125,
        0.404374999999998,
        0.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6354,
        'Salmon, atlantic, aquaculture, gravad',
        100,
        20.3571428571429,
        2.68642857142857,
        8.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6355,
        'Flounder, fillet, breaded, raw, frozen',
        100,
        11.1302083333333,
        22.3297916666667,
        2.29166666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6356,
        'Fish fingers, breaded, frozen',
        100,
        12.828125,
        15.1502083333333,
        8.425,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6357,
        'Salmon, atlantic, aquaculture, raw',
        100,
        15.8479166666667,
        0,
        18.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6358,
        'Salmon, pacific, wild, raw',
        100,
        21.1908333333333,
        0,
        2.26666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6359,
        'Striped catfish, raw, frozen',
        100,
        14.56,
        0,
        1.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6360,
        'Blue mussel, boiled',
        100,
        17.7708333333333,
        4.145,
        2.75,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6361,
        'Eggs, chicken, battery hens, raw',
        100,
        11.98828125,
        1.11671875,
        9.403125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6362,
        'Eggs, chicken, free-range (outdoor) hens, raw',
        100,
        12.2833333333333,
        1.198,
        9.22,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6363,
        'Eggs, chicken, free-range hens (indoor), raw',
        100,
        12.25390625,
        1.14109375,
        9.5125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6364,
        'Eggs, chicken, organic farming, raw',
        100,
        12.34375,
        1.30875,
        9.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6365,
        'Eggs, chicken, boiled, frozen',
        100,
        13.7083333333333,
        1.465,
        9.46666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6366,
        'Eggs, chicken, yolk, pasteurized',
        100,
        12.96875,
        2.59125,
        20.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6367,
        'Eggs, chicken, scrambled, industrially produced',
        100,
        10.84375,
        0.856250000000001,
        7.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6368,
        'Eggs, chicken, scrambled eggs mix, industrially produced',
        100,
        10.875,
        3.82,
        8.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6369,
        'Rye bread, dark, industrially produced',
        100,
        5.29519466666667,
        45.5308053333333,
        1.31,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6370,
        'Rye bread, dark, whole grains, industrially produced',
        100,
        5.09542,
        45.8200085714286,
        1.44857142857143,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6371,
        'Wheat bread, with coarse meal and high fat seeds, retail bakery',
        100,
        9.30769285714286,
        49.47445,
        5.89642857142857,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6372,
        'Rye bread, dark, retail bakery',
        100,
        6.04654285714286,
        49.2534571428572,
        1.25714285714286,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6373,
        'Rye bread, dark, whole grains, retail bakery',
        100,
        5.11291,
        48.25109,
        1.19,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6374,
        'Rye bread, dark, with high fat seeds, retail bakery',
        100,
        7.43908,
        41.4915866666667,
        6.76,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6375,
        'Rye bread, with siftings (soft) and high fat seeds, retail bakery',
        100,
        8.05885384615385,
        40.1819153846154,
        8.23846153846154,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6376,
        'Yogurt, with fruit, 0.1 % fat, no sugar added',
        100,
        3.90775,
        6.63725,
        0.085,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6377,
        'Cheese, cottage, 10 % fidm.',
        100,
        13.079,
        3.531,
        2.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6378,
        'Cheese, cottage, 5 % fidm.',
        100,
        13.5894,
        2.6906,
        1.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6379,
        'Lactose, dry, powder',
        100,
        0,
        97.77,
        0.09,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6380,
        'Yogurt, with fruit, 0.4 % fat',
        100,
        4.7212,
        13.5688,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6381,
        'Buttermilk dessert',
        100,
        3.2538,
        10.5512,
        1.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6382,
        'Milkshake',
        100,
        5.1359,
        12.4016,
        1.775,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6383,
        'Blended spread, 75% fat',
        100,
        0.55825,
        1.12175,
        75.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6384,
        'Blended spread, reduced fat, 57% fat',
        100,
        0.4466,
        0.6234,
        59.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6385,
        'Pork, thick flank, completely trimmed, raw',
        100,
        20.8475,
        0,
        1.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6386,
        'Pork, thin belly with rind, raw',
        100,
        17.1591666666667,
        0,
        25.6416666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6387,
        'Pork, loin, 0-1 mm fat, raw',
        100,
        21.87,
        0,
        4.73333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6388,
        'Pork, jaw, trimmed, raw',
        100,
        19.8125,
        0.1275,
        4.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6389,
        'Oat drink, unfortified',
        100,
        0.6456725,
        9.1018275,
        1.125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6390,
        'Rice drink, unfortified',
        100,
        0.0520625,
        10.3554375,
        0.95,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6391,
        'Almond drink, unfortified',
        100,
        0.88578,
        4.72422,
        2.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6392,
        'Skyr, 0.2 % fat',
        100,
        10.99912,
        6.04578410958904,
        0.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6393,
        'Galia melon, raw',
        100,
        0.4765625,
        8.70885416666667,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6394,
        'Cream pastry, chocolate cream puff',
        100,
        3.25875,
        34.26725,
        23.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6395,
        'Caraway seed, raw',
        100,
        19.8008,
        49.8692,
        14.59,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6396,
        'Cumin seeds, raw',
        100,
        17.808,
        44.242,
        22.27,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6397, 'Tahini', 100, 17.013, 21.177, 53.76, NULL);
INSERT INTO temp."Raw"
VALUES (
        6398,
        'Rice drink, with added calcium',
        100,
        0.06664,
        11.53736,
        0.98,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6399,
        'Oat drink, with added calcium',
        100,
        0.1749,
        6.42843333333333,
        1.43333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6400,
        'Soy drink, with added calcium',
        100,
        3.1933175,
        3.4316825,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6401,
        'Rolls, italien type, retail bakery',
        100,
        9.0174,
        50.7866,
        2.68,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6402,
        'Wheat bread, pita, white',
        100,
        7.562,
        52.788,
        1.56666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6403,
        'Wheat bread, pita, with coarse meal',
        100,
        8.493,
        52.0403333333333,
        1.83333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6404,
        'Breakfast cereal, with bran',
        100,
        10.974975,
        79.145025,
        3.225,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6405, 'Sea Buckthorn', 100, 0.7, 9.9, 5, NULL);
INSERT INTO temp."Raw"
VALUES (
        6406,
        'Veal, rump, raw',
        100,
        20.625,
        0.121666666666666,
        6.66666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6407,
        'Pepper, sweet, yellow, raw',
        100,
        0.94375,
        5.45,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6408,
        'Spelt wheat flour, bolted',
        100,
        11.9985,
        73.6265,
        1.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6409,
        'Spelt wheat flour, whole',
        100,
        13.409,
        72.231,
        2.25,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6410,
        'Quinoa, white, raw',
        100,
        11.715,
        68.12,
        6.55,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6411,
        'Quinoa, red, raw',
        100,
        12.815,
        67.165,
        6.95,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6412,
        'Quinoa, black, raw',
        100,
        12.0175,
        67.1925,
        6.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6413,
        'Rye flour, unspecified',
        100,
        8.07455,
        78.06545,
        1.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6414,
        'Chia seeds',
        100,
        18.1525,
        37.2575,
        34.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6415,
        'Pearled spelt',
        100,
        11.058,
        73.892,
        2.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6416,
        'Pearled barley',
        100,
        8.48265,
        77.36235,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6417, 'Oil margarine', 100, 0, 0, 81.56, NULL);
INSERT INTO temp."Raw"
VALUES (
        6418,
        'Oil margarine, fortified with A- and D-vitamin',
        100,
        0,
        0,
        80.86,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6419,
        'Plant margarine, 80%, fry/bake, enriched with A- and D-vitamin',
        100,
        0,
        0,
        80.63,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6420,
        'Plant margarine, 80%, fry/bake, enriched with A-vitamin',
        100,
        0,
        0,
        80.63,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6421,
        'Plant margarine, 75%, fry/bake, fortified with A-vitamin',
        100,
        0,
        0,
        75.99,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6422,
        'Plant margarine, 60%, enriched with A-vitamin',
        100,
        0,
        0,
        60.74,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6423,
        'Margarine, 40%, enriched with A- and D-vitamin',
        100,
        0,
        0,
        40.55,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6424,
        'Beef, mince, 8-12% fat, raw',
        100,
        19.995,
        0,
        10.3333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6425,
        'Chicken, mince, 3-10% fat, raw',
        100,
        16.98,
        0,
        7.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6426,
        'Baby spinach, raw',
        100,
        2.46875,
        2.23,
        0.0375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6427,
        'Salad, romaine, raw',
        100,
        1.625,
        2.955,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6428,
        'Squash, red kuri, Danish, raw',
        100,
        1.1953125,
        8.2234375,
        0.225,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6429,
        'Butternut squash, raw',
        100,
        0.796875,
        7.981875,
        0.1625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6430,
        'Energy drink, Red Bull',
        100,
        0.46,
        93.64,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6431,
        'Whey protein powder',
        100,
        66.67,
        17.94,
        5.13,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6432,
        'Rice cake/cracker, puffed brown rice, plain',
        100,
        8.2,
        0,
        2.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6433,
        'Spinach, whole leaf, frozen',
        100,
        2.1875,
        2.87125,
        0.325,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6434,
        'Cabbage, red, Danish, raw',
        100,
        1.15625,
        6.13,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6435,
        'Cabbage, pointed, Danish, raw',
        100,
        1.546875,
        4.498125,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6436,
        'Parsnip, Danish, raw',
        100,
        1.26785714285714,
        17.165,
        0.814285714285714,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6437,
        'Veal, top round, raw',
        100,
        20.09375,
        0.709583333333333,
        10.2666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6438,
        'Veal, topside, trimmed, raw',
        100,
        22.285,
        0,
        2.01666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6439,
        'Veal, heart of rump, trimmed, raw',
        100,
        21.8666666666667,
        0,
        1.86666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6440,
        'Veal, shortloin, raw',
        100,
        21.2733333333333,
        0,
        8.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6441,
        'Beef, bottom round, raw',
        100,
        19.0104166666667,
        0.456249999999999,
        16.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6442,
        'Beef, flank steak, raw',
        100,
        20.6466666666667,
        0,
        6.61666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6443,
        'Carrots, imported, without peel, raw',
        100,
        0.34375,
        10.32625,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6444,
        'Carrots, Danish, without peel, raw',
        100,
        0.510416666666667,
        8.14625,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6445,
        'Parsnips, imported, raw',
        100,
        1.375,
        17.025,
        0.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6446,
        'Celery, root, imported, raw',
        100,
        1.25,
        0,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6447,
        'Cream yoghurt, plain, 10% fat (Greek and Turkey style)',
        100,
        5.01468,
        6.05032,
        8.035,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6448,
        'Cream yoghurt, plain, 2% fat (Greek and Turkey style)',
        100,
        9.15211,
        3.78289,
        2.14,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6449,
        'Milk, acidophilus cultured from partly skimmed milk, 1.5% fat',
        100,
        3.71954,
        4.06046,
        1.44,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6450,
        'Milk, acidophilus cultured from skimmed milk, 0.5% fat',
        100,
        4.49152,
        4.76848,
        0.55,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6451,
        'Pizza topping (Grated cheese, Mozzarella)',
        100,
        19.7461,
        8.1389,
        22.65,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6452,
        'Skinkeost 30+ (Processed cheese with ham)',
        100,
        14.7378,
        2.8622,
        16.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6453,
        'Rejeost, light, 8% fat, (Processed cheese with shrimp)',
        100,
        17.6726,
        4.9674,
        6.75,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6454,
        'Feta, 5% (Salad cheese)',
        100,
        21.692,
        1.638,
        9.23,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6455,
        'Cheese, Cream, 30-40 % fidm.',
        100,
        10.3994,
        4.5106,
        10.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6456,
        'Cheese, Cream, 45-55 % fidm.',
        100,
        5.6144,
        3.4756,
        24.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6457,
        'Cheese, Brie, 11%',
        100,
        23.2232,
        1.4768,
        8.565,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6458,
        'Goat cheese, soft, 45-55% fidm.',
        100,
        17.8214666666667,
        2.44186666666666,
        26.4333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6459,
        'Goat cheese, in brine, 20-25%',
        100,
        15.2163,
        0.803699999999997,
        22.95,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6460,
        'Goat cheese, hard, 45% fidm.',
        100,
        21.7558,
        1.1642,
        30.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6461,
        'Cheese, firm, 6%/10% fidm.',
        100,
        28.8695,
        0.4605,
        6.365,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6462,
        'Cheese, firm, 45% fidm., Danish',
        100,
        24.2227333333333,
        1.09393333333333,
        24.8333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6463,
        'Cheese, firm, 50% fidm., Danish',
        100,
        23.8612,
        1.3688,
        29.6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6464,
        'Veal, brisket, raw',
        100,
        20.4583333333333,
        0,
        7.26666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6465,
        'Beef, brisket, boneless, raw',
        100,
        19.3020833333333,
        0.422916666666658,
        13.1333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6466,
        'Cold cuts, with wheat protein',
        100,
        29.23745,
        19.21255,
        4.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6467,
        'Seitan',
        100,
        26.4609125,
        7.2165875,
        7.1625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6468,
        'Mince, with mycoprotein',
        100,
        9.92933333333333,
        11.8273333333333,
        2.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6469,
        'Mince, with soy protein',
        100,
        16.6089625,
        12.2210375,
        7.8375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6470,
        'Mince balls, with soy protein',
        100,
        14.0466,
        10.0784,
        8.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6471,
        'Pieces, with soy protein',
        100,
        18.6241166666667,
        9.40088333333333,
        2.75,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6472,
        'Cold cuts, with soy and pea protein',
        100,
        6.42306666666667,
        8.18693333333334,
        11.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6473,
        'Mince, with pea protein',
        100,
        14.0342666666667,
        11.8207333333333,
        14.0833333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6474,
        'Sausage, with pea protein',
        100,
        13.6322666666667,
        9.45773333333333,
        13.3333333333333,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6475,
        'Cold cuts, with eggwhite',
        100,
        7.875,
        4.61,
        14.2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6476,
        'Pieces, with mycoprotein',
        100,
        8.85866666666667,
        11.1113333333333,
        2.06666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6477,
        'Sausage, with soy protein',
        100,
        11.434275,
        9.473225,
        10.475,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6478,
        'Beef, entrecote/rib eye, raw',
        100,
        18.64,
        0,
        12.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6479,
        'Aronia berries',
        100,
        0.7,
        14.4,
        0.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6480,
        'Oatmilk, with added calcium and vitamins',
        100,
        0.7579,
        9.7571,
        2.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6481,
        'Almondmilk, with added calcium',
        100,
        0.5439,
        2.4411,
        1.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6482,
        'Lentils, red, dried, raw',
        100,
        22.527,
        62.9796666666667,
        1.95,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6483,
        'Lentils, green, dried, raw',
        100,
        20.466,
        64.209,
        1.86666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6484,
        'Beans, white, dried, raw',
        100,
        18.0114,
        62.95485,
        2.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6485,
        'Beans, red kidney, dried, raw',
        100,
        18.9618,
        61.9282,
        1.8625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6486,
        'Chickpeas, dry, raw',
        100,
        18.792,
        62.398,
        6,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6487,
        'Le Puy green lentil, dried, raw',
        100,
        20.385,
        62.475,
        2,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6488,
        'Lentils, beluga, dried, raw',
        100,
        22.545,
        62.39,
        1.95,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6489,
        'Edamame beans (Soy beans), shelled, frozen',
        100,
        10.70625,
        8.95375,
        7.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6490,
        'Lentils, green, boiled, canned',
        100,
        5.67,
        17.945,
        0.55,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6491,
        'Baked beans (white beans in tomato sauce)',
        100,
        3.7224,
        18.4176,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6492,
        'Chili beans (red kidney beans in chilisauce)',
        100,
        4.488,
        18.612,
        0.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6493,
        'Beans, red kidney, boiled, canned',
        100,
        6.072,
        19.673,
        0.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6494,
        'Beans, red kidney, cooked, ready to eat',
        100,
        8.0784,
        27.3716,
        0.85,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6495,
        'Chickpeas, boiled, canned',
        100,
        6.237,
        20.763,
        2.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6496,
        'Chickpeas, steamed, ready to eat',
        100,
        8.532,
        27.978,
        2.8,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6497,
        'Beans, black, boiled, canned',
        100,
        6.6264,
        22.1136,
        1.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6498,
        'White beans, dried and boiled',
        100,
        8.7296,
        29.1470666666667,
        1.16666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6499,
        'Beans, red kidney, dried and boiled',
        100,
        8.448,
        28.272,
        0.866666666666667,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6500,
        'Blueberries, frozen',
        100,
        0,
        12.595,
        0.45,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6501,
        'Raspberries, frozen',
        100,
        1.21875,
        12.17625,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6502,
        'Strawberries, frozen',
        100,
        0,
        8.1,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6503,
        'Sweet corn kernels, frozen',
        100,
        3.25,
        15.43,
        1.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6504,
        'Peas, green, frozen',
        100,
        4.9044,
        14.5956,
        0.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6505,
        'Green beans (haricots verts), frozen',
        100,
        1.89,
        6.875,
        0.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6506,
        'Cauliflower, frozen',
        100,
        1.1875,
        2.8725,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6507,
        'Broccoli, frozen',
        100,
        2.21875,
        3.37625,
        0.3,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6508,
        'Brussel sprouts, frozen',
        100,
        2.8125,
        7.6175,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6509,
        'Soup vegetables, frozen',
        100,
        1,
        6.785,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6510,
        'Root vegetables, frozen',
        100,
        1.0625,
        7.9775,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6511, 'Mango, frozen', 100, 0, 14.59, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        6512,
        'Pomegranate seeds, frozen',
        100,
        1.03125,
        17.47875,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6513,
        'Pineapple, frozen',
        100,
        0,
        13.57,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6514,
        'Mixed berries. frozen',
        100,
        0.90625,
        12.34375,
        0.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6515,
        'Baby carrots, frozen',
        100,
        0.8125,
        6.2025,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6516, 'Leek, frozen', 100, 0.875, 6.58, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        6517,
        'Avocado, frozen',
        100,
        1.46875,
        7.50625,
        12.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6518,
        'Wok mix, organic, frozen',
        100,
        1.4375,
        6.2225,
        0.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6519,
        'Wok mix, frozen',
        100,
        1.21875,
        6.56125,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6520,
        'Pearl onion. frozen',
        100,
        1.21875,
        10.79125,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6521,
        'Bean mix, frozen',
        100,
        5.75,
        13.615,
        1.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6522,
        'Root fruit fritters, frozen',
        100,
        2.03125,
        19.46875,
        6.9,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6523,
        'Sweet potato fries, frozen',
        100,
        1.6875,
        27.1975,
        6.15,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6524,
        'Potato wedges, frozen',
        100,
        1.9375,
        21.1325,
        3.1,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6525,
        'Halved potatoes, frozen',
        100,
        2,
        18.435,
        1.95,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6526,
        'Curly fries, frozen',
        100,
        2.5625,
        26.0175,
        7.35,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6527,
        'Potato rösti, frozen',
        100,
        2.0625,
        24.3725,
        7.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6528,
        'Potato croquettes, frozen',
        100,
        2.46875,
        26.53125,
        5.7,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6529,
        'Steak fries (thick fries with skin), frozen',
        100,
        2,
        21.4,
        3.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6530,
        'French fries, crinkle-cut, frozen',
        100,
        2.25,
        23.67,
        3.4,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6531,
        'Blueberries, raw',
        100,
        0.4921875,
        13.8528125,
        0.2375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6532,
        'Blackberry, raw',
        100,
        1.3125,
        9.0575,
        0.475,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6533,
        'Raspberry, raw',
        100,
        1.0390625,
        10.5634375,
        0.2625,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6534, 'Apricot, raw', 100, 0.75, 9.635, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        6535,
        'Plum, raw',
        100,
        0.2734375,
        12.2728125,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6536,
        'Peach, raw',
        100,
        0.6640625,
        10.7421875,
        0.0375,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6537, 'Cherry, raw', 100, 1, 14.835, 0, NULL);
INSERT INTO temp."Raw"
VALUES (
        6538,
        'Pineapple, raw',
        100,
        0.078125,
        12.070625,
        0.05,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6539,
        'Grapefruit, raw',
        100,
        0.4765625,
        10.3871875,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6540,
        'Melon, honeydew, raw',
        100,
        0.1640625,
        8.5946875,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6541,
        'Kiwi fruit, raw',
        100,
        0.9140625,
        12.1184375,
        0.7125,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6542,
        'Mango, raw',
        100,
        0.1953125,
        13.7084375,
        0,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (
        6543,
        'Wholewheat tortilla',
        100,
        4,
        22,
        0.5,
        NULL
    );
INSERT INTO temp."Raw"
VALUES (6544, 'Coconut milk', 100, 2, 3, 17, NULL);
INSERT INTO temp."Raw"
VALUES (6545, 'Hummus', 100, 3.33, 20, 16.67, NULL);
UPDATE SQLITE_SEQUENCE
SET seq = 6545
WHERE name = 'temp."Raw"';
INSERT INTO documents (title, kind, body)
SELECT t.name,
    'rawIngredient',
    json_object(
        'kind',
        'rawIngredient',
        'name',
        t.name,
        'container',
        t.container,
        'proteins',
        t.proteins,
        'carbs',
        t.carbs,
        'fat',
        t.fat
    )
FROM temp."Raw" t;