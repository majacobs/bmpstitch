use image::Rgba;

#[derive(Clone, Copy)]
pub struct Floss {
    pub number: &'static str,
    pub name: &'static str,
    pub color: Rgba<u8>,
}

pub fn get_dmc_floss() -> Vec<Floss> {
    vec![
        Floss {
            number: "3713",
            name: "Salmon Very Light",
            color: [255, 226, 226, 255].into(),
        },
        Floss {
            number: "761",
            name: "Salmon Light",
            color: [255, 201, 201, 255].into(),
        },
        Floss {
            number: "760",
            name: "Salmon",
            color: [245, 173, 173, 255].into(),
        },
        Floss {
            number: "3712",
            name: "Salmon Medium",
            color: [241, 135, 135, 255].into(),
        },
        Floss {
            number: "3328",
            name: "Salmon Dark",
            color: [227, 109, 109, 255].into(),
        },
        Floss {
            number: "347",
            name: "Salmon Very Dark",
            color: [191, 45, 45, 255].into(),
        },
        Floss {
            number: "353",
            name: "Peach",
            color: [254, 215, 204, 255].into(),
        },
        Floss {
            number: "352",
            name: "Coral Light",
            color: [253, 156, 151, 255].into(),
        },
        Floss {
            number: "351",
            name: "Coral",
            color: [233, 106, 103, 255].into(),
        },
        Floss {
            number: "350",
            name: "Coral Medium",
            color: [224, 72, 72, 255].into(),
        },
        Floss {
            number: "349",
            name: "Coral Dark",
            color: [210, 16, 53, 255].into(),
        },
        Floss {
            number: "817",
            name: "Coral Red Very Dark",
            color: [187, 5, 31, 255].into(),
        },
        Floss {
            number: "3708",
            name: "Melon Light",
            color: [255, 203, 213, 255].into(),
        },
        Floss {
            number: "3706",
            name: "Melon Medium",
            color: [255, 173, 188, 255].into(),
        },
        Floss {
            number: "3705",
            name: "Melon Dark",
            color: [255, 121, 146, 255].into(),
        },
        Floss {
            number: "3801",
            name: "Melon Very Dark",
            color: [231, 73, 103, 255].into(),
        },
        Floss {
            number: "666",
            name: "Bright Red",
            color: [227, 29, 66, 255].into(),
        },
        Floss {
            number: "321",
            name: "Red",
            color: [199, 43, 59, 255].into(),
        },
        Floss {
            number: "304",
            name: "Red Medium",
            color: [183, 31, 51, 255].into(),
        },
        Floss {
            number: "498",
            name: "Red Dark",
            color: [167, 19, 43, 255].into(),
        },
        Floss {
            number: "816",
            name: "Garnet",
            color: [151, 11, 35, 255].into(),
        },
        Floss {
            number: "815",
            name: "Garnet Medium",
            color: [135, 7, 31, 255].into(),
        },
        Floss {
            number: "814",
            name: "Garnet Dark",
            color: [123, 0, 27, 255].into(),
        },
        Floss {
            number: "894",
            name: "Carnation Very Light",
            color: [255, 178, 187, 255].into(),
        },
        Floss {
            number: "893",
            name: "Carnation Light",
            color: [252, 144, 162, 255].into(),
        },
        Floss {
            number: "892",
            name: "Carnation Medium",
            color: [255, 121, 140, 255].into(),
        },
        Floss {
            number: "891",
            name: "Carnation Dark",
            color: [255, 87, 115, 255].into(),
        },
        Floss {
            number: "818",
            name: "Baby Pink",
            color: [255, 223, 217, 255].into(),
        },
        Floss {
            number: "957",
            name: "Geranium Pale",
            color: [253, 181, 181, 255].into(),
        },
        Floss {
            number: "956",
            name: "Geranium",
            color: [255, 145, 145, 255].into(),
        },
        Floss {
            number: "309",
            name: "Rose Dark",
            color: [186, 74, 74, 255].into(),
        },
        Floss {
            number: "963",
            name: "Dusty Rose Ult Vy Lt",
            color: [255, 215, 215, 255].into(),
        },
        Floss {
            number: "3716",
            name: "Dusty Rose Med Vy Lt",
            color: [255, 189, 189, 255].into(),
        },
        Floss {
            number: "962",
            name: "Dusty Rose Medium",
            color: [230, 138, 138, 255].into(),
        },
        Floss {
            number: "961",
            name: "Dusty Rose Dark",
            color: [207, 115, 115, 255].into(),
        },
        Floss {
            number: "3833",
            name: "Raspberry Light",
            color: [234, 134, 153, 255].into(),
        },
        Floss {
            number: "3832",
            name: "Raspberry Medium",
            color: [219, 85, 110, 255].into(),
        },
        Floss {
            number: "3831",
            name: "Raspberry Dark",
            color: [179, 47, 72, 255].into(),
        },
        Floss {
            number: "777",
            name: "Raspberry Very Dark",
            color: [145, 53, 70, 255].into(),
        },
        Floss {
            number: "819",
            name: "Baby Pink Light",
            color: [255, 238, 235, 255].into(),
        },
        Floss {
            number: "3326",
            name: "Rose Light",
            color: [251, 173, 180, 255].into(),
        },
        Floss {
            number: "776",
            name: "Pink Medium",
            color: [252, 176, 185, 255].into(),
        },
        Floss {
            number: "899",
            name: "Rose Medium",
            color: [242, 118, 136, 255].into(),
        },
        Floss {
            number: "335",
            name: "Rose",
            color: [238, 84, 110, 255].into(),
        },
        Floss {
            number: "326",
            name: "Rose Very Dark",
            color: [179, 59, 75, 255].into(),
        },
        Floss {
            number: "151",
            name: "Dusty Rose Vry Lt",
            color: [240, 206, 212, 255].into(),
        },
        Floss {
            number: "3354",
            name: "Dusty Rose Light",
            color: [228, 166, 172, 255].into(),
        },
        Floss {
            number: "3733",
            name: "Dusty Rose",
            color: [232, 135, 155, 255].into(),
        },
        Floss {
            number: "3731",
            name: "Dusty Rose Very Dark",
            color: [218, 103, 131, 255].into(),
        },
        Floss {
            number: "3350",
            name: "Dusty Rose Ultra Dark",
            color: [188, 67, 101, 255].into(),
        },
        Floss {
            number: "150",
            name: "Dusty Rose Ult Vy Dk",
            color: [171, 2, 73, 255].into(),
        },
        Floss {
            number: "3689",
            name: "Mauve Light",
            color: [251, 191, 194, 255].into(),
        },
        Floss {
            number: "3688",
            name: "Mauve Medium",
            color: [231, 169, 172, 255].into(),
        },
        Floss {
            number: "3687",
            name: "Mauve",
            color: [201, 107, 112, 255].into(),
        },
        Floss {
            number: "3803",
            name: "Mauve Dark",
            color: [171, 51, 87, 255].into(),
        },
        Floss {
            number: "3685",
            name: "Mauve Very Dark",
            color: [136, 21, 49, 255].into(),
        },
        Floss {
            number: "605",
            name: "Cranberry Very Light",
            color: [255, 192, 205, 255].into(),
        },
        Floss {
            number: "604",
            name: "Cranberry Light",
            color: [255, 176, 190, 255].into(),
        },
        Floss {
            number: "603",
            name: "Cranberry",
            color: [255, 164, 190, 255].into(),
        },
        Floss {
            number: "602",
            name: "Cranberry Medium",
            color: [226, 72, 116, 255].into(),
        },
        Floss {
            number: "601",
            name: "Cranberry Dark",
            color: [209, 40, 106, 255].into(),
        },
        Floss {
            number: "600",
            name: "Cranberry Very Dark",
            color: [205, 47, 99, 255].into(),
        },
        Floss {
            number: "3806",
            name: "Cyclamen Pink Light",
            color: [255, 140, 174, 255].into(),
        },
        Floss {
            number: "3805",
            name: "Cyclamen Pink",
            color: [243, 71, 139, 255].into(),
        },
        Floss {
            number: "3804",
            name: "Cyclamen Pink Dark",
            color: [224, 40, 118, 255].into(),
        },
        Floss {
            number: "3609",
            name: "Plum Ultra Light",
            color: [244, 174, 213, 255].into(),
        },
        Floss {
            number: "3608",
            name: "Plum Very Light",
            color: [234, 156, 196, 255].into(),
        },
        Floss {
            number: "3607",
            name: "Plum Light",
            color: [197, 73, 137, 255].into(),
        },
        Floss {
            number: "718",
            name: "Plum",
            color: [156, 36, 98, 255].into(),
        },
        Floss {
            number: "917",
            name: "Plum Medium",
            color: [155, 19, 89, 255].into(),
        },
        Floss {
            number: "915",
            name: "Plum Dark",
            color: [130, 0, 67, 255].into(),
        },
        Floss {
            number: "225",
            name: "Shell Pink Ult Vy Lt",
            color: [255, 223, 213, 255].into(),
        },
        Floss {
            number: "224",
            name: "Shell Pink Very Light",
            color: [235, 183, 175, 255].into(),
        },
        Floss {
            number: "152",
            name: "Shell Pink Med Light",
            color: [226, 160, 153, 255].into(),
        },
        Floss {
            number: "223",
            name: "Shell Pink Light",
            color: [204, 132, 124, 255].into(),
        },
        Floss {
            number: "3722",
            name: "Shell Pink Med",
            color: [188, 108, 100, 255].into(),
        },
        Floss {
            number: "3721",
            name: "Shell Pink Dark",
            color: [161, 75, 81, 255].into(),
        },
        Floss {
            number: "221",
            name: "Shell Pink Vy Dk",
            color: [136, 62, 67, 255].into(),
        },
        Floss {
            number: "778",
            name: "Antique Mauve Vy Lt",
            color: [223, 179, 187, 255].into(),
        },
        Floss {
            number: "3727",
            name: "Antique Mauve Light",
            color: [219, 169, 178, 255].into(),
        },
        Floss {
            number: "316",
            name: "Antique Mauve Med",
            color: [183, 115, 127, 255].into(),
        },
        Floss {
            number: "3726",
            name: "Antique Mauve Dark",
            color: [155, 91, 102, 255].into(),
        },
        Floss {
            number: "315",
            name: "Antique Mauve Md Dk",
            color: [129, 73, 82, 255].into(),
        },
        Floss {
            number: "3802",
            name: "Antique Mauve Vy Dk",
            color: [113, 65, 73, 255].into(),
        },
        Floss {
            number: "902",
            name: "Garnet Very Dark",
            color: [130, 38, 55, 255].into(),
        },
        Floss {
            number: "3743",
            name: "Antique Violet Vy Lt",
            color: [215, 203, 211, 255].into(),
        },
        Floss {
            number: "3042",
            name: "Antique Violet Light",
            color: [183, 157, 167, 255].into(),
        },
        Floss {
            number: "3041",
            name: "Antique Violet Medium",
            color: [149, 111, 124, 255].into(),
        },
        Floss {
            number: "3740",
            name: "Antique Violet Dark",
            color: [120, 87, 98, 255].into(),
        },
        Floss {
            number: "3836",
            name: "Grape Light",
            color: [186, 145, 170, 255].into(),
        },
        Floss {
            number: "3835",
            name: "Grape Medium",
            color: [148, 96, 131, 255].into(),
        },
        Floss {
            number: "3834",
            name: "Grape Dark",
            color: [114, 55, 93, 255].into(),
        },
        Floss {
            number: "154",
            name: "Grape Very Dark",
            color: [87, 36, 51, 255].into(),
        },
        Floss {
            number: "211",
            name: "Lavender Light",
            color: [227, 203, 227, 255].into(),
        },
        Floss {
            number: "210",
            name: "Lavender Medium",
            color: [195, 159, 195, 255].into(),
        },
        Floss {
            number: "209",
            name: "Lavender Dark",
            color: [163, 123, 167, 255].into(),
        },
        Floss {
            number: "208",
            name: "Lavender Very Dark",
            color: [131, 91, 139, 255].into(),
        },
        Floss {
            number: "3837",
            name: "Lavender Ultra Dark",
            color: [108, 58, 110, 255].into(),
        },
        Floss {
            number: "327",
            name: "Violet Dark",
            color: [99, 54, 102, 255].into(),
        },
        Floss {
            number: "153",
            name: "Violet Very Light",
            color: [230, 204, 217, 255].into(),
        },
        Floss {
            number: "554",
            name: "Violet Light",
            color: [219, 179, 203, 255].into(),
        },
        Floss {
            number: "553",
            name: "Violet",
            color: [163, 99, 139, 255].into(),
        },
        Floss {
            number: "552",
            name: "Violet  Medium",
            color: [128, 58, 107, 255].into(),
        },
        Floss {
            number: "550",
            name: "Violet Very Dark",
            color: [92, 24, 78, 255].into(),
        },
        Floss {
            number: "3747",
            name: "Blue Violet Vy Lt",
            color: [211, 215, 237, 255].into(),
        },
        Floss {
            number: "341",
            name: "Blue Violet Light",
            color: [183, 191, 221, 255].into(),
        },
        Floss {
            number: "156",
            name: "Blue Violet Med Lt",
            color: [163, 174, 209, 255].into(),
        },
        Floss {
            number: "340",
            name: "Blue Violet Medium",
            color: [173, 167, 199, 255].into(),
        },
        Floss {
            number: "155",
            name: "Blue Violet Med Dark",
            color: [152, 145, 182, 255].into(),
        },
        Floss {
            number: "3746",
            name: "Blue Violet Dark",
            color: [119, 107, 152, 255].into(),
        },
        Floss {
            number: "333",
            name: "Blue Violet Very Dark",
            color: [92, 84, 120, 255].into(),
        },
        Floss {
            number: "157",
            name: "Cornflower Blue Vy Lt",
            color: [187, 195, 217, 255].into(),
        },
        Floss {
            number: "794",
            name: "Cornflower Blue Light",
            color: [143, 156, 193, 255].into(),
        },
        Floss {
            number: "793",
            name: "Cornflower Blue Med",
            color: [112, 125, 162, 255].into(),
        },
        Floss {
            number: "3807",
            name: "Cornflower Blue",
            color: [96, 103, 140, 255].into(),
        },
        Floss {
            number: "792",
            name: "Cornflower Blue Dark",
            color: [85, 91, 123, 255].into(),
        },
        Floss {
            number: "158",
            name: "Cornflower Blu M V D",
            color: [76, 82, 110, 255].into(),
        },
        Floss {
            number: "791",
            name: "Cornflower Blue V D",
            color: [70, 69, 99, 255].into(),
        },
        Floss {
            number: "3840",
            name: "Lavender Blue Light",
            color: [176, 192, 218, 255].into(),
        },
        Floss {
            number: "3839",
            name: "Lavender Blue Med",
            color: [123, 142, 171, 255].into(),
        },
        Floss {
            number: "3838",
            name: "Lavender Blue Dark",
            color: [92, 114, 148, 255].into(),
        },
        Floss {
            number: "800",
            name: "Delft Blue Pale",
            color: [192, 204, 222, 255].into(),
        },
        Floss {
            number: "809",
            name: "Delft Blue",
            color: [148, 168, 198, 255].into(),
        },
        Floss {
            number: "799",
            name: "Delft Blue Medium",
            color: [116, 142, 182, 255].into(),
        },
        Floss {
            number: "798",
            name: "Delft Blue Dark",
            color: [70, 106, 142, 255].into(),
        },
        Floss {
            number: "797",
            name: "Royal Blue",
            color: [19, 71, 125, 255].into(),
        },
        Floss {
            number: "796",
            name: "Royal Blue Dark",
            color: [17, 65, 109, 255].into(),
        },
        Floss {
            number: "820",
            name: "Royal Blue Very Dark",
            color: [14, 54, 92, 255].into(),
        },
        Floss {
            number: "162",
            name: "Blue Ultra Very Light",
            color: [219, 236, 245, 255].into(),
        },
        Floss {
            number: "827",
            name: "Blue Very Light",
            color: [189, 221, 237, 255].into(),
        },
        Floss {
            number: "813",
            name: "Blue Light",
            color: [161, 194, 215, 255].into(),
        },
        Floss {
            number: "826",
            name: "Blue Medium",
            color: [107, 158, 191, 255].into(),
        },
        Floss {
            number: "825",
            name: "Blue Dark",
            color: [71, 129, 165, 255].into(),
        },
        Floss {
            number: "824",
            name: "Blue Very Dark",
            color: [57, 105, 135, 255].into(),
        },
        Floss {
            number: "996",
            name: "Electric Blue Medium",
            color: [48, 194, 236, 255].into(),
        },
        Floss {
            number: "3843",
            name: "Electric Blue",
            color: [20, 170, 208, 255].into(),
        },
        Floss {
            number: "995",
            name: "Electric Blue Dark",
            color: [38, 150, 182, 255].into(),
        },
        Floss {
            number: "3846",
            name: "Turquoise Bright Light",
            color: [6, 227, 230, 255].into(),
        },
        Floss {
            number: "3845",
            name: "Turquoise Bright Med",
            color: [4, 196, 202, 255].into(),
        },
        Floss {
            number: "3844",
            name: "Turquoise Bright Dark",
            color: [18, 174, 186, 255].into(),
        },
        Floss {
            number: "159",
            name: "Blue Gray Light",
            color: [199, 202, 215, 255].into(),
        },
        Floss {
            number: "160",
            name: "Blue Gray Medium",
            color: [153, 159, 183, 255].into(),
        },
        Floss {
            number: "161",
            name: "Blue Gray",
            color: [120, 128, 164, 255].into(),
        },
        Floss {
            number: "3756",
            name: "Baby Blue Ult Vy Lt",
            color: [238, 252, 252, 255].into(),
        },
        Floss {
            number: "775",
            name: "Baby Blue Very Light",
            color: [217, 235, 241, 255].into(),
        },
        Floss {
            number: "3841",
            name: "Baby Blue Pale",
            color: [205, 223, 237, 255].into(),
        },
        Floss {
            number: "3325",
            name: "Baby Blue Light",
            color: [184, 210, 230, 255].into(),
        },
        Floss {
            number: "3755",
            name: "Baby Blue",
            color: [147, 180, 206, 255].into(),
        },
        Floss {
            number: "334",
            name: "Baby Blue Medium",
            color: [115, 159, 193, 255].into(),
        },
        Floss {
            number: "322",
            name: "Baby Blue Dark",
            color: [90, 143, 184, 255].into(),
        },
        Floss {
            number: "312",
            name: "Baby Blue Very Dark",
            color: [53, 102, 139, 255].into(),
        },
        Floss {
            number: "803",
            name: "Baby Blue Ult Vy Dk",
            color: [44, 89, 124, 255].into(),
        },
        Floss {
            number: "336",
            name: "Navy Blue",
            color: [37, 59, 115, 255].into(),
        },
        Floss {
            number: "823",
            name: "Navy Blue Dark",
            color: [33, 48, 99, 255].into(),
        },
        Floss {
            number: "939",
            name: "Navy Blue Very Dark",
            color: [27, 40, 83, 255].into(),
        },
        Floss {
            number: "3753",
            name: "Antique Blue Ult Vy Lt",
            color: [219, 226, 233, 255].into(),
        },
        Floss {
            number: "3752",
            name: "Antique Blue Very Lt",
            color: [199, 209, 219, 255].into(),
        },
        Floss {
            number: "932",
            name: "Antique Blue Light",
            color: [162, 181, 198, 255].into(),
        },
        Floss {
            number: "931",
            name: "Antique Blue Medium",
            color: [106, 133, 158, 255].into(),
        },
        Floss {
            number: "930",
            name: "Antique Blue Dark",
            color: [69, 92, 113, 255].into(),
        },
        Floss {
            number: "3750",
            name: "Antique Blue Very Dk",
            color: [56, 76, 94, 255].into(),
        },
        Floss {
            number: "828",
            name: "Sky Blue Vy Lt",
            color: [197, 232, 237, 255].into(),
        },
        Floss {
            number: "3761",
            name: "Sky Blue Light",
            color: [172, 216, 226, 255].into(),
        },
        Floss {
            number: "519",
            name: "Sky Blue",
            color: [126, 177, 200, 255].into(),
        },
        Floss {
            number: "518",
            name: "Wedgewood Light",
            color: [79, 147, 167, 255].into(),
        },
        Floss {
            number: "3760",
            name: "Wedgewood Med",
            color: [62, 133, 162, 255].into(),
        },
        Floss {
            number: "517",
            name: "Wedgewood Dark",
            color: [59, 118, 143, 255].into(),
        },
        Floss {
            number: "3842",
            name: "Wedgewood Vry Dk",
            color: [50, 102, 124, 255].into(),
        },
        Floss {
            number: "311",
            name: "Wedgewood Ult VyDk",
            color: [28, 80, 102, 255].into(),
        },
        Floss {
            number: "747",
            name: "Peacock Blue Vy Lt",
            color: [229, 252, 253, 255].into(),
        },
        Floss {
            number: "3766",
            name: "Peacock Blue Light",
            color: [153, 207, 217, 255].into(),
        },
        Floss {
            number: "807",
            name: "Peacock Blue",
            color: [100, 171, 186, 255].into(),
        },
        Floss {
            number: "806",
            name: "Peacock Blue Dark",
            color: [61, 149, 165, 255].into(),
        },
        Floss {
            number: "3765",
            name: "Peacock Blue Vy Dk",
            color: [52, 127, 140, 255].into(),
        },
        Floss {
            number: "3811",
            name: "Turquoise Very Light",
            color: [188, 227, 230, 255].into(),
        },
        Floss {
            number: "598",
            name: "Turquoise Light",
            color: [144, 195, 204, 255].into(),
        },
        Floss {
            number: "597",
            name: "Turquoise",
            color: [91, 163, 179, 255].into(),
        },
        Floss {
            number: "3810",
            name: "Turquoise Dark",
            color: [72, 142, 154, 255].into(),
        },
        Floss {
            number: "3809",
            name: "Turquoise Vy Dark",
            color: [63, 124, 133, 255].into(),
        },
        Floss {
            number: "3808",
            name: "Turquoise Ult Vy Dk",
            color: [54, 105, 112, 255].into(),
        },
        Floss {
            number: "928",
            name: "Gray Green Vy Lt",
            color: [221, 227, 227, 255].into(),
        },
        Floss {
            number: "927",
            name: "Gray Green Light",
            color: [189, 203, 203, 255].into(),
        },
        Floss {
            number: "926",
            name: "Gray Green Med",
            color: [152, 174, 174, 255].into(),
        },
        Floss {
            number: "3768",
            name: "Gray Green Dark",
            color: [101, 127, 127, 255].into(),
        },
        Floss {
            number: "924",
            name: "Gray Green Vy Dark",
            color: [86, 106, 106, 255].into(),
        },
        Floss {
            number: "3849",
            name: "Teal Green Light",
            color: [82, 179, 164, 255].into(),
        },
        Floss {
            number: "3848",
            name: "Teal Green Med",
            color: [85, 147, 146, 255].into(),
        },
        Floss {
            number: "3847",
            name: "Teal Green Dark",
            color: [52, 125, 117, 255].into(),
        },
        Floss {
            number: "964",
            name: "Sea Green Light",
            color: [169, 226, 216, 255].into(),
        },
        Floss {
            number: "959",
            name: "Sea Green Med",
            color: [89, 199, 180, 255].into(),
        },
        Floss {
            number: "958",
            name: "Sea Green Dark",
            color: [62, 182, 161, 255].into(),
        },
        Floss {
            number: "3812",
            name: "Sea Green Vy Dk",
            color: [47, 140, 132, 255].into(),
        },
        Floss {
            number: "3851",
            name: "Green Bright Lt",
            color: [73, 179, 161, 255].into(),
        },
        Floss {
            number: "943",
            name: "Green Bright Md",
            color: [61, 147, 132, 255].into(),
        },
        Floss {
            number: "3850",
            name: "Green Bright Dk",
            color: [55, 132, 119, 255].into(),
        },
        Floss {
            number: "993",
            name: "Aquamarine Vy Lt",
            color: [144, 192, 180, 255].into(),
        },
        Floss {
            number: "992",
            name: "Aquamarine Lt",
            color: [111, 174, 159, 255].into(),
        },
        Floss {
            number: "3814",
            name: "Aquamarine",
            color: [80, 139, 125, 255].into(),
        },
        Floss {
            number: "991",
            name: "Aquamarine Dk",
            color: [71, 123, 110, 255].into(),
        },
        Floss {
            number: "966",
            name: "Jade Ultra Vy Lt",
            color: [185, 215, 192, 255].into(),
        },
        Floss {
            number: "564",
            name: "Jade Very Light",
            color: [167, 205, 175, 255].into(),
        },
        Floss {
            number: "563",
            name: "Jade Light",
            color: [143, 192, 152, 255].into(),
        },
        Floss {
            number: "562",
            name: "Jade Medium",
            color: [83, 151, 106, 255].into(),
        },
        Floss {
            number: "505",
            name: "Jade Green",
            color: [51, 131, 98, 255].into(),
        },
        Floss {
            number: "3817",
            name: "Celadon Green Lt",
            color: [153, 195, 170, 255].into(),
        },
        Floss {
            number: "3816",
            name: "Celadon Green",
            color: [101, 165, 125, 255].into(),
        },
        Floss {
            number: "163",
            name: "Celadon Green Md",
            color: [77, 131, 97, 255].into(),
        },
        Floss {
            number: "3815",
            name: "Celadon Green Dk",
            color: [71, 119, 89, 255].into(),
        },
        Floss {
            number: "561",
            name: "Celadon Green VD",
            color: [44, 106, 69, 255].into(),
        },
        Floss {
            number: "504",
            name: "Blue Green Vy Lt",
            color: [196, 222, 204, 255].into(),
        },
        Floss {
            number: "3813",
            name: "Blue Green Lt",
            color: [178, 212, 189, 255].into(),
        },
        Floss {
            number: "503",
            name: "Blue Green Med",
            color: [123, 172, 148, 255].into(),
        },
        Floss {
            number: "502",
            name: "Blue Green",
            color: [91, 144, 113, 255].into(),
        },
        Floss {
            number: "501",
            name: "Blue Green Dark",
            color: [57, 111, 82, 255].into(),
        },
        Floss {
            number: "500",
            name: "Blue Green Vy Dk",
            color: [4, 77, 51, 255].into(),
        },
        Floss {
            number: "955",
            name: "Nile Green Light",
            color: [162, 214, 173, 255].into(),
        },
        Floss {
            number: "954",
            name: "Nile Green",
            color: [136, 186, 145, 255].into(),
        },
        Floss {
            number: "913",
            name: "Nile Green Med",
            color: [109, 171, 119, 255].into(),
        },
        Floss {
            number: "912",
            name: "Emerald Green Lt",
            color: [27, 157, 107, 255].into(),
        },
        Floss {
            number: "911",
            name: "Emerald Green Med",
            color: [24, 144, 101, 255].into(),
        },
        Floss {
            number: "910",
            name: "Emerald Green Dark",
            color: [24, 126, 86, 255].into(),
        },
        Floss {
            number: "909",
            name: "Emerald Green Vy Dk",
            color: [21, 111, 73, 255].into(),
        },
        Floss {
            number: "3818",
            name: "Emerald Grn Ult V Dk",
            color: [17, 90, 59, 255].into(),
        },
        Floss {
            number: "369",
            name: "Pistachio Green Vy Lt",
            color: [215, 237, 204, 255].into(),
        },
        Floss {
            number: "368",
            name: "Pistachio Green Lt",
            color: [166, 194, 152, 255].into(),
        },
        Floss {
            number: "320",
            name: "Pistachio Green Med",
            color: [105, 136, 90, 255].into(),
        },
        Floss {
            number: "367",
            name: "Pistachio Green Dk",
            color: [97, 122, 82, 255].into(),
        },
        Floss {
            number: "319",
            name: "Pistachio Grn Vy Dk",
            color: [32, 95, 46, 255].into(),
        },
        Floss {
            number: "890",
            name: "Pistachio Grn Ult V D",
            color: [23, 73, 35, 255].into(),
        },
        Floss {
            number: "164",
            name: "Forest Green Lt",
            color: [200, 216, 184, 255].into(),
        },
        Floss {
            number: "989",
            name: "Forest Green ",
            color: [141, 166, 117, 255].into(),
        },
        Floss {
            number: "988",
            name: "Forest Green Med",
            color: [115, 139, 91, 255].into(),
        },
        Floss {
            number: "987",
            name: "Forest Green Dk",
            color: [88, 113, 65, 255].into(),
        },
        Floss {
            number: "986",
            name: "Forest Green Vy Dk",
            color: [64, 82, 48, 255].into(),
        },
        Floss {
            number: "772",
            name: "Yellow Green Vy Lt",
            color: [228, 236, 212, 255].into(),
        },
        Floss {
            number: "3348",
            name: "Yellow Green Lt",
            color: [204, 217, 177, 255].into(),
        },
        Floss {
            number: "3347",
            name: "Yellow Green Med",
            color: [113, 147, 92, 255].into(),
        },
        Floss {
            number: "3346",
            name: "Hunter Green",
            color: [64, 106, 58, 255].into(),
        },
        Floss {
            number: "3345",
            name: "Hunter Green Dk",
            color: [27, 89, 21, 255].into(),
        },
        Floss {
            number: "895",
            name: "Hunter Green Vy Dk",
            color: [27, 83, 0, 255].into(),
        },
        Floss {
            number: "704",
            name: "Chartreuse Bright",
            color: [158, 207, 52, 255].into(),
        },
        Floss {
            number: "703",
            name: "Chartreuse",
            color: [123, 181, 71, 255].into(),
        },
        Floss {
            number: "702",
            name: "Kelly Green",
            color: [71, 167, 47, 255].into(),
        },
        Floss {
            number: "701",
            name: "Green Light",
            color: [63, 143, 41, 255].into(),
        },
        Floss {
            number: "700",
            name: "Green Bright",
            color: [7, 115, 27, 255].into(),
        },
        Floss {
            number: "699",
            name: "Green",
            color: [5, 101, 23, 255].into(),
        },
        Floss {
            number: "907",
            name: "Parrot Green Lt",
            color: [199, 230, 102, 255].into(),
        },
        Floss {
            number: "906",
            name: "Parrot Green Md",
            color: [127, 179, 53, 255].into(),
        },
        Floss {
            number: "905",
            name: "Parrot Green Dk",
            color: [98, 138, 40, 255].into(),
        },
        Floss {
            number: "904",
            name: "Parrot Green V Dk",
            color: [85, 120, 34, 255].into(),
        },
        Floss {
            number: "472",
            name: "Avocado Grn U Lt",
            color: [216, 228, 152, 255].into(),
        },
        Floss {
            number: "471",
            name: "Avocado Grn V Lt",
            color: [174, 191, 121, 255].into(),
        },
        Floss {
            number: "470",
            name: "Avocado Grn Lt",
            color: [148, 171, 79, 255].into(),
        },
        Floss {
            number: "469",
            name: "Avocado Green",
            color: [114, 132, 60, 255].into(),
        },
        Floss {
            number: "937",
            name: "Avocado Green Md",
            color: [98, 113, 51, 255].into(),
        },
        Floss {
            number: "936",
            name: "Avocado Grn V Dk",
            color: [76, 88, 38, 255].into(),
        },
        Floss {
            number: "935",
            name: "Avocado Green Dk",
            color: [66, 77, 33, 255].into(),
        },
        Floss {
            number: "934",
            name: "Avocado Grn Black",
            color: [49, 57, 25, 255].into(),
        },
        Floss {
            number: "523",
            name: "Fern Green Lt",
            color: [171, 177, 151, 255].into(),
        },
        Floss {
            number: "3053",
            name: "Green Gray",
            color: [156, 164, 130, 255].into(),
        },
        Floss {
            number: "3052",
            name: "Green Gray Md",
            color: [136, 146, 104, 255].into(),
        },
        Floss {
            number: "3051",
            name: "Green Gray Dk",
            color: [95, 102, 72, 255].into(),
        },
        Floss {
            number: "524",
            name: "Fern Green Vy Lt",
            color: [196, 205, 172, 255].into(),
        },
        Floss {
            number: "522",
            name: "Fern Green",
            color: [150, 158, 126, 255].into(),
        },
        Floss {
            number: "520",
            name: "Fern Green Dark",
            color: [102, 109, 79, 255].into(),
        },
        Floss {
            number: "3364",
            name: "Pine Green",
            color: [131, 151, 95, 255].into(),
        },
        Floss {
            number: "3363",
            name: "Pine Green Md",
            color: [114, 130, 86, 255].into(),
        },
        Floss {
            number: "3362",
            name: "Pine Green Dk",
            color: [94, 107, 71, 255].into(),
        },
        Floss {
            number: "165",
            name: "Moss Green Vy Lt",
            color: [239, 244, 164, 255].into(),
        },
        Floss {
            number: "3819",
            name: "Moss Green Lt",
            color: [224, 232, 104, 255].into(),
        },
        Floss {
            number: "166",
            name: "Moss Green Md Lt",
            color: [192, 200, 64, 255].into(),
        },
        Floss {
            number: "581",
            name: "Moss Green",
            color: [167, 174, 56, 255].into(),
        },
        Floss {
            number: "580",
            name: "Moss Green Dk",
            color: [136, 141, 51, 255].into(),
        },
        Floss {
            number: "734",
            name: "Olive Green Lt",
            color: [199, 192, 119, 255].into(),
        },
        Floss {
            number: "733",
            name: "Olive Green Md",
            color: [188, 179, 76, 255].into(),
        },
        Floss {
            number: "732",
            name: "Olive Green",
            color: [148, 140, 54, 255].into(),
        },
        Floss {
            number: "731",
            name: "Olive Green Dk",
            color: [147, 139, 55, 255].into(),
        },
        Floss {
            number: "730",
            name: "Olive Green V Dk",
            color: [130, 123, 48, 255].into(),
        },
        Floss {
            number: "3013",
            name: "Khaki Green Lt",
            color: [185, 185, 130, 255].into(),
        },
        Floss {
            number: "3012",
            name: "Khaki Green Md",
            color: [166, 167, 93, 255].into(),
        },
        Floss {
            number: "3011",
            name: "Khaki Green Dk",
            color: [137, 138, 88, 255].into(),
        },
        Floss {
            number: "372",
            name: "Mustard Lt",
            color: [204, 183, 132, 255].into(),
        },
        Floss {
            number: "371",
            name: "Mustard",
            color: [191, 166, 113, 255].into(),
        },
        Floss {
            number: "370",
            name: "Mustard Medium",
            color: [184, 157, 100, 255].into(),
        },
        Floss {
            number: "834",
            name: "Golden Olive Vy Lt",
            color: [219, 190, 127, 255].into(),
        },
        Floss {
            number: "833",
            name: "Golden Olive Lt",
            color: [200, 171, 108, 255].into(),
        },
        Floss {
            number: "832",
            name: "Golden Olive",
            color: [189, 155, 81, 255].into(),
        },
        Floss {
            number: "831",
            name: "Golden Olive Md",
            color: [170, 143, 86, 255].into(),
        },
        Floss {
            number: "830",
            name: "Golden Olive Dk",
            color: [141, 120, 75, 255].into(),
        },
        Floss {
            number: "829",
            name: "Golden Olive Vy Dk",
            color: [126, 107, 66, 255].into(),
        },
        Floss {
            number: "613",
            name: "Drab Brown V Lt",
            color: [220, 196, 170, 255].into(),
        },
        Floss {
            number: "612",
            name: "Drab Brown Lt",
            color: [188, 154, 120, 255].into(),
        },
        Floss {
            number: "611",
            name: "Drab Brown",
            color: [150, 118, 86, 255].into(),
        },
        Floss {
            number: "610",
            name: "Drab Brown Dk",
            color: [121, 96, 71, 255].into(),
        },
        Floss {
            number: "3047",
            name: "Yellow Beige Lt",
            color: [231, 214, 193, 255].into(),
        },
        Floss {
            number: "3046",
            name: "Yellow Beige Md",
            color: [216, 188, 154, 255].into(),
        },
        Floss {
            number: "3045",
            name: "Yellow Beige Dk",
            color: [188, 150, 106, 255].into(),
        },
        Floss {
            number: "167",
            name: "Yellow Beige V Dk",
            color: [167, 124, 73, 255].into(),
        },
        Floss {
            number: "746",
            name: "Off White",
            color: [252, 252, 238, 255].into(),
        },
        Floss {
            number: "677",
            name: "Old Gold Vy Lt",
            color: [245, 236, 203, 255].into(),
        },
        Floss {
            number: "422",
            name: "Hazelnut Brown Lt",
            color: [198, 159, 123, 255].into(),
        },
        Floss {
            number: "3828",
            name: "Hazelnut Brown",
            color: [183, 139, 97, 255].into(),
        },
        Floss {
            number: "420",
            name: "Hazelnut Brown Dk",
            color: [160, 112, 66, 255].into(),
        },
        Floss {
            number: "869",
            name: "Hazelnut Brown V Dk",
            color: [131, 94, 57, 255].into(),
        },
        Floss {
            number: "728",
            name: "Topaz",
            color: [228, 180, 104, 255].into(),
        },
        Floss {
            number: "783",
            name: "Topaz Medium",
            color: [206, 145, 36, 255].into(),
        },
        Floss {
            number: "782",
            name: "Topaz Dark",
            color: [174, 119, 32, 255].into(),
        },
        Floss {
            number: "781",
            name: "Topaz Very Dark",
            color: [162, 109, 32, 255].into(),
        },
        Floss {
            number: "780",
            name: "Topaz Ultra Vy Dk",
            color: [148, 99, 26, 255].into(),
        },
        Floss {
            number: "676",
            name: "Old Gold Lt",
            color: [229, 206, 151, 255].into(),
        },
        Floss {
            number: "729",
            name: "Old Gold Medium",
            color: [208, 165, 62, 255].into(),
        },
        Floss {
            number: "680",
            name: "Old Gold Dark",
            color: [188, 141, 14, 255].into(),
        },
        Floss {
            number: "3829",
            name: "Old Gold Vy Dark",
            color: [169, 130, 4, 255].into(),
        },
        Floss {
            number: "3822",
            name: "Straw Light",
            color: [246, 220, 152, 255].into(),
        },
        Floss {
            number: "3821",
            name: "Straw",
            color: [243, 206, 117, 255].into(),
        },
        Floss {
            number: "3820",
            name: "Straw Dark",
            color: [223, 182, 95, 255].into(),
        },
        Floss {
            number: "3852",
            name: "Straw Very Dark",
            color: [205, 157, 55, 255].into(),
        },
        Floss {
            number: "445",
            name: "Lemon Light",
            color: [255, 251, 139, 255].into(),
        },
        Floss {
            number: "307",
            name: "Lemon",
            color: [253, 237, 84, 255].into(),
        },
        Floss {
            number: "973",
            name: "Canary Bright",
            color: [255, 227, 0, 255].into(),
        },
        Floss {
            number: "444",
            name: "Lemon Dark",
            color: [255, 214, 0, 255].into(),
        },
        Floss {
            number: "3078",
            name: "Golden Yellow Vy Lt",
            color: [253, 249, 205, 255].into(),
        },
        Floss {
            number: "727",
            name: "Topaz Vy Lt",
            color: [255, 241, 175, 255].into(),
        },
        Floss {
            number: "726",
            name: "Topaz Light",
            color: [253, 215, 85, 255].into(),
        },
        Floss {
            number: "725",
            name: "Topaz Med Lt",
            color: [255, 200, 64, 255].into(),
        },
        Floss {
            number: "972",
            name: "Canary Deep",
            color: [255, 181, 21, 255].into(),
        },
        Floss {
            number: "745",
            name: "Yellow Pale Light",
            color: [255, 233, 173, 255].into(),
        },
        Floss {
            number: "744",
            name: "Yellow Pale",
            color: [255, 231, 147, 255].into(),
        },
        Floss {
            number: "743",
            name: "Yellow Med",
            color: [254, 211, 118, 255].into(),
        },
        Floss {
            number: "742",
            name: "Tangerine Light",
            color: [255, 191, 87, 255].into(),
        },
        Floss {
            number: "741",
            name: "Tangerine Med",
            color: [255, 163, 43, 255].into(),
        },
        Floss {
            number: "740",
            name: "Tangerine",
            color: [255, 139, 0, 255].into(),
        },
        Floss {
            number: "970",
            name: "Pumpkin Light",
            color: [247, 139, 19, 255].into(),
        },
        Floss {
            number: "971",
            name: "Pumpkin",
            color: [246, 127, 0, 255].into(),
        },
        Floss {
            number: "947",
            name: "Burnt Orange",
            color: [255, 123, 77, 255].into(),
        },
        Floss {
            number: "946",
            name: "Burnt Orange Med",
            color: [235, 99, 7, 255].into(),
        },
        Floss {
            number: "900",
            name: "Burnt Orange Dark",
            color: [209, 88, 7, 255].into(),
        },
        Floss {
            number: "967",
            name: "Apricot Very Light",
            color: [255, 222, 213, 255].into(),
        },
        Floss {
            number: "3824",
            name: "Apricot Light",
            color: [254, 205, 194, 255].into(),
        },
        Floss {
            number: "3341",
            name: "Apricot",
            color: [252, 171, 152, 255].into(),
        },
        Floss {
            number: "3340",
            name: "Apricot Med",
            color: [255, 131, 111, 255].into(),
        },
        Floss {
            number: "608",
            name: "Burnt Orange Bright",
            color: [253, 93, 53, 255].into(),
        },
        Floss {
            number: "606",
            name: "Orange?Red Bright",
            color: [250, 50, 3, 255].into(),
        },
        Floss {
            number: "951",
            name: "Tawny Light",
            color: [255, 226, 207, 255].into(),
        },
        Floss {
            number: "3856",
            name: "Mahogany Ult Vy Lt",
            color: [255, 211, 181, 255].into(),
        },
        Floss {
            number: "722",
            name: "Orange Spice Light",
            color: [247, 151, 111, 255].into(),
        },
        Floss {
            number: "721",
            name: "Orange Spice Med",
            color: [242, 120, 66, 255].into(),
        },
        Floss {
            number: "720",
            name: "Orange Spice Dark",
            color: [229, 92, 31, 255].into(),
        },
        Floss {
            number: "3825",
            name: "Pumpkin Pale",
            color: [253, 189, 150, 255].into(),
        },
        Floss {
            number: "922",
            name: "Copper Light",
            color: [226, 115, 35, 255].into(),
        },
        Floss {
            number: "921",
            name: "Copper",
            color: [198, 98, 24, 255].into(),
        },
        Floss {
            number: "920",
            name: "Copper Med",
            color: [172, 84, 20, 255].into(),
        },
        Floss {
            number: "919",
            name: "Red?Copper",
            color: [166, 69, 16, 255].into(),
        },
        Floss {
            number: "918",
            name: "Red?Copper Dark",
            color: [130, 52, 10, 255].into(),
        },
        Floss {
            number: "3770",
            name: "Tawny Vy Light",
            color: [255, 238, 227, 255].into(),
        },
        Floss {
            number: "945",
            name: "Tawny",
            color: [251, 213, 187, 255].into(),
        },
        Floss {
            number: "402",
            name: "Mahogany Vy Lt",
            color: [247, 167, 119, 255].into(),
        },
        Floss {
            number: "3776",
            name: "Mahogany Light",
            color: [207, 121, 57, 255].into(),
        },
        Floss {
            number: "301",
            name: "Mahogany Med",
            color: [179, 95, 43, 255].into(),
        },
        Floss {
            number: "400",
            name: "Mahogany Dark",
            color: [143, 67, 15, 255].into(),
        },
        Floss {
            number: "300",
            name: "Mahogany Vy Dk",
            color: [111, 47, 0, 255].into(),
        },
        Floss {
            number: "3823",
            name: "Yellow Ultra Pale",
            color: [255, 253, 227, 255].into(),
        },
        Floss {
            number: "3855",
            name: "Autumn Gold Lt",
            color: [250, 211, 150, 255].into(),
        },
        Floss {
            number: "3854",
            name: "Autumn Gold Med",
            color: [242, 175, 104, 255].into(),
        },
        Floss {
            number: "3853",
            name: "Autumn Gold Dk",
            color: [242, 151, 70, 255].into(),
        },
        Floss {
            number: "3827",
            name: "Golden Brown Pale",
            color: [247, 187, 119, 255].into(),
        },
        Floss {
            number: "977",
            name: "Golden Brown Light",
            color: [220, 156, 86, 255].into(),
        },
        Floss {
            number: "976",
            name: "Golden Brown Med",
            color: [194, 129, 66, 255].into(),
        },
        Floss {
            number: "3826",
            name: "Golden Brown",
            color: [173, 114, 57, 255].into(),
        },
        Floss {
            number: "975",
            name: "Golden Brown Dk",
            color: [145, 79, 18, 255].into(),
        },
        Floss {
            number: "948",
            name: "Peach Very Light",
            color: [254, 231, 218, 255].into(),
        },
        Floss {
            number: "754",
            name: "Peach Light",
            color: [247, 203, 191, 255].into(),
        },
        Floss {
            number: "3771",
            name: "Terra Cotta Ult Vy Lt",
            color: [244, 187, 169, 255].into(),
        },
        Floss {
            number: "758",
            name: "Terra Cotta Vy Lt",
            color: [238, 170, 155, 255].into(),
        },
        Floss {
            number: "3778",
            name: "Terra Cotta Light",
            color: [217, 137, 120, 255].into(),
        },
        Floss {
            number: "356",
            name: "Terra Cotta Med",
            color: [197, 106, 91, 255].into(),
        },
        Floss {
            number: "3830",
            name: "Terra Cotta",
            color: [185, 85, 68, 255].into(),
        },
        Floss {
            number: "355",
            name: "Terra Cotta Dark",
            color: [152, 68, 54, 255].into(),
        },
        Floss {
            number: "3777",
            name: "Terra Cotta Vy Dk",
            color: [134, 48, 34, 255].into(),
        },
        Floss {
            number: "3779",
            name: "Rosewood Ult Vy Lt",
            color: [248, 202, 200, 255].into(),
        },
        Floss {
            number: "3859",
            name: "Rosewood Light",
            color: [186, 139, 124, 255].into(),
        },
        Floss {
            number: "3858",
            name: "Rosewood Med",
            color: [150, 74, 63, 255].into(),
        },
        Floss {
            number: "3857",
            name: "Rosewood Dark",
            color: [104, 37, 26, 255].into(),
        },
        Floss {
            number: "3774",
            name: "Desert Sand Vy Lt",
            color: [243, 225, 215, 255].into(),
        },
        Floss {
            number: "950",
            name: "Desert Sand Light",
            color: [238, 211, 196, 255].into(),
        },
        Floss {
            number: "3064",
            name: "Desert Sand",
            color: [196, 142, 112, 255].into(),
        },
        Floss {
            number: "407",
            name: "Desert Sand Med",
            color: [187, 129, 97, 255].into(),
        },
        Floss {
            number: "3773",
            name: "Desert Sand Dark",
            color: [182, 117, 82, 255].into(),
        },
        Floss {
            number: "3772",
            name: "Desert Sand Vy Dk",
            color: [160, 108, 80, 255].into(),
        },
        Floss {
            number: "632",
            name: "Desert Sand Ult Vy Dk",
            color: [135, 85, 57, 255].into(),
        },
        Floss {
            number: "453",
            name: "Shell Gray Light",
            color: [215, 206, 203, 255].into(),
        },
        Floss {
            number: "452",
            name: "Shell Gray Med",
            color: [192, 179, 174, 255].into(),
        },
        Floss {
            number: "451",
            name: "Shell Gray Dark",
            color: [145, 123, 115, 255].into(),
        },
        Floss {
            number: "3861",
            name: "Cocoa Light",
            color: [166, 136, 129, 255].into(),
        },
        Floss {
            number: "3860",
            name: "Cocoa",
            color: [125, 93, 87, 255].into(),
        },
        Floss {
            number: "779",
            name: "Cocoa Dark",
            color: [98, 75, 69, 255].into(),
        },
        Floss {
            number: "712",
            name: "Cream",
            color: [255, 251, 239, 255].into(),
        },
        Floss {
            number: "739",
            name: "Tan Ult Vy Lt",
            color: [248, 228, 200, 255].into(),
        },
        Floss {
            number: "738",
            name: "Tan Very Light",
            color: [236, 204, 158, 255].into(),
        },
        Floss {
            number: "437",
            name: "Tan Light",
            color: [228, 187, 142, 255].into(),
        },
        Floss {
            number: "436",
            name: "Tan",
            color: [203, 144, 81, 255].into(),
        },
        Floss {
            number: "435",
            name: "Brown Very Light",
            color: [184, 119, 72, 255].into(),
        },
        Floss {
            number: "434",
            name: "Brown Light",
            color: [152, 94, 51, 255].into(),
        },
        Floss {
            number: "433",
            name: "Brown Med",
            color: [122, 69, 31, 255].into(),
        },
        Floss {
            number: "801",
            name: "Coffee Brown Dk",
            color: [101, 57, 25, 255].into(),
        },
        Floss {
            number: "898",
            name: "Coffee Brown Vy Dk",
            color: [73, 42, 19, 255].into(),
        },
        Floss {
            number: "938",
            name: "Coffee Brown Ult Dk",
            color: [54, 31, 14, 255].into(),
        },
        Floss {
            number: "3371",
            name: "Black Brown",
            color: [30, 17, 8, 255].into(),
        },
        Floss {
            number: "543",
            name: "Beige Brown Ult Vy Lt",
            color: [242, 227, 206, 255].into(),
        },
        Floss {
            number: "3864",
            name: "Mocha Beige Light",
            color: [203, 182, 156, 255].into(),
        },
        Floss {
            number: "3863",
            name: "Mocha Beige Med",
            color: [164, 131, 92, 255].into(),
        },
        Floss {
            number: "3862",
            name: "Mocha Beige Dark",
            color: [138, 110, 78, 255].into(),
        },
        Floss {
            number: "3031",
            name: "Mocha Brown Vy Dk",
            color: [75, 60, 42, 255].into(),
        },
        Floss {
            number: "B5200",
            name: "Snow White",
            color: [255, 255, 255, 255].into(),
        },
        Floss {
            number: "White",
            name: "White",
            color: [252, 251, 248, 255].into(),
        },
        Floss {
            number: "3865",
            name: "Winter White",
            color: [249, 247, 241, 255].into(),
        },
        Floss {
            number: "Ecru",
            name: "Ecru",
            color: [240, 234, 218, 255].into(),
        },
        Floss {
            number: "822",
            name: "Beige Gray Light",
            color: [231, 226, 211, 255].into(),
        },
        Floss {
            number: "644",
            name: "Beige Gray Med",
            color: [221, 216, 203, 255].into(),
        },
        Floss {
            number: "642",
            name: "Beige Gray Dark",
            color: [164, 152, 120, 255].into(),
        },
        Floss {
            number: "640",
            name: "Beige Gray Vy Dk",
            color: [133, 123, 97, 255].into(),
        },
        Floss {
            number: "3787",
            name: "Brown Gray Dark",
            color: [98, 93, 80, 255].into(),
        },
        Floss {
            number: "3021",
            name: "Brown Gray Vy Dk",
            color: [79, 75, 65, 255].into(),
        },
        Floss {
            number: "3024",
            name: "Brown Gray Vy Lt",
            color: [235, 234, 231, 255].into(),
        },
        Floss {
            number: "3023",
            name: "Brown Gray Light",
            color: [177, 170, 151, 255].into(),
        },
        Floss {
            number: "3022",
            name: "Brown Gray Med",
            color: [142, 144, 120, 255].into(),
        },
        Floss {
            number: "535",
            name: "Ash Gray Vy Lt",
            color: [99, 100, 88, 255].into(),
        },
        Floss {
            number: "3033",
            name: "Mocha Brown Vy Lt",
            color: [227, 216, 204, 255].into(),
        },
        Floss {
            number: "3782",
            name: "Mocha Brown Lt",
            color: [210, 188, 166, 255].into(),
        },
        Floss {
            number: "3032",
            name: "Mocha Brown Med",
            color: [179, 159, 139, 255].into(),
        },
        Floss {
            number: "3790",
            name: "Beige Gray Ult Dk",
            color: [127, 106, 85, 255].into(),
        },
        Floss {
            number: "3781",
            name: "Mocha Brown Dk",
            color: [107, 87, 67, 255].into(),
        },
        Floss {
            number: "3866",
            name: "Mocha Brn Ult Vy Lt",
            color: [250, 246, 240, 255].into(),
        },
        Floss {
            number: "842",
            name: "Beige Brown Vy Lt",
            color: [209, 186, 161, 255].into(),
        },
        Floss {
            number: "841",
            name: "Beige Brown Lt",
            color: [182, 155, 126, 255].into(),
        },
        Floss {
            number: "840",
            name: "Beige Brown Med",
            color: [154, 124, 92, 255].into(),
        },
        Floss {
            number: "839",
            name: "Beige Brown Dk",
            color: [103, 85, 65, 255].into(),
        },
        Floss {
            number: "838",
            name: "Beige Brown Vy Dk",
            color: [89, 73, 55, 255].into(),
        },
        Floss {
            number: "3072",
            name: "Beaver Gray Vy Lt",
            color: [230, 232, 232, 255].into(),
        },
        Floss {
            number: "648",
            name: "Beaver Gray Lt",
            color: [188, 180, 172, 255].into(),
        },
        Floss {
            number: "647",
            name: "Beaver Gray Med",
            color: [176, 166, 156, 255].into(),
        },
        Floss {
            number: "646",
            name: "Beaver Gray Dk",
            color: [135, 125, 115, 255].into(),
        },
        Floss {
            number: "645",
            name: "Beaver Gray Vy Dk",
            color: [110, 101, 92, 255].into(),
        },
        Floss {
            number: "844",
            name: "Beaver Gray Ult Dk",
            color: [72, 72, 72, 255].into(),
        },
        Floss {
            number: "762",
            name: "Pearl Gray Vy Lt",
            color: [236, 236, 236, 255].into(),
        },
        Floss {
            number: "415",
            name: "Pearl Gray",
            color: [211, 211, 214, 255].into(),
        },
        Floss {
            number: "318",
            name: "Steel Gray Lt",
            color: [171, 171, 171, 255].into(),
        },
        Floss {
            number: "414",
            name: "Steel Gray Dk",
            color: [140, 140, 140, 255].into(),
        },
        Floss {
            number: "168",
            name: "Pewter Very Light",
            color: [209, 209, 209, 255].into(),
        },
        Floss {
            number: "169",
            name: "Pewter Light",
            color: [132, 132, 132, 255].into(),
        },
        Floss {
            number: "317",
            name: "Pewter Gray",
            color: [108, 108, 108, 255].into(),
        },
        Floss {
            number: "413",
            name: "Pewter Gray Dark",
            color: [86, 86, 86, 255].into(),
        },
        Floss {
            number: "3799",
            name: "Pewter Gray Vy Dk",
            color: [66, 66, 66, 255].into(),
        },
        Floss {
            number: "310",
            name: "Black",
            color: [0, 0, 0, 255].into(),
        },
    ]
}
