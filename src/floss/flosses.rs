use crate::color::Rgb;

#[derive(Clone, Copy)]
pub struct Floss<'a> {
    pub number: &'a str,
    pub name: &'a str,
    pub color: Rgb,
}

pub fn get_dmc_floss() -> Vec<Floss<'static>> {
    vec![
        Floss {
            number: "3713",
            name: "Salmon Very Light",
            color: Rgb::new(255, 226, 226),
        },
        Floss {
            number: "761",
            name: "Salmon Light",
            color: Rgb::new(255, 201, 201),
        },
        Floss {
            number: "760",
            name: "Salmon",
            color: Rgb::new(245, 173, 173),
        },
        Floss {
            number: "3712",
            name: "Salmon Medium",
            color: Rgb::new(241, 135, 135),
        },
        Floss {
            number: "3328",
            name: "Salmon Dark",
            color: Rgb::new(227, 109, 109),
        },
        Floss {
            number: "347",
            name: "Salmon Very Dark",
            color: Rgb::new(191, 45, 45),
        },
        Floss {
            number: "353",
            name: "Peach",
            color: Rgb::new(254, 215, 204),
        },
        Floss {
            number: "352",
            name: "Coral Light",
            color: Rgb::new(253, 156, 151),
        },
        Floss {
            number: "351",
            name: "Coral",
            color: Rgb::new(233, 106, 103),
        },
        Floss {
            number: "350",
            name: "Coral Medium",
            color: Rgb::new(224, 72, 72),
        },
        Floss {
            number: "349",
            name: "Coral Dark",
            color: Rgb::new(210, 16, 53),
        },
        Floss {
            number: "817",
            name: "Coral Red Very Dark",
            color: Rgb::new(187, 5, 31),
        },
        Floss {
            number: "3708",
            name: "Melon Light",
            color: Rgb::new(255, 203, 213),
        },
        Floss {
            number: "3706",
            name: "Melon Medium",
            color: Rgb::new(255, 173, 188),
        },
        Floss {
            number: "3705",
            name: "Melon Dark",
            color: Rgb::new(255, 121, 146),
        },
        Floss {
            number: "3801",
            name: "Melon Very Dark",
            color: Rgb::new(231, 73, 103),
        },
        Floss {
            number: "666",
            name: "Bright Red",
            color: Rgb::new(227, 29, 66),
        },
        Floss {
            number: "321",
            name: "Red",
            color: Rgb::new(199, 43, 59),
        },
        Floss {
            number: "304",
            name: "Red Medium",
            color: Rgb::new(183, 31, 51),
        },
        Floss {
            number: "498",
            name: "Red Dark",
            color: Rgb::new(167, 19, 43),
        },
        Floss {
            number: "816",
            name: "Garnet",
            color: Rgb::new(151, 11, 35),
        },
        Floss {
            number: "815",
            name: "Garnet Medium",
            color: Rgb::new(135, 7, 31),
        },
        Floss {
            number: "814",
            name: "Garnet Dark",
            color: Rgb::new(123, 0, 27),
        },
        Floss {
            number: "894",
            name: "Carnation Very Light",
            color: Rgb::new(255, 178, 187),
        },
        Floss {
            number: "893",
            name: "Carnation Light",
            color: Rgb::new(252, 144, 162),
        },
        Floss {
            number: "892",
            name: "Carnation Medium",
            color: Rgb::new(255, 121, 140),
        },
        Floss {
            number: "891",
            name: "Carnation Dark",
            color: Rgb::new(255, 87, 115),
        },
        Floss {
            number: "818",
            name: "Baby Pink",
            color: Rgb::new(255, 223, 217),
        },
        Floss {
            number: "957",
            name: "Geranium Pale",
            color: Rgb::new(253, 181, 181),
        },
        Floss {
            number: "956",
            name: "Geranium",
            color: Rgb::new(255, 145, 145),
        },
        Floss {
            number: "309",
            name: "Rose Dark",
            color: Rgb::new(186, 74, 74),
        },
        Floss {
            number: "963",
            name: "Dusty Rose Ult Vy Lt",
            color: Rgb::new(255, 215, 215),
        },
        Floss {
            number: "3716",
            name: "Dusty Rose Med Vy Lt",
            color: Rgb::new(255, 189, 189),
        },
        Floss {
            number: "962",
            name: "Dusty Rose Medium",
            color: Rgb::new(230, 138, 138),
        },
        Floss {
            number: "961",
            name: "Dusty Rose Dark",
            color: Rgb::new(207, 115, 115),
        },
        Floss {
            number: "3833",
            name: "Raspberry Light",
            color: Rgb::new(234, 134, 153),
        },
        Floss {
            number: "3832",
            name: "Raspberry Medium",
            color: Rgb::new(219, 85, 110),
        },
        Floss {
            number: "3831",
            name: "Raspberry Dark",
            color: Rgb::new(179, 47, 72),
        },
        Floss {
            number: "777",
            name: "Raspberry Very Dark",
            color: Rgb::new(145, 53, 70),
        },
        Floss {
            number: "819",
            name: "Baby Pink Light",
            color: Rgb::new(255, 238, 235),
        },
        Floss {
            number: "3326",
            name: "Rose Light",
            color: Rgb::new(251, 173, 180),
        },
        Floss {
            number: "776",
            name: "Pink Medium",
            color: Rgb::new(252, 176, 185),
        },
        Floss {
            number: "899",
            name: "Rose Medium",
            color: Rgb::new(242, 118, 136),
        },
        Floss {
            number: "335",
            name: "Rose",
            color: Rgb::new(238, 84, 110),
        },
        Floss {
            number: "326",
            name: "Rose Very Dark",
            color: Rgb::new(179, 59, 75),
        },
        Floss {
            number: "151",
            name: "Dusty Rose Vry Lt",
            color: Rgb::new(240, 206, 212),
        },
        Floss {
            number: "3354",
            name: "Dusty Rose Light",
            color: Rgb::new(228, 166, 172),
        },
        Floss {
            number: "3733",
            name: "Dusty Rose",
            color: Rgb::new(232, 135, 155),
        },
        Floss {
            number: "3731",
            name: "Dusty Rose Very Dark",
            color: Rgb::new(218, 103, 131),
        },
        Floss {
            number: "3350",
            name: "Dusty Rose Ultra Dark",
            color: Rgb::new(188, 67, 101),
        },
        Floss {
            number: "150",
            name: "Dusty Rose Ult Vy Dk",
            color: Rgb::new(171, 2, 73),
        },
        Floss {
            number: "3689",
            name: "Mauve Light",
            color: Rgb::new(251, 191, 194),
        },
        Floss {
            number: "3688",
            name: "Mauve Medium",
            color: Rgb::new(231, 169, 172),
        },
        Floss {
            number: "3687",
            name: "Mauve",
            color: Rgb::new(201, 107, 112),
        },
        Floss {
            number: "3803",
            name: "Mauve Dark",
            color: Rgb::new(171, 51, 87),
        },
        Floss {
            number: "3685",
            name: "Mauve Very Dark",
            color: Rgb::new(136, 21, 49),
        },
        Floss {
            number: "605",
            name: "Cranberry Very Light",
            color: Rgb::new(255, 192, 205),
        },
        Floss {
            number: "604",
            name: "Cranberry Light",
            color: Rgb::new(255, 176, 190),
        },
        Floss {
            number: "603",
            name: "Cranberry",
            color: Rgb::new(255, 164, 190),
        },
        Floss {
            number: "602",
            name: "Cranberry Medium",
            color: Rgb::new(226, 72, 116),
        },
        Floss {
            number: "601",
            name: "Cranberry Dark",
            color: Rgb::new(209, 40, 106),
        },
        Floss {
            number: "600",
            name: "Cranberry Very Dark",
            color: Rgb::new(205, 47, 99),
        },
        Floss {
            number: "3806",
            name: "Cyclamen Pink Light",
            color: Rgb::new(255, 140, 174),
        },
        Floss {
            number: "3805",
            name: "Cyclamen Pink",
            color: Rgb::new(243, 71, 139),
        },
        Floss {
            number: "3804",
            name: "Cyclamen Pink Dark",
            color: Rgb::new(224, 40, 118),
        },
        Floss {
            number: "3609",
            name: "Plum Ultra Light",
            color: Rgb::new(244, 174, 213),
        },
        Floss {
            number: "3608",
            name: "Plum Very Light",
            color: Rgb::new(234, 156, 196),
        },
        Floss {
            number: "3607",
            name: "Plum Light",
            color: Rgb::new(197, 73, 137),
        },
        Floss {
            number: "718",
            name: "Plum",
            color: Rgb::new(156, 36, 98),
        },
        Floss {
            number: "917",
            name: "Plum Medium",
            color: Rgb::new(155, 19, 89),
        },
        Floss {
            number: "915",
            name: "Plum Dark",
            color: Rgb::new(130, 0, 67),
        },
        Floss {
            number: "225",
            name: "Shell Pink Ult Vy Lt",
            color: Rgb::new(255, 223, 213),
        },
        Floss {
            number: "224",
            name: "Shell Pink Very Light",
            color: Rgb::new(235, 183, 175),
        },
        Floss {
            number: "152",
            name: "Shell Pink Med Light",
            color: Rgb::new(226, 160, 153),
        },
        Floss {
            number: "223",
            name: "Shell Pink Light",
            color: Rgb::new(204, 132, 124),
        },
        Floss {
            number: "3722",
            name: "Shell Pink Med",
            color: Rgb::new(188, 108, 100),
        },
        Floss {
            number: "3721",
            name: "Shell Pink Dark",
            color: Rgb::new(161, 75, 81),
        },
        Floss {
            number: "221",
            name: "Shell Pink Vy Dk",
            color: Rgb::new(136, 62, 67),
        },
        Floss {
            number: "778",
            name: "Antique Mauve Vy Lt",
            color: Rgb::new(223, 179, 187),
        },
        Floss {
            number: "3727",
            name: "Antique Mauve Light",
            color: Rgb::new(219, 169, 178),
        },
        Floss {
            number: "316",
            name: "Antique Mauve Med",
            color: Rgb::new(183, 115, 127),
        },
        Floss {
            number: "3726",
            name: "Antique Mauve Dark",
            color: Rgb::new(155, 91, 102),
        },
        Floss {
            number: "315",
            name: "Antique Mauve Md Dk",
            color: Rgb::new(129, 73, 82),
        },
        Floss {
            number: "3802",
            name: "Antique Mauve Vy Dk",
            color: Rgb::new(113, 65, 73),
        },
        Floss {
            number: "902",
            name: "Garnet Very Dark",
            color: Rgb::new(130, 38, 55),
        },
        Floss {
            number: "3743",
            name: "Antique Violet Vy Lt",
            color: Rgb::new(215, 203, 211),
        },
        Floss {
            number: "3042",
            name: "Antique Violet Light",
            color: Rgb::new(183, 157, 167),
        },
        Floss {
            number: "3041",
            name: "Antique Violet Medium",
            color: Rgb::new(149, 111, 124),
        },
        Floss {
            number: "3740",
            name: "Antique Violet Dark",
            color: Rgb::new(120, 87, 98),
        },
        Floss {
            number: "3836",
            name: "Grape Light",
            color: Rgb::new(186, 145, 170),
        },
        Floss {
            number: "3835",
            name: "Grape Medium",
            color: Rgb::new(148, 96, 131),
        },
        Floss {
            number: "3834",
            name: "Grape Dark",
            color: Rgb::new(114, 55, 93),
        },
        Floss {
            number: "154",
            name: "Grape Very Dark",
            color: Rgb::new(87, 36, 51),
        },
        Floss {
            number: "211",
            name: "Lavender Light",
            color: Rgb::new(227, 203, 227),
        },
        Floss {
            number: "210",
            name: "Lavender Medium",
            color: Rgb::new(195, 159, 195),
        },
        Floss {
            number: "209",
            name: "Lavender Dark",
            color: Rgb::new(163, 123, 167),
        },
        Floss {
            number: "208",
            name: "Lavender Very Dark",
            color: Rgb::new(131, 91, 139),
        },
        Floss {
            number: "3837",
            name: "Lavender Ultra Dark",
            color: Rgb::new(108, 58, 110),
        },
        Floss {
            number: "327",
            name: "Violet Dark",
            color: Rgb::new(99, 54, 102),
        },
        Floss {
            number: "153",
            name: "Violet Very Light",
            color: Rgb::new(230, 204, 217),
        },
        Floss {
            number: "554",
            name: "Violet Light",
            color: Rgb::new(219, 179, 203),
        },
        Floss {
            number: "553",
            name: "Violet",
            color: Rgb::new(163, 99, 139),
        },
        Floss {
            number: "552",
            name: "Violet  Medium",
            color: Rgb::new(128, 58, 107),
        },
        Floss {
            number: "550",
            name: "Violet Very Dark",
            color: Rgb::new(92, 24, 78),
        },
        Floss {
            number: "3747",
            name: "Blue Violet Vy Lt",
            color: Rgb::new(211, 215, 237),
        },
        Floss {
            number: "341",
            name: "Blue Violet Light",
            color: Rgb::new(183, 191, 221),
        },
        Floss {
            number: "156",
            name: "Blue Violet Med Lt",
            color: Rgb::new(163, 174, 209),
        },
        Floss {
            number: "340",
            name: "Blue Violet Medium",
            color: Rgb::new(173, 167, 199),
        },
        Floss {
            number: "155",
            name: "Blue Violet Med Dark",
            color: Rgb::new(152, 145, 182),
        },
        Floss {
            number: "3746",
            name: "Blue Violet Dark",
            color: Rgb::new(119, 107, 152),
        },
        Floss {
            number: "333",
            name: "Blue Violet Very Dark",
            color: Rgb::new(92, 84, 120),
        },
        Floss {
            number: "157",
            name: "Cornflower Blue Vy Lt",
            color: Rgb::new(187, 195, 217),
        },
        Floss {
            number: "794",
            name: "Cornflower Blue Light",
            color: Rgb::new(143, 156, 193),
        },
        Floss {
            number: "793",
            name: "Cornflower Blue Med",
            color: Rgb::new(112, 125, 162),
        },
        Floss {
            number: "3807",
            name: "Cornflower Blue",
            color: Rgb::new(96, 103, 140),
        },
        Floss {
            number: "792",
            name: "Cornflower Blue Dark",
            color: Rgb::new(85, 91, 123),
        },
        Floss {
            number: "158",
            name: "Cornflower Blu M V D",
            color: Rgb::new(76, 82, 110),
        },
        Floss {
            number: "791",
            name: "Cornflower Blue V D",
            color: Rgb::new(70, 69, 99),
        },
        Floss {
            number: "3840",
            name: "Lavender Blue Light",
            color: Rgb::new(176, 192, 218),
        },
        Floss {
            number: "3839",
            name: "Lavender Blue Med",
            color: Rgb::new(123, 142, 171),
        },
        Floss {
            number: "3838",
            name: "Lavender Blue Dark",
            color: Rgb::new(92, 114, 148),
        },
        Floss {
            number: "800",
            name: "Delft Blue Pale",
            color: Rgb::new(192, 204, 222),
        },
        Floss {
            number: "809",
            name: "Delft Blue",
            color: Rgb::new(148, 168, 198),
        },
        Floss {
            number: "799",
            name: "Delft Blue Medium",
            color: Rgb::new(116, 142, 182),
        },
        Floss {
            number: "798",
            name: "Delft Blue Dark",
            color: Rgb::new(70, 106, 142),
        },
        Floss {
            number: "797",
            name: "Royal Blue",
            color: Rgb::new(19, 71, 125),
        },
        Floss {
            number: "796",
            name: "Royal Blue Dark",
            color: Rgb::new(17, 65, 109),
        },
        Floss {
            number: "820",
            name: "Royal Blue Very Dark",
            color: Rgb::new(14, 54, 92),
        },
        Floss {
            number: "162",
            name: "Blue Ultra Very Light",
            color: Rgb::new(219, 236, 245),
        },
        Floss {
            number: "827",
            name: "Blue Very Light",
            color: Rgb::new(189, 221, 237),
        },
        Floss {
            number: "813",
            name: "Blue Light",
            color: Rgb::new(161, 194, 215),
        },
        Floss {
            number: "826",
            name: "Blue Medium",
            color: Rgb::new(107, 158, 191),
        },
        Floss {
            number: "825",
            name: "Blue Dark",
            color: Rgb::new(71, 129, 165),
        },
        Floss {
            number: "824",
            name: "Blue Very Dark",
            color: Rgb::new(57, 105, 135),
        },
        Floss {
            number: "996",
            name: "Electric Blue Medium",
            color: Rgb::new(48, 194, 236),
        },
        Floss {
            number: "3843",
            name: "Electric Blue",
            color: Rgb::new(20, 170, 208),
        },
        Floss {
            number: "995",
            name: "Electric Blue Dark",
            color: Rgb::new(38, 150, 182),
        },
        Floss {
            number: "3846",
            name: "Turquoise Bright Light",
            color: Rgb::new(6, 227, 230),
        },
        Floss {
            number: "3845",
            name: "Turquoise Bright Med",
            color: Rgb::new(4, 196, 202),
        },
        Floss {
            number: "3844",
            name: "Turquoise Bright Dark",
            color: Rgb::new(18, 174, 186),
        },
        Floss {
            number: "159",
            name: "Blue Gray Light",
            color: Rgb::new(199, 202, 215),
        },
        Floss {
            number: "160",
            name: "Blue Gray Medium",
            color: Rgb::new(153, 159, 183),
        },
        Floss {
            number: "161",
            name: "Blue Gray",
            color: Rgb::new(120, 128, 164),
        },
        Floss {
            number: "3756",
            name: "Baby Blue Ult Vy Lt",
            color: Rgb::new(238, 252, 252),
        },
        Floss {
            number: "775",
            name: "Baby Blue Very Light",
            color: Rgb::new(217, 235, 241),
        },
        Floss {
            number: "3841",
            name: "Baby Blue Pale",
            color: Rgb::new(205, 223, 237),
        },
        Floss {
            number: "3325",
            name: "Baby Blue Light",
            color: Rgb::new(184, 210, 230),
        },
        Floss {
            number: "3755",
            name: "Baby Blue",
            color: Rgb::new(147, 180, 206),
        },
        Floss {
            number: "334",
            name: "Baby Blue Medium",
            color: Rgb::new(115, 159, 193),
        },
        Floss {
            number: "322",
            name: "Baby Blue Dark",
            color: Rgb::new(90, 143, 184),
        },
        Floss {
            number: "312",
            name: "Baby Blue Very Dark",
            color: Rgb::new(53, 102, 139),
        },
        Floss {
            number: "803",
            name: "Baby Blue Ult Vy Dk",
            color: Rgb::new(44, 89, 124),
        },
        Floss {
            number: "336",
            name: "Navy Blue",
            color: Rgb::new(37, 59, 115),
        },
        Floss {
            number: "823",
            name: "Navy Blue Dark",
            color: Rgb::new(33, 48, 99),
        },
        Floss {
            number: "939",
            name: "Navy Blue Very Dark",
            color: Rgb::new(27, 40, 83),
        },
        Floss {
            number: "3753",
            name: "Antique Blue Ult Vy Lt",
            color: Rgb::new(219, 226, 233),
        },
        Floss {
            number: "3752",
            name: "Antique Blue Very Lt",
            color: Rgb::new(199, 209, 219),
        },
        Floss {
            number: "932",
            name: "Antique Blue Light",
            color: Rgb::new(162, 181, 198),
        },
        Floss {
            number: "931",
            name: "Antique Blue Medium",
            color: Rgb::new(106, 133, 158),
        },
        Floss {
            number: "930",
            name: "Antique Blue Dark",
            color: Rgb::new(69, 92, 113),
        },
        Floss {
            number: "3750",
            name: "Antique Blue Very Dk",
            color: Rgb::new(56, 76, 94),
        },
        Floss {
            number: "828",
            name: "Sky Blue Vy Lt",
            color: Rgb::new(197, 232, 237),
        },
        Floss {
            number: "3761",
            name: "Sky Blue Light",
            color: Rgb::new(172, 216, 226),
        },
        Floss {
            number: "519",
            name: "Sky Blue",
            color: Rgb::new(126, 177, 200),
        },
        Floss {
            number: "518",
            name: "Wedgewood Light",
            color: Rgb::new(79, 147, 167),
        },
        Floss {
            number: "3760",
            name: "Wedgewood Med",
            color: Rgb::new(62, 133, 162),
        },
        Floss {
            number: "517",
            name: "Wedgewood Dark",
            color: Rgb::new(59, 118, 143),
        },
        Floss {
            number: "3842",
            name: "Wedgewood Vry Dk",
            color: Rgb::new(50, 102, 124),
        },
        Floss {
            number: "311",
            name: "Wedgewood Ult VyDk",
            color: Rgb::new(28, 80, 102),
        },
        Floss {
            number: "747",
            name: "Peacock Blue Vy Lt",
            color: Rgb::new(229, 252, 253),
        },
        Floss {
            number: "3766",
            name: "Peacock Blue Light",
            color: Rgb::new(153, 207, 217),
        },
        Floss {
            number: "807",
            name: "Peacock Blue",
            color: Rgb::new(100, 171, 186),
        },
        Floss {
            number: "806",
            name: "Peacock Blue Dark",
            color: Rgb::new(61, 149, 165),
        },
        Floss {
            number: "3765",
            name: "Peacock Blue Vy Dk",
            color: Rgb::new(52, 127, 140),
        },
        Floss {
            number: "3811",
            name: "Turquoise Very Light",
            color: Rgb::new(188, 227, 230),
        },
        Floss {
            number: "598",
            name: "Turquoise Light",
            color: Rgb::new(144, 195, 204),
        },
        Floss {
            number: "597",
            name: "Turquoise",
            color: Rgb::new(91, 163, 179),
        },
        Floss {
            number: "3810",
            name: "Turquoise Dark",
            color: Rgb::new(72, 142, 154),
        },
        Floss {
            number: "3809",
            name: "Turquoise Vy Dark",
            color: Rgb::new(63, 124, 133),
        },
        Floss {
            number: "3808",
            name: "Turquoise Ult Vy Dk",
            color: Rgb::new(54, 105, 112),
        },
        Floss {
            number: "928",
            name: "Gray Green Vy Lt",
            color: Rgb::new(221, 227, 227),
        },
        Floss {
            number: "927",
            name: "Gray Green Light",
            color: Rgb::new(189, 203, 203),
        },
        Floss {
            number: "926",
            name: "Gray Green Med",
            color: Rgb::new(152, 174, 174),
        },
        Floss {
            number: "3768",
            name: "Gray Green Dark",
            color: Rgb::new(101, 127, 127),
        },
        Floss {
            number: "924",
            name: "Gray Green Vy Dark",
            color: Rgb::new(86, 106, 106),
        },
        Floss {
            number: "3849",
            name: "Teal Green Light",
            color: Rgb::new(82, 179, 164),
        },
        Floss {
            number: "3848",
            name: "Teal Green Med",
            color: Rgb::new(85, 147, 146),
        },
        Floss {
            number: "3847",
            name: "Teal Green Dark",
            color: Rgb::new(52, 125, 117),
        },
        Floss {
            number: "964",
            name: "Sea Green Light",
            color: Rgb::new(169, 226, 216),
        },
        Floss {
            number: "959",
            name: "Sea Green Med",
            color: Rgb::new(89, 199, 180),
        },
        Floss {
            number: "958",
            name: "Sea Green Dark",
            color: Rgb::new(62, 182, 161),
        },
        Floss {
            number: "3812",
            name: "Sea Green Vy Dk",
            color: Rgb::new(47, 140, 132),
        },
        Floss {
            number: "3851",
            name: "Green Bright Lt",
            color: Rgb::new(73, 179, 161),
        },
        Floss {
            number: "943",
            name: "Green Bright Md",
            color: Rgb::new(61, 147, 132),
        },
        Floss {
            number: "3850",
            name: "Green Bright Dk",
            color: Rgb::new(55, 132, 119),
        },
        Floss {
            number: "993",
            name: "Aquamarine Vy Lt",
            color: Rgb::new(144, 192, 180),
        },
        Floss {
            number: "992",
            name: "Aquamarine Lt",
            color: Rgb::new(111, 174, 159),
        },
        Floss {
            number: "3814",
            name: "Aquamarine",
            color: Rgb::new(80, 139, 125),
        },
        Floss {
            number: "991",
            name: "Aquamarine Dk",
            color: Rgb::new(71, 123, 110),
        },
        Floss {
            number: "966",
            name: "Jade Ultra Vy Lt",
            color: Rgb::new(185, 215, 192),
        },
        Floss {
            number: "564",
            name: "Jade Very Light",
            color: Rgb::new(167, 205, 175),
        },
        Floss {
            number: "563",
            name: "Jade Light",
            color: Rgb::new(143, 192, 152),
        },
        Floss {
            number: "562",
            name: "Jade Medium",
            color: Rgb::new(83, 151, 106),
        },
        Floss {
            number: "505",
            name: "Jade Green",
            color: Rgb::new(51, 131, 98),
        },
        Floss {
            number: "3817",
            name: "Celadon Green Lt",
            color: Rgb::new(153, 195, 170),
        },
        Floss {
            number: "3816",
            name: "Celadon Green",
            color: Rgb::new(101, 165, 125),
        },
        Floss {
            number: "163",
            name: "Celadon Green Md",
            color: Rgb::new(77, 131, 97),
        },
        Floss {
            number: "3815",
            name: "Celadon Green Dk",
            color: Rgb::new(71, 119, 89),
        },
        Floss {
            number: "561",
            name: "Celadon Green VD",
            color: Rgb::new(44, 106, 69),
        },
        Floss {
            number: "504",
            name: "Blue Green Vy Lt",
            color: Rgb::new(196, 222, 204),
        },
        Floss {
            number: "3813",
            name: "Blue Green Lt",
            color: Rgb::new(178, 212, 189),
        },
        Floss {
            number: "503",
            name: "Blue Green Med",
            color: Rgb::new(123, 172, 148),
        },
        Floss {
            number: "502",
            name: "Blue Green",
            color: Rgb::new(91, 144, 113),
        },
        Floss {
            number: "501",
            name: "Blue Green Dark",
            color: Rgb::new(57, 111, 82),
        },
        Floss {
            number: "500",
            name: "Blue Green Vy Dk",
            color: Rgb::new(4, 77, 51),
        },
        Floss {
            number: "955",
            name: "Nile Green Light",
            color: Rgb::new(162, 214, 173),
        },
        Floss {
            number: "954",
            name: "Nile Green",
            color: Rgb::new(136, 186, 145),
        },
        Floss {
            number: "913",
            name: "Nile Green Med",
            color: Rgb::new(109, 171, 119),
        },
        Floss {
            number: "912",
            name: "Emerald Green Lt",
            color: Rgb::new(27, 157, 107),
        },
        Floss {
            number: "911",
            name: "Emerald Green Med",
            color: Rgb::new(24, 144, 101),
        },
        Floss {
            number: "910",
            name: "Emerald Green Dark",
            color: Rgb::new(24, 126, 86),
        },
        Floss {
            number: "909",
            name: "Emerald Green Vy Dk",
            color: Rgb::new(21, 111, 73),
        },
        Floss {
            number: "3818",
            name: "Emerald Grn Ult V Dk",
            color: Rgb::new(17, 90, 59),
        },
        Floss {
            number: "369",
            name: "Pistachio Green Vy Lt",
            color: Rgb::new(215, 237, 204),
        },
        Floss {
            number: "368",
            name: "Pistachio Green Lt",
            color: Rgb::new(166, 194, 152),
        },
        Floss {
            number: "320",
            name: "Pistachio Green Med",
            color: Rgb::new(105, 136, 90),
        },
        Floss {
            number: "367",
            name: "Pistachio Green Dk",
            color: Rgb::new(97, 122, 82),
        },
        Floss {
            number: "319",
            name: "Pistachio Grn Vy Dk",
            color: Rgb::new(32, 95, 46),
        },
        Floss {
            number: "890",
            name: "Pistachio Grn Ult V D",
            color: Rgb::new(23, 73, 35),
        },
        Floss {
            number: "164",
            name: "Forest Green Lt",
            color: Rgb::new(200, 216, 184),
        },
        Floss {
            number: "989",
            name: "Forest Green ",
            color: Rgb::new(141, 166, 117),
        },
        Floss {
            number: "988",
            name: "Forest Green Med",
            color: Rgb::new(115, 139, 91),
        },
        Floss {
            number: "987",
            name: "Forest Green Dk",
            color: Rgb::new(88, 113, 65),
        },
        Floss {
            number: "986",
            name: "Forest Green Vy Dk",
            color: Rgb::new(64, 82, 48),
        },
        Floss {
            number: "772",
            name: "Yellow Green Vy Lt",
            color: Rgb::new(228, 236, 212),
        },
        Floss {
            number: "3348",
            name: "Yellow Green Lt",
            color: Rgb::new(204, 217, 177),
        },
        Floss {
            number: "3347",
            name: "Yellow Green Med",
            color: Rgb::new(113, 147, 92),
        },
        Floss {
            number: "3346",
            name: "Hunter Green",
            color: Rgb::new(64, 106, 58),
        },
        Floss {
            number: "3345",
            name: "Hunter Green Dk",
            color: Rgb::new(27, 89, 21),
        },
        Floss {
            number: "895",
            name: "Hunter Green Vy Dk",
            color: Rgb::new(27, 83, 0),
        },
        Floss {
            number: "704",
            name: "Chartreuse Bright",
            color: Rgb::new(158, 207, 52),
        },
        Floss {
            number: "703",
            name: "Chartreuse",
            color: Rgb::new(123, 181, 71),
        },
        Floss {
            number: "702",
            name: "Kelly Green",
            color: Rgb::new(71, 167, 47),
        },
        Floss {
            number: "701",
            name: "Green Light",
            color: Rgb::new(63, 143, 41),
        },
        Floss {
            number: "700",
            name: "Green Bright",
            color: Rgb::new(7, 115, 27),
        },
        Floss {
            number: "699",
            name: "Green",
            color: Rgb::new(5, 101, 23),
        },
        Floss {
            number: "907",
            name: "Parrot Green Lt",
            color: Rgb::new(199, 230, 102),
        },
        Floss {
            number: "906",
            name: "Parrot Green Md",
            color: Rgb::new(127, 179, 53),
        },
        Floss {
            number: "905",
            name: "Parrot Green Dk",
            color: Rgb::new(98, 138, 40),
        },
        Floss {
            number: "904",
            name: "Parrot Green V Dk",
            color: Rgb::new(85, 120, 34),
        },
        Floss {
            number: "472",
            name: "Avocado Grn U Lt",
            color: Rgb::new(216, 228, 152),
        },
        Floss {
            number: "471",
            name: "Avocado Grn V Lt",
            color: Rgb::new(174, 191, 121),
        },
        Floss {
            number: "470",
            name: "Avocado Grn Lt",
            color: Rgb::new(148, 171, 79),
        },
        Floss {
            number: "469",
            name: "Avocado Green",
            color: Rgb::new(114, 132, 60),
        },
        Floss {
            number: "937",
            name: "Avocado Green Md",
            color: Rgb::new(98, 113, 51),
        },
        Floss {
            number: "936",
            name: "Avocado Grn V Dk",
            color: Rgb::new(76, 88, 38),
        },
        Floss {
            number: "935",
            name: "Avocado Green Dk",
            color: Rgb::new(66, 77, 33),
        },
        Floss {
            number: "934",
            name: "Avocado Grn Black",
            color: Rgb::new(49, 57, 25),
        },
        Floss {
            number: "523",
            name: "Fern Green Lt",
            color: Rgb::new(171, 177, 151),
        },
        Floss {
            number: "3053",
            name: "Green Gray",
            color: Rgb::new(156, 164, 130),
        },
        Floss {
            number: "3052",
            name: "Green Gray Md",
            color: Rgb::new(136, 146, 104),
        },
        Floss {
            number: "3051",
            name: "Green Gray Dk",
            color: Rgb::new(95, 102, 72),
        },
        Floss {
            number: "524",
            name: "Fern Green Vy Lt",
            color: Rgb::new(196, 205, 172),
        },
        Floss {
            number: "522",
            name: "Fern Green",
            color: Rgb::new(150, 158, 126),
        },
        Floss {
            number: "520",
            name: "Fern Green Dark",
            color: Rgb::new(102, 109, 79),
        },
        Floss {
            number: "3364",
            name: "Pine Green",
            color: Rgb::new(131, 151, 95),
        },
        Floss {
            number: "3363",
            name: "Pine Green Md",
            color: Rgb::new(114, 130, 86),
        },
        Floss {
            number: "3362",
            name: "Pine Green Dk",
            color: Rgb::new(94, 107, 71),
        },
        Floss {
            number: "165",
            name: "Moss Green Vy Lt",
            color: Rgb::new(239, 244, 164),
        },
        Floss {
            number: "3819",
            name: "Moss Green Lt",
            color: Rgb::new(224, 232, 104),
        },
        Floss {
            number: "166",
            name: "Moss Green Md Lt",
            color: Rgb::new(192, 200, 64),
        },
        Floss {
            number: "581",
            name: "Moss Green",
            color: Rgb::new(167, 174, 56),
        },
        Floss {
            number: "580",
            name: "Moss Green Dk",
            color: Rgb::new(136, 141, 51),
        },
        Floss {
            number: "734",
            name: "Olive Green Lt",
            color: Rgb::new(199, 192, 119),
        },
        Floss {
            number: "733",
            name: "Olive Green Md",
            color: Rgb::new(188, 179, 76),
        },
        Floss {
            number: "732",
            name: "Olive Green",
            color: Rgb::new(148, 140, 54),
        },
        Floss {
            number: "731",
            name: "Olive Green Dk",
            color: Rgb::new(147, 139, 55),
        },
        Floss {
            number: "730",
            name: "Olive Green V Dk",
            color: Rgb::new(130, 123, 48),
        },
        Floss {
            number: "3013",
            name: "Khaki Green Lt",
            color: Rgb::new(185, 185, 130),
        },
        Floss {
            number: "3012",
            name: "Khaki Green Md",
            color: Rgb::new(166, 167, 93),
        },
        Floss {
            number: "3011",
            name: "Khaki Green Dk",
            color: Rgb::new(137, 138, 88),
        },
        Floss {
            number: "372",
            name: "Mustard Lt",
            color: Rgb::new(204, 183, 132),
        },
        Floss {
            number: "371",
            name: "Mustard",
            color: Rgb::new(191, 166, 113),
        },
        Floss {
            number: "370",
            name: "Mustard Medium",
            color: Rgb::new(184, 157, 100),
        },
        Floss {
            number: "834",
            name: "Golden Olive Vy Lt",
            color: Rgb::new(219, 190, 127),
        },
        Floss {
            number: "833",
            name: "Golden Olive Lt",
            color: Rgb::new(200, 171, 108),
        },
        Floss {
            number: "832",
            name: "Golden Olive",
            color: Rgb::new(189, 155, 81),
        },
        Floss {
            number: "831",
            name: "Golden Olive Md",
            color: Rgb::new(170, 143, 86),
        },
        Floss {
            number: "830",
            name: "Golden Olive Dk",
            color: Rgb::new(141, 120, 75),
        },
        Floss {
            number: "829",
            name: "Golden Olive Vy Dk",
            color: Rgb::new(126, 107, 66),
        },
        Floss {
            number: "613",
            name: "Drab Brown V Lt",
            color: Rgb::new(220, 196, 170),
        },
        Floss {
            number: "612",
            name: "Drab Brown Lt",
            color: Rgb::new(188, 154, 120),
        },
        Floss {
            number: "611",
            name: "Drab Brown",
            color: Rgb::new(150, 118, 86),
        },
        Floss {
            number: "610",
            name: "Drab Brown Dk",
            color: Rgb::new(121, 96, 71),
        },
        Floss {
            number: "3047",
            name: "Yellow Beige Lt",
            color: Rgb::new(231, 214, 193),
        },
        Floss {
            number: "3046",
            name: "Yellow Beige Md",
            color: Rgb::new(216, 188, 154),
        },
        Floss {
            number: "3045",
            name: "Yellow Beige Dk",
            color: Rgb::new(188, 150, 106),
        },
        Floss {
            number: "167",
            name: "Yellow Beige V Dk",
            color: Rgb::new(167, 124, 73),
        },
        Floss {
            number: "746",
            name: "Off White",
            color: Rgb::new(252, 252, 238),
        },
        Floss {
            number: "677",
            name: "Old Gold Vy Lt",
            color: Rgb::new(245, 236, 203),
        },
        Floss {
            number: "422",
            name: "Hazelnut Brown Lt",
            color: Rgb::new(198, 159, 123),
        },
        Floss {
            number: "3828",
            name: "Hazelnut Brown",
            color: Rgb::new(183, 139, 97),
        },
        Floss {
            number: "420",
            name: "Hazelnut Brown Dk",
            color: Rgb::new(160, 112, 66),
        },
        Floss {
            number: "869",
            name: "Hazelnut Brown V Dk",
            color: Rgb::new(131, 94, 57),
        },
        Floss {
            number: "728",
            name: "Topaz",
            color: Rgb::new(228, 180, 104),
        },
        Floss {
            number: "783",
            name: "Topaz Medium",
            color: Rgb::new(206, 145, 36),
        },
        Floss {
            number: "782",
            name: "Topaz Dark",
            color: Rgb::new(174, 119, 32),
        },
        Floss {
            number: "781",
            name: "Topaz Very Dark",
            color: Rgb::new(162, 109, 32),
        },
        Floss {
            number: "780",
            name: "Topaz Ultra Vy Dk",
            color: Rgb::new(148, 99, 26),
        },
        Floss {
            number: "676",
            name: "Old Gold Lt",
            color: Rgb::new(229, 206, 151),
        },
        Floss {
            number: "729",
            name: "Old Gold Medium",
            color: Rgb::new(208, 165, 62),
        },
        Floss {
            number: "680",
            name: "Old Gold Dark",
            color: Rgb::new(188, 141, 14),
        },
        Floss {
            number: "3829",
            name: "Old Gold Vy Dark",
            color: Rgb::new(169, 130, 4),
        },
        Floss {
            number: "3822",
            name: "Straw Light",
            color: Rgb::new(246, 220, 152),
        },
        Floss {
            number: "3821",
            name: "Straw",
            color: Rgb::new(243, 206, 117),
        },
        Floss {
            number: "3820",
            name: "Straw Dark",
            color: Rgb::new(223, 182, 95),
        },
        Floss {
            number: "3852",
            name: "Straw Very Dark",
            color: Rgb::new(205, 157, 55),
        },
        Floss {
            number: "445",
            name: "Lemon Light",
            color: Rgb::new(255, 251, 139),
        },
        Floss {
            number: "307",
            name: "Lemon",
            color: Rgb::new(253, 237, 84),
        },
        Floss {
            number: "973",
            name: "Canary Bright",
            color: Rgb::new(255, 227, 0),
        },
        Floss {
            number: "444",
            name: "Lemon Dark",
            color: Rgb::new(255, 214, 0),
        },
        Floss {
            number: "3078",
            name: "Golden Yellow Vy Lt",
            color: Rgb::new(253, 249, 205),
        },
        Floss {
            number: "727",
            name: "Topaz Vy Lt",
            color: Rgb::new(255, 241, 175),
        },
        Floss {
            number: "726",
            name: "Topaz Light",
            color: Rgb::new(253, 215, 85),
        },
        Floss {
            number: "725",
            name: "Topaz Med Lt",
            color: Rgb::new(255, 200, 64),
        },
        Floss {
            number: "972",
            name: "Canary Deep",
            color: Rgb::new(255, 181, 21),
        },
        Floss {
            number: "745",
            name: "Yellow Pale Light",
            color: Rgb::new(255, 233, 173),
        },
        Floss {
            number: "744",
            name: "Yellow Pale",
            color: Rgb::new(255, 231, 147),
        },
        Floss {
            number: "743",
            name: "Yellow Med",
            color: Rgb::new(254, 211, 118),
        },
        Floss {
            number: "742",
            name: "Tangerine Light",
            color: Rgb::new(255, 191, 87),
        },
        Floss {
            number: "741",
            name: "Tangerine Med",
            color: Rgb::new(255, 163, 43),
        },
        Floss {
            number: "740",
            name: "Tangerine",
            color: Rgb::new(255, 139, 0),
        },
        Floss {
            number: "970",
            name: "Pumpkin Light",
            color: Rgb::new(247, 139, 19),
        },
        Floss {
            number: "971",
            name: "Pumpkin",
            color: Rgb::new(246, 127, 0),
        },
        Floss {
            number: "947",
            name: "Burnt Orange",
            color: Rgb::new(255, 123, 77),
        },
        Floss {
            number: "946",
            name: "Burnt Orange Med",
            color: Rgb::new(235, 99, 7),
        },
        Floss {
            number: "900",
            name: "Burnt Orange Dark",
            color: Rgb::new(209, 88, 7),
        },
        Floss {
            number: "967",
            name: "Apricot Very Light",
            color: Rgb::new(255, 222, 213),
        },
        Floss {
            number: "3824",
            name: "Apricot Light",
            color: Rgb::new(254, 205, 194),
        },
        Floss {
            number: "3341",
            name: "Apricot",
            color: Rgb::new(252, 171, 152),
        },
        Floss {
            number: "3340",
            name: "Apricot Med",
            color: Rgb::new(255, 131, 111),
        },
        Floss {
            number: "608",
            name: "Burnt Orange Bright",
            color: Rgb::new(253, 93, 53),
        },
        Floss {
            number: "606",
            name: "Orange?Red Bright",
            color: Rgb::new(250, 50, 3),
        },
        Floss {
            number: "951",
            name: "Tawny Light",
            color: Rgb::new(255, 226, 207),
        },
        Floss {
            number: "3856",
            name: "Mahogany Ult Vy Lt",
            color: Rgb::new(255, 211, 181),
        },
        Floss {
            number: "722",
            name: "Orange Spice Light",
            color: Rgb::new(247, 151, 111),
        },
        Floss {
            number: "721",
            name: "Orange Spice Med",
            color: Rgb::new(242, 120, 66),
        },
        Floss {
            number: "720",
            name: "Orange Spice Dark",
            color: Rgb::new(229, 92, 31),
        },
        Floss {
            number: "3825",
            name: "Pumpkin Pale",
            color: Rgb::new(253, 189, 150),
        },
        Floss {
            number: "922",
            name: "Copper Light",
            color: Rgb::new(226, 115, 35),
        },
        Floss {
            number: "921",
            name: "Copper",
            color: Rgb::new(198, 98, 24),
        },
        Floss {
            number: "920",
            name: "Copper Med",
            color: Rgb::new(172, 84, 20),
        },
        Floss {
            number: "919",
            name: "Red?Copper",
            color: Rgb::new(166, 69, 16),
        },
        Floss {
            number: "918",
            name: "Red?Copper Dark",
            color: Rgb::new(130, 52, 10),
        },
        Floss {
            number: "3770",
            name: "Tawny Vy Light",
            color: Rgb::new(255, 238, 227),
        },
        Floss {
            number: "945",
            name: "Tawny",
            color: Rgb::new(251, 213, 187),
        },
        Floss {
            number: "402",
            name: "Mahogany Vy Lt",
            color: Rgb::new(247, 167, 119),
        },
        Floss {
            number: "3776",
            name: "Mahogany Light",
            color: Rgb::new(207, 121, 57),
        },
        Floss {
            number: "301",
            name: "Mahogany Med",
            color: Rgb::new(179, 95, 43),
        },
        Floss {
            number: "400",
            name: "Mahogany Dark",
            color: Rgb::new(143, 67, 15),
        },
        Floss {
            number: "300",
            name: "Mahogany Vy Dk",
            color: Rgb::new(111, 47, 0),
        },
        Floss {
            number: "3823",
            name: "Yellow Ultra Pale",
            color: Rgb::new(255, 253, 227),
        },
        Floss {
            number: "3855",
            name: "Autumn Gold Lt",
            color: Rgb::new(250, 211, 150),
        },
        Floss {
            number: "3854",
            name: "Autumn Gold Med",
            color: Rgb::new(242, 175, 104),
        },
        Floss {
            number: "3853",
            name: "Autumn Gold Dk",
            color: Rgb::new(242, 151, 70),
        },
        Floss {
            number: "3827",
            name: "Golden Brown Pale",
            color: Rgb::new(247, 187, 119),
        },
        Floss {
            number: "977",
            name: "Golden Brown Light",
            color: Rgb::new(220, 156, 86),
        },
        Floss {
            number: "976",
            name: "Golden Brown Med",
            color: Rgb::new(194, 129, 66),
        },
        Floss {
            number: "3826",
            name: "Golden Brown",
            color: Rgb::new(173, 114, 57),
        },
        Floss {
            number: "975",
            name: "Golden Brown Dk",
            color: Rgb::new(145, 79, 18),
        },
        Floss {
            number: "948",
            name: "Peach Very Light",
            color: Rgb::new(254, 231, 218),
        },
        Floss {
            number: "754",
            name: "Peach Light",
            color: Rgb::new(247, 203, 191),
        },
        Floss {
            number: "3771",
            name: "Terra Cotta Ult Vy Lt",
            color: Rgb::new(244, 187, 169),
        },
        Floss {
            number: "758",
            name: "Terra Cotta Vy Lt",
            color: Rgb::new(238, 170, 155),
        },
        Floss {
            number: "3778",
            name: "Terra Cotta Light",
            color: Rgb::new(217, 137, 120),
        },
        Floss {
            number: "356",
            name: "Terra Cotta Med",
            color: Rgb::new(197, 106, 91),
        },
        Floss {
            number: "3830",
            name: "Terra Cotta",
            color: Rgb::new(185, 85, 68),
        },
        Floss {
            number: "355",
            name: "Terra Cotta Dark",
            color: Rgb::new(152, 68, 54),
        },
        Floss {
            number: "3777",
            name: "Terra Cotta Vy Dk",
            color: Rgb::new(134, 48, 34),
        },
        Floss {
            number: "3779",
            name: "Rosewood Ult Vy Lt",
            color: Rgb::new(248, 202, 200),
        },
        Floss {
            number: "3859",
            name: "Rosewood Light",
            color: Rgb::new(186, 139, 124),
        },
        Floss {
            number: "3858",
            name: "Rosewood Med",
            color: Rgb::new(150, 74, 63),
        },
        Floss {
            number: "3857",
            name: "Rosewood Dark",
            color: Rgb::new(104, 37, 26),
        },
        Floss {
            number: "3774",
            name: "Desert Sand Vy Lt",
            color: Rgb::new(243, 225, 215),
        },
        Floss {
            number: "950",
            name: "Desert Sand Light",
            color: Rgb::new(238, 211, 196),
        },
        Floss {
            number: "3064",
            name: "Desert Sand",
            color: Rgb::new(196, 142, 112),
        },
        Floss {
            number: "407",
            name: "Desert Sand Med",
            color: Rgb::new(187, 129, 97),
        },
        Floss {
            number: "3773",
            name: "Desert Sand Dark",
            color: Rgb::new(182, 117, 82),
        },
        Floss {
            number: "3772",
            name: "Desert Sand Vy Dk",
            color: Rgb::new(160, 108, 80),
        },
        Floss {
            number: "632",
            name: "Desert Sand Ult Vy Dk",
            color: Rgb::new(135, 85, 57),
        },
        Floss {
            number: "453",
            name: "Shell Gray Light",
            color: Rgb::new(215, 206, 203),
        },
        Floss {
            number: "452",
            name: "Shell Gray Med",
            color: Rgb::new(192, 179, 174),
        },
        Floss {
            number: "451",
            name: "Shell Gray Dark",
            color: Rgb::new(145, 123, 115),
        },
        Floss {
            number: "3861",
            name: "Cocoa Light",
            color: Rgb::new(166, 136, 129),
        },
        Floss {
            number: "3860",
            name: "Cocoa",
            color: Rgb::new(125, 93, 87),
        },
        Floss {
            number: "779",
            name: "Cocoa Dark",
            color: Rgb::new(98, 75, 69),
        },
        Floss {
            number: "712",
            name: "Cream",
            color: Rgb::new(255, 251, 239),
        },
        Floss {
            number: "739",
            name: "Tan Ult Vy Lt",
            color: Rgb::new(248, 228, 200),
        },
        Floss {
            number: "738",
            name: "Tan Very Light",
            color: Rgb::new(236, 204, 158),
        },
        Floss {
            number: "437",
            name: "Tan Light",
            color: Rgb::new(228, 187, 142),
        },
        Floss {
            number: "436",
            name: "Tan",
            color: Rgb::new(203, 144, 81),
        },
        Floss {
            number: "435",
            name: "Brown Very Light",
            color: Rgb::new(184, 119, 72),
        },
        Floss {
            number: "434",
            name: "Brown Light",
            color: Rgb::new(152, 94, 51),
        },
        Floss {
            number: "433",
            name: "Brown Med",
            color: Rgb::new(122, 69, 31),
        },
        Floss {
            number: "801",
            name: "Coffee Brown Dk",
            color: Rgb::new(101, 57, 25),
        },
        Floss {
            number: "898",
            name: "Coffee Brown Vy Dk",
            color: Rgb::new(73, 42, 19),
        },
        Floss {
            number: "938",
            name: "Coffee Brown Ult Dk",
            color: Rgb::new(54, 31, 14),
        },
        Floss {
            number: "3371",
            name: "Black Brown",
            color: Rgb::new(30, 17, 8),
        },
        Floss {
            number: "543",
            name: "Beige Brown Ult Vy Lt",
            color: Rgb::new(242, 227, 206),
        },
        Floss {
            number: "3864",
            name: "Mocha Beige Light",
            color: Rgb::new(203, 182, 156),
        },
        Floss {
            number: "3863",
            name: "Mocha Beige Med",
            color: Rgb::new(164, 131, 92),
        },
        Floss {
            number: "3862",
            name: "Mocha Beige Dark",
            color: Rgb::new(138, 110, 78),
        },
        Floss {
            number: "3031",
            name: "Mocha Brown Vy Dk",
            color: Rgb::new(75, 60, 42),
        },
        Floss {
            number: "B5200",
            name: "Snow White",
            color: Rgb::new(255, 255, 255),
        },
        Floss {
            number: "White",
            name: "White",
            color: Rgb::new(252, 251, 248),
        },
        Floss {
            number: "3865",
            name: "Winter White",
            color: Rgb::new(249, 247, 241),
        },
        Floss {
            number: "Ecru",
            name: "Ecru",
            color: Rgb::new(240, 234, 218),
        },
        Floss {
            number: "822",
            name: "Beige Gray Light",
            color: Rgb::new(231, 226, 211),
        },
        Floss {
            number: "644",
            name: "Beige Gray Med",
            color: Rgb::new(221, 216, 203),
        },
        Floss {
            number: "642",
            name: "Beige Gray Dark",
            color: Rgb::new(164, 152, 120),
        },
        Floss {
            number: "640",
            name: "Beige Gray Vy Dk",
            color: Rgb::new(133, 123, 97),
        },
        Floss {
            number: "3787",
            name: "Brown Gray Dark",
            color: Rgb::new(98, 93, 80),
        },
        Floss {
            number: "3021",
            name: "Brown Gray Vy Dk",
            color: Rgb::new(79, 75, 65),
        },
        Floss {
            number: "3024",
            name: "Brown Gray Vy Lt",
            color: Rgb::new(235, 234, 231),
        },
        Floss {
            number: "3023",
            name: "Brown Gray Light",
            color: Rgb::new(177, 170, 151),
        },
        Floss {
            number: "3022",
            name: "Brown Gray Med",
            color: Rgb::new(142, 144, 120),
        },
        Floss {
            number: "535",
            name: "Ash Gray Vy Lt",
            color: Rgb::new(99, 100, 88),
        },
        Floss {
            number: "3033",
            name: "Mocha Brown Vy Lt",
            color: Rgb::new(227, 216, 204),
        },
        Floss {
            number: "3782",
            name: "Mocha Brown Lt",
            color: Rgb::new(210, 188, 166),
        },
        Floss {
            number: "3032",
            name: "Mocha Brown Med",
            color: Rgb::new(179, 159, 139),
        },
        Floss {
            number: "3790",
            name: "Beige Gray Ult Dk",
            color: Rgb::new(127, 106, 85),
        },
        Floss {
            number: "3781",
            name: "Mocha Brown Dk",
            color: Rgb::new(107, 87, 67),
        },
        Floss {
            number: "3866",
            name: "Mocha Brn Ult Vy Lt",
            color: Rgb::new(250, 246, 240),
        },
        Floss {
            number: "842",
            name: "Beige Brown Vy Lt",
            color: Rgb::new(209, 186, 161),
        },
        Floss {
            number: "841",
            name: "Beige Brown Lt",
            color: Rgb::new(182, 155, 126),
        },
        Floss {
            number: "840",
            name: "Beige Brown Med",
            color: Rgb::new(154, 124, 92),
        },
        Floss {
            number: "839",
            name: "Beige Brown Dk",
            color: Rgb::new(103, 85, 65),
        },
        Floss {
            number: "838",
            name: "Beige Brown Vy Dk",
            color: Rgb::new(89, 73, 55),
        },
        Floss {
            number: "3072",
            name: "Beaver Gray Vy Lt",
            color: Rgb::new(230, 232, 232),
        },
        Floss {
            number: "648",
            name: "Beaver Gray Lt",
            color: Rgb::new(188, 180, 172),
        },
        Floss {
            number: "647",
            name: "Beaver Gray Med",
            color: Rgb::new(176, 166, 156),
        },
        Floss {
            number: "646",
            name: "Beaver Gray Dk",
            color: Rgb::new(135, 125, 115),
        },
        Floss {
            number: "645",
            name: "Beaver Gray Vy Dk",
            color: Rgb::new(110, 101, 92),
        },
        Floss {
            number: "844",
            name: "Beaver Gray Ult Dk",
            color: Rgb::new(72, 72, 72),
        },
        Floss {
            number: "762",
            name: "Pearl Gray Vy Lt",
            color: Rgb::new(236, 236, 236),
        },
        Floss {
            number: "415",
            name: "Pearl Gray",
            color: Rgb::new(211, 211, 214),
        },
        Floss {
            number: "318",
            name: "Steel Gray Lt",
            color: Rgb::new(171, 171, 171),
        },
        Floss {
            number: "414",
            name: "Steel Gray Dk",
            color: Rgb::new(140, 140, 140),
        },
        Floss {
            number: "168",
            name: "Pewter Very Light",
            color: Rgb::new(209, 209, 209),
        },
        Floss {
            number: "169",
            name: "Pewter Light",
            color: Rgb::new(132, 132, 132),
        },
        Floss {
            number: "317",
            name: "Pewter Gray",
            color: Rgb::new(108, 108, 108),
        },
        Floss {
            number: "413",
            name: "Pewter Gray Dark",
            color: Rgb::new(86, 86, 86),
        },
        Floss {
            number: "3799",
            name: "Pewter Gray Vy Dk",
            color: Rgb::new(66, 66, 66),
        },
        Floss {
            number: "310",
            name: "Black",
            color: Rgb::new(0, 0, 0),
        },
    ]
}
