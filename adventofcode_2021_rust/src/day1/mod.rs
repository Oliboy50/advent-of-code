pub fn exec() {
    let input = get_input();

    let mut input_grouped_by_sliding_windows = vec![];
    for (i, x1) in input.iter().enumerate() {
        if let (Some(x2), Some(x3)) = (input.get(i + 1), input.get(i + 2)) {
            input_grouped_by_sliding_windows.push(x1 + x2 + x3);
        }
    }

    let mut number_of_increase = -1;
    let mut last_value = 0;
    for i in input_grouped_by_sliding_windows {
        if last_value < i {
            number_of_increase = number_of_increase + 1;
        }
        last_value = i;
    }

    println!("[day1][part2] = {}", number_of_increase);
}

fn get_input() -> Vec<i32> {
    vec![
        159, 158, 174, 196, 197, 194, 209, 213, 214, 222, 223, 228, 229, 236, 237, 238, 241, 248,
        255, 256, 257, 269, 276, 292, 302, 304, 308, 309, 311, 315, 314, 320, 323, 324, 329, 330,
        311, 340, 341, 342, 341, 347, 349, 350, 367, 370, 378, 379, 378, 379, 389, 390, 393, 388,
        393, 402, 405, 404, 402, 410, 411, 441, 449, 448, 449, 450, 412, 414, 415, 418, 420, 440,
        439, 458, 459, 457, 463, 465, 469, 470, 479, 480, 481, 482, 475, 468, 467, 472, 484, 486,
        498, 489, 493, 511, 510, 493, 495, 494, 514, 517, 520, 521, 544, 557, 563, 565, 568, 572,
        571, 572, 575, 584, 583, 599, 600, 605, 612, 615, 618, 625, 627, 633, 663, 665, 666, 669,
        683, 706, 716, 722, 731, 734, 739, 741, 754, 760, 765, 783, 788, 795, 811, 813, 818, 805,
        816, 821, 829, 830, 854, 855, 850, 859, 860, 866, 867, 866, 867, 874, 875, 876, 870, 875,
        878, 880, 884, 898, 915, 918, 922, 923, 949, 942, 945, 949, 970, 956, 969, 964, 972, 973,
        975, 974, 970, 989, 990, 993, 994, 1002, 1003, 1007, 1013, 1025, 1035, 1033, 1040, 1045,
        1055, 1056, 1057, 1059, 1060, 1063, 1051, 1056, 1057, 1047, 1054, 1055, 1051, 1052, 1058,
        1061, 1066, 1067, 1070, 1074, 1075, 1076, 1084, 1122, 1128, 1140, 1141, 1139, 1152, 1151,
        1152, 1156, 1167, 1166, 1165, 1169, 1162, 1175, 1202, 1212, 1218, 1219, 1240, 1238, 1224,
        1230, 1245, 1253, 1285, 1295, 1297, 1309, 1323, 1329, 1333, 1336, 1339, 1318, 1321, 1318,
        1345, 1347, 1349, 1352, 1353, 1360, 1358, 1345, 1352, 1356, 1357, 1370, 1373, 1393, 1423,
        1424, 1431, 1447, 1454, 1444, 1470, 1477, 1479, 1485, 1483, 1497, 1518, 1536, 1539, 1544,
        1549, 1558, 1557, 1581, 1585, 1591, 1632, 1633, 1636, 1639, 1633, 1637, 1638, 1639, 1640,
        1642, 1658, 1676, 1675, 1707, 1708, 1709, 1710, 1711, 1715, 1707, 1712, 1733, 1734, 1732,
        1722, 1730, 1746, 1747, 1748, 1749, 1755, 1759, 1771, 1772, 1771, 1770, 1771, 1773, 1772,
        1771, 1780, 1781, 1782, 1781, 1775, 1786, 1798, 1814, 1818, 1826, 1791, 1793, 1796, 1821,
        1822, 1825, 1834, 1838, 1867, 1907, 1908, 1910, 1921, 1926, 1924, 1921, 1941, 1945, 1958,
        1957, 1978, 1973, 1974, 1977, 1978, 1984, 1992, 1998, 2002, 1998, 1993, 1984, 2013, 2014,
        2017, 2016, 2019, 2024, 2031, 2032, 2042, 2041, 2047, 2055, 2057, 2059, 2066, 2086, 2090,
        2091, 2094, 2112, 2113, 2123, 2125, 2123, 2124, 2135, 2152, 2155, 2158, 2157, 2169, 2168,
        2166, 2167, 2168, 2185, 2190, 2189, 2190, 2198, 2170, 2171, 2177, 2184, 2186, 2187, 2162,
        2174, 2175, 2172, 2180, 2182, 2188, 2213, 2226, 2228, 2227, 2216, 2220, 2221, 2220, 2212,
        2215, 2220, 2223, 2226, 2228, 2236, 2237, 2238, 2258, 2257, 2256, 2260, 2261, 2271, 2294,
        2297, 2309, 2311, 2327, 2326, 2340, 2339, 2342, 2343, 2346, 2352, 2363, 2364, 2371, 2386,
        2387, 2392, 2401, 2407, 2415, 2418, 2432, 2457, 2458, 2459, 2461, 2463, 2464, 2465, 2486,
        2487, 2490, 2509, 2510, 2518, 2522, 2525, 2527, 2521, 2522, 2526, 2529, 2530, 2552, 2548,
        2564, 2566, 2568, 2569, 2570, 2565, 2576, 2577, 2581, 2582, 2587, 2588, 2568, 2569, 2563,
        2575, 2576, 2577, 2578, 2588, 2603, 2604, 2605, 2620, 2622, 2627, 2630, 2639, 2629, 2631,
        2632, 2644, 2646, 2655, 2658, 2663, 2660, 2659, 2657, 2655, 2656, 2657, 2658, 2669, 2679,
        2678, 2684, 2677, 2721, 2728, 2729, 2728, 2729, 2730, 2733, 2742, 2730, 2731, 2733, 2735,
        2740, 2750, 2746, 2748, 2751, 2771, 2786, 2792, 2797, 2799, 2803, 2814, 2825, 2832, 2839,
        2844, 2854, 2833, 2842, 2844, 2866, 2868, 2871, 2872, 2876, 2877, 2901, 2910, 2916, 2940,
        2951, 2966, 2967, 2972, 2976, 2978, 2977, 2980, 2993, 3004, 3005, 3031, 3030, 3036, 3035,
        3038, 3041, 3040, 3043, 3046, 3068, 3080, 3081, 3101, 3104, 3111, 3118, 3119, 3112, 3111,
        3113, 3140, 3165, 3164, 3174, 3184, 3200, 3183, 3185, 3187, 3188, 3197, 3215, 3233, 3232,
        3243, 3220, 3221, 3224, 3226, 3263, 3275, 3286, 3287, 3300, 3304, 3314, 3316, 3311, 3314,
        3324, 3325, 3329, 3347, 3341, 3346, 3348, 3363, 3369, 3396, 3384, 3390, 3391, 3394, 3393,
        3399, 3405, 3411, 3417, 3408, 3378, 3383, 3379, 3385, 3386, 3406, 3397, 3374, 3378, 3373,
        3358, 3357, 3381, 3387, 3422, 3420, 3415, 3416, 3432, 3419, 3422, 3423, 3436, 3437, 3439,
        3441, 3449, 3453, 3488, 3499, 3513, 3536, 3576, 3570, 3569, 3570, 3571, 3575, 3573, 3617,
        3594, 3589, 3587, 3596, 3583, 3588, 3589, 3583, 3594, 3612, 3617, 3634, 3628, 3629, 3630,
        3640, 3642, 3647, 3644, 3657, 3658, 3657, 3660, 3661, 3662, 3681, 3684, 3694, 3698, 3697,
        3707, 3709, 3710, 3723, 3741, 3747, 3752, 3753, 3747, 3751, 3756, 3752, 3781, 3800, 3804,
        3792, 3809, 3805, 3806, 3812, 3813, 3814, 3818, 3820, 3835, 3836, 3837, 3806, 3812, 3837,
        3840, 3845, 3842, 3879, 3880, 3890, 3919, 3927, 3929, 3920, 3916, 3928, 3892, 3865, 3876,
        3913, 3933, 3934, 3901, 3902, 3903, 3909, 3910, 3911, 3912, 3920, 3949, 3944, 3946, 3947,
        3963, 3976, 3977, 3979, 3985, 3997, 4006, 4014, 4021, 4026, 4030, 4031, 4054, 4044, 4045,
        4055, 4053, 4051, 4055, 4068, 4066, 4068, 4069, 4072, 4071, 4063, 4064, 4067, 4075, 4099,
        4140, 4141, 4172, 4171, 4172, 4182, 4191, 4192, 4194, 4201, 4207, 4211, 4213, 4220, 4258,
        4257, 4256, 4258, 4271, 4270, 4263, 4265, 4266, 4292, 4290, 4297, 4298, 4299, 4316, 4333,
        4335, 4346, 4349, 4354, 4355, 4357, 4373, 4371, 4361, 4366, 4369, 4373, 4374, 4378, 4380,
        4394, 4395, 4400, 4406, 4412, 4419, 4417, 4424, 4440, 4443, 4449, 4459, 4461, 4466, 4461,
        4463, 4462, 4464, 4472, 4475, 4476, 4478, 4464, 4481, 4483, 4493, 4504, 4505, 4513, 4516,
        4527, 4526, 4514, 4515, 4544, 4545, 4560, 4562, 4561, 4573, 4572, 4571, 4592, 4600, 4589,
        4606, 4612, 4628, 4629, 4597, 4623, 4624, 4619, 4620, 4636, 4637, 4647, 4649, 4650, 4653,
        4655, 4663, 4681, 4677, 4696, 4698, 4704, 4709, 4706, 4696, 4698, 4697, 4698, 4711, 4716,
        4740, 4741, 4742, 4743, 4740, 4750, 4742, 4745, 4753, 4754, 4755, 4756, 4757, 4758, 4788,
        4791, 4802, 4811, 4800, 4808, 4817, 4789, 4809, 4820, 4821, 4802, 4804, 4805, 4807, 4808,
        4799, 4810, 4807, 4806, 4791, 4801, 4808, 4818, 4828, 4841, 4837, 4841, 4860, 4880, 4909,
        4913, 4910, 4909, 4925, 4927, 4929, 4932, 4933, 4932, 4933, 4949, 4938, 4940, 4930, 4944,
        4950, 4944, 4957, 4967, 4973, 4979, 4974, 4962, 4949, 4950, 4952, 4956, 4961, 4962, 4981,
        4983, 4985, 5000, 5001, 5004, 5031, 5035, 5036, 5037, 5040, 5048, 5057, 5056, 5045, 5047,
        5031, 5034, 5036, 5047, 5065, 5068, 5069, 5053, 5046, 5043, 5048, 5050, 5059, 5092, 5088,
        5089, 5090, 5091, 5090, 5091, 5104, 5109, 5112, 5113, 5107, 5106, 5121, 5114, 5122, 5117,
        5135, 5146, 5149, 5150, 5151, 5157, 5158, 5160, 5158, 5163, 5181, 5182, 5160, 5161, 5131,
        5149, 5152, 5150, 5151, 5154, 5153, 5163, 5170, 5190, 5193, 5199, 5201, 5205, 5206, 5202,
        5206, 5208, 5209, 5208, 5212, 5224, 5225, 5226, 5234, 5216, 5239, 5233, 5232, 5209, 5218,
        5227, 5226, 5230, 5233, 5244, 5249, 5250, 5251, 5249, 5266, 5264, 5296, 5303, 5322, 5308,
        5331, 5334, 5339, 5345, 5346, 5347, 5350, 5351, 5357, 5353, 5354, 5356, 5357, 5370, 5371,
        5370, 5373, 5376, 5384, 5385, 5392, 5394, 5395, 5407, 5413, 5419, 5422, 5435, 5432, 5437,
        5440, 5442, 5444, 5441, 5442, 5445, 5447, 5450, 5459, 5477, 5478, 5482, 5476, 5475, 5476,
        5480, 5481, 5475, 5487, 5507, 5506, 5507, 5510, 5505, 5508, 5509, 5520, 5515, 5518, 5525,
        5511, 5486, 5516, 5533, 5532, 5545, 5548, 5547, 5560, 5562, 5566, 5570, 5574, 5576, 5586,
        5558, 5566, 5552, 5541, 5545, 5553, 5550, 5546, 5555, 5551, 5547, 5562, 5578, 5604, 5608,
        5614, 5613, 5601, 5611, 5622, 5621, 5619, 5618, 5625, 5627, 5630, 5623, 5624, 5619, 5616,
        5618, 5629, 5635, 5636, 5637, 5638, 5647, 5648, 5649, 5650, 5649, 5650, 5652, 5655, 5664,
        5665, 5666, 5673, 5674, 5689, 5694, 5696, 5715, 5716, 5713, 5712, 5743, 5767, 5768, 5769,
        5770, 5778, 5793, 5794, 5793, 5799, 5800, 5803, 5804, 5805, 5806, 5829, 5830, 5833, 5834,
        5836, 5838, 5844, 5843, 5851, 5852, 5851, 5887, 5891, 5872, 5877, 5880, 5875, 5879, 5880,
        5887, 5878, 5882, 5884, 5883, 5864, 5876, 5882, 5896, 5899, 5886, 5887, 5891, 5894, 5937,
        5934, 5931, 5940, 5943, 5941, 5942, 5951, 5948, 5958, 5947, 5945, 5947, 5958, 5964, 5967,
        5966, 5957, 5958, 5959, 5962, 5946, 5970, 5978, 5972, 5986, 5974, 5968, 5985, 5997, 6001,
        6042, 6038, 6041, 6068, 6082, 6087, 6090, 6101, 6102, 6106, 6110, 6097, 6098, 6105, 6106,
        6122, 6121, 6140, 6173, 6187, 6193, 6195, 6197, 6198, 6208, 6215, 6217, 6220, 6232, 6247,
        6248, 6252, 6249, 6252, 6269, 6270, 6274, 6299, 6301, 6313, 6334, 6335, 6342, 6343, 6344,
        6378, 6377, 6376, 6378, 6369, 6362, 6369, 6371, 6370, 6379, 6384, 6387, 6407, 6373, 6392,
        6402, 6405, 6407, 6412, 6420, 6419, 6422, 6435, 6436, 6435, 6437, 6421, 6422, 6421, 6425,
        6415, 6431, 6425, 6426, 6427, 6426, 6429, 6443, 6435, 6454, 6455, 6456, 6474, 6475, 6476,
        6483, 6485, 6489, 6490, 6493, 6499, 6505, 6513, 6517, 6507, 6508, 6514, 6516, 6515, 6530,
        6523, 6520, 6525, 6513, 6514, 6529, 6531, 6524, 6537, 6525, 6529, 6535, 6555, 6561, 6562,
        6570, 6571, 6572, 6573, 6579, 6577, 6579, 6582, 6593, 6597, 6600, 6598, 6593, 6610, 6612,
        6613, 6614, 6592, 6593, 6599, 6593, 6594, 6559, 6560, 6564, 6561, 6560, 6557, 6562, 6564,
        6538, 6551, 6552, 6557, 6567, 6568, 6566, 6547, 6549, 6550, 6551, 6552, 6547, 6545, 6550,
        6555, 6561, 6584, 6597, 6599, 6607, 6616, 6619, 6626, 6637, 6645, 6646, 6648, 6664, 6665,
        6669, 6673, 6677, 6684, 6698, 6699, 6700, 6704, 6712, 6724, 6738, 6748, 6751, 6753, 6756,
        6781, 6782, 6785, 6794, 6795, 6770, 6781, 6777, 6779, 6784, 6793, 6785, 6786, 6790, 6773,
        6778, 6789, 6799, 6804, 6805, 6806, 6816, 6823, 6829, 6831, 6834, 6836, 6838, 6857, 6859,
        6860, 6889, 6913, 6914, 6917, 6919, 6929, 6924, 6919, 6924, 6899, 6904, 6924, 6925, 6932,
        6936, 6931, 6921, 6922, 6923, 6947, 6951, 6963, 6960, 6965, 6970, 6973, 6965, 6966, 6978,
        7010, 7015, 7012, 7017, 7027, 7035, 7036, 7026, 7035, 7007, 7006, 6997, 6995, 7003, 7021,
        7028, 7031, 7041, 7039, 7056, 7072, 7080, 7081, 7055, 7056, 7088, 7101, 7105, 7107, 7108,
        7104, 7105, 7084, 7087, 7084, 7088, 7086, 7098, 7100, 7081, 7083, 7081, 7084, 7085, 7089,
        7098, 7097, 7101, 7104, 7105, 7114, 7117, 7118, 7115, 7117, 7122, 7121, 7127, 7128, 7132,
        7149, 7166, 7127, 7122, 7123, 7124, 7126, 7127, 7146, 7148, 7161, 7166, 7167, 7160, 7155,
        7154, 7155, 7158, 7177, 7189, 7192, 7194, 7204, 7212, 7199, 7206, 7207, 7212, 7219, 7198,
        7208, 7217, 7223, 7216, 7217, 7224, 7223, 7222, 7219, 7223, 7222, 7224, 7230, 7231, 7230,
        7231, 7229, 7231, 7232, 7228, 7241, 7243, 7240, 7239, 7241, 7248, 7251, 7254, 7292, 7297,
        7296, 7297, 7301, 7306, 7308, 7311, 7310, 7312, 7317, 7319, 7324, 7323, 7330, 7332, 7345,
        7352, 7355, 7382, 7403, 7391, 7395, 7416, 7417, 7418, 7410, 7411, 7412, 7414, 7415, 7429,
        7408, 7407, 7408, 7411, 7410, 7413, 7411, 7408, 7410, 7408, 7409, 7410, 7416, 7417, 7419,
        7420, 7424, 7428, 7450, 7453, 7468, 7469, 7470, 7449, 7450, 7435, 7436, 7455, 7460, 7461,
        7462, 7465, 7485, 7496, 7502, 7503, 7514, 7526, 7538, 7528, 7534, 7543, 7544, 7545, 7519,
        7531, 7532, 7539, 7540, 7542, 7551, 7567, 7576, 7574, 7569, 7579, 7580, 7588, 7573, 7578,
        7579, 7581, 7600, 7634, 7631, 7630, 7639, 7648, 7649, 7652, 7653, 7656, 7665, 7635, 7643,
        7649, 7656, 7657, 7656, 7661, 7666, 7674, 7680, 7676, 7692, 7677, 7678, 7689, 7700, 7723,
        7729, 7738, 7743, 7748, 7769, 7770, 7776, 7784, 7786, 7788, 7805, 7806, 7801, 7825, 7830,
        7832, 7815, 7816, 7828, 7830, 7831, 7834, 7858, 7860, 7862, 7873, 7879, 7883, 7894, 7910,
        7916, 7917, 7935, 7942, 7930, 7932, 7934, 7935, 7932, 7936, 7944, 7951, 7953, 7954, 7960,
        7963, 7976, 7968, 7972, 7977, 7978, 7976, 7977, 7991, 8016, 8056, 8058, 8059, 8074, 8078,
        8087, 8088, 8083, 8088, 8094, 8103, 8118, 8114, 8115, 8111, 8116, 8115, 8105, 8106, 8107,
        8108, 8112, 8120, 8129, 8134, 8138, 8137, 8138, 8152, 8158, 8171, 8191, 8201, 8207, 8209,
        8236, 8237, 8239, 8244, 8239, 8241, 8252, 8243, 8244, 8260, 8254, 8248, 8251, 8246, 8249,
        8262, 8265, 8266, 8263, 8264, 8267, 8273, 8297, 8302, 8303, 8302, 8310, 8313, 8314, 8332,
        8333, 8306, 8308, 8307, 8334, 8332, 8329, 8330, 8332, 8337, 8338, 8343, 8350, 8349, 8361,
        8365, 8368, 8371, 8372, 8375, 8402, 8412, 8445, 8447, 8472, 8482, 8483, 8486, 8493, 8495,
        8494, 8493, 8494, 8499, 8501, 8503, 8513, 8509, 8510, 8514, 8515, 8514, 8518, 8535, 8538,
        8543, 8545, 8557, 8568,
    ]
}
