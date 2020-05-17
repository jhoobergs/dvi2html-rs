use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct FontCharacterMetrics {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FontData {
    pub characters: HashMap<u32, FontCharacterMetrics>,
    pub smallest_character_code: u32,
    pub largest_character_code: u32,
    pub design_size: u32,
}

pub struct FontDataHelper {
    data: HashMap<String, FontData>,
}

impl FontDataHelper {
    pub fn init() -> Result<FontDataHelper> {
        //let json = fs::read_to_string("fontsfull.json").expect("Failed to read the font json file");
        let json = r#"
{
  "cmb10": {
    "characters": {
      "0": {
        "width": 629149,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 873816,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 815562,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 734006,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 699053,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 821387,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 757307,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 815562,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 757307,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 815562,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 757307,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 611672,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 291272,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 320400,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 524290,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 524290,
        "height": 625066,
        "depth": 0
      },
      "23": {
        "width": 792261,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 466035,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 586912,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 757307,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 815562,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 524290,
        "height": 567979,
        "depth": 101946
      },
      "29": {
        "width": 949547,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1066056,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 815562,
        "height": 770413,
        "depth": 50973
      },
      "32": {
        "width": 291272,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 320398,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 570893,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 873816,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 524290,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 873816,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 524290,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "44": {
        "width": 291272,
        "height": 163112,
        "depth": 203890
      },
      "45": {
        "width": 349526,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 291272,
        "height": 163112,
        "depth": 0
      },
      "47": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 291272,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 291272,
        "height": 466034,
        "depth": 203890
      },
      "60": {
        "width": 320398,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 815562,
        "height": 410110,
        "depth": 4294853118
      },
      "62": {
        "width": 495163,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 495163,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 792261,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 745658,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 757307,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 803912,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 687403,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 658277,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 824301,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 821387,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 416520,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 541766,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 821389,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 629149,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 996150,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 821387,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 786435,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 716530,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 786435,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 796629,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 582544,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 728181,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 806824,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 792261,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1083533,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 792261,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 792261,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 640798,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 293603,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 570893,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 293603,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 509726,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 466035,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 489336,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 524290,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 320400,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 553418,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 873816,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 582544,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 524290,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 582544,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 553416,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 448560,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 413606,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 409237,
        "height": 665763,
        "depth": 0
      },
      "117": {
        "width": 582544,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 553418,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 757307,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 553418,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 553418,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 466035,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 524290,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1048579,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmbsy10": {
    "characters": {
      "0": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "1": {
        "width": 334960,
        "height": 496616,
        "depth": 4294939624
      },
      "2": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "3": {
        "width": 602928,
        "height": 495162,
        "depth": 4294938170
      },
      "4": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "5": {
        "width": 602928,
        "height": 496616,
        "depth": 4294939624
      },
      "6": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "7": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "8": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "9": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "10": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "11": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "12": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "13": {
        "width": 1205856,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 602928,
        "height": 496616,
        "depth": 4294939624
      },
      "15": {
        "width": 602928,
        "height": 496616,
        "depth": 4294939624
      },
      "16": {
        "width": 937888,
        "height": 526619,
        "depth": 2331
      },
      "17": {
        "width": 937888,
        "height": 526619,
        "depth": 2331
      },
      "18": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "19": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "20": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "21": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "22": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "23": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "24": {
        "width": 937888,
        "height": 410110,
        "depth": 4294853118
      },
      "25": {
        "width": 937888,
        "height": 549920,
        "depth": 25632
      },
      "26": {
        "width": 937888,
        "height": 614000,
        "depth": 89712
      },
      "27": {
        "width": 937888,
        "height": 614000,
        "depth": 89712
      },
      "28": {
        "width": 1205856,
        "height": 614000,
        "depth": 89712
      },
      "29": {
        "width": 1205856,
        "height": 614000,
        "depth": 89712
      },
      "30": {
        "width": 937888,
        "height": 614000,
        "depth": 89712
      },
      "31": {
        "width": 937888,
        "height": 614000,
        "depth": 89712
      },
      "32": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "33": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "34": {
        "width": 602928,
        "height": 728178,
        "depth": 203888
      },
      "35": {
        "width": 602928,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "37": {
        "width": 1205856,
        "height": 728178,
        "depth": 203888
      },
      "38": {
        "width": 1205856,
        "height": 728178,
        "depth": 203888
      },
      "39": {
        "width": 937888,
        "height": 526619,
        "depth": 2331
      },
      "40": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "41": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "42": {
        "width": 736912,
        "height": 728178,
        "depth": 203888
      },
      "43": {
        "width": 736912,
        "height": 728178,
        "depth": 203888
      },
      "44": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "45": {
        "width": 1205856,
        "height": 728178,
        "depth": 203888
      },
      "46": {
        "width": 1205856,
        "height": 728178,
        "depth": 203888
      },
      "47": {
        "width": 937888,
        "height": 466034,
        "depth": 0
      },
      "48": {
        "width": 361174,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1205856,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 803904,
        "height": 614000,
        "depth": 89712
      },
      "51": {
        "width": 803904,
        "height": 614000,
        "depth": 89712
      },
      "52": {
        "width": 1071872,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 1071872,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203888
      },
      "55": {
        "width": 0,
        "height": 410110,
        "depth": 4294853118
      },
      "56": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 803904,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 602928,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 870896,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 870896,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 937888,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 937888,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 736912,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 965453,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 784171,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 642758,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 935413,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 636424,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 853634,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 714704,
        "height": 719440,
        "depth": 101946
      },
      "72": {
        "width": 1035352,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 673595,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 817302,
        "height": 719440,
        "depth": 101946
      },
      "75": {
        "width": 913571,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 826477,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1444755,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 982853,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 949539,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 849197,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 984782,
        "height": 719440,
        "depth": 101946
      },
      "82": {
        "width": 1037646,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 730213,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 675432,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 749432,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 773176,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1225371,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 856395,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 795459,
        "height": 719440,
        "depth": 101946
      },
      "90": {
        "width": 858229,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 803904,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 803904,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 803904,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 803904,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 803904,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 736912,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 736912,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 535936,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 535936,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 535936,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 535936,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 468944,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 468944,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 334960,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 736912,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 334960,
        "height": 728178,
        "depth": 203888
      },
      "112": {
        "width": 1004880,
        "height": 62915,
        "depth": 985661
      },
      "113": {
        "width": 943714,
        "height": 719440,
        "depth": 0
      },
      "114": {
        "width": 1004880,
        "height": 719440,
        "depth": 0
      },
      "115": {
        "width": 596378,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 803904,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 803904,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "119": {
        "width": 937888,
        "height": 730509,
        "depth": 206221
      },
      "120": {
        "width": 551957,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 535936,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 535936,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 736912,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 937888,
        "height": 728178,
        "depth": 135926
      },
      "125": {
        "width": 937888,
        "height": 728178,
        "depth": 135926
      },
      "126": {
        "width": 937888,
        "height": 728178,
        "depth": 135926
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmbsy6": {
    "characters": {
      "0": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "1": {
        "width": 432051,
        "height": 537395,
        "depth": 13107
      },
      "2": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "3": {
        "width": 746621,
        "height": 495163,
        "depth": 4294938171
      },
      "4": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "5": {
        "width": 746621,
        "height": 537395,
        "depth": 13107
      },
      "6": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "7": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "8": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "9": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "10": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "11": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "12": {
        "width": 1139835,
        "height": 734000,
        "depth": 209712
      },
      "13": {
        "width": 1454405,
        "height": 728179,
        "depth": 203891
      },
      "14": {
        "width": 746621,
        "height": 537395,
        "depth": 13107
      },
      "15": {
        "width": 746621,
        "height": 537395,
        "depth": 13107
      },
      "16": {
        "width": 1139835,
        "height": 573515,
        "depth": 49227
      },
      "17": {
        "width": 1139835,
        "height": 573515,
        "depth": 49227
      },
      "18": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "19": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "20": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "21": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "22": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "23": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "24": {
        "width": 1139835,
        "height": 438365,
        "depth": 4294881373
      },
      "25": {
        "width": 1139835,
        "height": 600547,
        "depth": 76259
      },
      "26": {
        "width": 1139835,
        "height": 670216,
        "depth": 145928
      },
      "27": {
        "width": 1139835,
        "height": 670216,
        "depth": 145928
      },
      "28": {
        "width": 1454405,
        "height": 670216,
        "depth": 145928
      },
      "29": {
        "width": 1454405,
        "height": 670216,
        "depth": 145928
      },
      "30": {
        "width": 1139835,
        "height": 670216,
        "depth": 145928
      },
      "31": {
        "width": 1139835,
        "height": 670216,
        "depth": 145928
      },
      "32": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "33": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "34": {
        "width": 746621,
        "height": 728179,
        "depth": 203888
      },
      "35": {
        "width": 746621,
        "height": 728179,
        "depth": 203888
      },
      "36": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "37": {
        "width": 1454405,
        "height": 728179,
        "depth": 203888
      },
      "38": {
        "width": 1454405,
        "height": 728179,
        "depth": 203888
      },
      "39": {
        "width": 1139835,
        "height": 573515,
        "depth": 49227
      },
      "40": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "41": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "42": {
        "width": 903907,
        "height": 728179,
        "depth": 203888
      },
      "43": {
        "width": 903907,
        "height": 728179,
        "depth": 203888
      },
      "44": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "45": {
        "width": 1454405,
        "height": 728179,
        "depth": 203888
      },
      "46": {
        "width": 1454405,
        "height": 728179,
        "depth": 203888
      },
      "47": {
        "width": 1139835,
        "height": 466035,
        "depth": 0
      },
      "48": {
        "width": 449525,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1454405,
        "height": 466035,
        "depth": 0
      },
      "50": {
        "width": 982549,
        "height": 670216,
        "depth": 145928
      },
      "51": {
        "width": 982549,
        "height": 670216,
        "depth": 145928
      },
      "52": {
        "width": 1297120,
        "height": 728179,
        "depth": 203891
      },
      "53": {
        "width": 1297120,
        "height": 728179,
        "depth": 203891
      },
      "54": {
        "width": 0,
        "height": 728179,
        "depth": 203888
      },
      "55": {
        "width": 0,
        "height": 438365,
        "depth": 4294881373
      },
      "56": {
        "width": 825264,
        "height": 728179,
        "depth": 0
      },
      "57": {
        "width": 825264,
        "height": 728179,
        "depth": 0
      },
      "58": {
        "width": 982549,
        "height": 466035,
        "depth": 0
      },
      "59": {
        "width": 746621,
        "height": 786432,
        "depth": 58253
      },
      "60": {
        "width": 1061192,
        "height": 728179,
        "depth": 0
      },
      "61": {
        "width": 1061192,
        "height": 728179,
        "depth": 0
      },
      "62": {
        "width": 1139835,
        "height": 728179,
        "depth": 0
      },
      "63": {
        "width": 1139835,
        "height": 728179,
        "depth": 0
      },
      "64": {
        "width": 903907,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1175323,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 950005,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 802763,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1143187,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 785947,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 1025291,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 874709,
        "height": 719440,
        "depth": 101947
      },
      "72": {
        "width": 1274584,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 849915,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 998280,
        "height": 719440,
        "depth": 101947
      },
      "75": {
        "width": 1105035,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 1002795,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1722277,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1184803,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 1141000,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 1048232,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1194888,
        "height": 719440,
        "depth": 101947
      },
      "82": {
        "width": 1275712,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 896045,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 854077,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 908464,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 959589,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1490427,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1039701,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 960123,
        "height": 719440,
        "depth": 101947
      },
      "90": {
        "width": 1027555,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 982549,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 982549,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 982549,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 982549,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 982549,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 903907,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 903907,
        "height": 728179,
        "depth": 0
      },
      "98": {
        "width": 667979,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 667979,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 667979,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 667979,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 746621,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 746621,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 589336,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 589336,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 432051,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 746621,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 746621,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 903907,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 746621,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 432051,
        "height": 728179,
        "depth": 203888
      },
      "112": {
        "width": 1199059,
        "height": 82139,
        "depth": 966437
      },
      "113": {
        "width": 1135952,
        "height": 719440,
        "depth": 0
      },
      "114": {
        "width": 1218477,
        "height": 719440,
        "depth": 0
      },
      "115": {
        "width": 716768,
        "height": 728179,
        "depth": 203891
      },
      "116": {
        "width": 982549,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 982549,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "119": {
        "width": 1139835,
        "height": 805368,
        "depth": 281080
      },
      "120": {
        "width": 681088,
        "height": 728179,
        "depth": 203891
      },
      "121": {
        "width": 667979,
        "height": 728179,
        "depth": 203891
      },
      "122": {
        "width": 667979,
        "height": 728179,
        "depth": 203891
      },
      "123": {
        "width": 903907,
        "height": 728179,
        "depth": 203891
      },
      "124": {
        "width": 1139835,
        "height": 728179,
        "depth": 135928
      },
      "125": {
        "width": 1139835,
        "height": 728179,
        "depth": 135928
      },
      "126": {
        "width": 1139835,
        "height": 728179,
        "depth": 135928
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 6291456
  },
  "cmbsy7": {
    "characters": {
      "0": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "1": {
        "width": 389056,
        "height": 522832,
        "depth": 4294965840
      },
      "2": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "3": {
        "width": 686985,
        "height": 495161,
        "depth": 4294938169
      },
      "4": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "5": {
        "width": 686985,
        "height": 522832,
        "depth": 4294965840
      },
      "6": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "7": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "8": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "9": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "10": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "11": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "12": {
        "width": 1059397,
        "height": 709038,
        "depth": 184750
      },
      "13": {
        "width": 1357326,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 686985,
        "height": 522832,
        "depth": 4294965840
      },
      "15": {
        "width": 686985,
        "height": 522832,
        "depth": 4294965840
      },
      "16": {
        "width": 1059397,
        "height": 561323,
        "depth": 37035
      },
      "17": {
        "width": 1059397,
        "height": 561323,
        "depth": 37035
      },
      "18": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "19": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "20": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "21": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "22": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "23": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "24": {
        "width": 1059397,
        "height": 430832,
        "depth": 4294873840
      },
      "25": {
        "width": 1059397,
        "height": 587422,
        "depth": 63134
      },
      "26": {
        "width": 1059397,
        "height": 655694,
        "depth": 131406
      },
      "27": {
        "width": 1059397,
        "height": 655694,
        "depth": 131406
      },
      "28": {
        "width": 1357326,
        "height": 655694,
        "depth": 131406
      },
      "29": {
        "width": 1357326,
        "height": 655694,
        "depth": 131406
      },
      "30": {
        "width": 1059397,
        "height": 655694,
        "depth": 131406
      },
      "31": {
        "width": 1059397,
        "height": 655694,
        "depth": 131406
      },
      "32": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "33": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "34": {
        "width": 686985,
        "height": 728178,
        "depth": 203890
      },
      "35": {
        "width": 686985,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "37": {
        "width": 1357326,
        "height": 728178,
        "depth": 203890
      },
      "38": {
        "width": 1357326,
        "height": 728178,
        "depth": 203890
      },
      "39": {
        "width": 1059397,
        "height": 561323,
        "depth": 37035
      },
      "40": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "41": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "42": {
        "width": 835950,
        "height": 728178,
        "depth": 203890
      },
      "43": {
        "width": 835950,
        "height": 728178,
        "depth": 203890
      },
      "44": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "45": {
        "width": 1357326,
        "height": 728178,
        "depth": 203890
      },
      "46": {
        "width": 1357326,
        "height": 728178,
        "depth": 203890
      },
      "47": {
        "width": 1059397,
        "height": 466034,
        "depth": 0
      },
      "48": {
        "width": 406530,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1357326,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 910432,
        "height": 655694,
        "depth": 131406
      },
      "51": {
        "width": 910432,
        "height": 655694,
        "depth": 131406
      },
      "52": {
        "width": 1208361,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 1208361,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203890
      },
      "55": {
        "width": 0,
        "height": 430832,
        "depth": 4294873840
      },
      "56": {
        "width": 761467,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 761467,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 910432,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 686985,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 984914,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 984914,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 1059397,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 1059397,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 835950,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 1092055,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 882459,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 737301,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1060667,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 724226,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 955666,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 809248,
        "height": 719440,
        "depth": 101945
      },
      "72": {
        "width": 1180830,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 778626,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 925328,
        "height": 719440,
        "depth": 101945
      },
      "75": {
        "width": 1028341,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 931509,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1617451,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1104361,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 1064306,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 968832,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1111534,
        "height": 719440,
        "depth": 101945
      },
      "82": {
        "width": 1182377,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 828501,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 781961,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 844183,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 885737,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1388491,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 965918,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 892999,
        "height": 719440,
        "depth": 101945
      },
      "90": {
        "width": 958763,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 910432,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 910432,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 910432,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 910432,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 910432,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 835950,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 835950,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 612503,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 612503,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 612503,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 612503,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 686985,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 686985,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 538021,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 538021,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 389056,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 686985,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 686985,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 835950,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 686985,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 389056,
        "height": 728178,
        "depth": 203890
      },
      "112": {
        "width": 1125557,
        "height": 76395,
        "depth": 972181
      },
      "113": {
        "width": 1058981,
        "height": 719440,
        "depth": 0
      },
      "114": {
        "width": 1133879,
        "height": 719440,
        "depth": 0
      },
      "115": {
        "width": 670654,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 910432,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 910432,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "119": {
        "width": 1059397,
        "height": 786185,
        "depth": 261897
      },
      "120": {
        "width": 629771,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 612503,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 612503,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 835950,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 1059397,
        "height": 728178,
        "depth": 135927
      },
      "125": {
        "width": 1059397,
        "height": 728178,
        "depth": 135927
      },
      "126": {
        "width": 1059397,
        "height": 728178,
        "depth": 135927
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmbsy8": {
    "characters": {
      "0": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "1": {
        "width": 356810,
        "height": 511912,
        "depth": 4294954920
      },
      "2": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "3": {
        "width": 642258,
        "height": 495162,
        "depth": 4294938170
      },
      "4": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "5": {
        "width": 642258,
        "height": 511912,
        "depth": 4294954920
      },
      "6": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "7": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "8": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "9": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "10": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "11": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "12": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "13": {
        "width": 1284516,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 642258,
        "height": 511912,
        "depth": 4294954920
      },
      "15": {
        "width": 642258,
        "height": 511912,
        "depth": 4294954920
      },
      "16": {
        "width": 999068,
        "height": 549194,
        "depth": 24906
      },
      "17": {
        "width": 999068,
        "height": 549194,
        "depth": 24906
      },
      "18": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "19": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "20": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "21": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "22": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "23": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "24": {
        "width": 999068,
        "height": 423364,
        "depth": 4294866372
      },
      "25": {
        "width": 999068,
        "height": 574358,
        "depth": 50070
      },
      "26": {
        "width": 999068,
        "height": 641234,
        "depth": 116946
      },
      "27": {
        "width": 999068,
        "height": 641234,
        "depth": 116946
      },
      "28": {
        "width": 1284516,
        "height": 641234,
        "depth": 116946
      },
      "29": {
        "width": 1284516,
        "height": 641234,
        "depth": 116946
      },
      "30": {
        "width": 999068,
        "height": 641234,
        "depth": 116946
      },
      "31": {
        "width": 999068,
        "height": 641234,
        "depth": 116946
      },
      "32": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "33": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "34": {
        "width": 642258,
        "height": 728178,
        "depth": 203888
      },
      "35": {
        "width": 642258,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "37": {
        "width": 1284516,
        "height": 728178,
        "depth": 203888
      },
      "38": {
        "width": 1284516,
        "height": 728178,
        "depth": 203888
      },
      "39": {
        "width": 999068,
        "height": 549194,
        "depth": 24906
      },
      "40": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "41": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "42": {
        "width": 784982,
        "height": 728178,
        "depth": 203888
      },
      "43": {
        "width": 784982,
        "height": 728178,
        "depth": 203888
      },
      "44": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "45": {
        "width": 1284516,
        "height": 728178,
        "depth": 203888
      },
      "46": {
        "width": 1284516,
        "height": 728178,
        "depth": 203888
      },
      "47": {
        "width": 999068,
        "height": 466034,
        "depth": 0
      },
      "48": {
        "width": 381568,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1284516,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 856344,
        "height": 641234,
        "depth": 116946
      },
      "51": {
        "width": 856344,
        "height": 641234,
        "depth": 116946
      },
      "52": {
        "width": 1141792,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 1141792,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203888
      },
      "55": {
        "width": 0,
        "height": 423364,
        "depth": 4294866372
      },
      "56": {
        "width": 713620,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 713620,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 856344,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 642258,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 927706,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 927706,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 999068,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 999068,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 784982,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 1029604,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 831806,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 688208,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 998780,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 677942,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 903454,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 760152,
        "height": 719440,
        "depth": 101946
      },
      "72": {
        "width": 1110518,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 725164,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 870618,
        "height": 719440,
        "depth": 101946
      },
      "75": {
        "width": 970820,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 878046,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1538832,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1044034,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 1006788,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 909286,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1049024,
        "height": 719440,
        "depth": 101946
      },
      "82": {
        "width": 1112376,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 777848,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 727872,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 795976,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 830350,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1312044,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 910584,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 842656,
        "height": 719440,
        "depth": 101946
      },
      "90": {
        "width": 907172,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 856344,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 856344,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 856344,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 856344,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 856344,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 784982,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 784982,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 570896,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 570896,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 570896,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 570896,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 642258,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 642258,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 499534,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 499534,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 356810,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 642258,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 642258,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 784982,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 642258,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 356810,
        "height": 728178,
        "depth": 203888
      },
      "112": {
        "width": 1070430,
        "height": 70778,
        "depth": 977798
      },
      "113": {
        "width": 1001250,
        "height": 719440,
        "depth": 0
      },
      "114": {
        "width": 1070430,
        "height": 719440,
        "depth": 0
      },
      "115": {
        "width": 632424,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 856344,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 856344,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "119": {
        "width": 999068,
        "height": 767064,
        "depth": 242776
      },
      "120": {
        "width": 587644,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 570896,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 570896,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 784982,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 999068,
        "height": 728178,
        "depth": 135926
      },
      "125": {
        "width": 999068,
        "height": 728178,
        "depth": 135926
      },
      "126": {
        "width": 999068,
        "height": 728178,
        "depth": 135926
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmbsy9": {
    "characters": {
      "0": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "1": {
        "width": 344667,
        "height": 503412,
        "depth": 4294946420
      },
      "2": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "3": {
        "width": 620400,
        "height": 495161,
        "depth": 4294938169
      },
      "4": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "5": {
        "width": 620400,
        "height": 503412,
        "depth": 4294946420
      },
      "6": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "7": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "8": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "9": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "10": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "11": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "12": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "13": {
        "width": 1240800,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 620400,
        "height": 503412,
        "depth": 4294946420
      },
      "15": {
        "width": 620400,
        "height": 503412,
        "depth": 4294946420
      },
      "16": {
        "width": 965067,
        "height": 537687,
        "depth": 13399
      },
      "17": {
        "width": 965067,
        "height": 537687,
        "depth": 13399
      },
      "18": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "19": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "20": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "21": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "22": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "23": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "24": {
        "width": 965067,
        "height": 416519,
        "depth": 4294859527
      },
      "25": {
        "width": 965067,
        "height": 561922,
        "depth": 37634
      },
      "26": {
        "width": 965067,
        "height": 627399,
        "depth": 103111
      },
      "27": {
        "width": 965067,
        "height": 627399,
        "depth": 103111
      },
      "28": {
        "width": 1240800,
        "height": 627399,
        "depth": 103111
      },
      "29": {
        "width": 1240800,
        "height": 627399,
        "depth": 103111
      },
      "30": {
        "width": 965067,
        "height": 627399,
        "depth": 103111
      },
      "31": {
        "width": 965067,
        "height": 627399,
        "depth": 103111
      },
      "32": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "33": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "34": {
        "width": 620400,
        "height": 728178,
        "depth": 203890
      },
      "35": {
        "width": 620400,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "37": {
        "width": 1240800,
        "height": 728178,
        "depth": 203890
      },
      "38": {
        "width": 1240800,
        "height": 728178,
        "depth": 203890
      },
      "39": {
        "width": 965067,
        "height": 537687,
        "depth": 13399
      },
      "40": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "41": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "42": {
        "width": 758267,
        "height": 728178,
        "depth": 203890
      },
      "43": {
        "width": 758267,
        "height": 728178,
        "depth": 203890
      },
      "44": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "45": {
        "width": 1240800,
        "height": 728178,
        "depth": 203890
      },
      "46": {
        "width": 1240800,
        "height": 728178,
        "depth": 203890
      },
      "47": {
        "width": 965067,
        "height": 466034,
        "depth": 0
      },
      "48": {
        "width": 375093,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1240800,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 827200,
        "height": 627399,
        "depth": 103111
      },
      "51": {
        "width": 827200,
        "height": 627399,
        "depth": 103111
      },
      "52": {
        "width": 1102933,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 1102933,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203890
      },
      "55": {
        "width": 0,
        "height": 416519,
        "depth": 4294859527
      },
      "56": {
        "width": 689333,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 689333,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 827200,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 620400,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 896133,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 896133,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 965067,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 965067,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 758267,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 993952,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 805333,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 662949,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 963563,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 654869,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 875767,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 734896,
        "height": 719440,
        "depth": 101945
      },
      "72": {
        "width": 1068745,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 696505,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 840987,
        "height": 719440,
        "depth": 101945
      },
      "75": {
        "width": 939004,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 849387,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1490597,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1010034,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 974971,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 875892,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1013321,
        "height": 719440,
        "depth": 101945
      },
      "82": {
        "width": 1070844,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 751374,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 698729,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 770108,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 800196,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1265495,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 880469,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 816425,
        "height": 719440,
        "depth": 101945
      },
      "90": {
        "width": 879972,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 827200,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 827200,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 827200,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 827200,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 827200,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 758267,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 758267,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 551467,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 551467,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 551467,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 551467,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 620400,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 620400,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 482533,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 482533,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 344667,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 620400,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 620400,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 758267,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 620400,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 344667,
        "height": 728178,
        "depth": 203890
      },
      "112": {
        "width": 1034000,
        "height": 66411,
        "depth": 982165
      },
      "113": {
        "width": 969276,
        "height": 719440,
        "depth": 0
      },
      "114": {
        "width": 1034000,
        "height": 719440,
        "depth": 0
      },
      "115": {
        "width": 615632,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 827200,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 827200,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "119": {
        "width": 965067,
        "height": 748567,
        "depth": 224279
      },
      "120": {
        "width": 567812,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 551467,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 551467,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 758267,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 965067,
        "height": 728178,
        "depth": 135927
      },
      "125": {
        "width": 965067,
        "height": 728178,
        "depth": 135927
      },
      "126": {
        "width": 965067,
        "height": 728178,
        "depth": 135927
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmbx10": {
    "characters": {
      "0": {
        "width": 725261,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1004880,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 937888,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 844682,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 803904,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 943714,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 937888,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 937888,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 703416,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1004880,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1004880,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 334960,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 368456,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 602928,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 602928,
        "height": 625066,
        "depth": 0
      },
      "23": {
        "width": 911674,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 535936,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 626230,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 870896,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 937888,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 602928,
        "height": 567979,
        "depth": 101946
      },
      "29": {
        "width": 1092261,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1226245,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 937888,
        "height": 770413,
        "depth": 50973
      },
      "32": {
        "width": 334960,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 367000,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 632056,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1004880,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 602928,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 1004880,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 937888,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 468944,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 468944,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 602928,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "44": {
        "width": 334960,
        "height": 163112,
        "depth": 203890
      },
      "45": {
        "width": 401952,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 334960,
        "height": 163112,
        "depth": 0
      },
      "47": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 334960,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 334960,
        "height": 466034,
        "depth": 203890
      },
      "60": {
        "width": 367000,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 937888,
        "height": 410110,
        "depth": 4294853118
      },
      "62": {
        "width": 569432,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 569432,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 937888,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 857789,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 924781,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 792253,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 758757,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 948083,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 943714,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 457294,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 623317,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 945170,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 725261,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1144690,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 943714,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 905848,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 824293,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 905848,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 904392,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 669920,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 838856,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 927694,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1246634,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 736912,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 334960,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 632056,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 334960,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 586179,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 535936,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 552685,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 368456,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 602928,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 368456,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 636424,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1004880,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 669920,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 602928,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 669920,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 636424,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 496616,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 475643,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 468944,
        "height": 665763,
        "depth": 0
      },
      "117": {
        "width": 669920,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 636424,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 870896,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 636424,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 636424,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 535936,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 602928,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1205856,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmbx12": {
    "characters": {
      "0": {
        "width": 708760,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 983040,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 917504,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 825269,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 786432,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 922360,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 851968,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 917504,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 851968,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 917504,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 851968,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 688128,
        "height": 728177,
        "depth": 0
      },
      "12": {
        "width": 655360,
        "height": 728177,
        "depth": 0
      },
      "13": {
        "width": 655360,
        "height": 728177,
        "depth": 0
      },
      "14": {
        "width": 983040,
        "height": 728177,
        "depth": 0
      },
      "15": {
        "width": 983040,
        "height": 728177,
        "depth": 0
      },
      "16": {
        "width": 327680,
        "height": 466033,
        "depth": 0
      },
      "17": {
        "width": 360448,
        "height": 466033,
        "depth": 203889
      },
      "18": {
        "width": 589824,
        "height": 728177,
        "depth": 0
      },
      "19": {
        "width": 589824,
        "height": 728177,
        "depth": 0
      },
      "20": {
        "width": 589824,
        "height": 662641,
        "depth": 0
      },
      "21": {
        "width": 589824,
        "height": 728177,
        "depth": 0
      },
      "22": {
        "width": 589824,
        "height": 621473,
        "depth": 0
      },
      "23": {
        "width": 890805,
        "height": 728177,
        "depth": 0
      },
      "24": {
        "width": 524288,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 601961,
        "height": 728177,
        "depth": 0
      },
      "26": {
        "width": 851968,
        "height": 466033,
        "depth": 0
      },
      "27": {
        "width": 917504,
        "height": 466033,
        "depth": 0
      },
      "28": {
        "width": 589824,
        "height": 567979,
        "depth": 101945
      },
      "29": {
        "width": 1067995,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1199067,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 917504,
        "height": 770412,
        "depth": 50972
      },
      "32": {
        "width": 327680,
        "height": 466033,
        "depth": 0
      },
      "33": {
        "width": 359235,
        "height": 728177,
        "depth": 0
      },
      "34": {
        "width": 609243,
        "height": 728177,
        "depth": 0
      },
      "35": {
        "width": 983040,
        "height": 728177,
        "depth": 203888
      },
      "36": {
        "width": 589824,
        "height": 786432,
        "depth": 58255
      },
      "37": {
        "width": 983040,
        "height": 786432,
        "depth": 58255
      },
      "38": {
        "width": 917504,
        "height": 728177,
        "depth": 0
      },
      "39": {
        "width": 327680,
        "height": 728177,
        "depth": 0
      },
      "40": {
        "width": 458752,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 458752,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 589824,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 917504,
        "height": 655360,
        "depth": 131072
      },
      "44": {
        "width": 327680,
        "height": 150491,
        "depth": 203889
      },
      "45": {
        "width": 393216,
        "height": 466033,
        "depth": 0
      },
      "46": {
        "width": 327680,
        "height": 150491,
        "depth": 0
      },
      "47": {
        "width": 589824,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 589824,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 327680,
        "height": 466033,
        "depth": 0
      },
      "59": {
        "width": 327680,
        "height": 466033,
        "depth": 203889
      },
      "60": {
        "width": 359235,
        "height": 524288,
        "depth": 203889
      },
      "61": {
        "width": 917504,
        "height": 405159,
        "depth": 4294848167
      },
      "62": {
        "width": 557056,
        "height": 524288,
        "depth": 203889
      },
      "63": {
        "width": 557056,
        "height": 728177,
        "depth": 0
      },
      "64": {
        "width": 917504,
        "height": 728177,
        "depth": 0
      },
      "65": {
        "width": 890805,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 838619,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 851968,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 904155,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 774296,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 741528,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 927213,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 922360,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 439335,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 609243,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 923573,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 708760,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1118968,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 922360,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 885949,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 805851,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 885949,
        "height": 719440,
        "depth": 203889
      },
      "82": {
        "width": 879883,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 655360,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 820413,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 906583,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 890805,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1218485,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 890805,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 890805,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 720896,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 327680,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 609243,
        "height": 728177,
        "depth": 0
      },
      "93": {
        "width": 327680,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 589824,
        "height": 728177,
        "depth": 0
      },
      "95": {
        "width": 327680,
        "height": 728177,
        "depth": 0
      },
      "96": {
        "width": 327680,
        "height": 728177,
        "depth": 0
      },
      "97": {
        "width": 573440,
        "height": 466033,
        "depth": 0
      },
      "98": {
        "width": 655360,
        "height": 728177,
        "depth": 0
      },
      "99": {
        "width": 524288,
        "height": 466033,
        "depth": 0
      },
      "100": {
        "width": 655360,
        "height": 728177,
        "depth": 0
      },
      "101": {
        "width": 538245,
        "height": 466033,
        "depth": 0
      },
      "102": {
        "width": 360448,
        "height": 728177,
        "depth": 0
      },
      "103": {
        "width": 589824,
        "height": 466033,
        "depth": 203889
      },
      "104": {
        "width": 655360,
        "height": 728177,
        "depth": 0
      },
      "105": {
        "width": 327680,
        "height": 728177,
        "depth": 0
      },
      "106": {
        "width": 360448,
        "height": 728177,
        "depth": 203889
      },
      "107": {
        "width": 622592,
        "height": 728177,
        "depth": 0
      },
      "108": {
        "width": 327680,
        "height": 728177,
        "depth": 0
      },
      "109": {
        "width": 983040,
        "height": 466033,
        "depth": 0
      },
      "110": {
        "width": 655360,
        "height": 466033,
        "depth": 0
      },
      "111": {
        "width": 589824,
        "height": 466033,
        "depth": 0
      },
      "112": {
        "width": 655360,
        "height": 466033,
        "depth": 203889
      },
      "113": {
        "width": 622592,
        "height": 466033,
        "depth": 203889
      },
      "114": {
        "width": 481812,
        "height": 466033,
        "depth": 0
      },
      "115": {
        "width": 465307,
        "height": 466033,
        "depth": 0
      },
      "116": {
        "width": 458752,
        "height": 665763,
        "depth": 0
      },
      "117": {
        "width": 655360,
        "height": 466033,
        "depth": 0
      },
      "118": {
        "width": 622592,
        "height": 466033,
        "depth": 0
      },
      "119": {
        "width": 851968,
        "height": 466033,
        "depth": 0
      },
      "120": {
        "width": 622592,
        "height": 466033,
        "depth": 0
      },
      "121": {
        "width": 622592,
        "height": 466033,
        "depth": 203889
      },
      "122": {
        "width": 524288,
        "height": 466033,
        "depth": 0
      },
      "123": {
        "width": 589824,
        "height": 466033,
        "depth": 0
      },
      "124": {
        "width": 1179648,
        "height": 466033,
        "depth": 0
      },
      "125": {
        "width": 589824,
        "height": 728177,
        "depth": 0
      },
      "126": {
        "width": 589824,
        "height": 728177,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 12582912
  },
  "cmbx5": {
    "characters": {
      "0": {
        "width": 940787,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1301958,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 1217491,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 1083507,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 1048557,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1208752,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 1133024,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 1217491,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 1133024,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 1217491,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 1133024,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 956806,
        "height": 728179,
        "depth": 0
      },
      "12": {
        "width": 914573,
        "height": 728179,
        "depth": 0
      },
      "13": {
        "width": 914573,
        "height": 728179,
        "depth": 0
      },
      "14": {
        "width": 1371859,
        "height": 728179,
        "depth": 0
      },
      "15": {
        "width": 1371859,
        "height": 728179,
        "depth": 0
      },
      "16": {
        "width": 457286,
        "height": 466035,
        "depth": 0
      },
      "17": {
        "width": 499520,
        "height": 466035,
        "depth": 203891
      },
      "18": {
        "width": 795155,
        "height": 728179,
        "depth": 0
      },
      "19": {
        "width": 795155,
        "height": 728179,
        "depth": 0
      },
      "20": {
        "width": 795155,
        "height": 662643,
        "depth": 0
      },
      "21": {
        "width": 795155,
        "height": 728179,
        "depth": 0
      },
      "22": {
        "width": 795155,
        "height": 645456,
        "depth": 0
      },
      "23": {
        "width": 1167974,
        "height": 728179,
        "depth": 0
      },
      "24": {
        "width": 710688,
        "height": 0,
        "depth": 178406
      },
      "25": {
        "width": 799533,
        "height": 728179,
        "depth": 0
      },
      "26": {
        "width": 1133024,
        "height": 466035,
        "depth": 0
      },
      "27": {
        "width": 1217491,
        "height": 466035,
        "depth": 0
      },
      "28": {
        "width": 795155,
        "height": 567981,
        "depth": 101946
      },
      "29": {
        "width": 1403901,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1572835,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 1217491,
        "height": 770413,
        "depth": 50973
      },
      "32": {
        "width": 457286,
        "height": 466035,
        "depth": 0
      },
      "33": {
        "width": 498064,
        "height": 728179,
        "depth": 0
      },
      "34": {
        "width": 806810,
        "height": 728179,
        "depth": 0
      },
      "35": {
        "width": 1301958,
        "height": 728179,
        "depth": 203891
      },
      "36": {
        "width": 795155,
        "height": 786432,
        "depth": 58253
      },
      "37": {
        "width": 1301958,
        "height": 786432,
        "depth": 58253
      },
      "38": {
        "width": 1217491,
        "height": 728179,
        "depth": 0
      },
      "39": {
        "width": 457286,
        "height": 728179,
        "depth": 0
      },
      "40": {
        "width": 626221,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 626221,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 795155,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 1217491,
        "height": 768947,
        "depth": 244659
      },
      "44": {
        "width": 457286,
        "height": 180589,
        "depth": 203891
      },
      "45": {
        "width": 541754,
        "height": 466035,
        "depth": 0
      },
      "46": {
        "width": 457286,
        "height": 180589,
        "depth": 0
      },
      "47": {
        "width": 795155,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "49": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "50": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "51": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "52": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "53": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "54": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "56": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 795155,
        "height": 675750,
        "depth": 0
      },
      "58": {
        "width": 457286,
        "height": 466035,
        "depth": 0
      },
      "59": {
        "width": 457286,
        "height": 466035,
        "depth": 203891
      },
      "60": {
        "width": 498064,
        "height": 524288,
        "depth": 203891
      },
      "61": {
        "width": 1217491,
        "height": 447043,
        "depth": 4294890051
      },
      "62": {
        "width": 752922,
        "height": 524288,
        "depth": 203891
      },
      "63": {
        "width": 752922,
        "height": 728179,
        "depth": 0
      },
      "64": {
        "width": 1217491,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1167974,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 1108266,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 1133024,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1192733,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 1025254,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 983021,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 1226230,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1208752,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 576704,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 812630,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1210208,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 940787,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1462154,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1208752,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 1176714,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 1066032,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1176714,
        "height": 719440,
        "depth": 203891
      },
      "82": {
        "width": 1156326,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 879622,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 1092246,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 1188365,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 1167974,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1590310,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1167974,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 1167974,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 964090,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 460141,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 806810,
        "height": 728179,
        "depth": 0
      },
      "93": {
        "width": 460141,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 795155,
        "height": 728179,
        "depth": 0
      },
      "95": {
        "width": 457286,
        "height": 728179,
        "depth": 0
      },
      "96": {
        "width": 457286,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 774038,
        "height": 466035,
        "depth": 0
      },
      "98": {
        "width": 879622,
        "height": 728179,
        "depth": 0
      },
      "99": {
        "width": 710688,
        "height": 466035,
        "depth": 0
      },
      "100": {
        "width": 879622,
        "height": 728179,
        "depth": 0
      },
      "101": {
        "width": 725981,
        "height": 466035,
        "depth": 0
      },
      "102": {
        "width": 499520,
        "height": 728179,
        "depth": 0
      },
      "103": {
        "width": 795155,
        "height": 466035,
        "depth": 203891
      },
      "104": {
        "width": 879622,
        "height": 728179,
        "depth": 0
      },
      "105": {
        "width": 457286,
        "height": 728179,
        "depth": 0
      },
      "106": {
        "width": 499520,
        "height": 728179,
        "depth": 203891
      },
      "107": {
        "width": 837389,
        "height": 728179,
        "depth": 0
      },
      "108": {
        "width": 457286,
        "height": 728179,
        "depth": 0
      },
      "109": {
        "width": 1301958,
        "height": 466035,
        "depth": 0
      },
      "110": {
        "width": 879622,
        "height": 466035,
        "depth": 0
      },
      "111": {
        "width": 795155,
        "height": 466035,
        "depth": 0
      },
      "112": {
        "width": 879622,
        "height": 466035,
        "depth": 203891
      },
      "113": {
        "width": 837389,
        "height": 466035,
        "depth": 203891
      },
      "114": {
        "width": 652438,
        "height": 466035,
        "depth": 0
      },
      "115": {
        "width": 634669,
        "height": 466035,
        "depth": 0
      },
      "116": {
        "width": 626221,
        "height": 665766,
        "depth": 0
      },
      "117": {
        "width": 879622,
        "height": 466035,
        "depth": 0
      },
      "118": {
        "width": 837389,
        "height": 466035,
        "depth": 0
      },
      "119": {
        "width": 1133024,
        "height": 466035,
        "depth": 0
      },
      "120": {
        "width": 837389,
        "height": 466035,
        "depth": 0
      },
      "121": {
        "width": 837389,
        "height": 466035,
        "depth": 203891
      },
      "122": {
        "width": 710688,
        "height": 466035,
        "depth": 0
      },
      "123": {
        "width": 795155,
        "height": 466035,
        "depth": 0
      },
      "124": {
        "width": 1590310,
        "height": 466035,
        "depth": 0
      },
      "125": {
        "width": 795155,
        "height": 728179,
        "depth": 0
      },
      "126": {
        "width": 795155,
        "height": 728179,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 5242880
  },
  "cmbx6": {
    "characters": {
      "0": {
        "width": 865072,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1199059,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 1120416,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 1000027,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 963131,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1116533,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 1041773,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 1120416,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 1041773,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 1120416,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 1041773,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 864587,
        "height": 728179,
        "depth": 0
      },
      "12": {
        "width": 825264,
        "height": 728179,
        "depth": 0
      },
      "13": {
        "width": 825264,
        "height": 728179,
        "depth": 0
      },
      "14": {
        "width": 1237896,
        "height": 728179,
        "depth": 0
      },
      "15": {
        "width": 1237896,
        "height": 728179,
        "depth": 0
      },
      "16": {
        "width": 412632,
        "height": 466035,
        "depth": 0
      },
      "17": {
        "width": 451955,
        "height": 466035,
        "depth": 203891
      },
      "18": {
        "width": 727203,
        "height": 728179,
        "depth": 0
      },
      "19": {
        "width": 727203,
        "height": 728179,
        "depth": 0
      },
      "20": {
        "width": 727203,
        "height": 662643,
        "depth": 0
      },
      "21": {
        "width": 727203,
        "height": 728179,
        "depth": 0
      },
      "22": {
        "width": 727203,
        "height": 640795,
        "depth": 0
      },
      "23": {
        "width": 1078669,
        "height": 728179,
        "depth": 0
      },
      "24": {
        "width": 648560,
        "height": 0,
        "depth": 178405
      },
      "25": {
        "width": 742256,
        "height": 728179,
        "depth": 0
      },
      "26": {
        "width": 1041773,
        "height": 466035,
        "depth": 0
      },
      "27": {
        "width": 1120416,
        "height": 466035,
        "depth": 0
      },
      "28": {
        "width": 727203,
        "height": 567981,
        "depth": 101947
      },
      "29": {
        "width": 1296149,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1453435,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 1120416,
        "height": 770413,
        "depth": 50973
      },
      "32": {
        "width": 412632,
        "height": 466035,
        "depth": 0
      },
      "33": {
        "width": 450496,
        "height": 728179,
        "depth": 0
      },
      "34": {
        "width": 744680,
        "height": 728179,
        "depth": 0
      },
      "35": {
        "width": 1199059,
        "height": 728179,
        "depth": 203888
      },
      "36": {
        "width": 727203,
        "height": 786432,
        "depth": 58253
      },
      "37": {
        "width": 1199059,
        "height": 786432,
        "depth": 58253
      },
      "38": {
        "width": 1120416,
        "height": 728179,
        "depth": 0
      },
      "39": {
        "width": 412632,
        "height": 728179,
        "depth": 0
      },
      "40": {
        "width": 569917,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 569917,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 727203,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 1120416,
        "height": 734000,
        "depth": 209712
      },
      "44": {
        "width": 412632,
        "height": 174763,
        "depth": 203891
      },
      "45": {
        "width": 491275,
        "height": 466035,
        "depth": 0
      },
      "46": {
        "width": 412632,
        "height": 174763,
        "depth": 0
      },
      "47": {
        "width": 727203,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 727203,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 412632,
        "height": 466035,
        "depth": 0
      },
      "59": {
        "width": 412632,
        "height": 466035,
        "depth": 203891
      },
      "60": {
        "width": 450496,
        "height": 524288,
        "depth": 203891
      },
      "61": {
        "width": 1120416,
        "height": 438365,
        "depth": 4294881373
      },
      "62": {
        "width": 687883,
        "height": 524288,
        "depth": 203891
      },
      "63": {
        "width": 687883,
        "height": 728179,
        "depth": 0
      },
      "64": {
        "width": 1120416,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1078669,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 1020901,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 1041773,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1099544,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 943715,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 904395,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 1129640,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1116533,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 531085,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 745651,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1117992,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 865072,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1352461,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1116533,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 1082552,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 981579,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1082552,
        "height": 719440,
        "depth": 203891
      },
      "82": {
        "width": 1068960,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 805845,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 1003909,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 1097603,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 1078669,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1471883,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1078669,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 1078669,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 884488,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 414381,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 744680,
        "height": 728179,
        "depth": 0
      },
      "93": {
        "width": 414381,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 727203,
        "height": 728179,
        "depth": 0
      },
      "95": {
        "width": 412632,
        "height": 728179,
        "depth": 0
      },
      "96": {
        "width": 412632,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 707541,
        "height": 466035,
        "depth": 0
      },
      "98": {
        "width": 805845,
        "height": 728179,
        "depth": 0
      },
      "99": {
        "width": 648560,
        "height": 466035,
        "depth": 0
      },
      "100": {
        "width": 805845,
        "height": 728179,
        "depth": 0
      },
      "101": {
        "width": 664824,
        "height": 466035,
        "depth": 0
      },
      "102": {
        "width": 451955,
        "height": 728179,
        "depth": 0
      },
      "103": {
        "width": 727203,
        "height": 466035,
        "depth": 203891
      },
      "104": {
        "width": 805845,
        "height": 728179,
        "depth": 0
      },
      "105": {
        "width": 412632,
        "height": 728179,
        "depth": 0
      },
      "106": {
        "width": 451955,
        "height": 728179,
        "depth": 203891
      },
      "107": {
        "width": 766525,
        "height": 728179,
        "depth": 0
      },
      "108": {
        "width": 412632,
        "height": 728179,
        "depth": 0
      },
      "109": {
        "width": 1199059,
        "height": 466035,
        "depth": 0
      },
      "110": {
        "width": 805845,
        "height": 466035,
        "depth": 0
      },
      "111": {
        "width": 727203,
        "height": 466035,
        "depth": 0
      },
      "112": {
        "width": 805845,
        "height": 466035,
        "depth": 203891
      },
      "113": {
        "width": 766523,
        "height": 466035,
        "depth": 203891
      },
      "114": {
        "width": 598805,
        "height": 466035,
        "depth": 0
      },
      "115": {
        "width": 577781,
        "height": 466035,
        "depth": 0
      },
      "116": {
        "width": 569917,
        "height": 665765,
        "depth": 0
      },
      "117": {
        "width": 805845,
        "height": 466035,
        "depth": 0
      },
      "118": {
        "width": 766525,
        "height": 466035,
        "depth": 0
      },
      "119": {
        "width": 1041773,
        "height": 466035,
        "depth": 0
      },
      "120": {
        "width": 766525,
        "height": 466035,
        "depth": 0
      },
      "121": {
        "width": 766525,
        "height": 466035,
        "depth": 203891
      },
      "122": {
        "width": 648560,
        "height": 466035,
        "depth": 0
      },
      "123": {
        "width": 727203,
        "height": 466035,
        "depth": 0
      },
      "124": {
        "width": 1454405,
        "height": 466035,
        "depth": 0
      },
      "125": {
        "width": 727203,
        "height": 728179,
        "depth": 0
      },
      "126": {
        "width": 727203,
        "height": 728179,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 6291456
  },
  "cmbx7": {
    "characters": {
      "0": {
        "width": 810981,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1125554,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 1051072,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 940389,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 902107,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1050656,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 976590,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 1051072,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 976590,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 1051072,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 976590,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 798704,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 761463,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 761463,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1142194,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1142194,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 380731,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 417973,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 678661,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 678661,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 678661,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 678661,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 678661,
        "height": 635803,
        "depth": 0
      },
      "23": {
        "width": 1014871,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 604178,
        "height": 0,
        "depth": 178405
      },
      "25": {
        "width": 701339,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 976590,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 1051072,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 678661,
        "height": 567979,
        "depth": 101945
      },
      "29": {
        "width": 1219177,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1368142,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 1051072,
        "height": 770414,
        "depth": 50974
      },
      "32": {
        "width": 380731,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 416517,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 700297,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1125554,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 678661,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 1125554,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 1051072,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 380731,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 529696,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 529696,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 678661,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 1051072,
        "height": 709038,
        "depth": 184750
      },
      "44": {
        "width": 380731,
        "height": 170601,
        "depth": 203890
      },
      "45": {
        "width": 455214,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 380731,
        "height": 170601,
        "depth": 0
      },
      "47": {
        "width": 678661,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 678661,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 380731,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 380731,
        "height": 466034,
        "depth": 203890
      },
      "60": {
        "width": 416517,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 1051072,
        "height": 430832,
        "depth": 4294873840
      },
      "62": {
        "width": 641419,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 641419,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 1051072,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 1014871,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 958489,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 976590,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1032971,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 885463,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 848222,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 1060642,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1050656,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 502649,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 697801,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1052112,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 810981,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1274103,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1050656,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 1015287,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 921248,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1015287,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 1006549,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 753143,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 940805,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 1032763,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 1014871,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1387282,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1014871,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 1014871,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 827625,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 381689,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 700297,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 381689,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 678661,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 380731,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 380731,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 660039,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 753143,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 604178,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 753143,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 621136,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 417973,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 678661,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 753143,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 380731,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 417973,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 715902,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 380731,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1125554,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 753143,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 678661,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 753143,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 715902,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 560489,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 537145,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 529696,
        "height": 665765,
        "depth": 0
      },
      "117": {
        "width": 753143,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 715902,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 976590,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 715902,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 715902,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 604178,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 678661,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1357321,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 678661,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 678661,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmbx8": {
    "characters": {
      "0": {
        "width": 770418,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1070430,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 999068,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 895664,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 856344,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1001250,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 927706,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 999068,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 927706,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 999068,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 927706,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 749302,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 713620,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 713620,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1070430,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1070430,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 356810,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 392492,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 642258,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 642258,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 642258,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 642258,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 642258,
        "height": 631328,
        "depth": 0
      },
      "23": {
        "width": 967026,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 570896,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 668834,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 927706,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 999068,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 642258,
        "height": 567980,
        "depth": 101946
      },
      "29": {
        "width": 1161452,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1304176,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 999068,
        "height": 770412,
        "depth": 50972
      },
      "32": {
        "width": 356810,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 391034,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 667014,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1070430,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 642258,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 1070430,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 999068,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 356810,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 499534,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 499534,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 642258,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 999068,
        "height": 690316,
        "depth": 166028
      },
      "44": {
        "width": 356810,
        "height": 167480,
        "depth": 203890
      },
      "45": {
        "width": 428172,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 356810,
        "height": 167480,
        "depth": 0
      },
      "47": {
        "width": 642258,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "49": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "50": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "51": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "52": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "53": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "54": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "56": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "58": {
        "width": 356810,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 356810,
        "height": 466034,
        "depth": 203890
      },
      "60": {
        "width": 391034,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 999068,
        "height": 423364,
        "depth": 4294866372
      },
      "62": {
        "width": 606578,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 606578,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 999068,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 967026,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 911686,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 927706,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 983048,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 841780,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 806100,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 1008898,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1001250,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 481326,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 661918,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1002708,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 770418,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1215336,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1001250,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 964844,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 876004,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 964844,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 959744,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 713620,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 893482,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 984138,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 967026,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1323836,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 967026,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 967026,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 784982,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 356810,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 667014,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 356810,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 642258,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 356810,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 356810,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 624418,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 713620,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 570896,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 713620,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 588372,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 392492,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 642258,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 713620,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 356810,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 392492,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 677940,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 356810,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1070430,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 713620,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 642258,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 713620,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 677938,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 530846,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 506670,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 499534,
        "height": 665764,
        "depth": 0
      },
      "117": {
        "width": 713620,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 677940,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 927706,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 677940,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 677940,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 570896,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 642258,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1284516,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 642258,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 642258,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmbx9": {
    "characters": {
      "0": {
        "width": 745321,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1034000,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 965067,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 867332,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 827200,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 969276,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 896133,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 965067,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 896133,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 965067,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 896133,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 723801,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 689333,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 689333,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1034000,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1034000,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 344667,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 379134,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 620400,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 620400,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 620400,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 620400,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 620400,
        "height": 627851,
        "depth": 0
      },
      "23": {
        "width": 936265,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 551467,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 646782,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 896133,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 965067,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 620400,
        "height": 567979,
        "depth": 101945
      },
      "29": {
        "width": 1122999,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1260866,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 965067,
        "height": 770412,
        "depth": 50972
      },
      "32": {
        "width": 344667,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 377678,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 647588,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1034000,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 620400,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 1034000,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 965067,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 344667,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 482533,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 482533,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 620400,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 965067,
        "height": 675744,
        "depth": 151456
      },
      "44": {
        "width": 344667,
        "height": 165054,
        "depth": 203890
      },
      "45": {
        "width": 413600,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 344667,
        "height": 165054,
        "depth": 0
      },
      "47": {
        "width": 620400,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 344667,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 344667,
        "height": 466034,
        "depth": 203890
      },
      "60": {
        "width": 377678,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 965067,
        "height": 416519,
        "depth": 4294859527
      },
      "62": {
        "width": 585934,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 585934,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 965067,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 936265,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 881733,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 896133,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 950667,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 814254,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 779788,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 975100,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 969276,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 467973,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 640466,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 970732,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 745321,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1176076,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 969276,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 932055,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 847266,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 932055,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 929794,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 689333,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 863122,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 952772,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 936265,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1280932,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 936265,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 936265,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 758267,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 344667,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 647588,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 344667,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 620400,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 344667,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 344667,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 603166,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 689333,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 551467,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 689333,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 569349,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 379134,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 620400,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 689333,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 344667,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 379134,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 654868,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 344667,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1034000,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 689333,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 620400,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 689333,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 654866,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 511826,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 489428,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 482533,
        "height": 665764,
        "depth": 0
      },
      "117": {
        "width": 689333,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 654868,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 896133,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 654868,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 654868,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 551467,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 620400,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1240800,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 620400,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 620400,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmbxsl10": {
    "characters": {
      "0": {
        "width": 725261,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1004880,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 937888,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 844682,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 803904,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 943714,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 937888,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 937888,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 703416,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1004880,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1004880,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 334960,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 368456,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 602928,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 602928,
        "height": 625066,
        "depth": 0
      },
      "23": {
        "width": 969930,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 535936,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 626230,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 870896,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 937888,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 602928,
        "height": 567979,
        "depth": 101946
      },
      "29": {
        "width": 1092261,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1226245,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 937888,
        "height": 770413,
        "depth": 50973
      },
      "32": {
        "width": 334960,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 367000,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 632056,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1004880,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 602928,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 1004880,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 937888,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 468944,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 468944,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 602928,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 937888,
        "height": 664096,
        "depth": 139808
      },
      "44": {
        "width": 334960,
        "height": 163112,
        "depth": 203890
      },
      "45": {
        "width": 401952,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 334960,
        "height": 163112,
        "depth": 0
      },
      "47": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 334960,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 334960,
        "height": 466034,
        "depth": 203890
      },
      "60": {
        "width": 367000,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 937888,
        "height": 410110,
        "depth": 4294853118
      },
      "62": {
        "width": 569432,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 569432,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 937888,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 857789,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 870896,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 924781,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 792253,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 758757,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 948083,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 943714,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 457294,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 623317,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 945170,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 725261,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1144690,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 943714,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 905848,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 824293,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 905848,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 904392,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 669920,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 838856,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 927694,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1246634,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 736912,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 334960,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 632056,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 334960,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 586179,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 535936,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 552685,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 368456,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 602928,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 669920,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 368456,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 636424,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 334960,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1004880,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 669920,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 602928,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 669920,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 636424,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 496616,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 475643,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 468944,
        "height": 665763,
        "depth": 0
      },
      "117": {
        "width": 669920,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 636424,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 870896,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 636424,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 636424,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 535936,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 602928,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1205856,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 602928,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmbxti10": {
    "characters": {
      "0": {
        "width": 731666,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 990312,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 928563,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 845843,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 805066,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 939632,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 866814,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 928563,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 866814,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 928563,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 866814,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 792256,
        "height": 728178,
        "depth": 203890
      },
      "12": {
        "width": 707205,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 738078,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 1095466,
        "height": 728178,
        "depth": 203890
      },
      "15": {
        "width": 1110902,
        "height": 728178,
        "depth": 203890
      },
      "16": {
        "width": 372824,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 403699,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 619819,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 619819,
        "height": 623318,
        "depth": 0
      },
      "23": {
        "width": 994973,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 558070,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 697302,
        "height": 728178,
        "depth": 203890
      },
      "26": {
        "width": 866814,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 866814,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 619819,
        "height": 567979,
        "depth": 101946
      },
      "29": {
        "width": 1072450,
        "height": 719440,
        "depth": 0
      },
      "30": {
        "width": 1195947,
        "height": 719440,
        "depth": 0
      },
      "31": {
        "width": 928563,
        "height": 770413,
        "depth": 50973
      },
      "32": {
        "width": 311075,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 404864,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 650696,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 990312,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 910723,
        "height": 728178,
        "depth": 0
      },
      "37": {
        "width": 990312,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 928563,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 372824,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 496322,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 496322,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 619819,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 928563,
        "height": 632637,
        "depth": 108349
      },
      "44": {
        "width": 372824,
        "height": 154374,
        "depth": 203890
      },
      "45": {
        "width": 434573,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 372824,
        "height": 154374,
        "depth": 0
      },
      "47": {
        "width": 619819,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 619819,
        "height": 675749,
        "depth": 203890
      },
      "53": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 619819,
        "height": 675749,
        "depth": 203890
      },
      "56": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 619819,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 372824,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 372824,
        "height": 466034,
        "depth": 203890
      },
      "60": {
        "width": 404864,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 928563,
        "height": 410110,
        "depth": 4294853118
      },
      "62": {
        "width": 619819,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 928563,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 907592,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 856330,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 866814,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 918078,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 793414,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 762541,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 938758,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 939632,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 494576,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 640208,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 938467,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 731666,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1124878,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 939632,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 896523,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 825454,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 896523,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 901186,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 681568,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 834774,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 923613,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 907592,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1216336,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 907592,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 907592,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 743317,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 373408,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 650696,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 373408,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 372824,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 372824,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 619819,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 558070,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 558070,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 558070,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 419432,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 558070,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 372824,
        "height": 726931,
        "depth": 0
      },
      "106": {
        "width": 372824,
        "height": 726931,
        "depth": 203890
      },
      "107": {
        "width": 558070,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 311075,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 990312,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 681568,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 619819,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 619819,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 558070,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 526034,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 510595,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 403699,
        "height": 665763,
        "depth": 0
      },
      "117": {
        "width": 650694,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 558070,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 805066,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 587782,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 588946,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 514382,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 619819,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1239638,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 619819,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmcsc10": {
    "characters": {
      "0": {
        "width": 716520,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 946622,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 885456,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 792250,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 763123,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 824290,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 885456,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 824290,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 885456,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 824290,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 579624,
        "height": 728178,
        "depth": 203888
      },
      "12": {
        "width": 579624,
        "height": 728178,
        "depth": 203888
      },
      "13": {
        "width": 334958,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 334958,
        "height": 524288,
        "depth": 203890
      },
      "15": {
        "width": 549042,
        "height": 524288,
        "depth": 203890
      },
      "16": {
        "width": 316902,
        "height": 538851,
        "depth": 0
      },
      "17": {
        "width": 445061,
        "height": 538851,
        "depth": 0
      },
      "18": {
        "width": 579624,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 579624,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 579624,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 579624,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 579624,
        "height": 592590,
        "depth": 0
      },
      "23": {
        "width": 853416,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 518458,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 960026,
        "height": 538851,
        "depth": 0
      },
      "26": {
        "width": 771283,
        "height": 538851,
        "depth": 0
      },
      "27": {
        "width": 864490,
        "height": 538851,
        "depth": 0
      },
      "28": {
        "width": 666426,
        "height": 592590,
        "depth": 50973
      },
      "29": {
        "width": 1022352,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1144685,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 885456,
        "height": 767499,
        "depth": 50973
      },
      "32": {
        "width": 334958,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 334958,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 579624,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 946622,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 579624,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 946622,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 885456,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 334958,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 457291,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 457291,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 579624,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 885456,
        "height": 629142,
        "depth": 104854
      },
      "44": {
        "width": 334958,
        "height": 110683,
        "depth": 203890
      },
      "45": {
        "width": 396125,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 334958,
        "height": 110683,
        "depth": 0
      },
      "47": {
        "width": 579624,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 579624,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 334958,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 334958,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 885456,
        "height": 565285,
        "depth": 40997
      },
      "61": {
        "width": 885456,
        "height": 384696,
        "depth": 4294827704
      },
      "62": {
        "width": 885456,
        "height": 565285,
        "depth": 40997
      },
      "63": {
        "width": 549042,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 885456,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 808270,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 824290,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 869437,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 777686,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 747104,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 892738,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 425251,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 594187,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 884000,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 716520,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1036915,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 885456,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 777686,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 885456,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 838853,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 640790,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 824290,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1159248,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 853416,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 701957,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 334958,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 579624,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 334958,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 579624,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 334958,
        "height": 700301,
        "depth": 0
      },
      "96": {
        "width": 334958,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 643125,
        "height": 538851,
        "depth": 0
      },
      "98": {
        "width": 608173,
        "height": 538851,
        "depth": 0
      },
      "99": {
        "width": 619822,
        "height": 538851,
        "depth": 0
      },
      "100": {
        "width": 654776,
        "height": 538851,
        "depth": 0
      },
      "101": {
        "width": 584870,
        "height": 538851,
        "depth": 0
      },
      "102": {
        "width": 561570,
        "height": 538851,
        "depth": 0
      },
      "103": {
        "width": 672251,
        "height": 538851,
        "depth": 0
      },
      "104": {
        "width": 643125,
        "height": 538851,
        "depth": 0
      },
      "105": {
        "width": 316902,
        "height": 538851,
        "depth": 0
      },
      "106": {
        "width": 445061,
        "height": 538851,
        "depth": 0
      },
      "107": {
        "width": 666427,
        "height": 538851,
        "depth": 0
      },
      "108": {
        "width": 538267,
        "height": 538851,
        "depth": 0
      },
      "109": {
        "width": 782934,
        "height": 538851,
        "depth": 0
      },
      "110": {
        "width": 643125,
        "height": 538851,
        "depth": 0
      },
      "111": {
        "width": 666426,
        "height": 538851,
        "depth": 0
      },
      "112": {
        "width": 584870,
        "height": 538851,
        "depth": 0
      },
      "113": {
        "width": 666426,
        "height": 538851,
        "depth": 151461
      },
      "114": {
        "width": 631474,
        "height": 538851,
        "depth": 0
      },
      "115": {
        "width": 480013,
        "height": 538851,
        "depth": 0
      },
      "116": {
        "width": 619822,
        "height": 538851,
        "depth": 0
      },
      "117": {
        "width": 643125,
        "height": 538851,
        "depth": 0
      },
      "118": {
        "width": 643125,
        "height": 538851,
        "depth": 0
      },
      "119": {
        "width": 876141,
        "height": 538851,
        "depth": 0
      },
      "120": {
        "width": 643125,
        "height": 538851,
        "depth": 0
      },
      "121": {
        "width": 643125,
        "height": 538851,
        "depth": 0
      },
      "122": {
        "width": 526616,
        "height": 538851,
        "depth": 0
      },
      "123": {
        "width": 579624,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1159248,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 579624,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 579624,
        "height": 700301,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmcsc8": {
    "characters": {
      "0": {
        "width": 754028,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 1001244,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 935708,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 834492,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 804636,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 870172,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 935708,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 870172,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 935708,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 870172,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 608028,
        "height": 728178,
        "depth": 203888
      },
      "12": {
        "width": 608028,
        "height": 728178,
        "depth": 203888
      },
      "13": {
        "width": 345884,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 345884,
        "height": 524288,
        "depth": 203890
      },
      "15": {
        "width": 575260,
        "height": 524288,
        "depth": 203890
      },
      "16": {
        "width": 332780,
        "height": 530842,
        "depth": 0
      },
      "17": {
        "width": 464946,
        "height": 530842,
        "depth": 0
      },
      "18": {
        "width": 608028,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 608028,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 608028,
        "height": 657180,
        "depth": 0
      },
      "21": {
        "width": 608028,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 608028,
        "height": 601474,
        "depth": 0
      },
      "23": {
        "width": 900028,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 542492,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 1004896,
        "height": 530842,
        "depth": 0
      },
      "26": {
        "width": 796270,
        "height": 530842,
        "depth": 0
      },
      "27": {
        "width": 890934,
        "height": 530842,
        "depth": 0
      },
      "28": {
        "width": 691776,
        "height": 581814,
        "depth": 50972
      },
      "29": {
        "width": 1081708,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1212780,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 935708,
        "height": 767498,
        "depth": 50972
      },
      "32": {
        "width": 345884,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 345884,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 608028,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1001244,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 608028,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 1001244,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 935708,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 345884,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 476956,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 476956,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 608028,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 935708,
        "height": 657180,
        "depth": 131072
      },
      "44": {
        "width": 345884,
        "height": 116508,
        "depth": 203890
      },
      "45": {
        "width": 411420,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 345884,
        "height": 116508,
        "depth": 0
      },
      "47": {
        "width": 608028,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "49": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "50": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "51": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "52": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "53": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "54": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "56": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 608028,
        "height": 675750,
        "depth": 0
      },
      "58": {
        "width": 345884,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 345884,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 935708,
        "height": 590480,
        "depth": 66192
      },
      "61": {
        "width": 935708,
        "height": 396348,
        "depth": 4294839356
      },
      "62": {
        "width": 935708,
        "height": 590480,
        "depth": 66192
      },
      "63": {
        "width": 575260,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 935708,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 852332,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 870172,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 917868,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 819564,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 786796,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 943172,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 441276,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 622956,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 932796,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 754028,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1096636,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 935708,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 819564,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 935708,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 885100,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 673564,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 870172,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1227708,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 900028,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 739100,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 345884,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 608028,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 345884,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 608028,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 345884,
        "height": 703212,
        "depth": 0
      },
      "96": {
        "width": 345884,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 664104,
        "height": 530842,
        "depth": 0
      },
      "98": {
        "width": 630608,
        "height": 530842,
        "depth": 0
      },
      "99": {
        "width": 644444,
        "height": 530842,
        "depth": 0
      },
      "100": {
        "width": 677940,
        "height": 530842,
        "depth": 0
      },
      "101": {
        "width": 606942,
        "height": 530842,
        "depth": 0
      },
      "102": {
        "width": 583276,
        "height": 530842,
        "depth": 0
      },
      "103": {
        "width": 696692,
        "height": 530842,
        "depth": 0
      },
      "104": {
        "width": 664104,
        "height": 530842,
        "depth": 0
      },
      "105": {
        "width": 332780,
        "height": 530842,
        "depth": 0
      },
      "106": {
        "width": 464946,
        "height": 530842,
        "depth": 0
      },
      "107": {
        "width": 687770,
        "height": 530842,
        "depth": 0
      },
      "108": {
        "width": 559610,
        "height": 530842,
        "depth": 0
      },
      "109": {
        "width": 806100,
        "height": 530842,
        "depth": 0
      },
      "110": {
        "width": 664104,
        "height": 530842,
        "depth": 0
      },
      "111": {
        "width": 691776,
        "height": 530842,
        "depth": 0
      },
      "112": {
        "width": 606942,
        "height": 530842,
        "depth": 0
      },
      "113": {
        "width": 691776,
        "height": 530842,
        "depth": 138354
      },
      "114": {
        "width": 654274,
        "height": 530842,
        "depth": 0
      },
      "115": {
        "width": 502448,
        "height": 530842,
        "depth": 0
      },
      "116": {
        "width": 644444,
        "height": 530842,
        "depth": 0
      },
      "117": {
        "width": 664104,
        "height": 530842,
        "depth": 0
      },
      "118": {
        "width": 664104,
        "height": 530842,
        "depth": 0
      },
      "119": {
        "width": 900764,
        "height": 530842,
        "depth": 0
      },
      "120": {
        "width": 664104,
        "height": 530842,
        "depth": 0
      },
      "121": {
        "width": 664104,
        "height": 530842,
        "depth": 0
      },
      "122": {
        "width": 549780,
        "height": 530842,
        "depth": 0
      },
      "123": {
        "width": 608028,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1216056,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 608028,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 608028,
        "height": 703212,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmcsc9": {
    "characters": {
      "0": {
        "width": 713931,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 945003,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 883513,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 790308,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 760533,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 822023,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 883513,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 822023,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 883513,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 822023,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 576064,
        "height": 728178,
        "depth": 203890
      },
      "12": {
        "width": 576064,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 330105,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 330105,
        "height": 524288,
        "depth": 203890
      },
      "15": {
        "width": 545319,
        "height": 524288,
        "depth": 203890
      },
      "16": {
        "width": 326222,
        "height": 551474,
        "depth": 0
      },
      "17": {
        "width": 455996,
        "height": 551474,
        "depth": 0
      },
      "18": {
        "width": 576064,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 576064,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 576064,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 576064,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 576064,
        "height": 600421,
        "depth": 0
      },
      "23": {
        "width": 851797,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 514574,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 983836,
        "height": 551474,
        "depth": 0
      },
      "26": {
        "width": 784480,
        "height": 551474,
        "depth": 0
      },
      "27": {
        "width": 878332,
        "height": 551474,
        "depth": 0
      },
      "28": {
        "width": 679623,
        "height": 600421,
        "depth": 50972
      },
      "29": {
        "width": 1021380,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1144359,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 883513,
        "height": 767499,
        "depth": 50972
      },
      "32": {
        "width": 330105,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 330105,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 576064,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 945003,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 576064,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 945003,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 883513,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 330105,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 453084,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 453084,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 576064,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 883513,
        "height": 631083,
        "depth": 106795
      },
      "44": {
        "width": 330105,
        "height": 113273,
        "depth": 203890
      },
      "45": {
        "width": 391595,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 330105,
        "height": 113273,
        "depth": 0
      },
      "47": {
        "width": 576064,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 576064,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 330105,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 330105,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 883513,
        "height": 577737,
        "depth": 53449
      },
      "61": {
        "width": 883513,
        "height": 390377,
        "depth": 4294833385
      },
      "62": {
        "width": 883513,
        "height": 577737,
        "depth": 53449
      },
      "63": {
        "width": 545319,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 883513,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 806165,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 822023,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 867655,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 775420,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 744676,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 890956,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 421369,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 590951,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 882542,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 713931,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1036267,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 883513,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 775420,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 883513,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 836910,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 637554,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 822023,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1159246,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 851797,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 699044,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 330105,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 576064,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 330105,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 576064,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 330105,
        "height": 701595,
        "depth": 0
      },
      "96": {
        "width": 330105,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 654706,
        "height": 551474,
        "depth": 0
      },
      "98": {
        "width": 620238,
        "height": 551474,
        "depth": 0
      },
      "99": {
        "width": 632697,
        "height": 551474,
        "depth": 0
      },
      "100": {
        "width": 667164,
        "height": 551474,
        "depth": 0
      },
      "101": {
        "width": 596775,
        "height": 551474,
        "depth": 0
      },
      "102": {
        "width": 573312,
        "height": 551474,
        "depth": 0
      },
      "103": {
        "width": 685125,
        "height": 551474,
        "depth": 0
      },
      "104": {
        "width": 654706,
        "height": 551474,
        "depth": 0
      },
      "105": {
        "width": 326222,
        "height": 551474,
        "depth": 0
      },
      "106": {
        "width": 455996,
        "height": 551474,
        "depth": 0
      },
      "107": {
        "width": 678169,
        "height": 551474,
        "depth": 0
      },
      "108": {
        "width": 549849,
        "height": 551474,
        "depth": 0
      },
      "109": {
        "width": 795484,
        "height": 551474,
        "depth": 0
      },
      "110": {
        "width": 654706,
        "height": 551474,
        "depth": 0
      },
      "111": {
        "width": 679623,
        "height": 551474,
        "depth": 0
      },
      "112": {
        "width": 596775,
        "height": 551474,
        "depth": 0
      },
      "113": {
        "width": 679623,
        "height": 551474,
        "depth": 145636
      },
      "114": {
        "width": 643701,
        "height": 551474,
        "depth": 0
      },
      "115": {
        "width": 491918,
        "height": 551474,
        "depth": 0
      },
      "116": {
        "width": 632697,
        "height": 551474,
        "depth": 0
      },
      "117": {
        "width": 654706,
        "height": 551474,
        "depth": 0
      },
      "118": {
        "width": 654706,
        "height": 551474,
        "depth": 0
      },
      "119": {
        "width": 889337,
        "height": 551474,
        "depth": 0
      },
      "120": {
        "width": 654706,
        "height": 551474,
        "depth": 0
      },
      "121": {
        "width": 654706,
        "height": 551474,
        "depth": 0
      },
      "122": {
        "width": 538844,
        "height": 551474,
        "depth": 0
      },
      "123": {
        "width": 576064,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1152128,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 576064,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 576064,
        "height": 701595,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmdunh10": {
    "characters": {
      "0": {
        "width": 655362,
        "height": 1007798,
        "depth": 0
      },
      "1": {
        "width": 873816,
        "height": 1007798,
        "depth": 0
      },
      "2": {
        "width": 815562,
        "height": 1007798,
        "depth": 0
      },
      "3": {
        "width": 728179,
        "height": 1007798,
        "depth": 0
      },
      "4": {
        "width": 699053,
        "height": 1007798,
        "depth": 0
      },
      "5": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "6": {
        "width": 757307,
        "height": 1007798,
        "depth": 0
      },
      "7": {
        "width": 815562,
        "height": 1007798,
        "depth": 0
      },
      "8": {
        "width": 757307,
        "height": 1007798,
        "depth": 0
      },
      "9": {
        "width": 815562,
        "height": 1007798,
        "depth": 0
      },
      "10": {
        "width": 757307,
        "height": 1007798,
        "depth": 0
      },
      "11": {
        "width": 611672,
        "height": 1019450,
        "depth": 0
      },
      "12": {
        "width": 582544,
        "height": 1019450,
        "depth": 0
      },
      "13": {
        "width": 582544,
        "height": 1019450,
        "depth": 0
      },
      "14": {
        "width": 873816,
        "height": 1019450,
        "depth": 0
      },
      "15": {
        "width": 873816,
        "height": 1019450,
        "depth": 0
      },
      "16": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 320400,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 524290,
        "height": 902941,
        "depth": 0
      },
      "19": {
        "width": 524290,
        "height": 902941,
        "depth": 0
      },
      "20": {
        "width": 524290,
        "height": 790074,
        "depth": 0
      },
      "21": {
        "width": 524290,
        "height": 902941,
        "depth": 0
      },
      "22": {
        "width": 524290,
        "height": 706082,
        "depth": 0
      },
      "23": {
        "width": 786434,
        "height": 1019450,
        "depth": 0
      },
      "24": {
        "width": 466035,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 524291,
        "height": 1019450,
        "depth": 0
      },
      "26": {
        "width": 757307,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 815562,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 524290,
        "height": 553416,
        "depth": 101946
      },
      "29": {
        "width": 946634,
        "height": 1007798,
        "depth": 0
      },
      "30": {
        "width": 1063142,
        "height": 1007798,
        "depth": 0
      },
      "31": {
        "width": 815562,
        "height": 1058771,
        "depth": 50973
      },
      "32": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 291272,
        "height": 1019450,
        "depth": 0
      },
      "34": {
        "width": 524290,
        "height": 1019450,
        "depth": 0
      },
      "35": {
        "width": 873816,
        "height": 1019450,
        "depth": 495162
      },
      "36": {
        "width": 524290,
        "height": 1077702,
        "depth": 58253
      },
      "37": {
        "width": 873816,
        "height": 1077702,
        "depth": 58253
      },
      "38": {
        "width": 815562,
        "height": 1019450,
        "depth": 0
      },
      "39": {
        "width": 291272,
        "height": 1019450,
        "depth": 0
      },
      "40": {
        "width": 407781,
        "height": 1077702,
        "depth": 553414
      },
      "41": {
        "width": 407781,
        "height": 1077702,
        "depth": 553414
      },
      "42": {
        "width": 524290,
        "height": 1077702,
        "depth": 0
      },
      "43": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "44": {
        "width": 291272,
        "height": 110683,
        "depth": 203890
      },
      "45": {
        "width": 349526,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 291272,
        "height": 110683,
        "depth": 0
      },
      "47": {
        "width": 524290,
        "height": 1077702,
        "depth": 553414
      },
      "48": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "49": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "50": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "51": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "52": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "53": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "54": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "55": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "56": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "57": {
        "width": 524290,
        "height": 967021,
        "depth": 0
      },
      "58": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 291272,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 291272,
        "height": 815560,
        "depth": 203890
      },
      "61": {
        "width": 815562,
        "height": 384696,
        "depth": 4294827704
      },
      "62": {
        "width": 495163,
        "height": 815560,
        "depth": 203890
      },
      "63": {
        "width": 495163,
        "height": 1019450,
        "depth": 0
      },
      "64": {
        "width": 815562,
        "height": 1019450,
        "depth": 0
      },
      "65": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "66": {
        "width": 742744,
        "height": 1007798,
        "depth": 0
      },
      "67": {
        "width": 757307,
        "height": 1007798,
        "depth": 0
      },
      "68": {
        "width": 800998,
        "height": 1007798,
        "depth": 0
      },
      "69": {
        "width": 713616,
        "height": 1007798,
        "depth": 0
      },
      "70": {
        "width": 684490,
        "height": 1007798,
        "depth": 0
      },
      "71": {
        "width": 822843,
        "height": 1007798,
        "depth": 0
      },
      "72": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "73": {
        "width": 378653,
        "height": 1007798,
        "depth": 0
      },
      "74": {
        "width": 538853,
        "height": 1007798,
        "depth": 0
      },
      "75": {
        "width": 815562,
        "height": 1007798,
        "depth": 0
      },
      "76": {
        "width": 655362,
        "height": 1007798,
        "depth": 0
      },
      "77": {
        "width": 961197,
        "height": 1007798,
        "depth": 0
      },
      "78": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "79": {
        "width": 815562,
        "height": 1007798,
        "depth": 0
      },
      "80": {
        "width": 713616,
        "height": 1007798,
        "depth": 0
      },
      "81": {
        "width": 815562,
        "height": 1007798,
        "depth": 203890
      },
      "82": {
        "width": 771870,
        "height": 1007798,
        "depth": 0
      },
      "83": {
        "width": 582544,
        "height": 1007798,
        "depth": 0
      },
      "84": {
        "width": 757307,
        "height": 1007798,
        "depth": 0
      },
      "85": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "86": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "87": {
        "width": 1077706,
        "height": 1007798,
        "depth": 0
      },
      "88": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "89": {
        "width": 786434,
        "height": 1007798,
        "depth": 0
      },
      "90": {
        "width": 640798,
        "height": 1007798,
        "depth": 0
      },
      "91": {
        "width": 291272,
        "height": 1077702,
        "depth": 553414
      },
      "92": {
        "width": 524290,
        "height": 1019450,
        "depth": 0
      },
      "93": {
        "width": 291272,
        "height": 1077702,
        "depth": 553414
      },
      "94": {
        "width": 524290,
        "height": 902941,
        "depth": 0
      },
      "95": {
        "width": 291272,
        "height": 706082,
        "depth": 0
      },
      "96": {
        "width": 291272,
        "height": 1019450,
        "depth": 0
      },
      "97": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 582544,
        "height": 1019450,
        "depth": 0
      },
      "99": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 582544,
        "height": 1019450,
        "depth": 0
      },
      "101": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 320400,
        "height": 1019450,
        "depth": 0
      },
      "103": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 582544,
        "height": 1019450,
        "depth": 0
      },
      "105": {
        "width": 291272,
        "height": 706082,
        "depth": 0
      },
      "106": {
        "width": 320400,
        "height": 706082,
        "depth": 203890
      },
      "107": {
        "width": 553418,
        "height": 1019450,
        "depth": 0
      },
      "108": {
        "width": 291272,
        "height": 1019450,
        "depth": 0
      },
      "109": {
        "width": 873816,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 582544,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 582544,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 553416,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 410694,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 413606,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 407781,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 582544,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 553418,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 757307,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 553418,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 553418,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1048579,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 524290,
        "height": 902941,
        "depth": 0
      },
      "126": {
        "width": 524290,
        "height": 706082,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmex10": {
    "characters": {
      "0": {
        "width": 480600,
        "height": 41942,
        "depth": 1216362
      },
      "1": {
        "width": 480600,
        "height": 41942,
        "depth": 1216362
      },
      "2": {
        "width": 436909,
        "height": 41942,
        "depth": 1216362
      },
      "3": {
        "width": 436909,
        "height": 41942,
        "depth": 1216362
      },
      "4": {
        "width": 495163,
        "height": 41942,
        "depth": 1216362
      },
      "5": {
        "width": 495163,
        "height": 41942,
        "depth": 1216362
      },
      "6": {
        "width": 495163,
        "height": 41942,
        "depth": 1216362
      },
      "7": {
        "width": 495163,
        "height": 41942,
        "depth": 1216362
      },
      "8": {
        "width": 611672,
        "height": 41942,
        "depth": 1216362
      },
      "9": {
        "width": 611672,
        "height": 41942,
        "depth": 1216362
      },
      "10": {
        "width": 495163,
        "height": 41942,
        "depth": 1216362
      },
      "11": {
        "width": 495163,
        "height": 41942,
        "depth": 1216362
      },
      "12": {
        "width": 349526,
        "height": 0,
        "depth": 629152
      },
      "13": {
        "width": 582544,
        "height": 0,
        "depth": 629152
      },
      "14": {
        "width": 605845,
        "height": 41942,
        "depth": 1216362
      },
      "15": {
        "width": 605845,
        "height": 41942,
        "depth": 1216362
      },
      "16": {
        "width": 626235,
        "height": 41942,
        "depth": 1845514
      },
      "17": {
        "width": 626235,
        "height": 41942,
        "depth": 1845514
      },
      "18": {
        "width": 771872,
        "height": 41942,
        "depth": 2474666
      },
      "19": {
        "width": 771872,
        "height": 41942,
        "depth": 2474666
      },
      "20": {
        "width": 553418,
        "height": 41942,
        "depth": 2474666
      },
      "21": {
        "width": 553418,
        "height": 41942,
        "depth": 2474666
      },
      "22": {
        "width": 611672,
        "height": 41942,
        "depth": 2474666
      },
      "23": {
        "width": 611672,
        "height": 41942,
        "depth": 2474666
      },
      "24": {
        "width": 611672,
        "height": 41942,
        "depth": 2474666
      },
      "25": {
        "width": 611672,
        "height": 41942,
        "depth": 2474666
      },
      "26": {
        "width": 786434,
        "height": 41942,
        "depth": 2474666
      },
      "27": {
        "width": 786434,
        "height": 41942,
        "depth": 2474666
      },
      "28": {
        "width": 786434,
        "height": 41942,
        "depth": 2474666
      },
      "29": {
        "width": 786434,
        "height": 41942,
        "depth": 2474666
      },
      "30": {
        "width": 1095182,
        "height": 41942,
        "depth": 2474666
      },
      "31": {
        "width": 1095182,
        "height": 41942,
        "depth": 2474666
      },
      "32": {
        "width": 830126,
        "height": 41942,
        "depth": 3103818
      },
      "33": {
        "width": 830126,
        "height": 41942,
        "depth": 3103818
      },
      "34": {
        "width": 611672,
        "height": 41942,
        "depth": 3103818
      },
      "35": {
        "width": 611672,
        "height": 41942,
        "depth": 3103818
      },
      "36": {
        "width": 669926,
        "height": 41942,
        "depth": 3103818
      },
      "37": {
        "width": 669926,
        "height": 41942,
        "depth": 3103818
      },
      "38": {
        "width": 669926,
        "height": 41942,
        "depth": 3103818
      },
      "39": {
        "width": 669926,
        "height": 41942,
        "depth": 3103818
      },
      "40": {
        "width": 844691,
        "height": 41942,
        "depth": 3103818
      },
      "41": {
        "width": 844691,
        "height": 41942,
        "depth": 3103818
      },
      "42": {
        "width": 844691,
        "height": 41942,
        "depth": 3103818
      },
      "43": {
        "width": 844691,
        "height": 41942,
        "depth": 3103818
      },
      "44": {
        "width": 1339851,
        "height": 41942,
        "depth": 3103818
      },
      "45": {
        "width": 1339851,
        "height": 41942,
        "depth": 3103818
      },
      "46": {
        "width": 850515,
        "height": 41942,
        "depth": 1845514
      },
      "47": {
        "width": 850515,
        "height": 41942,
        "depth": 1845514
      },
      "48": {
        "width": 917507,
        "height": 41942,
        "depth": 1845514
      },
      "49": {
        "width": 917507,
        "height": 41942,
        "depth": 1845514
      },
      "50": {
        "width": 699053,
        "height": 41942,
        "depth": 1845514
      },
      "51": {
        "width": 699053,
        "height": 41942,
        "depth": 1845514
      },
      "52": {
        "width": 699053,
        "height": 41942,
        "depth": 1845514
      },
      "53": {
        "width": 699053,
        "height": 41942,
        "depth": 1845514
      },
      "54": {
        "width": 699053,
        "height": 0,
        "depth": 629152
      },
      "55": {
        "width": 699053,
        "height": 0,
        "depth": 629152
      },
      "56": {
        "width": 932070,
        "height": 0,
        "depth": 943728
      },
      "57": {
        "width": 932070,
        "height": 0,
        "depth": 943728
      },
      "58": {
        "width": 932070,
        "height": 0,
        "depth": 943728
      },
      "59": {
        "width": 932070,
        "height": 0,
        "depth": 943728
      },
      "60": {
        "width": 932070,
        "height": 0,
        "depth": 1887456
      },
      "61": {
        "width": 932070,
        "height": 0,
        "depth": 1887456
      },
      "62": {
        "width": 932070,
        "height": 0,
        "depth": 314576
      },
      "63": {
        "width": 699053,
        "height": 0,
        "depth": 629152
      },
      "64": {
        "width": 917507,
        "height": 41942,
        "depth": 1845514
      },
      "65": {
        "width": 917507,
        "height": 41942,
        "depth": 1845514
      },
      "66": {
        "width": 917507,
        "height": 0,
        "depth": 629152
      },
      "67": {
        "width": 917507,
        "height": 0,
        "depth": 629152
      },
      "68": {
        "width": 640798,
        "height": 41942,
        "depth": 1845514
      },
      "69": {
        "width": 640798,
        "height": 41942,
        "depth": 1845514
      },
      "70": {
        "width": 873816,
        "height": 0,
        "depth": 1048590
      },
      "71": {
        "width": 1165088,
        "height": 104859,
        "depth": 1572877
      },
      "72": {
        "width": 495162,
        "height": 0,
        "depth": 1165096
      },
      "73": {
        "width": 582544,
        "height": 0,
        "depth": 2330194
      },
      "74": {
        "width": 1165088,
        "height": 0,
        "depth": 1048590
      },
      "75": {
        "width": 1584520,
        "height": 104859,
        "depth": 1572877
      },
      "76": {
        "width": 1165088,
        "height": 0,
        "depth": 1048590
      },
      "77": {
        "width": 1584520,
        "height": 104859,
        "depth": 1572877
      },
      "78": {
        "width": 1165088,
        "height": 0,
        "depth": 1048590
      },
      "79": {
        "width": 1584520,
        "height": 104859,
        "depth": 1572877
      },
      "80": {
        "width": 1106834,
        "height": 0,
        "depth": 1048590
      },
      "81": {
        "width": 990325,
        "height": 0,
        "depth": 1048590
      },
      "82": {
        "width": 495162,
        "height": 0,
        "depth": 1165096
      },
      "83": {
        "width": 873816,
        "height": 0,
        "depth": 1048590
      },
      "84": {
        "width": 873816,
        "height": 0,
        "depth": 1048590
      },
      "85": {
        "width": 873816,
        "height": 0,
        "depth": 1048590
      },
      "86": {
        "width": 873816,
        "height": 0,
        "depth": 1048590
      },
      "87": {
        "width": 873816,
        "height": 0,
        "depth": 1048590
      },
      "88": {
        "width": 1514614,
        "height": 104859,
        "depth": 1572877
      },
      "89": {
        "width": 1339851,
        "height": 104859,
        "depth": 1572877
      },
      "90": {
        "width": 582544,
        "height": 0,
        "depth": 2330194
      },
      "91": {
        "width": 1165088,
        "height": 104859,
        "depth": 1572877
      },
      "92": {
        "width": 1165088,
        "height": 104859,
        "depth": 1572877
      },
      "93": {
        "width": 1165088,
        "height": 104859,
        "depth": 1572877
      },
      "94": {
        "width": 1165088,
        "height": 104859,
        "depth": 1572877
      },
      "95": {
        "width": 1165088,
        "height": 104859,
        "depth": 1572877
      },
      "96": {
        "width": 990325,
        "height": 0,
        "depth": 1048590
      },
      "97": {
        "width": 1339851,
        "height": 104859,
        "depth": 1572877
      },
      "98": {
        "width": 582544,
        "height": 757306,
        "depth": 0
      },
      "99": {
        "width": 1048579,
        "height": 786432,
        "depth": 0
      },
      "100": {
        "width": 1514614,
        "height": 786432,
        "depth": 0
      },
      "101": {
        "width": 582544,
        "height": 757306,
        "depth": 0
      },
      "102": {
        "width": 1048579,
        "height": 786432,
        "depth": 0
      },
      "103": {
        "width": 1514614,
        "height": 786432,
        "depth": 0
      },
      "104": {
        "width": 495163,
        "height": 41942,
        "depth": 1845514
      },
      "105": {
        "width": 495163,
        "height": 41942,
        "depth": 1845514
      },
      "106": {
        "width": 553418,
        "height": 41942,
        "depth": 1845514
      },
      "107": {
        "width": 553418,
        "height": 41942,
        "depth": 1845514
      },
      "108": {
        "width": 553418,
        "height": 41942,
        "depth": 1845514
      },
      "109": {
        "width": 553418,
        "height": 41942,
        "depth": 1845514
      },
      "110": {
        "width": 699053,
        "height": 41942,
        "depth": 1845514
      },
      "111": {
        "width": 699053,
        "height": 41942,
        "depth": 1845514
      },
      "112": {
        "width": 1048579,
        "height": 41942,
        "depth": 1216362
      },
      "113": {
        "width": 1048579,
        "height": 41942,
        "depth": 1845514
      },
      "114": {
        "width": 1048579,
        "height": 41942,
        "depth": 2474666
      },
      "115": {
        "width": 1048579,
        "height": 41942,
        "depth": 3103818
      },
      "116": {
        "width": 1106834,
        "height": 0,
        "depth": 1887456
      },
      "117": {
        "width": 1106834,
        "height": 0,
        "depth": 629152
      },
      "118": {
        "width": 1106834,
        "height": 41942,
        "depth": 587210
      },
      "119": {
        "width": 815562,
        "height": 0,
        "depth": 629152
      },
      "120": {
        "width": 699053,
        "height": 0,
        "depth": 629152
      },
      "121": {
        "width": 699053,
        "height": 0,
        "depth": 629152
      },
      "122": {
        "width": 471864,
        "height": 125827,
        "depth": 0
      },
      "123": {
        "width": 471864,
        "height": 125827,
        "depth": 0
      },
      "124": {
        "width": 471864,
        "height": 125827,
        "depth": 0
      },
      "125": {
        "width": 471864,
        "height": 125827,
        "depth": 0
      },
      "126": {
        "width": 815562,
        "height": 0,
        "depth": 629152
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmex7": {
    "characters": {
      "0": {
        "width": 565383,
        "height": 50930,
        "depth": 1207374
      },
      "1": {
        "width": 565383,
        "height": 50930,
        "depth": 1207374
      },
      "2": {
        "width": 517010,
        "height": 50930,
        "depth": 1207374
      },
      "3": {
        "width": 517010,
        "height": 50930,
        "depth": 1207374
      },
      "4": {
        "width": 581506,
        "height": 50930,
        "depth": 1207374
      },
      "5": {
        "width": 581506,
        "height": 50930,
        "depth": 1207374
      },
      "6": {
        "width": 581506,
        "height": 50930,
        "depth": 1207374
      },
      "7": {
        "width": 581506,
        "height": 50930,
        "depth": 1207374
      },
      "8": {
        "width": 710498,
        "height": 50930,
        "depth": 1207374
      },
      "9": {
        "width": 710498,
        "height": 50930,
        "depth": 1207374
      },
      "10": {
        "width": 581506,
        "height": 50930,
        "depth": 1207374
      },
      "11": {
        "width": 581506,
        "height": 50930,
        "depth": 1207374
      },
      "12": {
        "width": 420265,
        "height": 0,
        "depth": 629152
      },
      "13": {
        "width": 678249,
        "height": 0,
        "depth": 629152
      },
      "14": {
        "width": 670759,
        "height": 50930,
        "depth": 1207374
      },
      "15": {
        "width": 670759,
        "height": 50930,
        "depth": 1207374
      },
      "16": {
        "width": 726622,
        "height": 50930,
        "depth": 1836526
      },
      "17": {
        "width": 726622,
        "height": 50930,
        "depth": 1836526
      },
      "18": {
        "width": 887863,
        "height": 50930,
        "depth": 2465678
      },
      "19": {
        "width": 887863,
        "height": 50930,
        "depth": 2465678
      },
      "20": {
        "width": 646002,
        "height": 50930,
        "depth": 2465678
      },
      "21": {
        "width": 646002,
        "height": 50930,
        "depth": 2465678
      },
      "22": {
        "width": 710498,
        "height": 50930,
        "depth": 2465678
      },
      "23": {
        "width": 710498,
        "height": 50930,
        "depth": 2465678
      },
      "24": {
        "width": 710498,
        "height": 50930,
        "depth": 2465678
      },
      "25": {
        "width": 710498,
        "height": 50930,
        "depth": 2465678
      },
      "26": {
        "width": 903984,
        "height": 50930,
        "depth": 2465678
      },
      "27": {
        "width": 903984,
        "height": 50930,
        "depth": 2465678
      },
      "28": {
        "width": 903984,
        "height": 50930,
        "depth": 2465678
      },
      "29": {
        "width": 903984,
        "height": 50930,
        "depth": 2465678
      },
      "30": {
        "width": 1212526,
        "height": 50930,
        "depth": 2465678
      },
      "31": {
        "width": 1212526,
        "height": 50930,
        "depth": 2465678
      },
      "32": {
        "width": 952359,
        "height": 50930,
        "depth": 3094830
      },
      "33": {
        "width": 952359,
        "height": 50930,
        "depth": 3094830
      },
      "34": {
        "width": 710498,
        "height": 50930,
        "depth": 3094830
      },
      "35": {
        "width": 710498,
        "height": 50930,
        "depth": 3094830
      },
      "36": {
        "width": 774994,
        "height": 50930,
        "depth": 3094830
      },
      "37": {
        "width": 774994,
        "height": 50930,
        "depth": 3094830
      },
      "38": {
        "width": 774994,
        "height": 50930,
        "depth": 3094830
      },
      "39": {
        "width": 774994,
        "height": 50930,
        "depth": 3094830
      },
      "40": {
        "width": 968485,
        "height": 50930,
        "depth": 3094830
      },
      "41": {
        "width": 968485,
        "height": 50930,
        "depth": 3094830
      },
      "42": {
        "width": 968485,
        "height": 50930,
        "depth": 3094830
      },
      "43": {
        "width": 968485,
        "height": 50930,
        "depth": 3094830
      },
      "44": {
        "width": 1483408,
        "height": 50930,
        "depth": 3094830
      },
      "45": {
        "width": 1483408,
        "height": 50930,
        "depth": 3094830
      },
      "46": {
        "width": 941641,
        "height": 50930,
        "depth": 1836526
      },
      "47": {
        "width": 941641,
        "height": 50930,
        "depth": 1836526
      },
      "48": {
        "width": 1049102,
        "height": 50930,
        "depth": 1836526
      },
      "49": {
        "width": 1049102,
        "height": 50930,
        "depth": 1836526
      },
      "50": {
        "width": 807241,
        "height": 50930,
        "depth": 1836526
      },
      "51": {
        "width": 807241,
        "height": 50930,
        "depth": 1836526
      },
      "52": {
        "width": 807241,
        "height": 50930,
        "depth": 1836526
      },
      "53": {
        "width": 807241,
        "height": 50930,
        "depth": 1836526
      },
      "54": {
        "width": 807241,
        "height": 0,
        "depth": 629152
      },
      "55": {
        "width": 807241,
        "height": 0,
        "depth": 629152
      },
      "56": {
        "width": 1065225,
        "height": 0,
        "depth": 943728
      },
      "57": {
        "width": 1065225,
        "height": 0,
        "depth": 943728
      },
      "58": {
        "width": 1065225,
        "height": 0,
        "depth": 943728
      },
      "59": {
        "width": 1065225,
        "height": 0,
        "depth": 943728
      },
      "60": {
        "width": 1065225,
        "height": 0,
        "depth": 1887456
      },
      "61": {
        "width": 1065225,
        "height": 0,
        "depth": 1887456
      },
      "62": {
        "width": 1065225,
        "height": 0,
        "depth": 314576
      },
      "63": {
        "width": 807241,
        "height": 0,
        "depth": 629152
      },
      "64": {
        "width": 1049102,
        "height": 50930,
        "depth": 1836526
      },
      "65": {
        "width": 1049102,
        "height": 50930,
        "depth": 1836526
      },
      "66": {
        "width": 1049102,
        "height": 0,
        "depth": 629152
      },
      "67": {
        "width": 1049102,
        "height": 0,
        "depth": 629152
      },
      "68": {
        "width": 742745,
        "height": 50930,
        "depth": 1836526
      },
      "69": {
        "width": 742745,
        "height": 50930,
        "depth": 1836526
      },
      "70": {
        "width": 1000729,
        "height": 0,
        "depth": 1048590
      },
      "71": {
        "width": 1323209,
        "height": 104859,
        "depth": 1572878
      },
      "72": {
        "width": 581504,
        "height": 0,
        "depth": 1165097
      },
      "73": {
        "width": 678249,
        "height": 0,
        "depth": 2330194
      },
      "74": {
        "width": 1323209,
        "height": 0,
        "depth": 1048590
      },
      "75": {
        "width": 1787579,
        "height": 104859,
        "depth": 1572878
      },
      "76": {
        "width": 1323209,
        "height": 0,
        "depth": 1048590
      },
      "77": {
        "width": 1787579,
        "height": 104859,
        "depth": 1572878
      },
      "78": {
        "width": 1323209,
        "height": 0,
        "depth": 1048590
      },
      "79": {
        "width": 1787579,
        "height": 104859,
        "depth": 1572878
      },
      "80": {
        "width": 1258713,
        "height": 0,
        "depth": 1048590
      },
      "81": {
        "width": 1129721,
        "height": 0,
        "depth": 1048590
      },
      "82": {
        "width": 581504,
        "height": 0,
        "depth": 1165097
      },
      "83": {
        "width": 1000729,
        "height": 0,
        "depth": 1048590
      },
      "84": {
        "width": 1000729,
        "height": 0,
        "depth": 1048590
      },
      "85": {
        "width": 1000729,
        "height": 0,
        "depth": 1048590
      },
      "86": {
        "width": 1000729,
        "height": 0,
        "depth": 1048590
      },
      "87": {
        "width": 1000729,
        "height": 0,
        "depth": 1048590
      },
      "88": {
        "width": 1710185,
        "height": 104859,
        "depth": 1572878
      },
      "89": {
        "width": 1516697,
        "height": 104859,
        "depth": 1572878
      },
      "90": {
        "width": 678249,
        "height": 0,
        "depth": 2330194
      },
      "91": {
        "width": 1323209,
        "height": 104859,
        "depth": 1572878
      },
      "92": {
        "width": 1323209,
        "height": 104859,
        "depth": 1572878
      },
      "93": {
        "width": 1323209,
        "height": 104859,
        "depth": 1572878
      },
      "94": {
        "width": 1323209,
        "height": 104859,
        "depth": 1572878
      },
      "95": {
        "width": 1323209,
        "height": 104859,
        "depth": 1572878
      },
      "96": {
        "width": 1129721,
        "height": 0,
        "depth": 1048590
      },
      "97": {
        "width": 1516697,
        "height": 104859,
        "depth": 1572878
      },
      "98": {
        "width": 678249,
        "height": 757305,
        "depth": 0
      },
      "99": {
        "width": 1194217,
        "height": 786432,
        "depth": 0
      },
      "100": {
        "width": 1710185,
        "height": 786432,
        "depth": 0
      },
      "101": {
        "width": 678249,
        "height": 757305,
        "depth": 0
      },
      "102": {
        "width": 1194217,
        "height": 786432,
        "depth": 0
      },
      "103": {
        "width": 1710185,
        "height": 786432,
        "depth": 0
      },
      "104": {
        "width": 581506,
        "height": 50930,
        "depth": 1836526
      },
      "105": {
        "width": 581506,
        "height": 50930,
        "depth": 1836526
      },
      "106": {
        "width": 646002,
        "height": 50930,
        "depth": 1836526
      },
      "107": {
        "width": 646002,
        "height": 50930,
        "depth": 1836526
      },
      "108": {
        "width": 646002,
        "height": 50930,
        "depth": 1836526
      },
      "109": {
        "width": 646002,
        "height": 50930,
        "depth": 1836526
      },
      "110": {
        "width": 807241,
        "height": 50930,
        "depth": 1836526
      },
      "111": {
        "width": 807241,
        "height": 50930,
        "depth": 1836526
      },
      "112": {
        "width": 1177573,
        "height": 50930,
        "depth": 1207374
      },
      "113": {
        "width": 1177573,
        "height": 50930,
        "depth": 1836526
      },
      "114": {
        "width": 1177573,
        "height": 50930,
        "depth": 2465678
      },
      "115": {
        "width": 1177573,
        "height": 50930,
        "depth": 3094830
      },
      "116": {
        "width": 1242069,
        "height": 0,
        "depth": 1887456
      },
      "117": {
        "width": 1242069,
        "height": 0,
        "depth": 629152
      },
      "118": {
        "width": 1242069,
        "height": 50930,
        "depth": 578222
      },
      "119": {
        "width": 936233,
        "height": 0,
        "depth": 629152
      },
      "120": {
        "width": 807241,
        "height": 0,
        "depth": 629152
      },
      "121": {
        "width": 807241,
        "height": 0,
        "depth": 629152
      },
      "122": {
        "width": 505154,
        "height": 152791,
        "depth": 0
      },
      "123": {
        "width": 505154,
        "height": 152791,
        "depth": 0
      },
      "124": {
        "width": 505154,
        "height": 152791,
        "depth": 0
      },
      "125": {
        "width": 505154,
        "height": 152791,
        "depth": 0
      },
      "126": {
        "width": 936233,
        "height": 0,
        "depth": 629152
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmex8": {
    "characters": {
      "0": {
        "width": 510642,
        "height": 47186,
        "depth": 1211118
      },
      "1": {
        "width": 510642,
        "height": 47186,
        "depth": 1211118
      },
      "2": {
        "width": 464220,
        "height": 47186,
        "depth": 1211118
      },
      "3": {
        "width": 464220,
        "height": 47186,
        "depth": 1211118
      },
      "4": {
        "width": 526116,
        "height": 47186,
        "depth": 1211118
      },
      "5": {
        "width": 526116,
        "height": 47186,
        "depth": 1211118
      },
      "6": {
        "width": 526116,
        "height": 47186,
        "depth": 1211118
      },
      "7": {
        "width": 526116,
        "height": 47186,
        "depth": 1211118
      },
      "8": {
        "width": 649908,
        "height": 47186,
        "depth": 1211118
      },
      "9": {
        "width": 649908,
        "height": 47186,
        "depth": 1211118
      },
      "10": {
        "width": 526116,
        "height": 47186,
        "depth": 1211118
      },
      "11": {
        "width": 526116,
        "height": 47186,
        "depth": 1211118
      },
      "12": {
        "width": 371376,
        "height": 0,
        "depth": 629152
      },
      "13": {
        "width": 618960,
        "height": 0,
        "depth": 629152
      },
      "14": {
        "width": 643718,
        "height": 47186,
        "depth": 1211118
      },
      "15": {
        "width": 643718,
        "height": 47186,
        "depth": 1211118
      },
      "16": {
        "width": 665382,
        "height": 47186,
        "depth": 1840270
      },
      "17": {
        "width": 665382,
        "height": 47186,
        "depth": 1840270
      },
      "18": {
        "width": 820122,
        "height": 47186,
        "depth": 2469422
      },
      "19": {
        "width": 820122,
        "height": 47186,
        "depth": 2469422
      },
      "20": {
        "width": 588012,
        "height": 47186,
        "depth": 2469422
      },
      "21": {
        "width": 588012,
        "height": 47186,
        "depth": 2469422
      },
      "22": {
        "width": 649908,
        "height": 47186,
        "depth": 2469422
      },
      "23": {
        "width": 649908,
        "height": 47186,
        "depth": 2469422
      },
      "24": {
        "width": 649908,
        "height": 47186,
        "depth": 2469422
      },
      "25": {
        "width": 649908,
        "height": 47186,
        "depth": 2469422
      },
      "26": {
        "width": 835596,
        "height": 47186,
        "depth": 2469422
      },
      "27": {
        "width": 835596,
        "height": 47186,
        "depth": 2469422
      },
      "28": {
        "width": 835596,
        "height": 47186,
        "depth": 2469422
      },
      "29": {
        "width": 835596,
        "height": 47186,
        "depth": 2469422
      },
      "30": {
        "width": 1163644,
        "height": 47186,
        "depth": 2469422
      },
      "31": {
        "width": 1163644,
        "height": 47186,
        "depth": 2469422
      },
      "32": {
        "width": 882018,
        "height": 47186,
        "depth": 3098574
      },
      "33": {
        "width": 882018,
        "height": 47186,
        "depth": 3098574
      },
      "34": {
        "width": 649908,
        "height": 47186,
        "depth": 3098574
      },
      "35": {
        "width": 649908,
        "height": 47186,
        "depth": 3098574
      },
      "36": {
        "width": 711804,
        "height": 47186,
        "depth": 3098574
      },
      "37": {
        "width": 711804,
        "height": 47186,
        "depth": 3098574
      },
      "38": {
        "width": 711804,
        "height": 47186,
        "depth": 3098574
      },
      "39": {
        "width": 711804,
        "height": 47186,
        "depth": 3098574
      },
      "40": {
        "width": 897492,
        "height": 47186,
        "depth": 3098574
      },
      "41": {
        "width": 897492,
        "height": 47186,
        "depth": 3098574
      },
      "42": {
        "width": 897492,
        "height": 47186,
        "depth": 3098574
      },
      "43": {
        "width": 897492,
        "height": 47186,
        "depth": 3098574
      },
      "44": {
        "width": 1423608,
        "height": 47186,
        "depth": 3098574
      },
      "45": {
        "width": 1423608,
        "height": 47186,
        "depth": 3098574
      },
      "46": {
        "width": 903682,
        "height": 47186,
        "depth": 1840270
      },
      "47": {
        "width": 903682,
        "height": 47186,
        "depth": 1840270
      },
      "48": {
        "width": 974862,
        "height": 47186,
        "depth": 1840270
      },
      "49": {
        "width": 974862,
        "height": 47186,
        "depth": 1840270
      },
      "50": {
        "width": 742752,
        "height": 47186,
        "depth": 1840270
      },
      "51": {
        "width": 742752,
        "height": 47186,
        "depth": 1840270
      },
      "52": {
        "width": 742752,
        "height": 47186,
        "depth": 1840270
      },
      "53": {
        "width": 742752,
        "height": 47186,
        "depth": 1840270
      },
      "54": {
        "width": 742752,
        "height": 0,
        "depth": 629152
      },
      "55": {
        "width": 742752,
        "height": 0,
        "depth": 629152
      },
      "56": {
        "width": 990336,
        "height": 0,
        "depth": 943728
      },
      "57": {
        "width": 990336,
        "height": 0,
        "depth": 943728
      },
      "58": {
        "width": 990336,
        "height": 0,
        "depth": 943728
      },
      "59": {
        "width": 990336,
        "height": 0,
        "depth": 943728
      },
      "60": {
        "width": 990336,
        "height": 0,
        "depth": 1887456
      },
      "61": {
        "width": 990336,
        "height": 0,
        "depth": 1887456
      },
      "62": {
        "width": 990336,
        "height": 0,
        "depth": 314576
      },
      "63": {
        "width": 742752,
        "height": 0,
        "depth": 629152
      },
      "64": {
        "width": 974862,
        "height": 47186,
        "depth": 1840270
      },
      "65": {
        "width": 974862,
        "height": 47186,
        "depth": 1840270
      },
      "66": {
        "width": 974862,
        "height": 0,
        "depth": 629152
      },
      "67": {
        "width": 974862,
        "height": 0,
        "depth": 629152
      },
      "68": {
        "width": 680856,
        "height": 47186,
        "depth": 1840270
      },
      "69": {
        "width": 680856,
        "height": 47186,
        "depth": 1840270
      },
      "70": {
        "width": 928440,
        "height": 0,
        "depth": 1048590
      },
      "71": {
        "width": 1237920,
        "height": 104858,
        "depth": 1572876
      },
      "72": {
        "width": 526116,
        "height": 0,
        "depth": 1165096
      },
      "73": {
        "width": 618960,
        "height": 0,
        "depth": 2330194
      },
      "74": {
        "width": 1237920,
        "height": 0,
        "depth": 1048590
      },
      "75": {
        "width": 1683572,
        "height": 104858,
        "depth": 1572876
      },
      "76": {
        "width": 1237920,
        "height": 0,
        "depth": 1048590
      },
      "77": {
        "width": 1683572,
        "height": 104858,
        "depth": 1572876
      },
      "78": {
        "width": 1237920,
        "height": 0,
        "depth": 1048590
      },
      "79": {
        "width": 1683572,
        "height": 104858,
        "depth": 1572876
      },
      "80": {
        "width": 1176024,
        "height": 0,
        "depth": 1048590
      },
      "81": {
        "width": 1052232,
        "height": 0,
        "depth": 1048590
      },
      "82": {
        "width": 526116,
        "height": 0,
        "depth": 1165096
      },
      "83": {
        "width": 928440,
        "height": 0,
        "depth": 1048590
      },
      "84": {
        "width": 928440,
        "height": 0,
        "depth": 1048590
      },
      "85": {
        "width": 928440,
        "height": 0,
        "depth": 1048590
      },
      "86": {
        "width": 928440,
        "height": 0,
        "depth": 1048590
      },
      "87": {
        "width": 928440,
        "height": 0,
        "depth": 1048590
      },
      "88": {
        "width": 1609296,
        "height": 104858,
        "depth": 1572876
      },
      "89": {
        "width": 1423608,
        "height": 104858,
        "depth": 1572876
      },
      "90": {
        "width": 618960,
        "height": 0,
        "depth": 2330194
      },
      "91": {
        "width": 1237920,
        "height": 104858,
        "depth": 1572876
      },
      "92": {
        "width": 1237920,
        "height": 104858,
        "depth": 1572876
      },
      "93": {
        "width": 1237920,
        "height": 104858,
        "depth": 1572876
      },
      "94": {
        "width": 1237920,
        "height": 104858,
        "depth": 1572876
      },
      "95": {
        "width": 1237920,
        "height": 104858,
        "depth": 1572876
      },
      "96": {
        "width": 1052232,
        "height": 0,
        "depth": 1048590
      },
      "97": {
        "width": 1423608,
        "height": 104858,
        "depth": 1572876
      },
      "98": {
        "width": 618960,
        "height": 757306,
        "depth": 0
      },
      "99": {
        "width": 1114128,
        "height": 786432,
        "depth": 0
      },
      "100": {
        "width": 1609296,
        "height": 786432,
        "depth": 0
      },
      "101": {
        "width": 618960,
        "height": 757306,
        "depth": 0
      },
      "102": {
        "width": 1114128,
        "height": 786432,
        "depth": 0
      },
      "103": {
        "width": 1609296,
        "height": 786432,
        "depth": 0
      },
      "104": {
        "width": 526116,
        "height": 47186,
        "depth": 1840270
      },
      "105": {
        "width": 526116,
        "height": 47186,
        "depth": 1840270
      },
      "106": {
        "width": 588012,
        "height": 47186,
        "depth": 1840270
      },
      "107": {
        "width": 588012,
        "height": 47186,
        "depth": 1840270
      },
      "108": {
        "width": 588012,
        "height": 47186,
        "depth": 1840270
      },
      "109": {
        "width": 588012,
        "height": 47186,
        "depth": 1840270
      },
      "110": {
        "width": 742752,
        "height": 47186,
        "depth": 1840270
      },
      "111": {
        "width": 742752,
        "height": 47186,
        "depth": 1840270
      },
      "112": {
        "width": 1114128,
        "height": 47186,
        "depth": 1211118
      },
      "113": {
        "width": 1114128,
        "height": 47186,
        "depth": 1840270
      },
      "114": {
        "width": 1114128,
        "height": 47186,
        "depth": 2469422
      },
      "115": {
        "width": 1114128,
        "height": 47186,
        "depth": 3098574
      },
      "116": {
        "width": 1176024,
        "height": 0,
        "depth": 1887456
      },
      "117": {
        "width": 1176024,
        "height": 0,
        "depth": 629152
      },
      "118": {
        "width": 1176024,
        "height": 47186,
        "depth": 581966
      },
      "119": {
        "width": 866544,
        "height": 0,
        "depth": 629152
      },
      "120": {
        "width": 742752,
        "height": 0,
        "depth": 629152
      },
      "121": {
        "width": 742752,
        "height": 0,
        "depth": 629152
      },
      "122": {
        "width": 471864,
        "height": 141558,
        "depth": 0
      },
      "123": {
        "width": 471864,
        "height": 141558,
        "depth": 0
      },
      "124": {
        "width": 471864,
        "height": 141558,
        "depth": 0
      },
      "125": {
        "width": 471864,
        "height": 141558,
        "depth": 0
      },
      "126": {
        "width": 866544,
        "height": 0,
        "depth": 629152
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmex9": {
    "characters": {
      "0": {
        "width": 493943,
        "height": 44274,
        "depth": 1214030
      },
      "1": {
        "width": 493943,
        "height": 44274,
        "depth": 1214030
      },
      "2": {
        "width": 449040,
        "height": 44274,
        "depth": 1214030
      },
      "3": {
        "width": 449040,
        "height": 44274,
        "depth": 1214030
      },
      "4": {
        "width": 508912,
        "height": 44274,
        "depth": 1214030
      },
      "5": {
        "width": 508912,
        "height": 44274,
        "depth": 1214030
      },
      "6": {
        "width": 508912,
        "height": 44274,
        "depth": 1214030
      },
      "7": {
        "width": 508912,
        "height": 44274,
        "depth": 1214030
      },
      "8": {
        "width": 628658,
        "height": 44274,
        "depth": 1214030
      },
      "9": {
        "width": 628658,
        "height": 44274,
        "depth": 1214030
      },
      "10": {
        "width": 508914,
        "height": 44274,
        "depth": 1214030
      },
      "11": {
        "width": 508914,
        "height": 44274,
        "depth": 1214030
      },
      "12": {
        "width": 359232,
        "height": 0,
        "depth": 629152
      },
      "13": {
        "width": 598720,
        "height": 0,
        "depth": 629152
      },
      "14": {
        "width": 622668,
        "height": 44274,
        "depth": 1214030
      },
      "15": {
        "width": 622668,
        "height": 44274,
        "depth": 1214030
      },
      "16": {
        "width": 643623,
        "height": 44274,
        "depth": 1843182
      },
      "17": {
        "width": 643623,
        "height": 44274,
        "depth": 1843182
      },
      "18": {
        "width": 793303,
        "height": 44274,
        "depth": 2472334
      },
      "19": {
        "width": 793303,
        "height": 44274,
        "depth": 2472334
      },
      "20": {
        "width": 568784,
        "height": 44274,
        "depth": 2472334
      },
      "21": {
        "width": 568784,
        "height": 44274,
        "depth": 2472334
      },
      "22": {
        "width": 628656,
        "height": 44274,
        "depth": 2472334
      },
      "23": {
        "width": 628656,
        "height": 44274,
        "depth": 2472334
      },
      "24": {
        "width": 628656,
        "height": 44274,
        "depth": 2472334
      },
      "25": {
        "width": 628656,
        "height": 44274,
        "depth": 2472334
      },
      "26": {
        "width": 808274,
        "height": 44274,
        "depth": 2472334
      },
      "27": {
        "width": 808274,
        "height": 44274,
        "depth": 2472334
      },
      "28": {
        "width": 808274,
        "height": 44274,
        "depth": 2472334
      },
      "29": {
        "width": 808274,
        "height": 44274,
        "depth": 2472334
      },
      "30": {
        "width": 1125595,
        "height": 44274,
        "depth": 2472334
      },
      "31": {
        "width": 1125595,
        "height": 44274,
        "depth": 2472334
      },
      "32": {
        "width": 853175,
        "height": 44274,
        "depth": 3101486
      },
      "33": {
        "width": 853175,
        "height": 44274,
        "depth": 3101486
      },
      "34": {
        "width": 628656,
        "height": 44274,
        "depth": 3101486
      },
      "35": {
        "width": 628656,
        "height": 44274,
        "depth": 3101486
      },
      "36": {
        "width": 688528,
        "height": 44274,
        "depth": 3101486
      },
      "37": {
        "width": 688528,
        "height": 44274,
        "depth": 3101486
      },
      "38": {
        "width": 688528,
        "height": 44274,
        "depth": 3101486
      },
      "39": {
        "width": 688528,
        "height": 44274,
        "depth": 3101486
      },
      "40": {
        "width": 868144,
        "height": 44274,
        "depth": 3101486
      },
      "41": {
        "width": 868144,
        "height": 44274,
        "depth": 3101486
      },
      "42": {
        "width": 868144,
        "height": 44274,
        "depth": 3101486
      },
      "43": {
        "width": 868144,
        "height": 44274,
        "depth": 3101486
      },
      "44": {
        "width": 1377056,
        "height": 44274,
        "depth": 3101486
      },
      "45": {
        "width": 1377056,
        "height": 44274,
        "depth": 3101486
      },
      "46": {
        "width": 874132,
        "height": 44274,
        "depth": 1843182
      },
      "47": {
        "width": 874132,
        "height": 44274,
        "depth": 1843182
      },
      "48": {
        "width": 942983,
        "height": 44274,
        "depth": 1843182
      },
      "49": {
        "width": 942983,
        "height": 44274,
        "depth": 1843182
      },
      "50": {
        "width": 718464,
        "height": 44274,
        "depth": 1843182
      },
      "51": {
        "width": 718464,
        "height": 44274,
        "depth": 1843182
      },
      "52": {
        "width": 718464,
        "height": 44274,
        "depth": 1843182
      },
      "53": {
        "width": 718464,
        "height": 44274,
        "depth": 1843182
      },
      "54": {
        "width": 718464,
        "height": 0,
        "depth": 629152
      },
      "55": {
        "width": 718464,
        "height": 0,
        "depth": 629152
      },
      "56": {
        "width": 957952,
        "height": 0,
        "depth": 943728
      },
      "57": {
        "width": 957952,
        "height": 0,
        "depth": 943728
      },
      "58": {
        "width": 957952,
        "height": 0,
        "depth": 943728
      },
      "59": {
        "width": 957952,
        "height": 0,
        "depth": 943728
      },
      "60": {
        "width": 957952,
        "height": 0,
        "depth": 1887456
      },
      "61": {
        "width": 957952,
        "height": 0,
        "depth": 1887456
      },
      "62": {
        "width": 957952,
        "height": 0,
        "depth": 314576
      },
      "63": {
        "width": 718464,
        "height": 0,
        "depth": 629152
      },
      "64": {
        "width": 942983,
        "height": 44274,
        "depth": 1843182
      },
      "65": {
        "width": 942983,
        "height": 44274,
        "depth": 1843182
      },
      "66": {
        "width": 942983,
        "height": 0,
        "depth": 629152
      },
      "67": {
        "width": 942983,
        "height": 0,
        "depth": 629152
      },
      "68": {
        "width": 658592,
        "height": 44274,
        "depth": 1843182
      },
      "69": {
        "width": 658592,
        "height": 44274,
        "depth": 1843182
      },
      "70": {
        "width": 898080,
        "height": 0,
        "depth": 1048590
      },
      "71": {
        "width": 1197440,
        "height": 104859,
        "depth": 1572876
      },
      "72": {
        "width": 508912,
        "height": 0,
        "depth": 1165097
      },
      "73": {
        "width": 598720,
        "height": 0,
        "depth": 2330194
      },
      "74": {
        "width": 1197440,
        "height": 0,
        "depth": 1048590
      },
      "75": {
        "width": 1628517,
        "height": 104859,
        "depth": 1572876
      },
      "76": {
        "width": 1197440,
        "height": 0,
        "depth": 1048590
      },
      "77": {
        "width": 1628517,
        "height": 104859,
        "depth": 1572876
      },
      "78": {
        "width": 1197440,
        "height": 0,
        "depth": 1048590
      },
      "79": {
        "width": 1628517,
        "height": 104859,
        "depth": 1572876
      },
      "80": {
        "width": 1137568,
        "height": 0,
        "depth": 1048590
      },
      "81": {
        "width": 1017824,
        "height": 0,
        "depth": 1048590
      },
      "82": {
        "width": 508912,
        "height": 0,
        "depth": 1165097
      },
      "83": {
        "width": 898080,
        "height": 0,
        "depth": 1048590
      },
      "84": {
        "width": 898080,
        "height": 0,
        "depth": 1048590
      },
      "85": {
        "width": 898080,
        "height": 0,
        "depth": 1048590
      },
      "86": {
        "width": 898080,
        "height": 0,
        "depth": 1048590
      },
      "87": {
        "width": 898080,
        "height": 0,
        "depth": 1048590
      },
      "88": {
        "width": 1556672,
        "height": 104859,
        "depth": 1572876
      },
      "89": {
        "width": 1377056,
        "height": 104859,
        "depth": 1572876
      },
      "90": {
        "width": 598720,
        "height": 0,
        "depth": 2330194
      },
      "91": {
        "width": 1197440,
        "height": 104859,
        "depth": 1572876
      },
      "92": {
        "width": 1197440,
        "height": 104859,
        "depth": 1572876
      },
      "93": {
        "width": 1197440,
        "height": 104859,
        "depth": 1572876
      },
      "94": {
        "width": 1197440,
        "height": 104859,
        "depth": 1572876
      },
      "95": {
        "width": 1197440,
        "height": 104859,
        "depth": 1572876
      },
      "96": {
        "width": 1017824,
        "height": 0,
        "depth": 1048590
      },
      "97": {
        "width": 1377056,
        "height": 104859,
        "depth": 1572876
      },
      "98": {
        "width": 598720,
        "height": 757305,
        "depth": 0
      },
      "99": {
        "width": 1077696,
        "height": 786432,
        "depth": 0
      },
      "100": {
        "width": 1556672,
        "height": 786432,
        "depth": 0
      },
      "101": {
        "width": 598720,
        "height": 757305,
        "depth": 0
      },
      "102": {
        "width": 1077696,
        "height": 786432,
        "depth": 0
      },
      "103": {
        "width": 1556672,
        "height": 786432,
        "depth": 0
      },
      "104": {
        "width": 508912,
        "height": 44274,
        "depth": 1843182
      },
      "105": {
        "width": 508912,
        "height": 44274,
        "depth": 1843182
      },
      "106": {
        "width": 568784,
        "height": 44274,
        "depth": 1843182
      },
      "107": {
        "width": 568784,
        "height": 44274,
        "depth": 1843182
      },
      "108": {
        "width": 568784,
        "height": 44274,
        "depth": 1843182
      },
      "109": {
        "width": 568784,
        "height": 44274,
        "depth": 1843182
      },
      "110": {
        "width": 718464,
        "height": 44274,
        "depth": 1843182
      },
      "111": {
        "width": 718464,
        "height": 44274,
        "depth": 1843182
      },
      "112": {
        "width": 1077696,
        "height": 44274,
        "depth": 1214030
      },
      "113": {
        "width": 1077696,
        "height": 44274,
        "depth": 1843182
      },
      "114": {
        "width": 1077696,
        "height": 44274,
        "depth": 2472334
      },
      "115": {
        "width": 1077696,
        "height": 44274,
        "depth": 3101486
      },
      "116": {
        "width": 1137568,
        "height": 0,
        "depth": 1887456
      },
      "117": {
        "width": 1137568,
        "height": 0,
        "depth": 629152
      },
      "118": {
        "width": 1137568,
        "height": 44274,
        "depth": 584878
      },
      "119": {
        "width": 838208,
        "height": 0,
        "depth": 629152
      },
      "120": {
        "width": 718464,
        "height": 0,
        "depth": 629152
      },
      "121": {
        "width": 718464,
        "height": 0,
        "depth": 629152
      },
      "122": {
        "width": 471865,
        "height": 132821,
        "depth": 0
      },
      "123": {
        "width": 471865,
        "height": 132821,
        "depth": 0
      },
      "124": {
        "width": 471865,
        "height": 132821,
        "depth": 0
      },
      "125": {
        "width": 471865,
        "height": 132821,
        "depth": 0
      },
      "126": {
        "width": 838208,
        "height": 0,
        "depth": 629152
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmff10": {
    "characters": {
      "0": {
        "width": 559238,
        "height": 655360,
        "depth": 0
      },
      "1": {
        "width": 710699,
        "height": 655360,
        "depth": 0
      },
      "2": {
        "width": 664096,
        "height": 655360,
        "depth": 0
      },
      "3": {
        "width": 547587,
        "height": 655360,
        "depth": 0
      },
      "4": {
        "width": 570890,
        "height": 655360,
        "depth": 0
      },
      "5": {
        "width": 547587,
        "height": 655360,
        "depth": 0
      },
      "6": {
        "width": 617493,
        "height": 655360,
        "depth": 0
      },
      "7": {
        "width": 664096,
        "height": 655360,
        "depth": 0
      },
      "8": {
        "width": 617493,
        "height": 655360,
        "depth": 0
      },
      "9": {
        "width": 664096,
        "height": 655360,
        "depth": 0
      },
      "10": {
        "width": 617493,
        "height": 655360,
        "depth": 0
      },
      "11": {
        "width": 512637,
        "height": 640797,
        "depth": 0
      },
      "12": {
        "width": 480597,
        "height": 640797,
        "depth": 0
      },
      "13": {
        "width": 480597,
        "height": 640797,
        "depth": 0
      },
      "14": {
        "width": 725264,
        "height": 640797,
        "depth": 0
      },
      "15": {
        "width": 725264,
        "height": 640797,
        "depth": 0
      },
      "16": {
        "width": 227192,
        "height": 553414,
        "depth": 0
      },
      "17": {
        "width": 250494,
        "height": 553414,
        "depth": 291270
      },
      "18": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      },
      "19": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      },
      "20": {
        "width": 431080,
        "height": 618952,
        "depth": 0
      },
      "21": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      },
      "22": {
        "width": 431080,
        "height": 630309,
        "depth": 0
      },
      "23": {
        "width": 582539,
        "height": 640797,
        "depth": 0
      },
      "24": {
        "width": 384477,
        "height": 0,
        "depth": 254862
      },
      "25": {
        "width": 422344,
        "height": 640797,
        "depth": 0
      },
      "26": {
        "width": 617493,
        "height": 553414,
        "depth": 0
      },
      "27": {
        "width": 664096,
        "height": 553414,
        "depth": 0
      },
      "28": {
        "width": 431080,
        "height": 699050,
        "depth": 145635
      },
      "29": {
        "width": 745651,
        "height": 655360,
        "depth": 0
      },
      "30": {
        "width": 838858,
        "height": 655360,
        "depth": 0
      },
      "31": {
        "width": 664096,
        "height": 728178,
        "depth": 72818
      },
      "32": {
        "width": 227192,
        "height": 553414,
        "depth": 0
      },
      "33": {
        "width": 198064,
        "height": 640797,
        "depth": 0
      },
      "34": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      },
      "35": {
        "width": 710699,
        "height": 640797,
        "depth": 320400
      },
      "36": {
        "width": 431080,
        "height": 699051,
        "depth": 58254
      },
      "37": {
        "width": 710699,
        "height": 699051,
        "depth": 58254
      },
      "38": {
        "width": 655358,
        "height": 640797,
        "depth": 0
      },
      "39": {
        "width": 244667,
        "height": 640797,
        "depth": 0
      },
      "40": {
        "width": 337874,
        "height": 699051,
        "depth": 378653
      },
      "41": {
        "width": 337874,
        "height": 699051,
        "depth": 378653
      },
      "42": {
        "width": 431080,
        "height": 699051,
        "depth": 0
      },
      "43": {
        "width": 664096,
        "height": 439818,
        "depth": 119421
      },
      "44": {
        "width": 244667,
        "height": 75731,
        "depth": 203890
      },
      "45": {
        "width": 291270,
        "height": 553414,
        "depth": 0
      },
      "46": {
        "width": 244667,
        "height": 75731,
        "depth": 0
      },
      "47": {
        "width": 431080,
        "height": 699051,
        "depth": 378653
      },
      "48": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "49": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "50": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "51": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "52": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "53": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "54": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "55": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "56": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "57": {
        "width": 431080,
        "height": 611669,
        "depth": 0
      },
      "58": {
        "width": 244667,
        "height": 553414,
        "depth": 0
      },
      "59": {
        "width": 244667,
        "height": 553414,
        "depth": 203890
      },
      "60": {
        "width": 198064,
        "height": 349526,
        "depth": 291270
      },
      "61": {
        "width": 664096,
        "height": 324766,
        "depth": 4370
      },
      "62": {
        "width": 407779,
        "height": 349526,
        "depth": 291270
      },
      "63": {
        "width": 407779,
        "height": 640797,
        "depth": 0
      },
      "64": {
        "width": 664096,
        "height": 640797,
        "depth": 0
      },
      "65": {
        "width": 594190,
        "height": 655360,
        "depth": 0
      },
      "66": {
        "width": 582541,
        "height": 655360,
        "depth": 0
      },
      "67": {
        "width": 617493,
        "height": 655360,
        "depth": 0
      },
      "68": {
        "width": 629144,
        "height": 655360,
        "depth": 0
      },
      "69": {
        "width": 605842,
        "height": 655360,
        "depth": 0
      },
      "70": {
        "width": 582541,
        "height": 655360,
        "depth": 0
      },
      "71": {
        "width": 658270,
        "height": 655360,
        "depth": 0
      },
      "72": {
        "width": 547587,
        "height": 655360,
        "depth": 0
      },
      "73": {
        "width": 267968,
        "height": 655360,
        "depth": 0
      },
      "74": {
        "width": 419429,
        "height": 655360,
        "depth": 0
      },
      "75": {
        "width": 617493,
        "height": 655360,
        "depth": 0
      },
      "76": {
        "width": 559238,
        "height": 655360,
        "depth": 0
      },
      "77": {
        "width": 687397,
        "height": 655360,
        "depth": 0
      },
      "78": {
        "width": 547587,
        "height": 655360,
        "depth": 0
      },
      "79": {
        "width": 710699,
        "height": 655360,
        "depth": 0
      },
      "80": {
        "width": 559238,
        "height": 655360,
        "depth": 0
      },
      "81": {
        "width": 710699,
        "height": 655360,
        "depth": 203890
      },
      "82": {
        "width": 605842,
        "height": 655360,
        "depth": 0
      },
      "83": {
        "width": 477683,
        "height": 655360,
        "depth": 0
      },
      "84": {
        "width": 664096,
        "height": 655360,
        "depth": 0
      },
      "85": {
        "width": 570888,
        "height": 655360,
        "depth": 0
      },
      "86": {
        "width": 594190,
        "height": 655360,
        "depth": 0
      },
      "87": {
        "width": 827206,
        "height": 655360,
        "depth": 0
      },
      "88": {
        "width": 594190,
        "height": 655360,
        "depth": 0
      },
      "89": {
        "width": 594190,
        "height": 655360,
        "depth": 0
      },
      "90": {
        "width": 524286,
        "height": 655360,
        "depth": 0
      },
      "91": {
        "width": 247581,
        "height": 699051,
        "depth": 378653
      },
      "92": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      },
      "93": {
        "width": 247581,
        "height": 699051,
        "depth": 378653
      },
      "94": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      },
      "95": {
        "width": 244667,
        "height": 640797,
        "depth": 0
      },
      "96": {
        "width": 244667,
        "height": 640797,
        "depth": 0
      },
      "97": {
        "width": 422342,
        "height": 553414,
        "depth": 0
      },
      "98": {
        "width": 460208,
        "height": 640797,
        "depth": 0
      },
      "99": {
        "width": 384477,
        "height": 553414,
        "depth": 0
      },
      "100": {
        "width": 460208,
        "height": 640797,
        "depth": 0
      },
      "101": {
        "width": 384477,
        "height": 553414,
        "depth": 0
      },
      "102": {
        "width": 267970,
        "height": 640797,
        "depth": 0
      },
      "103": {
        "width": 431080,
        "height": 553414,
        "depth": 291270
      },
      "104": {
        "width": 460208,
        "height": 640797,
        "depth": 0
      },
      "105": {
        "width": 227192,
        "height": 640797,
        "depth": 0
      },
      "106": {
        "width": 250494,
        "height": 640797,
        "depth": 291270
      },
      "107": {
        "width": 436907,
        "height": 640797,
        "depth": 0
      },
      "108": {
        "width": 227192,
        "height": 640797,
        "depth": 0
      },
      "109": {
        "width": 693224,
        "height": 553414,
        "depth": 0
      },
      "110": {
        "width": 460208,
        "height": 553414,
        "depth": 0
      },
      "111": {
        "width": 431080,
        "height": 553414,
        "depth": 0
      },
      "112": {
        "width": 460208,
        "height": 553414,
        "depth": 291270
      },
      "113": {
        "width": 436906,
        "height": 553414,
        "depth": 291270
      },
      "114": {
        "width": 329136,
        "height": 553414,
        "depth": 0
      },
      "115": {
        "width": 342534,
        "height": 553414,
        "depth": 0
      },
      "116": {
        "width": 337874,
        "height": 640797,
        "depth": 0
      },
      "117": {
        "width": 460208,
        "height": 553414,
        "depth": 0
      },
      "118": {
        "width": 436907,
        "height": 553414,
        "depth": 0
      },
      "119": {
        "width": 600018,
        "height": 553414,
        "depth": 0
      },
      "120": {
        "width": 436907,
        "height": 553414,
        "depth": 0
      },
      "121": {
        "width": 436907,
        "height": 553414,
        "depth": 291270
      },
      "122": {
        "width": 380107,
        "height": 553414,
        "depth": 0
      },
      "123": {
        "width": 431080,
        "height": 553414,
        "depth": 0
      },
      "124": {
        "width": 862160,
        "height": 553414,
        "depth": 0
      },
      "125": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      },
      "126": {
        "width": 431080,
        "height": 640797,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmfi10": {
    "characters": {
      "0": {
        "width": 719434,
        "height": 655360,
        "depth": 0
      },
      "1": {
        "width": 929147,
        "height": 655360,
        "depth": 0
      },
      "2": {
        "width": 867981,
        "height": 655360,
        "depth": 0
      },
      "3": {
        "width": 722346,
        "height": 655360,
        "depth": 0
      },
      "4": {
        "width": 745648,
        "height": 655360,
        "depth": 0
      },
      "5": {
        "width": 736909,
        "height": 655360,
        "depth": 0
      },
      "6": {
        "width": 806814,
        "height": 655360,
        "depth": 0
      },
      "7": {
        "width": 867981,
        "height": 655360,
        "depth": 0
      },
      "8": {
        "width": 806814,
        "height": 655360,
        "depth": 0
      },
      "9": {
        "width": 867981,
        "height": 655360,
        "depth": 0
      },
      "10": {
        "width": 806814,
        "height": 655360,
        "depth": 0
      },
      "11": {
        "width": 634968,
        "height": 640797,
        "depth": 291270
      },
      "12": {
        "width": 573802,
        "height": 640797,
        "depth": 291270
      },
      "13": {
        "width": 604384,
        "height": 640797,
        "depth": 291270
      },
      "14": {
        "width": 906576,
        "height": 640797,
        "depth": 291270
      },
      "15": {
        "width": 921867,
        "height": 640797,
        "depth": 291270
      },
      "16": {
        "width": 317483,
        "height": 553414,
        "depth": 0
      },
      "17": {
        "width": 348067,
        "height": 553414,
        "depth": 291270
      },
      "18": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "19": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "20": {
        "width": 562149,
        "height": 618952,
        "depth": 0
      },
      "21": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "22": {
        "width": 562149,
        "height": 633222,
        "depth": 0
      },
      "23": {
        "width": 795163,
        "height": 640797,
        "depth": 0
      },
      "24": {
        "width": 500982,
        "height": 0,
        "depth": 254862
      },
      "25": {
        "width": 592734,
        "height": 640797,
        "depth": 291270
      },
      "26": {
        "width": 806814,
        "height": 553414,
        "depth": 0
      },
      "27": {
        "width": 806814,
        "height": 553414,
        "depth": 0
      },
      "28": {
        "width": 562149,
        "height": 699050,
        "depth": 145635
      },
      "29": {
        "width": 978662,
        "height": 655360,
        "depth": 0
      },
      "30": {
        "width": 1100995,
        "height": 655360,
        "depth": 0
      },
      "31": {
        "width": 867981,
        "height": 728178,
        "depth": 72818
      },
      "32": {
        "width": 256317,
        "height": 553414,
        "depth": 0
      },
      "33": {
        "width": 270880,
        "height": 640797,
        "depth": 0
      },
      "34": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "35": {
        "width": 929147,
        "height": 640797,
        "depth": 320400
      },
      "36": {
        "width": 763126,
        "height": 640797,
        "depth": 0
      },
      "37": {
        "width": 929147,
        "height": 699051,
        "depth": 58254
      },
      "38": {
        "width": 867981,
        "height": 640797,
        "depth": 0
      },
      "39": {
        "width": 317483,
        "height": 640797,
        "depth": 0
      },
      "40": {
        "width": 439816,
        "height": 699051,
        "depth": 378653
      },
      "41": {
        "width": 439816,
        "height": 699051,
        "depth": 378653
      },
      "42": {
        "width": 562149,
        "height": 699051,
        "depth": 0
      },
      "43": {
        "width": 867981,
        "height": 527197,
        "depth": 206800
      },
      "44": {
        "width": 317483,
        "height": 75731,
        "depth": 203890
      },
      "45": {
        "width": 378650,
        "height": 553414,
        "depth": 0
      },
      "46": {
        "width": 317483,
        "height": 75731,
        "depth": 0
      },
      "47": {
        "width": 562149,
        "height": 699051,
        "depth": 378653
      },
      "48": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "49": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "50": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "51": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "52": {
        "width": 562149,
        "height": 611669,
        "depth": 291270
      },
      "53": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "54": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "55": {
        "width": 562149,
        "height": 611669,
        "depth": 291270
      },
      "56": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "57": {
        "width": 562149,
        "height": 611669,
        "depth": 0
      },
      "58": {
        "width": 317483,
        "height": 553414,
        "depth": 0
      },
      "59": {
        "width": 317483,
        "height": 553414,
        "depth": 203890
      },
      "60": {
        "width": 270880,
        "height": 349526,
        "depth": 291270
      },
      "61": {
        "width": 867981,
        "height": 324766,
        "depth": 4370
      },
      "62": {
        "width": 562149,
        "height": 349526,
        "depth": 291270
      },
      "63": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "64": {
        "width": 867981,
        "height": 640797,
        "depth": 0
      },
      "65": {
        "width": 783512,
        "height": 655360,
        "depth": 0
      },
      "66": {
        "width": 764581,
        "height": 655360,
        "depth": 0
      },
      "67": {
        "width": 806814,
        "height": 655360,
        "depth": 0
      },
      "68": {
        "width": 825747,
        "height": 655360,
        "depth": 0
      },
      "69": {
        "width": 780600,
        "height": 655360,
        "depth": 0
      },
      "70": {
        "width": 750018,
        "height": 655360,
        "depth": 0
      },
      "71": {
        "width": 862155,
        "height": 655360,
        "depth": 0
      },
      "72": {
        "width": 736909,
        "height": 655360,
        "depth": 0
      },
      "73": {
        "width": 355347,
        "height": 655360,
        "depth": 0
      },
      "74": {
        "width": 550498,
        "height": 655360,
        "depth": 0
      },
      "75": {
        "width": 814096,
        "height": 655360,
        "depth": 0
      },
      "76": {
        "width": 719434,
        "height": 655360,
        "depth": 0
      },
      "77": {
        "width": 920408,
        "height": 655360,
        "depth": 0
      },
      "78": {
        "width": 736909,
        "height": 655360,
        "depth": 0
      },
      "79": {
        "width": 914584,
        "height": 655360,
        "depth": 0
      },
      "80": {
        "width": 733997,
        "height": 655360,
        "depth": 0
      },
      "81": {
        "width": 914584,
        "height": 655360,
        "depth": 203890
      },
      "82": {
        "width": 795163,
        "height": 655360,
        "depth": 0
      },
      "83": {
        "width": 623315,
        "height": 655360,
        "depth": 0
      },
      "84": {
        "width": 853418,
        "height": 655360,
        "depth": 0
      },
      "85": {
        "width": 760210,
        "height": 655360,
        "depth": 0
      },
      "86": {
        "width": 783512,
        "height": 655360,
        "depth": 0
      },
      "87": {
        "width": 1089344,
        "height": 655360,
        "depth": 0
      },
      "88": {
        "width": 783512,
        "height": 655360,
        "depth": 0
      },
      "89": {
        "width": 783512,
        "height": 655360,
        "depth": 0
      },
      "90": {
        "width": 684482,
        "height": 655360,
        "depth": 0
      },
      "91": {
        "width": 317483,
        "height": 699051,
        "depth": 378653
      },
      "92": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "93": {
        "width": 317483,
        "height": 699051,
        "depth": 378653
      },
      "94": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "95": {
        "width": 317483,
        "height": 640797,
        "depth": 0
      },
      "96": {
        "width": 317483,
        "height": 640797,
        "depth": 0
      },
      "97": {
        "width": 562149,
        "height": 553414,
        "depth": 0
      },
      "98": {
        "width": 500982,
        "height": 640797,
        "depth": 0
      },
      "99": {
        "width": 500982,
        "height": 553414,
        "depth": 0
      },
      "100": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "101": {
        "width": 500982,
        "height": 553414,
        "depth": 0
      },
      "102": {
        "width": 317485,
        "height": 640797,
        "depth": 291270
      },
      "103": {
        "width": 500982,
        "height": 553414,
        "depth": 291270
      },
      "104": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "105": {
        "width": 317483,
        "height": 640797,
        "depth": 0
      },
      "106": {
        "width": 317483,
        "height": 640797,
        "depth": 291270
      },
      "107": {
        "width": 500982,
        "height": 640797,
        "depth": 0
      },
      "108": {
        "width": 256317,
        "height": 640797,
        "depth": 0
      },
      "109": {
        "width": 929147,
        "height": 553414,
        "depth": 0
      },
      "110": {
        "width": 623315,
        "height": 553414,
        "depth": 0
      },
      "111": {
        "width": 562149,
        "height": 553414,
        "depth": 0
      },
      "112": {
        "width": 562149,
        "height": 553414,
        "depth": 291270
      },
      "113": {
        "width": 500982,
        "height": 553414,
        "depth": 291270
      },
      "114": {
        "width": 455109,
        "height": 553414,
        "depth": 0
      },
      "115": {
        "width": 439816,
        "height": 553414,
        "depth": 0
      },
      "116": {
        "width": 348067,
        "height": 640797,
        "depth": 0
      },
      "117": {
        "width": 592733,
        "height": 553414,
        "depth": 0
      },
      "118": {
        "width": 500982,
        "height": 553414,
        "depth": 0
      },
      "119": {
        "width": 745648,
        "height": 553414,
        "depth": 0
      },
      "120": {
        "width": 500984,
        "height": 553414,
        "depth": 0
      },
      "121": {
        "width": 531566,
        "height": 553414,
        "depth": 291270
      },
      "122": {
        "width": 439818,
        "height": 553414,
        "depth": 0
      },
      "123": {
        "width": 562149,
        "height": 553414,
        "depth": 0
      },
      "124": {
        "width": 1124298,
        "height": 553414,
        "depth": 0
      },
      "125": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      },
      "126": {
        "width": 562149,
        "height": 640797,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmfib8": {
    "characters": {
      "0": {
        "width": 841038,
        "height": 848328,
        "depth": 0
      },
      "1": {
        "width": 1146870,
        "height": 848328,
        "depth": 0
      },
      "2": {
        "width": 1070412,
        "height": 848328,
        "depth": 0
      },
      "3": {
        "width": 917496,
        "height": 848328,
        "depth": 0
      },
      "4": {
        "width": 917496,
        "height": 848328,
        "depth": 0
      },
      "5": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "6": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "7": {
        "width": 1070412,
        "height": 848328,
        "depth": 0
      },
      "8": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "9": {
        "width": 1070412,
        "height": 848328,
        "depth": 0
      },
      "10": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "11": {
        "width": 802810,
        "height": 848328,
        "depth": 0
      },
      "12": {
        "width": 757298,
        "height": 848328,
        "depth": 0
      },
      "13": {
        "width": 757298,
        "height": 848328,
        "depth": 0
      },
      "14": {
        "width": 1139588,
        "height": 848328,
        "depth": 0
      },
      "15": {
        "width": 1139588,
        "height": 848328,
        "depth": 0
      },
      "16": {
        "width": 367726,
        "height": 524288,
        "depth": 0
      },
      "17": {
        "width": 405956,
        "height": 524288,
        "depth": 200248
      },
      "18": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "19": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "20": {
        "width": 688122,
        "height": 767318,
        "depth": 0
      },
      "21": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "22": {
        "width": 688122,
        "height": 716526,
        "depth": 0
      },
      "23": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "24": {
        "width": 611664,
        "height": 0,
        "depth": 175218
      },
      "25": {
        "width": 680842,
        "height": 848328,
        "depth": 0
      },
      "26": {
        "width": 993954,
        "height": 524288,
        "depth": 0
      },
      "27": {
        "width": 1070412,
        "height": 524288,
        "depth": 0
      },
      "28": {
        "width": 688122,
        "height": 624412,
        "depth": 100124
      },
      "29": {
        "width": 1223328,
        "height": 848328,
        "depth": 0
      },
      "30": {
        "width": 1376244,
        "height": 848328,
        "depth": 0
      },
      "31": {
        "width": 1070412,
        "height": 898390,
        "depth": 50062
      },
      "32": {
        "width": 367726,
        "height": 524288,
        "depth": 0
      },
      "33": {
        "width": 382290,
        "height": 848328,
        "depth": 0
      },
      "34": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "35": {
        "width": 1146870,
        "height": 848328,
        "depth": 200248
      },
      "36": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "37": {
        "width": 1146870,
        "height": 848328,
        "depth": 0
      },
      "38": {
        "width": 1063130,
        "height": 848328,
        "depth": 0
      },
      "39": {
        "width": 382290,
        "height": 848328,
        "depth": 0
      },
      "40": {
        "width": 535206,
        "height": 848328,
        "depth": 200248
      },
      "41": {
        "width": 535206,
        "height": 848328,
        "depth": 200248
      },
      "42": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "43": {
        "width": 1070412,
        "height": 782788,
        "depth": 134708
      },
      "44": {
        "width": 382290,
        "height": 123790,
        "depth": 200248
      },
      "45": {
        "width": 458748,
        "height": 524288,
        "depth": 0
      },
      "46": {
        "width": 382290,
        "height": 123790,
        "depth": 0
      },
      "47": {
        "width": 688122,
        "height": 848328,
        "depth": 200248
      },
      "48": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "49": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "50": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "51": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "52": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "53": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "54": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "55": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "56": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "57": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "58": {
        "width": 382290,
        "height": 524288,
        "depth": 0
      },
      "59": {
        "width": 382290,
        "height": 524288,
        "depth": 200248
      },
      "60": {
        "width": 382290,
        "height": 648080,
        "depth": 200248
      },
      "61": {
        "width": 1070412,
        "height": 490064,
        "depth": 4294809280
      },
      "62": {
        "width": 649894,
        "height": 648080,
        "depth": 200248
      },
      "63": {
        "width": 649894,
        "height": 848328,
        "depth": 0
      },
      "64": {
        "width": 1070412,
        "height": 848328,
        "depth": 0
      },
      "65": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "66": {
        "width": 955726,
        "height": 848328,
        "depth": 0
      },
      "67": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "68": {
        "width": 1032184,
        "height": 848328,
        "depth": 0
      },
      "69": {
        "width": 917496,
        "height": 848328,
        "depth": 0
      },
      "70": {
        "width": 879268,
        "height": 848328,
        "depth": 0
      },
      "71": {
        "width": 1070412,
        "height": 848328,
        "depth": 0
      },
      "72": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "73": {
        "width": 458748,
        "height": 848328,
        "depth": 0
      },
      "74": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "75": {
        "width": 1032184,
        "height": 848328,
        "depth": 0
      },
      "76": {
        "width": 841038,
        "height": 848328,
        "depth": 0
      },
      "77": {
        "width": 1223328,
        "height": 848328,
        "depth": 0
      },
      "78": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "79": {
        "width": 1070412,
        "height": 848328,
        "depth": 0
      },
      "80": {
        "width": 917496,
        "height": 848328,
        "depth": 0
      },
      "81": {
        "width": 1070412,
        "height": 848328,
        "depth": 200248
      },
      "82": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "83": {
        "width": 764580,
        "height": 848328,
        "depth": 0
      },
      "84": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "85": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "86": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "87": {
        "width": 1376244,
        "height": 848328,
        "depth": 0
      },
      "88": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "89": {
        "width": 993954,
        "height": 848328,
        "depth": 0
      },
      "90": {
        "width": 841038,
        "height": 848328,
        "depth": 0
      },
      "91": {
        "width": 382292,
        "height": 848328,
        "depth": 200248
      },
      "92": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "93": {
        "width": 382292,
        "height": 848328,
        "depth": 200248
      },
      "94": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "95": {
        "width": 382290,
        "height": 819982,
        "depth": 0
      },
      "96": {
        "width": 382290,
        "height": 848328,
        "depth": 0
      },
      "97": {
        "width": 661726,
        "height": 524288,
        "depth": 0
      },
      "98": {
        "width": 750016,
        "height": 848328,
        "depth": 0
      },
      "99": {
        "width": 611664,
        "height": 524288,
        "depth": 0
      },
      "100": {
        "width": 750016,
        "height": 848328,
        "depth": 0
      },
      "101": {
        "width": 625318,
        "height": 524288,
        "depth": 0
      },
      "102": {
        "width": 420520,
        "height": 848328,
        "depth": 0
      },
      "103": {
        "width": 688122,
        "height": 524288,
        "depth": 200248
      },
      "104": {
        "width": 750016,
        "height": 848328,
        "depth": 0
      },
      "105": {
        "width": 367726,
        "height": 819982,
        "depth": 0
      },
      "106": {
        "width": 405956,
        "height": 819982,
        "depth": 200248
      },
      "107": {
        "width": 711788,
        "height": 848328,
        "depth": 0
      },
      "108": {
        "width": 367726,
        "height": 848328,
        "depth": 0
      },
      "109": {
        "width": 1132306,
        "height": 524288,
        "depth": 0
      },
      "110": {
        "width": 750016,
        "height": 524288,
        "depth": 0
      },
      "111": {
        "width": 688122,
        "height": 524288,
        "depth": 0
      },
      "112": {
        "width": 750016,
        "height": 524288,
        "depth": 200248
      },
      "113": {
        "width": 711786,
        "height": 524288,
        "depth": 200248
      },
      "114": {
        "width": 537028,
        "height": 524288,
        "depth": 0
      },
      "115": {
        "width": 542852,
        "height": 524288,
        "depth": 0
      },
      "116": {
        "width": 535206,
        "height": 748984,
        "depth": 0
      },
      "117": {
        "width": 750016,
        "height": 524288,
        "depth": 0
      },
      "118": {
        "width": 711788,
        "height": 524288,
        "depth": 0
      },
      "119": {
        "width": 979390,
        "height": 524288,
        "depth": 0
      },
      "120": {
        "width": 711788,
        "height": 524288,
        "depth": 0
      },
      "121": {
        "width": 711788,
        "height": 524288,
        "depth": 200248
      },
      "122": {
        "width": 608022,
        "height": 524288,
        "depth": 0
      },
      "123": {
        "width": 688122,
        "height": 524288,
        "depth": 0
      },
      "124": {
        "width": 1376244,
        "height": 524288,
        "depth": 0
      },
      "125": {
        "width": 688122,
        "height": 848328,
        "depth": 0
      },
      "126": {
        "width": 688122,
        "height": 810880,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cminch": {
    "characters": {
      "48": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "49": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "50": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "51": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "52": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "53": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "54": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "55": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "56": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 576717,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 0,
        "height": 0,
        "depth": 0
      },
      "59": {
        "width": 0,
        "height": 0,
        "depth": 0
      },
      "60": {
        "width": 0,
        "height": 0,
        "depth": 0
      },
      "61": {
        "width": 0,
        "height": 0,
        "depth": 0
      },
      "62": {
        "width": 0,
        "height": 0,
        "depth": 0
      },
      "63": {
        "width": 0,
        "height": 0,
        "depth": 0
      },
      "64": {
        "width": 0,
        "height": 0,
        "depth": 0
      },
      "65": {
        "width": 768955,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 768956,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 736916,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 833035,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 672836,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 640796,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 768956,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 833035,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 346613,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 544677,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 800995,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 608756,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 1025274,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 833035,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 833035,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 736916,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 833035,
        "height": 728178,
        "depth": 110683
      },
      "82": {
        "width": 736916,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 640796,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 768956,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 800995,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 768955,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 1089354,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 768955,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 768955,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 48,
    "largest_character_code": 90,
    "design_size": 109124000
  },
  "cmitt10": {
    "characters": {
      "0": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "1": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "2": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "3": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "4": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "5": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "6": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "7": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "8": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "9": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "10": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "11": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "12": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "13": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "14": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "15": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "16": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "18": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "19": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "20": {
        "width": 550498,
        "height": 593466,
        "depth": 0
      },
      "21": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "22": {
        "width": 550498,
        "height": 593027,
        "depth": 0
      },
      "23": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "24": {
        "width": 550498,
        "height": 0,
        "depth": 203891
      },
      "25": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "26": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 550498,
        "height": 567979,
        "depth": 116509
      },
      "29": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "30": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "31": {
        "width": 550498,
        "height": 699051,
        "depth": 58254
      },
      "32": {
        "width": 550498,
        "height": 230104,
        "depth": 116509
      },
      "33": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "34": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "35": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "36": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "37": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "38": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "39": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "40": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "41": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "42": {
        "width": 550498,
        "height": 546134,
        "depth": 0
      },
      "43": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "44": {
        "width": 550498,
        "height": 131072,
        "depth": 145635
      },
      "45": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "46": {
        "width": 550498,
        "height": 131072,
        "depth": 0
      },
      "47": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "48": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "49": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "50": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "51": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "52": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "53": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "54": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "55": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "56": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "57": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "58": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 550498,
        "height": 451470,
        "depth": 145635
      },
      "60": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "61": {
        "width": 550498,
        "height": 435813,
        "depth": 4294762312
      },
      "62": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "63": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "64": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "65": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "66": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "67": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "68": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "69": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "70": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "71": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "72": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "73": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "74": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "75": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "76": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "77": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "78": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "79": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "80": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "81": {
        "width": 550498,
        "height": 640797,
        "depth": 145635
      },
      "82": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "83": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "84": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "85": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "86": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "87": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "88": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "89": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "90": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "91": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "92": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "93": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "94": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "95": {
        "width": 550498,
        "height": 0,
        "depth": 99757
      },
      "96": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "97": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "99": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "101": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "103": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "104": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "105": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "106": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "107": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "108": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "109": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "113": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "114": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "117": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "122": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "124": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "125": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "126": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmmi10": {
    "characters": {
      "0": {
        "width": 645166,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 873816,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 799829,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 728179,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 778424,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 871630,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 817746,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 611669,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 699051,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 641962,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 809918,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 670776,
        "height": 451470,
        "depth": 0
      },
      "12": {
        "width": 593102,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 542880,
        "height": 451470,
        "depth": 203890
      },
      "14": {
        "width": 466034,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 425621,
        "height": 451470,
        "depth": 0
      },
      "16": {
        "width": 458754,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 520651,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 492248,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 371130,
        "height": 451470,
        "depth": 0
      },
      "20": {
        "width": 604147,
        "height": 451470,
        "depth": 0
      },
      "21": {
        "width": 611672,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 631819,
        "height": 451470,
        "depth": 203890
      },
      "23": {
        "width": 517979,
        "height": 451470,
        "depth": 0
      },
      "24": {
        "width": 458754,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 597717,
        "height": 451470,
        "depth": 0
      },
      "26": {
        "width": 542130,
        "height": 451470,
        "depth": 203890
      },
      "27": {
        "width": 599171,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 458390,
        "height": 451470,
        "depth": 0
      },
      "29": {
        "width": 566525,
        "height": 451470,
        "depth": 0
      },
      "30": {
        "width": 624778,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 656086,
        "height": 451470,
        "depth": 203890
      },
      "32": {
        "width": 683034,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 652691,
        "height": 451470,
        "depth": 0
      },
      "34": {
        "width": 488970,
        "height": 451470,
        "depth": 0
      },
      "35": {
        "width": 620170,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 868357,
        "height": 451470,
        "depth": 0
      },
      "37": {
        "width": 542130,
        "height": 451470,
        "depth": 203890
      },
      "38": {
        "width": 380474,
        "height": 451470,
        "depth": 101946
      },
      "39": {
        "width": 685944,
        "height": 451470,
        "depth": 203890
      },
      "40": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "41": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "42": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "43": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "44": {
        "width": 291272,
        "height": 486275,
        "depth": 4294929283
      },
      "45": {
        "width": 291272,
        "height": 486275,
        "depth": 4294929283
      },
      "46": {
        "width": 524290,
        "height": 487880,
        "depth": 4294930888
      },
      "47": {
        "width": 524290,
        "height": 487880,
        "depth": 4294930888
      },
      "48": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "49": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "51": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "52": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "53": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "54": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "56": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "58": {
        "width": 291272,
        "height": 110683,
        "depth": 0
      },
      "59": {
        "width": 291272,
        "height": 110683,
        "depth": 203890
      },
      "60": {
        "width": 815562,
        "height": 565285,
        "depth": 40997
      },
      "61": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 815562,
        "height": 565285,
        "depth": 40997
      },
      "63": {
        "width": 524290,
        "height": 487880,
        "depth": 4294930888
      },
      "64": {
        "width": 556693,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 795355,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 749440,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 868134,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 774054,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 674294,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 824442,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 871630,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 460938,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 581450,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 890563,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 713616,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1017266,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 842502,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 799829,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 673200,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 828957,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 796173,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 642982,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 612763,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 715944,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 611670,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 990323,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 868718,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 608758,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 715800,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 407781,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 407781,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 407781,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1048579,
        "height": 375013,
        "depth": 4294818021
      },
      "95": {
        "width": 1048579,
        "height": 375013,
        "depth": 4294818021
      },
      "96": {
        "width": 436910,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 554267,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 450014,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 453778,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 545771,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 488245,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 513368,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 500138,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 604147,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 361248,
        "height": 691562,
        "depth": 0
      },
      "106": {
        "width": 431811,
        "height": 691562,
        "depth": 203890
      },
      "107": {
        "width": 545893,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 312874,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 920664,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 629392,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 508269,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 527566,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 468099,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 473075,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 491520,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 378654,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 600266,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 508270,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 750694,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 599291,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 514098,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 487640,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 338120,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 402685,
        "height": 451470,
        "depth": 203890
      },
      "125": {
        "width": 667376,
        "height": 451470,
        "depth": 203890
      },
      "126": {
        "width": 524290,
        "height": 749149,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmmi12": {
    "characters": {
      "0": {
        "width": 636120,
        "height": 716527,
        "depth": 0
      },
      "1": {
        "width": 855600,
        "height": 716527,
        "depth": 0
      },
      "2": {
        "width": 784652,
        "height": 716527,
        "depth": 0
      },
      "3": {
        "width": 712637,
        "height": 716527,
        "depth": 0
      },
      "4": {
        "width": 764064,
        "height": 716527,
        "depth": 0
      },
      "5": {
        "width": 850688,
        "height": 716527,
        "depth": 0
      },
      "6": {
        "width": 802991,
        "height": 716527,
        "depth": 0
      },
      "7": {
        "width": 598920,
        "height": 716527,
        "depth": 0
      },
      "8": {
        "width": 684480,
        "height": 716527,
        "depth": 0
      },
      "9": {
        "width": 627085,
        "height": 716527,
        "depth": 0
      },
      "10": {
        "width": 794435,
        "height": 716527,
        "depth": 0
      },
      "11": {
        "width": 653047,
        "height": 451471,
        "depth": 0
      },
      "12": {
        "width": 579624,
        "height": 728177,
        "depth": 203889
      },
      "13": {
        "width": 532560,
        "height": 451471,
        "depth": 203889
      },
      "14": {
        "width": 454741,
        "height": 728177,
        "depth": 0
      },
      "15": {
        "width": 414572,
        "height": 451471,
        "depth": 0
      },
      "16": {
        "width": 448432,
        "height": 728177,
        "depth": 203889
      },
      "17": {
        "width": 506564,
        "height": 451471,
        "depth": 203889
      },
      "18": {
        "width": 478468,
        "height": 728177,
        "depth": 0
      },
      "19": {
        "width": 362872,
        "height": 451471,
        "depth": 0
      },
      "20": {
        "width": 591032,
        "height": 451471,
        "depth": 0
      },
      "21": {
        "width": 598920,
        "height": 728177,
        "depth": 0
      },
      "22": {
        "width": 617732,
        "height": 451471,
        "depth": 203889
      },
      "23": {
        "width": 507293,
        "height": 451471,
        "depth": 0
      },
      "24": {
        "width": 448432,
        "height": 728177,
        "depth": 203889
      },
      "25": {
        "width": 582416,
        "height": 451471,
        "depth": 0
      },
      "26": {
        "width": 529563,
        "height": 451471,
        "depth": 203889
      },
      "27": {
        "width": 583568,
        "height": 451471,
        "depth": 0
      },
      "28": {
        "width": 445883,
        "height": 451471,
        "depth": 0
      },
      "29": {
        "width": 553409,
        "height": 451471,
        "depth": 0
      },
      "30": {
        "width": 607657,
        "height": 728177,
        "depth": 203889
      },
      "31": {
        "width": 643219,
        "height": 451471,
        "depth": 203889
      },
      "32": {
        "width": 667489,
        "height": 728177,
        "depth": 203889
      },
      "33": {
        "width": 639333,
        "height": 451471,
        "depth": 0
      },
      "34": {
        "width": 480472,
        "height": 451471,
        "depth": 0
      },
      "35": {
        "width": 605109,
        "height": 728177,
        "depth": 0
      },
      "36": {
        "width": 848199,
        "height": 451471,
        "depth": 0
      },
      "37": {
        "width": 529563,
        "height": 451471,
        "depth": 203889
      },
      "38": {
        "width": 371365,
        "height": 451471,
        "depth": 101945
      },
      "39": {
        "width": 672585,
        "height": 451471,
        "depth": 203889
      },
      "40": {
        "width": 1026720,
        "height": 378433,
        "depth": 4294821441
      },
      "41": {
        "width": 1026720,
        "height": 378433,
        "depth": 4294821441
      },
      "42": {
        "width": 1026720,
        "height": 378433,
        "depth": 4294821441
      },
      "43": {
        "width": 1026720,
        "height": 378433,
        "depth": 4294821441
      },
      "44": {
        "width": 285200,
        "height": 475499,
        "depth": 4294918507
      },
      "45": {
        "width": 285200,
        "height": 475499,
        "depth": 4294918507
      },
      "46": {
        "width": 513360,
        "height": 487880,
        "depth": 4294930888
      },
      "47": {
        "width": 513360,
        "height": 487880,
        "depth": 4294930888
      },
      "48": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "49": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "50": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "51": {
        "width": 513360,
        "height": 451471,
        "depth": 203889
      },
      "52": {
        "width": 513360,
        "height": 451471,
        "depth": 203889
      },
      "53": {
        "width": 513360,
        "height": 451471,
        "depth": 203889
      },
      "54": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 513360,
        "height": 451471,
        "depth": 203889
      },
      "56": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 513360,
        "height": 451471,
        "depth": 203889
      },
      "58": {
        "width": 285200,
        "height": 101945,
        "depth": 0
      },
      "59": {
        "width": 285200,
        "height": 101945,
        "depth": 203889
      },
      "60": {
        "width": 798560,
        "height": 552251,
        "depth": 27963
      },
      "61": {
        "width": 513360,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 798560,
        "height": 552251,
        "depth": 27963
      },
      "63": {
        "width": 513360,
        "height": 487880,
        "depth": 4294930888
      },
      "64": {
        "width": 542001,
        "height": 728177,
        "depth": 0
      },
      "65": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "66": {
        "width": 779993,
        "height": 716527,
        "depth": 0
      },
      "67": {
        "width": 734567,
        "height": 716527,
        "depth": 0
      },
      "68": {
        "width": 852471,
        "height": 716527,
        "depth": 0
      },
      "69": {
        "width": 760029,
        "height": 716527,
        "depth": 0
      },
      "70": {
        "width": 664640,
        "height": 716527,
        "depth": 0
      },
      "71": {
        "width": 809872,
        "height": 716527,
        "depth": 0
      },
      "72": {
        "width": 850688,
        "height": 716527,
        "depth": 0
      },
      "73": {
        "width": 452833,
        "height": 716527,
        "depth": 0
      },
      "74": {
        "width": 567491,
        "height": 716527,
        "depth": 0
      },
      "75": {
        "width": 873504,
        "height": 716527,
        "depth": 0
      },
      "76": {
        "width": 698559,
        "height": 716527,
        "depth": 0
      },
      "77": {
        "width": 993288,
        "height": 716527,
        "depth": 0
      },
      "78": {
        "width": 822168,
        "height": 716527,
        "depth": 0
      },
      "79": {
        "width": 784652,
        "height": 716527,
        "depth": 0
      },
      "80": {
        "width": 661788,
        "height": 716527,
        "depth": 0
      },
      "81": {
        "width": 813172,
        "height": 716527,
        "depth": 203889
      },
      "82": {
        "width": 781496,
        "height": 716527,
        "depth": 0
      },
      "83": {
        "width": 631447,
        "height": 716527,
        "depth": 0
      },
      "84": {
        "width": 601772,
        "height": 716527,
        "depth": 0
      },
      "85": {
        "width": 697309,
        "height": 716527,
        "depth": 0
      },
      "86": {
        "width": 598557,
        "height": 716527,
        "depth": 0
      },
      "87": {
        "width": 969317,
        "height": 716527,
        "depth": 0
      },
      "88": {
        "width": 852113,
        "height": 716527,
        "depth": 0
      },
      "89": {
        "width": 595705,
        "height": 716527,
        "depth": 0
      },
      "90": {
        "width": 702747,
        "height": 716527,
        "depth": 0
      },
      "91": {
        "width": 399280,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 399280,
        "height": 728177,
        "depth": 203889
      },
      "93": {
        "width": 399280,
        "height": 728177,
        "depth": 203889
      },
      "94": {
        "width": 1026720,
        "height": 375012,
        "depth": 4294818020
      },
      "95": {
        "width": 1026720,
        "height": 375012,
        "depth": 4294818020
      },
      "96": {
        "width": 430836,
        "height": 728177,
        "depth": 0
      },
      "97": {
        "width": 538967,
        "height": 451471,
        "depth": 0
      },
      "98": {
        "width": 436537,
        "height": 728177,
        "depth": 0
      },
      "99": {
        "width": 441877,
        "height": 451471,
        "depth": 0
      },
      "100": {
        "width": 533507,
        "height": 728177,
        "depth": 0
      },
      "101": {
        "width": 475860,
        "height": 451471,
        "depth": 0
      },
      "102": {
        "width": 506080,
        "height": 728177,
        "depth": 203889
      },
      "103": {
        "width": 491636,
        "height": 451471,
        "depth": 203889
      },
      "104": {
        "width": 591032,
        "height": 728177,
        "depth": 0
      },
      "105": {
        "width": 350260,
        "height": 687436,
        "depth": 0
      },
      "106": {
        "width": 424768,
        "height": 687436,
        "depth": 203889
      },
      "107": {
        "width": 533992,
        "height": 728177,
        "depth": 0
      },
      "108": {
        "width": 305832,
        "height": 728177,
        "depth": 0
      },
      "109": {
        "width": 898076,
        "height": 451471,
        "depth": 0
      },
      "110": {
        "width": 612876,
        "height": 451471,
        "depth": 0
      },
      "111": {
        "width": 493577,
        "height": 451471,
        "depth": 0
      },
      "112": {
        "width": 515303,
        "height": 451471,
        "depth": 203889
      },
      "113": {
        "width": 455228,
        "height": 451471,
        "depth": 203889
      },
      "114": {
        "width": 462692,
        "height": 451471,
        "depth": 0
      },
      "115": {
        "width": 483628,
        "height": 451471,
        "depth": 0
      },
      "116": {
        "width": 370760,
        "height": 644959,
        "depth": 0
      },
      "117": {
        "width": 584356,
        "height": 451471,
        "depth": 0
      },
      "118": {
        "width": 496369,
        "height": 451471,
        "depth": 0
      },
      "119": {
        "width": 733935,
        "height": 451471,
        "depth": 0
      },
      "120": {
        "width": 583448,
        "height": 451471,
        "depth": 0
      },
      "121": {
        "width": 500617,
        "height": 451471,
        "depth": 203889
      },
      "122": {
        "width": 476952,
        "height": 451471,
        "depth": 0
      },
      "123": {
        "width": 327676,
        "height": 451471,
        "depth": 0
      },
      "124": {
        "width": 396248,
        "height": 451471,
        "depth": 203889
      },
      "125": {
        "width": 653655,
        "height": 451471,
        "depth": 203889
      },
      "126": {
        "width": 513360,
        "height": 747401,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 12582912
  },
  "cmmi5": {
    "characters": {
      "0": {
        "width": 873094,
        "height": 716528,
        "depth": 0
      },
      "1": {
        "width": 1208794,
        "height": 716528,
        "depth": 0
      },
      "2": {
        "width": 1098397,
        "height": 716528,
        "depth": 0
      },
      "3": {
        "width": 1013638,
        "height": 716528,
        "depth": 0
      },
      "4": {
        "width": 1067158,
        "height": 716528,
        "depth": 0
      },
      "5": {
        "width": 1164371,
        "height": 716528,
        "depth": 0
      },
      "6": {
        "width": 1117040,
        "height": 716528,
        "depth": 0
      },
      "7": {
        "width": 881107,
        "height": 716528,
        "depth": 0
      },
      "8": {
        "width": 990336,
        "height": 716528,
        "depth": 0
      },
      "9": {
        "width": 936886,
        "height": 716528,
        "depth": 0
      },
      "10": {
        "width": 1112125,
        "height": 716528,
        "depth": 0
      },
      "11": {
        "width": 957686,
        "height": 451469,
        "depth": 0
      },
      "12": {
        "width": 829040,
        "height": 728179,
        "depth": 203891
      },
      "13": {
        "width": 783187,
        "height": 451469,
        "depth": 203891
      },
      "14": {
        "width": 685949,
        "height": 728179,
        "depth": 0
      },
      "15": {
        "width": 643350,
        "height": 451469,
        "depth": 0
      },
      "16": {
        "width": 699059,
        "height": 728179,
        "depth": 203891
      },
      "17": {
        "width": 779891,
        "height": 451469,
        "depth": 203891
      },
      "18": {
        "width": 709978,
        "height": 728179,
        "depth": 0
      },
      "19": {
        "width": 576480,
        "height": 451469,
        "depth": 0
      },
      "20": {
        "width": 867757,
        "height": 451469,
        "depth": 0
      },
      "21": {
        "width": 881107,
        "height": 728179,
        "depth": 0
      },
      "22": {
        "width": 891059,
        "height": 451469,
        "depth": 203891
      },
      "23": {
        "width": 746634,
        "height": 451469,
        "depth": 0
      },
      "24": {
        "width": 699059,
        "height": 728179,
        "depth": 203891
      },
      "25": {
        "width": 871520,
        "height": 451469,
        "depth": 0
      },
      "26": {
        "width": 761318,
        "height": 451469,
        "depth": 203891
      },
      "27": {
        "width": 854768,
        "height": 451469,
        "depth": 0
      },
      "28": {
        "width": 714717,
        "height": 451469,
        "depth": 0
      },
      "29": {
        "width": 830134,
        "height": 451469,
        "depth": 0
      },
      "30": {
        "width": 882560,
        "height": 728179,
        "depth": 203891
      },
      "31": {
        "width": 906592,
        "height": 451469,
        "depth": 203891
      },
      "32": {
        "width": 975773,
        "height": 728179,
        "depth": 203891
      },
      "33": {
        "width": 929411,
        "height": 451469,
        "depth": 0
      },
      "34": {
        "width": 707430,
        "height": 451469,
        "depth": 0
      },
      "35": {
        "width": 896886,
        "height": 728179,
        "depth": 0
      },
      "36": {
        "width": 1200419,
        "height": 451469,
        "depth": 0
      },
      "37": {
        "width": 761318,
        "height": 451469,
        "depth": 203891
      },
      "38": {
        "width": 606214,
        "height": 451469,
        "depth": 101946
      },
      "39": {
        "width": 962662,
        "height": 451469,
        "depth": 203891
      },
      "40": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "41": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "42": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "43": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "44": {
        "width": 480602,
        "height": 539814,
        "depth": 15526
      },
      "45": {
        "width": 480602,
        "height": 539814,
        "depth": 15526
      },
      "46": {
        "width": 771878,
        "height": 487878,
        "depth": 4294930886
      },
      "47": {
        "width": 771878,
        "height": 487878,
        "depth": 4294930886
      },
      "48": {
        "width": 771878,
        "height": 451469,
        "depth": 0
      },
      "49": {
        "width": 771878,
        "height": 451469,
        "depth": 0
      },
      "50": {
        "width": 771878,
        "height": 451469,
        "depth": 0
      },
      "51": {
        "width": 771878,
        "height": 451469,
        "depth": 203891
      },
      "52": {
        "width": 771878,
        "height": 451469,
        "depth": 203891
      },
      "53": {
        "width": 771878,
        "height": 451469,
        "depth": 203891
      },
      "54": {
        "width": 771878,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 771878,
        "height": 451469,
        "depth": 203891
      },
      "56": {
        "width": 771878,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 771878,
        "height": 451469,
        "depth": 203891
      },
      "58": {
        "width": 480602,
        "height": 128160,
        "depth": 0
      },
      "59": {
        "width": 480602,
        "height": 128160,
        "depth": 203891
      },
      "60": {
        "width": 1135974,
        "height": 630106,
        "depth": 105818
      },
      "61": {
        "width": 771878,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 1135974,
        "height": 630106,
        "depth": 105818
      },
      "63": {
        "width": 771878,
        "height": 487878,
        "depth": 4294930886
      },
      "64": {
        "width": 785344,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1086458,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 1087366,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 1044368,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 1163789,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 1055872,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 909504,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 1115728,
        "height": 716528,
        "depth": 0
      },
      "72": {
        "width": 1164371,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 657187,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 810470,
        "height": 716528,
        "depth": 0
      },
      "75": {
        "width": 1194230,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 1001987,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1346419,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 1127962,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 1098397,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 917878,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 1134806,
        "height": 716528,
        "depth": 203891
      },
      "82": {
        "width": 1079994,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 897856,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 872733,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 989750,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 868000,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1341325,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 1166922,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 864358,
        "height": 716528,
        "depth": 0
      },
      "90": {
        "width": 988880,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 626240,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 626240,
        "height": 728179,
        "depth": 203891
      },
      "93": {
        "width": 626240,
        "height": 728179,
        "depth": 203891
      },
      "94": {
        "width": 1427251,
        "height": 375011,
        "depth": 4294818019
      },
      "95": {
        "width": 1427251,
        "height": 375011,
        "depth": 4294818019
      },
      "96": {
        "width": 626237,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 812048,
        "height": 451469,
        "depth": 0
      },
      "98": {
        "width": 664102,
        "height": 728179,
        "depth": 0
      },
      "99": {
        "width": 680973,
        "height": 451469,
        "depth": 0
      },
      "100": {
        "width": 775638,
        "height": 728179,
        "depth": 0
      },
      "101": {
        "width": 709904,
        "height": 451469,
        "depth": 0
      },
      "102": {
        "width": 717261,
        "height": 728179,
        "depth": 203891
      },
      "103": {
        "width": 734618,
        "height": 451469,
        "depth": 203891
      },
      "104": {
        "width": 867757,
        "height": 728179,
        "depth": 0
      },
      "105": {
        "width": 559491,
        "height": 698842,
        "depth": 0
      },
      "106": {
        "width": 616771,
        "height": 698842,
        "depth": 203891
      },
      "107": {
        "width": 794938,
        "height": 728179,
        "depth": 0
      },
      "108": {
        "width": 503661,
        "height": 728179,
        "depth": 0
      },
      "109": {
        "width": 1287683,
        "height": 451469,
        "depth": 0
      },
      "110": {
        "width": 923587,
        "height": 451469,
        "depth": 0
      },
      "111": {
        "width": 736922,
        "height": 451469,
        "depth": 0
      },
      "112": {
        "width": 775642,
        "height": 451469,
        "depth": 203891
      },
      "113": {
        "width": 690925,
        "height": 451469,
        "depth": 203891
      },
      "114": {
        "width": 703914,
        "height": 451469,
        "depth": 0
      },
      "115": {
        "width": 702698,
        "height": 451469,
        "depth": 0
      },
      "116": {
        "width": 591046,
        "height": 644957,
        "depth": 0
      },
      "117": {
        "width": 887178,
        "height": 451469,
        "depth": 0
      },
      "118": {
        "width": 757315,
        "height": 451469,
        "depth": 0
      },
      "119": {
        "width": 1057997,
        "height": 451469,
        "depth": 0
      },
      "120": {
        "width": 830131,
        "height": 451469,
        "depth": 0
      },
      "121": {
        "width": 766054,
        "height": 451469,
        "depth": 203891
      },
      "122": {
        "width": 722118,
        "height": 451469,
        "depth": 0
      },
      "123": {
        "width": 559491,
        "height": 451469,
        "depth": 0
      },
      "124": {
        "width": 580362,
        "height": 451469,
        "depth": 203891
      },
      "125": {
        "width": 932445,
        "height": 451469,
        "depth": 203891
      },
      "126": {
        "width": 771878,
        "height": 757539,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 5242880
  },
  "cmmi6": {
    "characters": {
      "0": {
        "width": 778661,
        "height": 716528,
        "depth": 0
      },
      "1": {
        "width": 1077696,
        "height": 716528,
        "depth": 0
      },
      "2": {
        "width": 979440,
        "height": 716528,
        "depth": 0
      },
      "3": {
        "width": 900992,
        "height": 716528,
        "depth": 0
      },
      "4": {
        "width": 951483,
        "height": 716528,
        "depth": 0
      },
      "5": {
        "width": 1048085,
        "height": 716528,
        "depth": 0
      },
      "6": {
        "width": 997843,
        "height": 716528,
        "depth": 0
      },
      "7": {
        "width": 771864,
        "height": 716528,
        "depth": 0
      },
      "8": {
        "width": 873808,
        "height": 716528,
        "depth": 0
      },
      "9": {
        "width": 819144,
        "height": 716528,
        "depth": 0
      },
      "10": {
        "width": 991955,
        "height": 716528,
        "depth": 0
      },
      "11": {
        "width": 843592,
        "height": 451469,
        "depth": 0
      },
      "12": {
        "width": 731941,
        "height": 728179,
        "depth": 203891
      },
      "13": {
        "width": 683656,
        "height": 451469,
        "depth": 203891
      },
      "14": {
        "width": 593707,
        "height": 728179,
        "depth": 0
      },
      "15": {
        "width": 548680,
        "height": 451469,
        "depth": 0
      },
      "16": {
        "width": 599531,
        "height": 728179,
        "depth": 203891
      },
      "17": {
        "width": 675259,
        "height": 451469,
        "depth": 203891
      },
      "18": {
        "width": 618955,
        "height": 728179,
        "depth": 0
      },
      "19": {
        "width": 489093,
        "height": 451469,
        "depth": 0
      },
      "20": {
        "width": 760944,
        "height": 451469,
        "depth": 0
      },
      "21": {
        "width": 771864,
        "height": 728179,
        "depth": 0
      },
      "22": {
        "width": 786432,
        "height": 451469,
        "depth": 203891
      },
      "23": {
        "width": 651715,
        "height": 451469,
        "depth": 0
      },
      "24": {
        "width": 599531,
        "height": 728179,
        "depth": 203891
      },
      "25": {
        "width": 762032,
        "height": 451469,
        "depth": 0
      },
      "26": {
        "width": 670048,
        "height": 451469,
        "depth": 203891
      },
      "27": {
        "width": 751355,
        "height": 451469,
        "depth": 0
      },
      "28": {
        "width": 610331,
        "height": 451469,
        "depth": 0
      },
      "29": {
        "width": 723320,
        "height": 451469,
        "depth": 0
      },
      "30": {
        "width": 778179,
        "height": 728179,
        "depth": 203891
      },
      "31": {
        "width": 804632,
        "height": 451469,
        "depth": 203891
      },
      "32": {
        "width": 859245,
        "height": 728179,
        "depth": 203891
      },
      "33": {
        "width": 817739,
        "height": 451469,
        "depth": 0
      },
      "34": {
        "width": 615184,
        "height": 451469,
        "depth": 0
      },
      "35": {
        "width": 787157,
        "height": 728179,
        "depth": 0
      },
      "36": {
        "width": 1071507,
        "height": 451469,
        "depth": 0
      },
      "37": {
        "width": 670048,
        "height": 451469,
        "depth": 203891
      },
      "38": {
        "width": 511541,
        "height": 451469,
        "depth": 101947
      },
      "39": {
        "width": 850992,
        "height": 451469,
        "depth": 203891
      },
      "40": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "41": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "42": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "43": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "44": {
        "width": 398069,
        "height": 528512,
        "depth": 4224
      },
      "45": {
        "width": 398069,
        "height": 528512,
        "depth": 4224
      },
      "46": {
        "width": 669920,
        "height": 487880,
        "depth": 4294930888
      },
      "47": {
        "width": 669920,
        "height": 487880,
        "depth": 4294930888
      },
      "48": {
        "width": 669920,
        "height": 451469,
        "depth": 0
      },
      "49": {
        "width": 669920,
        "height": 451469,
        "depth": 0
      },
      "50": {
        "width": 669920,
        "height": 451469,
        "depth": 0
      },
      "51": {
        "width": 669920,
        "height": 451469,
        "depth": 203891
      },
      "52": {
        "width": 669920,
        "height": 451469,
        "depth": 203891
      },
      "53": {
        "width": 669920,
        "height": 451469,
        "depth": 203891
      },
      "54": {
        "width": 669920,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 669920,
        "height": 451469,
        "depth": 203891
      },
      "56": {
        "width": 669920,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 669920,
        "height": 451469,
        "depth": 203891
      },
      "58": {
        "width": 398069,
        "height": 126219,
        "depth": 0
      },
      "59": {
        "width": 398069,
        "height": 126219,
        "depth": 203891
      },
      "60": {
        "width": 1009733,
        "height": 616549,
        "depth": 92261
      },
      "61": {
        "width": 669920,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 1009733,
        "height": 616549,
        "depth": 92261
      },
      "63": {
        "width": 669920,
        "height": 487880,
        "depth": 4294930888
      },
      "64": {
        "width": 690677,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 968955,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 971565,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 926624,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 1046773,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 943472,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 812643,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 999200,
        "height": 716528,
        "depth": 0
      },
      "72": {
        "width": 1048085,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 574288,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 714707,
        "height": 716528,
        "depth": 0
      },
      "75": {
        "width": 1075512,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 887400,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1217992,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 1014104,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 979440,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 817861,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 1013421,
        "height": 716528,
        "depth": 203891
      },
      "82": {
        "width": 966923,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 793469,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 766645,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 878808,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 765069,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1206827,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 1050027,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 761669,
        "height": 716528,
        "depth": 0
      },
      "90": {
        "width": 878421,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 533995,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 533995,
        "height": 728179,
        "depth": 203891
      },
      "93": {
        "width": 533995,
        "height": 728179,
        "depth": 203891
      },
      "94": {
        "width": 1281584,
        "height": 375013,
        "depth": 4294818021
      },
      "95": {
        "width": 1281584,
        "height": 375013,
        "depth": 4294818021
      },
      "96": {
        "width": 543707,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 707667,
        "height": 451469,
        "depth": 0
      },
      "98": {
        "width": 574291,
        "height": 728179,
        "depth": 0
      },
      "99": {
        "width": 586304,
        "height": 451469,
        "depth": 0
      },
      "100": {
        "width": 673685,
        "height": 728179,
        "depth": 0
      },
      "101": {
        "width": 617616,
        "height": 451469,
        "depth": 0
      },
      "102": {
        "width": 629872,
        "height": 728179,
        "depth": 203891
      },
      "103": {
        "width": 637269,
        "height": 451469,
        "depth": 203891
      },
      "104": {
        "width": 760944,
        "height": 728179,
        "depth": 0
      },
      "105": {
        "width": 467248,
        "height": 694717,
        "depth": 0
      },
      "106": {
        "width": 536424,
        "height": 694717,
        "depth": 203891
      },
      "107": {
        "width": 692981,
        "height": 728179,
        "depth": 0
      },
      "108": {
        "width": 421131,
        "height": 728179,
        "depth": 0
      },
      "109": {
        "width": 1146875,
        "height": 451469,
        "depth": 0
      },
      "110": {
        "width": 807061,
        "height": 451469,
        "depth": 0
      },
      "111": {
        "width": 642253,
        "height": 451469,
        "depth": 0
      },
      "112": {
        "width": 673685,
        "height": 451469,
        "depth": 203891
      },
      "113": {
        "width": 598437,
        "height": 451469,
        "depth": 203891
      },
      "114": {
        "width": 608027,
        "height": 451469,
        "depth": 0
      },
      "115": {
        "width": 612883,
        "height": 451469,
        "depth": 0
      },
      "116": {
        "width": 500013,
        "height": 644957,
        "depth": 0
      },
      "117": {
        "width": 773080,
        "height": 451469,
        "depth": 0
      },
      "118": {
        "width": 655357,
        "height": 451469,
        "depth": 0
      },
      "119": {
        "width": 936613,
        "height": 451469,
        "depth": 0
      },
      "120": {
        "width": 731816,
        "height": 451469,
        "depth": 0
      },
      "121": {
        "width": 663851,
        "height": 451469,
        "depth": 203891
      },
      "122": {
        "width": 625019,
        "height": 451469,
        "depth": 0
      },
      "123": {
        "width": 467248,
        "height": 451469,
        "depth": 0
      },
      "124": {
        "width": 502443,
        "height": 451469,
        "depth": 203891
      },
      "125": {
        "width": 825387,
        "height": 451469,
        "depth": 203891
      },
      "126": {
        "width": 669920,
        "height": 755267,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 6291456
  },
  "cmmi7": {
    "characters": {
      "0": {
        "width": 727454,
        "height": 716528,
        "depth": 0
      },
      "1": {
        "width": 1000729,
        "height": 716528,
        "depth": 0
      },
      "2": {
        "width": 911138,
        "height": 716528,
        "depth": 0
      },
      "3": {
        "width": 836370,
        "height": 716528,
        "depth": 0
      },
      "4": {
        "width": 885520,
        "height": 716528,
        "depth": 0
      },
      "5": {
        "width": 981070,
        "height": 716528,
        "depth": 0
      },
      "6": {
        "width": 929367,
        "height": 716528,
        "depth": 0
      },
      "7": {
        "width": 710494,
        "height": 716528,
        "depth": 0
      },
      "8": {
        "width": 807239,
        "height": 716528,
        "depth": 0
      },
      "9": {
        "width": 751710,
        "height": 716528,
        "depth": 0
      },
      "10": {
        "width": 922789,
        "height": 716528,
        "depth": 0
      },
      "11": {
        "width": 778757,
        "height": 451470,
        "depth": 0
      },
      "12": {
        "width": 679239,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 629225,
        "height": 451470,
        "depth": 203890
      },
      "14": {
        "width": 544471,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 499273,
        "height": 451470,
        "depth": 0
      },
      "16": {
        "width": 545095,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 617189,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 570578,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 443326,
        "height": 451470,
        "depth": 0
      },
      "20": {
        "width": 701310,
        "height": 451470,
        "depth": 0
      },
      "21": {
        "width": 710498,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 728359,
        "height": 451470,
        "depth": 203890
      },
      "23": {
        "width": 600578,
        "height": 451470,
        "depth": 0
      },
      "24": {
        "width": 545095,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 700496,
        "height": 451470,
        "depth": 0
      },
      "26": {
        "width": 621504,
        "height": 451470,
        "depth": 203890
      },
      "27": {
        "width": 694149,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 552430,
        "height": 451470,
        "depth": 0
      },
      "29": {
        "width": 663687,
        "height": 451470,
        "depth": 0
      },
      "30": {
        "width": 720274,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 748466,
        "height": 451470,
        "depth": 203890
      },
      "32": {
        "width": 792679,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 754640,
        "height": 451470,
        "depth": 0
      },
      "34": {
        "width": 565952,
        "height": 451470,
        "depth": 0
      },
      "35": {
        "width": 723365,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 996103,
        "height": 451470,
        "depth": 0
      },
      "37": {
        "width": 621504,
        "height": 451470,
        "depth": 203890
      },
      "38": {
        "width": 460574,
        "height": 451470,
        "depth": 101945
      },
      "39": {
        "width": 787893,
        "height": 451470,
        "depth": 203890
      },
      "40": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "41": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "42": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "43": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "44": {
        "width": 355769,
        "height": 517861,
        "depth": 4294960869
      },
      "45": {
        "width": 355769,
        "height": 517861,
        "depth": 4294960869
      },
      "46": {
        "width": 613753,
        "height": 487879,
        "depth": 4294930887
      },
      "47": {
        "width": 613753,
        "height": 487879,
        "depth": 4294930887
      },
      "48": {
        "width": 613753,
        "height": 451470,
        "depth": 0
      },
      "49": {
        "width": 613753,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 613753,
        "height": 451470,
        "depth": 0
      },
      "51": {
        "width": 613753,
        "height": 451470,
        "depth": 203890
      },
      "52": {
        "width": 613753,
        "height": 451470,
        "depth": 203890
      },
      "53": {
        "width": 613753,
        "height": 451470,
        "depth": 203890
      },
      "54": {
        "width": 613753,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 613753,
        "height": 451470,
        "depth": 203890
      },
      "56": {
        "width": 613753,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 613753,
        "height": 451470,
        "depth": 203890
      },
      "58": {
        "width": 355769,
        "height": 120670,
        "depth": 0
      },
      "59": {
        "width": 355769,
        "height": 120670,
        "depth": 203890
      },
      "60": {
        "width": 936233,
        "height": 603639,
        "depth": 79351
      },
      "61": {
        "width": 613753,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 936233,
        "height": 603639,
        "depth": 79351
      },
      "63": {
        "width": 613753,
        "height": 487879,
        "depth": 4294930887
      },
      "64": {
        "width": 639705,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 900866,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 905106,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 859191,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 979445,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 879435,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 759703,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 932631,
        "height": 716528,
        "depth": 0
      },
      "72": {
        "width": 981070,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 530898,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 662747,
        "height": 716528,
        "depth": 0
      },
      "75": {
        "width": 1006558,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 821806,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1142309,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 948821,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 911138,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 762665,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 943387,
        "height": 716528,
        "depth": 203890
      },
      "82": {
        "width": 902414,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 735568,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 707531,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 816021,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 707378,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1126601,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 982370,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 704155,
        "height": 716528,
        "depth": 0
      },
      "90": {
        "width": 816187,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 484761,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 484761,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 484761,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1194217,
        "height": 375013,
        "depth": 4294818021
      },
      "95": {
        "width": 1194217,
        "height": 375013,
        "depth": 4294818021
      },
      "96": {
        "width": 501408,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 649765,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 526786,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 535335,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 623584,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 568347,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 584110,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 584402,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 701310,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 423819,
        "height": 693849,
        "depth": 0
      },
      "106": {
        "width": 495687,
        "height": 693849,
        "depth": 203890
      },
      "107": {
        "width": 636814,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 378827,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1062974,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 740494,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 591282,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 617515,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 549033,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 556194,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 565381,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 452514,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 708247,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 599191,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 866581,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 679289,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 607515,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 572320,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 418014,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 463440,
        "height": 451470,
        "depth": 203890
      },
      "125": {
        "width": 765579,
        "height": 451470,
        "depth": 203890
      },
      "126": {
        "width": 613753,
        "height": 753643,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmmi8": {
    "characters": {
      "0": {
        "width": 674118,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 928440,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 845352,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 772608,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 821486,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 915240,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 863448,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 649908,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 742752,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 686572,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 856348,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 715566,
        "height": 451470,
        "depth": 0
      },
      "12": {
        "width": 625148,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 573836,
        "height": 451470,
        "depth": 203890
      },
      "14": {
        "width": 492984,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 450384,
        "height": 451470,
        "depth": 0
      },
      "16": {
        "width": 489708,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 559066,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 519742,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 394434,
        "height": 451470,
        "depth": 0
      },
      "20": {
        "width": 642018,
        "height": 451470,
        "depth": 0
      },
      "21": {
        "width": 649908,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 670234,
        "height": 451470,
        "depth": 203890
      },
      "23": {
        "width": 547658,
        "height": 451470,
        "depth": 0
      },
      "24": {
        "width": 489708,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 639774,
        "height": 451470,
        "depth": 0
      },
      "26": {
        "width": 570534,
        "height": 451470,
        "depth": 203890
      },
      "27": {
        "width": 636680,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 494440,
        "height": 451470,
        "depth": 0
      },
      "29": {
        "width": 604396,
        "height": 451470,
        "depth": 0
      },
      "30": {
        "width": 662288,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 691776,
        "height": 451470,
        "depth": 203890
      },
      "32": {
        "width": 728188,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 692750,
        "height": 451470,
        "depth": 0
      },
      "34": {
        "width": 514464,
        "height": 451470,
        "depth": 0
      },
      "35": {
        "width": 662772,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 924980,
        "height": 451470,
        "depth": 0
      },
      "37": {
        "width": 570534,
        "height": 451470,
        "depth": 203890
      },
      "38": {
        "width": 407786,
        "height": 451470,
        "depth": 101946
      },
      "39": {
        "width": 726004,
        "height": 451470,
        "depth": 203890
      },
      "40": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "41": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "42": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "43": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "44": {
        "width": 309480,
        "height": 506958,
        "depth": 4294949966
      },
      "45": {
        "width": 309480,
        "height": 506958,
        "depth": 4294949966
      },
      "46": {
        "width": 557064,
        "height": 487880,
        "depth": 4294930888
      },
      "47": {
        "width": 557064,
        "height": 487880,
        "depth": 4294930888
      },
      "48": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "49": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "51": {
        "width": 557064,
        "height": 451470,
        "depth": 203890
      },
      "52": {
        "width": 557064,
        "height": 451470,
        "depth": 203890
      },
      "53": {
        "width": 557064,
        "height": 451470,
        "depth": 203890
      },
      "54": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 557064,
        "height": 451470,
        "depth": 203890
      },
      "56": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 557064,
        "height": 451470,
        "depth": 203890
      },
      "58": {
        "width": 309480,
        "height": 116508,
        "depth": 0
      },
      "59": {
        "width": 309480,
        "height": 116508,
        "depth": 203890
      },
      "60": {
        "width": 866544,
        "height": 590480,
        "depth": 66192
      },
      "61": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 866544,
        "height": 590480,
        "depth": 66192
      },
      "63": {
        "width": 557064,
        "height": 487880,
        "depth": 4294930888
      },
      "64": {
        "width": 586918,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 840328,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 794052,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 914020,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 816480,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 705066,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 868144,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 915240,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 483062,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 608582,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 939544,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 757680,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1069980,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 884292,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 845352,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 706340,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 876300,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 839098,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 677578,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 648634,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 753728,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 648816,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1051140,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 916334,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 645722,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 754948,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 433272,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 433272,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 433272,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1114128,
        "height": 375012,
        "depth": 4294818020
      },
      "95": {
        "width": 1114128,
        "height": 375012,
        "depth": 4294818020
      },
      "96": {
        "width": 455116,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 591774,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 476600,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 482546,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 573264,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 516832,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 535216,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 530182,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 642018,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 379316,
        "height": 693200,
        "depth": 0
      },
      "106": {
        "width": 450564,
        "height": 693200,
        "depth": 203890
      },
      "107": {
        "width": 580122,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 332538,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 985480,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 676000,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 538496,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 560826,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 497414,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 502752,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 515192,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 402324,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 645052,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 542500,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 799490,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 627150,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 550692,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 518226,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 366520,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 419616,
        "height": 451470,
        "depth": 203890
      },
      "125": {
        "width": 706160,
        "height": 451470,
        "depth": 203890
      },
      "126": {
        "width": 557064,
        "height": 751772,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmmi9": {
    "characters": {
      "0": {
        "width": 658188,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 898080,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 820052,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 748238,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 797552,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 891447,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 838048,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 628656,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 718464,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 661776,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 830542,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 690672,
        "height": 451470,
        "depth": 0
      },
      "12": {
        "width": 607339,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 556631,
        "height": 451470,
        "depth": 203890
      },
      "14": {
        "width": 478005,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 436620,
        "height": 451470,
        "depth": 0
      },
      "16": {
        "width": 472503,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 538121,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 504464,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 381483,
        "height": 451470,
        "depth": 0
      },
      "20": {
        "width": 620971,
        "height": 451470,
        "depth": 0
      },
      "21": {
        "width": 628656,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 649289,
        "height": 451470,
        "depth": 203890
      },
      "23": {
        "width": 531566,
        "height": 451470,
        "depth": 0
      },
      "24": {
        "width": 472503,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 616802,
        "height": 451470,
        "depth": 0
      },
      "26": {
        "width": 555154,
        "height": 451470,
        "depth": 203890
      },
      "27": {
        "width": 616236,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 474404,
        "height": 451470,
        "depth": 0
      },
      "29": {
        "width": 583348,
        "height": 451470,
        "depth": 0
      },
      "30": {
        "width": 641440,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 671943,
        "height": 451470,
        "depth": 203890
      },
      "32": {
        "width": 703092,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 670485,
        "height": 451470,
        "depth": 0
      },
      "34": {
        "width": 500295,
        "height": 451470,
        "depth": 0
      },
      "35": {
        "width": 639095,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 893913,
        "height": 451470,
        "depth": 0
      },
      "37": {
        "width": 555154,
        "height": 451470,
        "depth": 203890
      },
      "38": {
        "width": 392606,
        "height": 451470,
        "depth": 101945
      },
      "39": {
        "width": 703739,
        "height": 451470,
        "depth": 203890
      },
      "40": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "41": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "42": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "43": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "44": {
        "width": 299360,
        "height": 496471,
        "depth": 4294939479
      },
      "45": {
        "width": 299360,
        "height": 496471,
        "depth": 4294939479
      },
      "46": {
        "width": 538848,
        "height": 487879,
        "depth": 4294930887
      },
      "47": {
        "width": 538848,
        "height": 487879,
        "depth": 4294930887
      },
      "48": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "49": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "51": {
        "width": 538848,
        "height": 451470,
        "depth": 203890
      },
      "52": {
        "width": 538848,
        "height": 451470,
        "depth": 203890
      },
      "53": {
        "width": 538848,
        "height": 451470,
        "depth": 203890
      },
      "54": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 538848,
        "height": 451470,
        "depth": 203890
      },
      "56": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 538848,
        "height": 451470,
        "depth": 203890
      },
      "58": {
        "width": 299360,
        "height": 113273,
        "depth": 0
      },
      "59": {
        "width": 299360,
        "height": 113273,
        "depth": 203890
      },
      "60": {
        "width": 838208,
        "height": 577737,
        "depth": 53449
      },
      "61": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 838208,
        "height": 577737,
        "depth": 53449
      },
      "63": {
        "width": 538848,
        "height": 487879,
        "depth": 4294930887
      },
      "64": {
        "width": 570119,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 815493,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 769259,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 888679,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 793063,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 688124,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 843856,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 891447,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 471088,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 593787,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 912645,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 733351,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1041127,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 861511,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 820052,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 688084,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 849988,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 815403,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 658350,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 628697,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 733012,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 628496,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1017664,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 890192,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 625500,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 733191,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 419104,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 419104,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 419104,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1077696,
        "height": 375012,
        "depth": 4294818020
      },
      "95": {
        "width": 1077696,
        "height": 375012,
        "depth": 4294818020
      },
      "96": {
        "width": 444996,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 570928,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 461824,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 466556,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 558388,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 501024,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 523072,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 513886,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 620971,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 368772,
        "height": 691886,
        "depth": 0
      },
      "106": {
        "width": 440546,
        "height": 691886,
        "depth": 203890
      },
      "107": {
        "width": 561099,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 321611,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 949458,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 650098,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 521696,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 540994,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 481524,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 486260,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 502037,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 389168,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 620162,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 523476,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 772370,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 610859,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 530757,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 501227,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 350738,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 410610,
        "height": 451470,
        "depth": 203890
      },
      "125": {
        "width": 685010,
        "height": 451470,
        "depth": 203890
      },
      "126": {
        "width": 538848,
        "height": 750315,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmmib10": {
    "characters": {
      "0": {
        "width": 688853,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1004880,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 909344,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 844682,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 882110,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1030003,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 928059,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 703416,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 803904,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 748706,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 921597,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 797594,
        "height": 466034,
        "depth": 0
      },
      "12": {
        "width": 691766,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 618691,
        "height": 466034,
        "depth": 203890
      },
      "14": {
        "width": 547586,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 506810,
        "height": 466034,
        "depth": 0
      },
      "16": {
        "width": 533024,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 629144,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 589093,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 432051,
        "height": 466034,
        "depth": 0
      },
      "20": {
        "width": 700019,
        "height": 466034,
        "depth": 0
      },
      "21": {
        "width": 703416,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 742254,
        "height": 466034,
        "depth": 203890
      },
      "23": {
        "width": 604870,
        "height": 466034,
        "depth": 0
      },
      "24": {
        "width": 533024,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 715554,
        "height": 466034,
        "depth": 0
      },
      "26": {
        "width": 641523,
        "height": 466034,
        "depth": 203890
      },
      "27": {
        "width": 719194,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 546131,
        "height": 466034,
        "depth": 0
      },
      "29": {
        "width": 661182,
        "height": 466034,
        "depth": 0
      },
      "30": {
        "width": 747106,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 752934,
        "height": 466034,
        "depth": 203890
      },
      "32": {
        "width": 795166,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 752688,
        "height": 466034,
        "depth": 0
      },
      "34": {
        "width": 554506,
        "height": 466034,
        "depth": 0
      },
      "35": {
        "width": 725141,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 1022358,
        "height": 466034,
        "depth": 0
      },
      "37": {
        "width": 641523,
        "height": 466034,
        "depth": 203890
      },
      "38": {
        "width": 444186,
        "height": 466034,
        "depth": 101946
      },
      "39": {
        "width": 783514,
        "height": 466034,
        "depth": 203890
      },
      "40": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "41": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "42": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "43": {
        "width": 1205856,
        "height": 410110,
        "depth": 4294853118
      },
      "44": {
        "width": 334960,
        "height": 526619,
        "depth": 2331
      },
      "45": {
        "width": 334960,
        "height": 526619,
        "depth": 2331
      },
      "46": {
        "width": 602928,
        "height": 495162,
        "depth": 4294938170
      },
      "47": {
        "width": 602928,
        "height": 495162,
        "depth": 4294938170
      },
      "48": {
        "width": 602928,
        "height": 466034,
        "depth": 0
      },
      "49": {
        "width": 602928,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 602928,
        "height": 466034,
        "depth": 0
      },
      "51": {
        "width": 602928,
        "height": 466034,
        "depth": 203890
      },
      "52": {
        "width": 602928,
        "height": 466034,
        "depth": 203890
      },
      "53": {
        "width": 602928,
        "height": 466034,
        "depth": 203890
      },
      "54": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 602928,
        "height": 466034,
        "depth": 203890
      },
      "56": {
        "width": 602928,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 602928,
        "height": 466034,
        "depth": 203890
      },
      "58": {
        "width": 334960,
        "height": 163112,
        "depth": 0
      },
      "59": {
        "width": 334960,
        "height": 163112,
        "depth": 203890
      },
      "60": {
        "width": 937888,
        "height": 614000,
        "depth": 89712
      },
      "61": {
        "width": 602928,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 937888,
        "height": 614000,
        "depth": 89712
      },
      "63": {
        "width": 602928,
        "height": 495162,
        "depth": 4294938170
      },
      "64": {
        "width": 658997,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 911674,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 908490,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 856624,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 983691,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 849416,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 722349,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 929806,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1030003,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 535938,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 661912,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1018352,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 792253,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1197483,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 996507,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 877304,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 758213,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 910800,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 914725,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 726355,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 667552,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 839149,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 710699,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1146147,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 993230,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 707347,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 810094,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 468944,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 468944,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 468944,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1205856,
        "height": 378653,
        "depth": 4294821661
      },
      "95": {
        "width": 1205856,
        "height": 378653,
        "depth": 4294821661
      },
      "96": {
        "width": 496618,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 663610,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 546130,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 538363,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 639338,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 580501,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 595651,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 571373,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 700019,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 424459,
        "height": 726931,
        "depth": 0
      },
      "106": {
        "width": 493704,
        "height": 726931,
        "depth": 203890
      },
      "107": {
        "width": 633027,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 365059,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1082554,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 747594,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 613122,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 630114,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 568461,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 554386,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 557056,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 435448,
        "height": 665763,
        "depth": 0
      },
      "117": {
        "width": 714098,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 594190,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 871869,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 691040,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 618949,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 582056,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 412634,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 460208,
        "height": 466034,
        "depth": 203890
      },
      "125": {
        "width": 776234,
        "height": 466034,
        "depth": 203890
      },
      "126": {
        "width": 602928,
        "height": 759635,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmmib6": {
    "characters": {
      "0": {
        "width": 810219,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1218477,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 1093808,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 1019445,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 1058717,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1215445,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 1112531,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 864581,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 982547,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 930264,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 1108979,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 978672,
        "height": 466035,
        "depth": 0
      },
      "12": {
        "width": 832067,
        "height": 728179,
        "depth": 203891
      },
      "13": {
        "width": 756557,
        "height": 466035,
        "depth": 203891
      },
      "14": {
        "width": 669437,
        "height": 728179,
        "depth": 0
      },
      "15": {
        "width": 627203,
        "height": 466035,
        "depth": 0
      },
      "16": {
        "width": 670891,
        "height": 728179,
        "depth": 203891
      },
      "17": {
        "width": 786917,
        "height": 466035,
        "depth": 203891
      },
      "18": {
        "width": 709488,
        "height": 728179,
        "depth": 0
      },
      "19": {
        "width": 547592,
        "height": 466035,
        "depth": 0
      },
      "20": {
        "width": 862163,
        "height": 466035,
        "depth": 0
      },
      "21": {
        "width": 864587,
        "height": 728179,
        "depth": 0
      },
      "22": {
        "width": 900029,
        "height": 466035,
        "depth": 203891
      },
      "23": {
        "width": 732544,
        "height": 466035,
        "depth": 0
      },
      "24": {
        "width": 670891,
        "height": 728179,
        "depth": 203891
      },
      "25": {
        "width": 884976,
        "height": 466035,
        "depth": 0
      },
      "26": {
        "width": 761917,
        "height": 466035,
        "depth": 203891
      },
      "27": {
        "width": 868715,
        "height": 466035,
        "depth": 0
      },
      "28": {
        "width": 702445,
        "height": 466035,
        "depth": 0
      },
      "29": {
        "width": 823325,
        "height": 466035,
        "depth": 0
      },
      "30": {
        "width": 899541,
        "height": 728179,
        "depth": 203891
      },
      "31": {
        "width": 910221,
        "height": 466035,
        "depth": 203891
      },
      "32": {
        "width": 980611,
        "height": 728179,
        "depth": 203891
      },
      "33": {
        "width": 922109,
        "height": 466035,
        "depth": 0
      },
      "34": {
        "width": 676109,
        "height": 466035,
        "depth": 0
      },
      "35": {
        "width": 899667,
        "height": 728179,
        "depth": 0
      },
      "36": {
        "width": 1238384,
        "height": 466035,
        "depth": 0
      },
      "37": {
        "width": 761917,
        "height": 466035,
        "depth": 203891
      },
      "38": {
        "width": 570400,
        "height": 466035,
        "depth": 101947
      },
      "39": {
        "width": 950512,
        "height": 466035,
        "depth": 203891
      },
      "40": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "41": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "42": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "43": {
        "width": 1454405,
        "height": 438365,
        "depth": 4294881373
      },
      "44": {
        "width": 432051,
        "height": 573515,
        "depth": 49227
      },
      "45": {
        "width": 432051,
        "height": 573515,
        "depth": 49227
      },
      "46": {
        "width": 746621,
        "height": 495163,
        "depth": 4294938171
      },
      "47": {
        "width": 746621,
        "height": 495163,
        "depth": 4294938171
      },
      "48": {
        "width": 746621,
        "height": 466035,
        "depth": 0
      },
      "49": {
        "width": 746621,
        "height": 466035,
        "depth": 0
      },
      "50": {
        "width": 746621,
        "height": 466035,
        "depth": 0
      },
      "51": {
        "width": 746621,
        "height": 466035,
        "depth": 203891
      },
      "52": {
        "width": 746621,
        "height": 466035,
        "depth": 203891
      },
      "53": {
        "width": 746621,
        "height": 466035,
        "depth": 203891
      },
      "54": {
        "width": 746621,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 746621,
        "height": 466035,
        "depth": 203891
      },
      "56": {
        "width": 746621,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 746621,
        "height": 466035,
        "depth": 203891
      },
      "58": {
        "width": 432051,
        "height": 174763,
        "depth": 0
      },
      "59": {
        "width": 432051,
        "height": 174763,
        "depth": 203891
      },
      "60": {
        "width": 1139835,
        "height": 670216,
        "depth": 145928
      },
      "61": {
        "width": 746621,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 1139835,
        "height": 670216,
        "depth": 145928
      },
      "63": {
        "width": 746621,
        "height": 495163,
        "depth": 4294938171
      },
      "64": {
        "width": 788131,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1098088,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 1088107,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 1038179,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1166219,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 1014472,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 849541,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 1108448,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1215445,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 627691,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 790677,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1207680,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 963133,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1412051,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1176123,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 1055944,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 899963,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1095267,
        "height": 719440,
        "depth": 203891
      },
      "82": {
        "width": 1085952,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 875875,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 814160,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 1006141,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 862160,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1373336,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1178189,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 858229,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 974176,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 589336,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 589336,
        "height": 728179,
        "depth": 203891
      },
      "93": {
        "width": 589336,
        "height": 728179,
        "depth": 203891
      },
      "94": {
        "width": 1454405,
        "height": 378653,
        "depth": 4294821661
      },
      "95": {
        "width": 1454405,
        "height": 378653,
        "depth": 4294821661
      },
      "96": {
        "width": 590797,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 821387,
        "height": 466035,
        "depth": 0
      },
      "98": {
        "width": 663613,
        "height": 728179,
        "depth": 0
      },
      "99": {
        "width": 666040,
        "height": 466035,
        "depth": 0
      },
      "100": {
        "width": 782064,
        "height": 728179,
        "depth": 0
      },
      "101": {
        "width": 708080,
        "height": 466035,
        "depth": 0
      },
      "102": {
        "width": 697597,
        "height": 728179,
        "depth": 203891
      },
      "103": {
        "width": 703416,
        "height": 466035,
        "depth": 203891
      },
      "104": {
        "width": 862163,
        "height": 728179,
        "depth": 0
      },
      "105": {
        "width": 546624,
        "height": 728179,
        "depth": 0
      },
      "106": {
        "width": 586427,
        "height": 728179,
        "depth": 203891
      },
      "107": {
        "width": 783520,
        "height": 728179,
        "depth": 0
      },
      "108": {
        "width": 468947,
        "height": 728179,
        "depth": 0
      },
      "109": {
        "width": 1333051,
        "height": 466035,
        "depth": 0
      },
      "110": {
        "width": 939837,
        "height": 466035,
        "depth": 0
      },
      "111": {
        "width": 742256,
        "height": 466035,
        "depth": 0
      },
      "112": {
        "width": 782064,
        "height": 466035,
        "depth": 203891
      },
      "113": {
        "width": 691768,
        "height": 466035,
        "depth": 203891
      },
      "114": {
        "width": 688373,
        "height": 466035,
        "depth": 0
      },
      "115": {
        "width": 669680,
        "height": 466035,
        "depth": 0
      },
      "116": {
        "width": 568709,
        "height": 665765,
        "depth": 0
      },
      "117": {
        "width": 900517,
        "height": 466035,
        "depth": 0
      },
      "118": {
        "width": 744683,
        "height": 466035,
        "depth": 0
      },
      "119": {
        "width": 1068963,
        "height": 466035,
        "depth": 0
      },
      "120": {
        "width": 826968,
        "height": 466035,
        "depth": 0
      },
      "121": {
        "width": 770899,
        "height": 466035,
        "depth": 203891
      },
      "122": {
        "width": 717987,
        "height": 466035,
        "depth": 0
      },
      "123": {
        "width": 546624,
        "height": 466035,
        "depth": 0
      },
      "124": {
        "width": 547107,
        "height": 466035,
        "depth": 203891
      },
      "125": {
        "width": 933035,
        "height": 466035,
        "depth": 203891
      },
      "126": {
        "width": 746621,
        "height": 769248,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 6291456
  },
  "cmmib7": {
    "characters": {
      "0": {
        "width": 758553,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1133879,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 1019616,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 948713,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 987328,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1139653,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 1038331,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 798709,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 910432,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 857106,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 1033742,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 906203,
        "height": 466034,
        "depth": 0
      },
      "12": {
        "width": 774162,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 699003,
        "height": 466034,
        "depth": 203890
      },
      "14": {
        "width": 618121,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 574224,
        "height": 466034,
        "depth": 0
      },
      "16": {
        "width": 613335,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 723810,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 659211,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 498007,
        "height": 466034,
        "depth": 0
      },
      "20": {
        "width": 795936,
        "height": 466034,
        "depth": 0
      },
      "21": {
        "width": 798709,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 836923,
        "height": 466034,
        "depth": 203890
      },
      "23": {
        "width": 680190,
        "height": 466034,
        "depth": 0
      },
      "24": {
        "width": 613335,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 817712,
        "height": 466034,
        "depth": 0
      },
      "26": {
        "width": 712683,
        "height": 466034,
        "depth": 203890
      },
      "27": {
        "width": 809079,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 638302,
        "height": 466034,
        "depth": 0
      },
      "29": {
        "width": 757099,
        "height": 466034,
        "depth": 0
      },
      "30": {
        "width": 837824,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 845733,
        "height": 466034,
        "depth": 203890
      },
      "32": {
        "width": 906064,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 853287,
        "height": 466034,
        "depth": 0
      },
      "34": {
        "width": 624363,
        "height": 466034,
        "depth": 0
      },
      "35": {
        "width": 829017,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 1154478,
        "height": 466034,
        "depth": 0
      },
      "37": {
        "width": 712683,
        "height": 466034,
        "depth": 203890
      },
      "38": {
        "width": 517008,
        "height": 466034,
        "depth": 101945
      },
      "39": {
        "width": 882553,
        "height": 466034,
        "depth": 203890
      },
      "40": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "41": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "42": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "43": {
        "width": 1357326,
        "height": 430832,
        "depth": 4294873840
      },
      "44": {
        "width": 389056,
        "height": 561323,
        "depth": 37035
      },
      "45": {
        "width": 389056,
        "height": 561323,
        "depth": 37035
      },
      "46": {
        "width": 686985,
        "height": 495161,
        "depth": 4294938169
      },
      "47": {
        "width": 686985,
        "height": 495161,
        "depth": 4294938169
      },
      "48": {
        "width": 686985,
        "height": 466034,
        "depth": 0
      },
      "49": {
        "width": 686985,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 686985,
        "height": 466034,
        "depth": 0
      },
      "51": {
        "width": 686985,
        "height": 466034,
        "depth": 203890
      },
      "52": {
        "width": 686985,
        "height": 466034,
        "depth": 203890
      },
      "53": {
        "width": 686985,
        "height": 466034,
        "depth": 203890
      },
      "54": {
        "width": 686985,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 686985,
        "height": 466034,
        "depth": 203890
      },
      "56": {
        "width": 686985,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 686985,
        "height": 466034,
        "depth": 203890
      },
      "58": {
        "width": 389056,
        "height": 170601,
        "depth": 0
      },
      "59": {
        "width": 389056,
        "height": 170601,
        "depth": 203890
      },
      "60": {
        "width": 1059397,
        "height": 655694,
        "depth": 131406
      },
      "61": {
        "width": 686985,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 1059397,
        "height": 655694,
        "depth": 131406
      },
      "63": {
        "width": 686985,
        "height": 495161,
        "depth": 4294938169
      },
      "64": {
        "width": 734734,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 1023195,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 1015641,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 965024,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1092715,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 947205,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 795794,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 1036334,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1139653,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 587431,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 735513,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1131746,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 893787,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1325858,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1102411,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 983831,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 841022,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 1021072,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 1016206,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 814158,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 753481,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 936937,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 799751,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1283886,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1103815,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 796025,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 907259,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 538021,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 538021,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 538021,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1357326,
        "height": 378654,
        "depth": 4294821662
      },
      "95": {
        "width": 1357326,
        "height": 378654,
        "depth": 4294821662
      },
      "96": {
        "width": 551961,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 757239,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 614377,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 612642,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 719998,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 655031,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 662231,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 648981,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 795936,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 490448,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 546553,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 721454,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 423525,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1235271,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 862859,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 688859,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 719998,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 641493,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 635321,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 624261,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 511842,
        "height": 665765,
        "depth": 0
      },
      "117": {
        "width": 825618,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 682617,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 990256,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 773225,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 709872,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 664240,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 490448,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 509312,
        "height": 466034,
        "depth": 203890
      },
      "125": {
        "width": 870279,
        "height": 466034,
        "depth": 203890
      },
      "126": {
        "width": 686985,
        "height": 766377,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmmib8": {
    "characters": {
      "0": {
        "width": 719808,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1070430,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 963966,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 895664,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 933786,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1082442,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 982684,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 749298,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 856342,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 802240,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 977314,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 851854,
        "height": 466034,
        "depth": 0
      },
      "12": {
        "width": 730732,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 655834,
        "height": 466034,
        "depth": 203890
      },
      "14": {
        "width": 579632,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 537034,
        "height": 466034,
        "depth": 0
      },
      "16": {
        "width": 570168,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 674666,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 621502,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 460820,
        "height": 466034,
        "depth": 0
      },
      "20": {
        "width": 746268,
        "height": 466034,
        "depth": 0
      },
      "21": {
        "width": 749302,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 787774,
        "height": 466034,
        "depth": 203890
      },
      "23": {
        "width": 639102,
        "height": 466034,
        "depth": 0
      },
      "24": {
        "width": 570168,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 765444,
        "height": 466034,
        "depth": 0
      },
      "26": {
        "width": 673932,
        "height": 466034,
        "depth": 203890
      },
      "27": {
        "width": 762528,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 590194,
        "height": 466034,
        "depth": 0
      },
      "29": {
        "width": 707432,
        "height": 466034,
        "depth": 0
      },
      "30": {
        "width": 791534,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 797358,
        "height": 466034,
        "depth": 203890
      },
      "32": {
        "width": 850156,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 801668,
        "height": 466034,
        "depth": 0
      },
      "34": {
        "width": 585548,
        "height": 466034,
        "depth": 0
      },
      "35": {
        "width": 776036,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 1089728,
        "height": 466034,
        "depth": 0
      },
      "37": {
        "width": 673932,
        "height": 466034,
        "depth": 203890
      },
      "38": {
        "width": 476960,
        "height": 466034,
        "depth": 101946
      },
      "39": {
        "width": 831584,
        "height": 466034,
        "depth": 203890
      },
      "40": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "41": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "42": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "43": {
        "width": 1284516,
        "height": 423364,
        "depth": 4294866372
      },
      "44": {
        "width": 356810,
        "height": 549194,
        "depth": 24906
      },
      "45": {
        "width": 356810,
        "height": 549194,
        "depth": 24906
      },
      "46": {
        "width": 642258,
        "height": 495162,
        "depth": 4294938170
      },
      "47": {
        "width": 642258,
        "height": 495162,
        "depth": 4294938170
      },
      "48": {
        "width": 642258,
        "height": 466034,
        "depth": 0
      },
      "49": {
        "width": 642258,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 642258,
        "height": 466034,
        "depth": 0
      },
      "51": {
        "width": 642258,
        "height": 466034,
        "depth": 203890
      },
      "52": {
        "width": 642258,
        "height": 466034,
        "depth": 203890
      },
      "53": {
        "width": 642258,
        "height": 466034,
        "depth": 203890
      },
      "54": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 642258,
        "height": 466034,
        "depth": 203890
      },
      "56": {
        "width": 642258,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 642258,
        "height": 466034,
        "depth": 203890
      },
      "58": {
        "width": 356810,
        "height": 167480,
        "depth": 0
      },
      "59": {
        "width": 356810,
        "height": 167480,
        "depth": 203890
      },
      "60": {
        "width": 999068,
        "height": 641234,
        "depth": 116946
      },
      "61": {
        "width": 642258,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 999068,
        "height": 641234,
        "depth": 116946
      },
      "63": {
        "width": 642258,
        "height": 495162,
        "depth": 4294938170
      },
      "64": {
        "width": 694684,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 967026,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 961294,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 910156,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1037586,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 896758,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 755490,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 982244,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1082442,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 558696,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 693774,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1074798,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 841780,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1260846,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1046760,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 929742,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 796812,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 965424,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 965434,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 767870,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 707976,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 884668,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 752940,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1216792,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1048036,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 749372,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 857072,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 499534,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 499534,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 499534,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1284516,
        "height": 378654,
        "depth": 4294821662
      },
      "95": {
        "width": 1284516,
        "height": 378654,
        "depth": 4294821662
      },
      "96": {
        "width": 519194,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 709130,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 577448,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 572594,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 673448,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 614876,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 632056,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 606332,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 746268,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 450988,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 514824,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 674906,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 389456,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1161942,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 805132,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 648810,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 673450,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 601964,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 595532,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 590188,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 469196,
        "height": 665764,
        "depth": 0
      },
      "117": {
        "width": 769452,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 636070,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 931228,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 732914,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 662286,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 620292,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 448322,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 479144,
        "height": 466034,
        "depth": 203890
      },
      "125": {
        "width": 821390,
        "height": 466034,
        "depth": 203890
      },
      "126": {
        "width": 642258,
        "height": 763568,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmmib9": {
    "characters": {
      "0": {
        "width": 702601,
        "height": 719440,
        "depth": 0
      },
      "1": {
        "width": 1034000,
        "height": 719440,
        "depth": 0
      },
      "2": {
        "width": 933607,
        "height": 719440,
        "depth": 0
      },
      "3": {
        "width": 867332,
        "height": 719440,
        "depth": 0
      },
      "4": {
        "width": 905067,
        "height": 719440,
        "depth": 0
      },
      "5": {
        "width": 1054110,
        "height": 719440,
        "depth": 0
      },
      "6": {
        "width": 952324,
        "height": 719440,
        "depth": 0
      },
      "7": {
        "width": 723797,
        "height": 719440,
        "depth": 0
      },
      "8": {
        "width": 827198,
        "height": 719440,
        "depth": 0
      },
      "9": {
        "width": 772487,
        "height": 719440,
        "depth": 0
      },
      "10": {
        "width": 946348,
        "height": 719440,
        "depth": 0
      },
      "11": {
        "width": 822510,
        "height": 466034,
        "depth": 0
      },
      "12": {
        "width": 709888,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 635191,
        "height": 466034,
        "depth": 203890
      },
      "14": {
        "width": 562633,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 521049,
        "height": 466034,
        "depth": 0
      },
      "16": {
        "width": 549524,
        "height": 728178,
        "depth": 203890
      },
      "17": {
        "width": 649367,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 605113,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 444832,
        "height": 466034,
        "depth": 0
      },
      "20": {
        "width": 720565,
        "height": 466034,
        "depth": 0
      },
      "21": {
        "width": 723801,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 762476,
        "height": 466034,
        "depth": 203890
      },
      "23": {
        "width": 620078,
        "height": 466034,
        "depth": 0
      },
      "24": {
        "width": 549524,
        "height": 728178,
        "depth": 203890
      },
      "25": {
        "width": 737719,
        "height": 466034,
        "depth": 0
      },
      "26": {
        "width": 656731,
        "height": 466034,
        "depth": 203890
      },
      "27": {
        "width": 739257,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 565707,
        "height": 466034,
        "depth": 0
      },
      "29": {
        "width": 681730,
        "height": 466034,
        "depth": 0
      },
      "30": {
        "width": 768464,
        "height": 728178,
        "depth": 203890
      },
      "31": {
        "width": 772670,
        "height": 466034,
        "depth": 203890
      },
      "32": {
        "width": 819596,
        "height": 728178,
        "depth": 203890
      },
      "33": {
        "width": 774448,
        "height": 466034,
        "depth": 0
      },
      "34": {
        "width": 568295,
        "height": 466034,
        "depth": 0
      },
      "35": {
        "width": 747751,
        "height": 728178,
        "depth": 0
      },
      "36": {
        "width": 1052288,
        "height": 466034,
        "depth": 0
      },
      "37": {
        "width": 656731,
        "height": 466034,
        "depth": 203890
      },
      "38": {
        "width": 458745,
        "height": 466034,
        "depth": 101945
      },
      "39": {
        "width": 804869,
        "height": 466034,
        "depth": 203890
      },
      "40": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "41": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "42": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "43": {
        "width": 1240800,
        "height": 416519,
        "depth": 4294859527
      },
      "44": {
        "width": 344667,
        "height": 537687,
        "depth": 13399
      },
      "45": {
        "width": 344667,
        "height": 537687,
        "depth": 13399
      },
      "46": {
        "width": 620400,
        "height": 495161,
        "depth": 4294938169
      },
      "47": {
        "width": 620400,
        "height": 495161,
        "depth": 4294938169
      },
      "48": {
        "width": 620400,
        "height": 466034,
        "depth": 0
      },
      "49": {
        "width": 620400,
        "height": 466034,
        "depth": 0
      },
      "50": {
        "width": 620400,
        "height": 466034,
        "depth": 0
      },
      "51": {
        "width": 620400,
        "height": 466034,
        "depth": 203890
      },
      "52": {
        "width": 620400,
        "height": 466034,
        "depth": 203890
      },
      "53": {
        "width": 620400,
        "height": 466034,
        "depth": 203890
      },
      "54": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 620400,
        "height": 466034,
        "depth": 203890
      },
      "56": {
        "width": 620400,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 620400,
        "height": 466034,
        "depth": 203890
      },
      "58": {
        "width": 344667,
        "height": 165054,
        "depth": 0
      },
      "59": {
        "width": 344667,
        "height": 165054,
        "depth": 203890
      },
      "60": {
        "width": 965067,
        "height": 627399,
        "depth": 103111
      },
      "61": {
        "width": 620400,
        "height": 786432,
        "depth": 262144
      },
      "62": {
        "width": 965067,
        "height": 627399,
        "depth": 103111
      },
      "63": {
        "width": 620400,
        "height": 495161,
        "depth": 4294938169
      },
      "64": {
        "width": 676473,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 936265,
        "height": 719440,
        "depth": 0
      },
      "66": {
        "width": 931948,
        "height": 719440,
        "depth": 0
      },
      "67": {
        "width": 880404,
        "height": 719440,
        "depth": 0
      },
      "68": {
        "width": 1007634,
        "height": 719440,
        "depth": 0
      },
      "69": {
        "width": 870444,
        "height": 719440,
        "depth": 0
      },
      "70": {
        "width": 737068,
        "height": 719440,
        "depth": 0
      },
      "71": {
        "width": 953099,
        "height": 719440,
        "depth": 0
      },
      "72": {
        "width": 1054110,
        "height": 719440,
        "depth": 0
      },
      "73": {
        "width": 546050,
        "height": 719440,
        "depth": 0
      },
      "74": {
        "width": 676672,
        "height": 719440,
        "depth": 0
      },
      "75": {
        "width": 1043429,
        "height": 719440,
        "depth": 0
      },
      "76": {
        "width": 814254,
        "height": 719440,
        "depth": 0
      },
      "77": {
        "width": 1226443,
        "height": 719440,
        "depth": 0
      },
      "78": {
        "width": 1019643,
        "height": 719440,
        "depth": 0
      },
      "79": {
        "width": 900596,
        "height": 719440,
        "depth": 0
      },
      "80": {
        "width": 775360,
        "height": 719440,
        "depth": 0
      },
      "81": {
        "width": 935063,
        "height": 719440,
        "depth": 203890
      },
      "82": {
        "width": 938871,
        "height": 719440,
        "depth": 0
      },
      "83": {
        "width": 744796,
        "height": 719440,
        "depth": 0
      },
      "84": {
        "width": 685506,
        "height": 719440,
        "depth": 0
      },
      "85": {
        "width": 860180,
        "height": 719440,
        "depth": 0
      },
      "86": {
        "width": 729465,
        "height": 719440,
        "depth": 0
      },
      "87": {
        "width": 1177531,
        "height": 719440,
        "depth": 0
      },
      "88": {
        "width": 1017579,
        "height": 719440,
        "depth": 0
      },
      "89": {
        "width": 726020,
        "height": 719440,
        "depth": 0
      },
      "90": {
        "width": 830964,
        "height": 719440,
        "depth": 0
      },
      "91": {
        "width": 482533,
        "height": 786432,
        "depth": 0
      },
      "92": {
        "width": 482533,
        "height": 728178,
        "depth": 203890
      },
      "93": {
        "width": 482533,
        "height": 728178,
        "depth": 203890
      },
      "94": {
        "width": 1240800,
        "height": 378652,
        "depth": 4294821660
      },
      "95": {
        "width": 1240800,
        "height": 378652,
        "depth": 4294821660
      },
      "96": {
        "width": 506647,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 684644,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 561664,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 554380,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 654060,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 596583,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 615067,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 586094,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 720565,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 437228,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 503086,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 651632,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 375897,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1117822,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 773156,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 630597,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 650176,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 584156,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 574286,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 573396,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 448958,
        "height": 665764,
        "depth": 0
      },
      "117": {
        "width": 738690,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 612796,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 898238,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 711264,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 638203,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 599044,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 428489,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 468620,
        "height": 466034,
        "depth": 203890
      },
      "125": {
        "width": 796293,
        "height": 466034,
        "depth": 203890
      },
      "126": {
        "width": 620400,
        "height": 761383,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmr10": {
    "characters": {
      "0": {
        "width": 655362,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 873816,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 728179,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 699053,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 611672,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 320400,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 524290,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 524290,
        "height": 595357,
        "depth": 0
      },
      "23": {
        "width": 786434,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 466035,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 524291,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 757307,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 815562,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 524290,
        "height": 553416,
        "depth": 101946
      },
      "29": {
        "width": 946634,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1063142,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 815562,
        "height": 767499,
        "depth": 50973
      },
      "32": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 873816,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 524290,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 873816,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 524290,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "44": {
        "width": 291272,
        "height": 110683,
        "depth": 203890
      },
      "45": {
        "width": 349526,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 291272,
        "height": 110683,
        "depth": 0
      },
      "47": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 291272,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 291272,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 815562,
        "height": 384696,
        "depth": 4294827704
      },
      "62": {
        "width": 495163,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 495163,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 742744,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 800998,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 713616,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 684490,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 822843,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 378653,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 538853,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 655362,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 961197,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 713616,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 815562,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 771870,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 582544,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1077706,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 640798,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 291272,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 291272,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 291272,
        "height": 700301,
        "depth": 0
      },
      "96": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 291272,
        "height": 700301,
        "depth": 0
      },
      "106": {
        "width": 320400,
        "height": 700301,
        "depth": 203890
      },
      "107": {
        "width": 553418,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 873816,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 582544,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 582544,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 553416,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 410694,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 413606,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 407781,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 582544,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 553418,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 757307,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 553418,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 553418,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1048579,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 524290,
        "height": 700301,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmr12": {
    "characters": {
      "0": {
        "width": 641519,
        "height": 716527,
        "depth": 0
      },
      "1": {
        "width": 855600,
        "height": 716527,
        "depth": 0
      },
      "2": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "3": {
        "width": 712637,
        "height": 716527,
        "depth": 0
      },
      "4": {
        "width": 684480,
        "height": 716527,
        "depth": 0
      },
      "5": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "6": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "7": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "8": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "9": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "10": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "11": {
        "width": 598920,
        "height": 728177,
        "depth": 0
      },
      "12": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "13": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "14": {
        "width": 855600,
        "height": 728177,
        "depth": 0
      },
      "15": {
        "width": 855600,
        "height": 728177,
        "depth": 0
      },
      "16": {
        "width": 285200,
        "height": 451471,
        "depth": 0
      },
      "17": {
        "width": 313720,
        "height": 451471,
        "depth": 203889
      },
      "18": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "19": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "20": {
        "width": 513360,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "22": {
        "width": 513360,
        "height": 592249,
        "depth": 0
      },
      "23": {
        "width": 769677,
        "height": 728177,
        "depth": 0
      },
      "24": {
        "width": 456320,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "26": {
        "width": 741520,
        "height": 451471,
        "depth": 0
      },
      "27": {
        "width": 798560,
        "height": 451471,
        "depth": 0
      },
      "28": {
        "width": 513360,
        "height": 553416,
        "depth": 101945
      },
      "29": {
        "width": 926719,
        "height": 716527,
        "depth": 0
      },
      "30": {
        "width": 1040799,
        "height": 716527,
        "depth": 0
      },
      "31": {
        "width": 798560,
        "height": 767499,
        "depth": 50972
      },
      "32": {
        "width": 285200,
        "height": 451471,
        "depth": 0
      },
      "33": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "34": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "35": {
        "width": 855600,
        "height": 728177,
        "depth": 203888
      },
      "36": {
        "width": 513360,
        "height": 786432,
        "depth": 58255
      },
      "37": {
        "width": 855600,
        "height": 786432,
        "depth": 58255
      },
      "38": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "39": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "40": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 513360,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 798560,
        "height": 604384,
        "depth": 80096
      },
      "44": {
        "width": 285200,
        "height": 101945,
        "depth": 203889
      },
      "45": {
        "width": 342240,
        "height": 451471,
        "depth": 0
      },
      "46": {
        "width": 285200,
        "height": 101945,
        "depth": 0
      },
      "47": {
        "width": 513360,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 285200,
        "height": 451471,
        "depth": 0
      },
      "59": {
        "width": 285200,
        "height": 451471,
        "depth": 203889
      },
      "60": {
        "width": 285200,
        "height": 524288,
        "depth": 203889
      },
      "61": {
        "width": 798560,
        "height": 378433,
        "depth": 4294821441
      },
      "62": {
        "width": 484840,
        "height": 524288,
        "depth": 203889
      },
      "63": {
        "width": 484840,
        "height": 728177,
        "depth": 0
      },
      "64": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "65": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "66": {
        "width": 727079,
        "height": 716527,
        "depth": 0
      },
      "67": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "68": {
        "width": 784119,
        "height": 716527,
        "depth": 0
      },
      "69": {
        "width": 698559,
        "height": 716527,
        "depth": 0
      },
      "70": {
        "width": 670039,
        "height": 716527,
        "depth": 0
      },
      "71": {
        "width": 805600,
        "height": 716527,
        "depth": 0
      },
      "72": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "73": {
        "width": 370397,
        "height": 716527,
        "depth": 0
      },
      "74": {
        "width": 527439,
        "height": 716527,
        "depth": 0
      },
      "75": {
        "width": 798197,
        "height": 716527,
        "depth": 0
      },
      "76": {
        "width": 641519,
        "height": 716527,
        "depth": 0
      },
      "77": {
        "width": 940797,
        "height": 716527,
        "depth": 0
      },
      "78": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "79": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "80": {
        "width": 698559,
        "height": 716527,
        "depth": 0
      },
      "81": {
        "width": 798560,
        "height": 716527,
        "depth": 203889
      },
      "82": {
        "width": 755599,
        "height": 716527,
        "depth": 0
      },
      "83": {
        "width": 570400,
        "height": 716527,
        "depth": 0
      },
      "84": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "85": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "86": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "87": {
        "width": 1054877,
        "height": 716527,
        "depth": 0
      },
      "88": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "89": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "90": {
        "width": 627440,
        "height": 716527,
        "depth": 0
      },
      "91": {
        "width": 285200,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "93": {
        "width": 285200,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "95": {
        "width": 285200,
        "height": 695932,
        "depth": 0
      },
      "96": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "97": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "98": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "99": {
        "width": 456320,
        "height": 451471,
        "depth": 0
      },
      "100": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "101": {
        "width": 456320,
        "height": 451471,
        "depth": 0
      },
      "102": {
        "width": 313720,
        "height": 728177,
        "depth": 0
      },
      "103": {
        "width": 513360,
        "height": 451471,
        "depth": 203889
      },
      "104": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "105": {
        "width": 285200,
        "height": 695932,
        "depth": 0
      },
      "106": {
        "width": 313720,
        "height": 695932,
        "depth": 203889
      },
      "107": {
        "width": 541880,
        "height": 728177,
        "depth": 0
      },
      "108": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "109": {
        "width": 855600,
        "height": 451471,
        "depth": 0
      },
      "110": {
        "width": 570400,
        "height": 451471,
        "depth": 0
      },
      "111": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "112": {
        "width": 570400,
        "height": 451471,
        "depth": 203889
      },
      "113": {
        "width": 541880,
        "height": 451471,
        "depth": 203889
      },
      "114": {
        "width": 399280,
        "height": 451471,
        "depth": 0
      },
      "115": {
        "width": 404984,
        "height": 451471,
        "depth": 0
      },
      "116": {
        "width": 399280,
        "height": 644959,
        "depth": 0
      },
      "117": {
        "width": 570400,
        "height": 451471,
        "depth": 0
      },
      "118": {
        "width": 541880,
        "height": 451471,
        "depth": 0
      },
      "119": {
        "width": 741520,
        "height": 451471,
        "depth": 0
      },
      "120": {
        "width": 541880,
        "height": 451471,
        "depth": 0
      },
      "121": {
        "width": 541880,
        "height": 451471,
        "depth": 203889
      },
      "122": {
        "width": 456320,
        "height": 451471,
        "depth": 0
      },
      "123": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "124": {
        "width": 1026720,
        "height": 451471,
        "depth": 0
      },
      "125": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "126": {
        "width": 513360,
        "height": 695932,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 12582912
  },
  "cmr17": {
    "characters": {
      "0": {
        "width": 603944,
        "height": 716379,
        "depth": 0
      },
      "1": {
        "width": 809585,
        "height": 716379,
        "depth": 0
      },
      "2": {
        "width": 754804,
        "height": 716379,
        "depth": 0
      },
      "3": {
        "width": 672211,
        "height": 716379,
        "depth": 0
      },
      "4": {
        "width": 645241,
        "height": 716379,
        "depth": 0
      },
      "5": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "6": {
        "width": 700022,
        "height": 716379,
        "depth": 0
      },
      "7": {
        "width": 754804,
        "height": 716379,
        "depth": 0
      },
      "8": {
        "width": 700022,
        "height": 716379,
        "depth": 0
      },
      "9": {
        "width": 754804,
        "height": 716379,
        "depth": 0
      },
      "10": {
        "width": 700022,
        "height": 716379,
        "depth": 0
      },
      "11": {
        "width": 550932,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 523541,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 523541,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 785311,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 785311,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 261770,
        "height": 451403,
        "depth": 0
      },
      "17": {
        "width": 289161,
        "height": 451403,
        "depth": 203957
      },
      "18": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 480896,
        "height": 658984,
        "depth": 0
      },
      "21": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 480896,
        "height": 586720,
        "depth": 0
      },
      "23": {
        "width": 726993,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 426115,
        "height": 0,
        "depth": 178463
      },
      "25": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 700022,
        "height": 451403,
        "depth": 0
      },
      "27": {
        "width": 754804,
        "height": 451403,
        "depth": 0
      },
      "28": {
        "width": 480896,
        "height": 553382,
        "depth": 101979
      },
      "29": {
        "width": 877852,
        "height": 716379,
        "depth": 0
      },
      "30": {
        "width": 987415,
        "height": 716379,
        "depth": 0
      },
      "31": {
        "width": 754804,
        "height": 767369,
        "depth": 50990
      },
      "32": {
        "width": 261770,
        "height": 451403,
        "depth": 0
      },
      "33": {
        "width": 261770,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 809585,
        "height": 728178,
        "depth": 203957
      },
      "36": {
        "width": 480896,
        "height": 785488,
        "depth": 57310
      },
      "37": {
        "width": 809585,
        "height": 785488,
        "depth": 57310
      },
      "38": {
        "width": 754804,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 261770,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 371333,
        "height": 785488,
        "depth": 261267
      },
      "41": {
        "width": 371333,
        "height": 785488,
        "depth": 261267
      },
      "42": {
        "width": 480896,
        "height": 785488,
        "depth": 0
      },
      "43": {
        "width": 754804,
        "height": 590799,
        "depth": 66579
      },
      "44": {
        "width": 261770,
        "height": 85966,
        "depth": 203957
      },
      "45": {
        "width": 316552,
        "height": 451403,
        "depth": 0
      },
      "46": {
        "width": 261770,
        "height": 85966,
        "depth": 0
      },
      "47": {
        "width": 480896,
        "height": 785488,
        "depth": 261267
      },
      "48": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "49": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "50": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "51": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "52": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "53": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "54": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "55": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "56": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "57": {
        "width": 480896,
        "height": 675924,
        "depth": 0
      },
      "58": {
        "width": 261770,
        "height": 451403,
        "depth": 0
      },
      "59": {
        "width": 261770,
        "height": 451403,
        "depth": 203957
      },
      "60": {
        "width": 261770,
        "height": 524220,
        "depth": 203957
      },
      "61": {
        "width": 754804,
        "height": 372851,
        "depth": 4294815927
      },
      "62": {
        "width": 453506,
        "height": 524220,
        "depth": 203957
      },
      "63": {
        "width": 453506,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 754804,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "66": {
        "width": 686117,
        "height": 716379,
        "depth": 0
      },
      "67": {
        "width": 700022,
        "height": 716379,
        "depth": 0
      },
      "68": {
        "width": 740898,
        "height": 716379,
        "depth": 0
      },
      "69": {
        "width": 658726,
        "height": 716379,
        "depth": 0
      },
      "70": {
        "width": 631335,
        "height": 716379,
        "depth": 0
      },
      "71": {
        "width": 761546,
        "height": 716379,
        "depth": 0
      },
      "72": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "73": {
        "width": 343522,
        "height": 716379,
        "depth": 0
      },
      "74": {
        "width": 494382,
        "height": 716379,
        "depth": 0
      },
      "75": {
        "width": 754383,
        "height": 716379,
        "depth": 0
      },
      "76": {
        "width": 603944,
        "height": 716379,
        "depth": 0
      },
      "77": {
        "width": 891337,
        "height": 716379,
        "depth": 0
      },
      "78": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "79": {
        "width": 754804,
        "height": 716379,
        "depth": 0
      },
      "80": {
        "width": 658726,
        "height": 716379,
        "depth": 0
      },
      "81": {
        "width": 754804,
        "height": 716379,
        "depth": 203957
      },
      "82": {
        "width": 713507,
        "height": 716379,
        "depth": 0
      },
      "83": {
        "width": 535678,
        "height": 716379,
        "depth": 0
      },
      "84": {
        "width": 700022,
        "height": 716379,
        "depth": 0
      },
      "85": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "86": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "87": {
        "width": 1000900,
        "height": 716379,
        "depth": 0
      },
      "88": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "89": {
        "width": 726993,
        "height": 716379,
        "depth": 0
      },
      "90": {
        "width": 590459,
        "height": 716379,
        "depth": 0
      },
      "91": {
        "width": 261770,
        "height": 785488,
        "depth": 261267
      },
      "92": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 261770,
        "height": 785488,
        "depth": 261267
      },
      "94": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 261770,
        "height": 687845,
        "depth": 0
      },
      "96": {
        "width": 261770,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 480896,
        "height": 451403,
        "depth": 0
      },
      "98": {
        "width": 535678,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 426115,
        "height": 451403,
        "depth": 0
      },
      "100": {
        "width": 535678,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 426115,
        "height": 451403,
        "depth": 0
      },
      "102": {
        "width": 289161,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 480896,
        "height": 451403,
        "depth": 203957
      },
      "104": {
        "width": 535678,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 261770,
        "height": 687845,
        "depth": 0
      },
      "106": {
        "width": 289161,
        "height": 687845,
        "depth": 203957
      },
      "107": {
        "width": 508287,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 261770,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 809585,
        "height": 451403,
        "depth": 0
      },
      "110": {
        "width": 535678,
        "height": 451403,
        "depth": 0
      },
      "111": {
        "width": 480896,
        "height": 451403,
        "depth": 0
      },
      "112": {
        "width": 535678,
        "height": 451403,
        "depth": 203957
      },
      "113": {
        "width": 508287,
        "height": 451403,
        "depth": 203957
      },
      "114": {
        "width": 371333,
        "height": 451403,
        "depth": 0
      },
      "115": {
        "width": 376812,
        "height": 451403,
        "depth": 0
      },
      "116": {
        "width": 371333,
        "height": 644862,
        "depth": 0
      },
      "117": {
        "width": 535678,
        "height": 451403,
        "depth": 0
      },
      "118": {
        "width": 508287,
        "height": 451403,
        "depth": 0
      },
      "119": {
        "width": 700022,
        "height": 451403,
        "depth": 0
      },
      "120": {
        "width": 508287,
        "height": 451403,
        "depth": 0
      },
      "121": {
        "width": 508287,
        "height": 451403,
        "depth": 203957
      },
      "122": {
        "width": 426115,
        "height": 451403,
        "depth": 0
      },
      "123": {
        "width": 480896,
        "height": 451403,
        "depth": 0
      },
      "124": {
        "width": 961793,
        "height": 451403,
        "depth": 0
      },
      "125": {
        "width": 480896,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 480896,
        "height": 687845,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 18119392
  },
  "cmr5": {
    "characters": {
      "0": {
        "width": 870915,
        "height": 712781,
        "depth": 0
      },
      "1": {
        "width": 1150541,
        "height": 712781,
        "depth": 0
      },
      "2": {
        "width": 1077722,
        "height": 712781,
        "depth": 0
      },
      "3": {
        "width": 955386,
        "height": 712781,
        "depth": 0
      },
      "4": {
        "width": 932083,
        "height": 712781,
        "depth": 0
      },
      "5": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "6": {
        "width": 1004902,
        "height": 712781,
        "depth": 0
      },
      "7": {
        "width": 1077722,
        "height": 712781,
        "depth": 0
      },
      "8": {
        "width": 1004902,
        "height": 712781,
        "depth": 0
      },
      "9": {
        "width": 1077722,
        "height": 712781,
        "depth": 0
      },
      "10": {
        "width": 1004902,
        "height": 712781,
        "depth": 0
      },
      "11": {
        "width": 713626,
        "height": 728179,
        "depth": 203891
      },
      "12": {
        "width": 713626,
        "height": 728179,
        "depth": 203891
      },
      "13": {
        "width": 422349,
        "height": 728179,
        "depth": 0
      },
      "14": {
        "width": 422349,
        "height": 524288,
        "depth": 203891
      },
      "15": {
        "width": 677216,
        "height": 524288,
        "depth": 203891
      },
      "16": {
        "width": 422349,
        "height": 451469,
        "depth": 0
      },
      "17": {
        "width": 458758,
        "height": 451469,
        "depth": 203891
      },
      "18": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "19": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "20": {
        "width": 713626,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "22": {
        "width": 713626,
        "height": 608755,
        "depth": 0
      },
      "23": {
        "width": 1028205,
        "height": 728179,
        "depth": 0
      },
      "24": {
        "width": 640806,
        "height": 0,
        "depth": 178406
      },
      "25": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "26": {
        "width": 1004902,
        "height": 451469,
        "depth": 0
      },
      "27": {
        "width": 1077722,
        "height": 451469,
        "depth": 0
      },
      "28": {
        "width": 713626,
        "height": 553414,
        "depth": 101946
      },
      "29": {
        "width": 1235011,
        "height": 712781,
        "depth": 0
      },
      "30": {
        "width": 1380650,
        "height": 712781,
        "depth": 0
      },
      "31": {
        "width": 1077722,
        "height": 767501,
        "depth": 50973
      },
      "32": {
        "width": 422349,
        "height": 451469,
        "depth": 0
      },
      "33": {
        "width": 422349,
        "height": 728179,
        "depth": 0
      },
      "34": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "35": {
        "width": 1150541,
        "height": 728179,
        "depth": 203891
      },
      "36": {
        "width": 713626,
        "height": 786432,
        "depth": 58253
      },
      "37": {
        "width": 1150541,
        "height": 786432,
        "depth": 58253
      },
      "38": {
        "width": 1077722,
        "height": 728179,
        "depth": 0
      },
      "39": {
        "width": 422349,
        "height": 728179,
        "depth": 0
      },
      "40": {
        "width": 567987,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 567987,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 713626,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 1077722,
        "height": 699059,
        "depth": 174771
      },
      "44": {
        "width": 422349,
        "height": 128160,
        "depth": 203891
      },
      "45": {
        "width": 495168,
        "height": 451469,
        "depth": 0
      },
      "46": {
        "width": 422349,
        "height": 128160,
        "depth": 0
      },
      "47": {
        "width": 713626,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "49": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "50": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "51": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "52": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "53": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "54": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "56": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 713626,
        "height": 675750,
        "depth": 0
      },
      "58": {
        "width": 422349,
        "height": 451469,
        "depth": 0
      },
      "59": {
        "width": 422349,
        "height": 451469,
        "depth": 203891
      },
      "60": {
        "width": 1077722,
        "height": 630106,
        "depth": 105818
      },
      "61": {
        "width": 1077722,
        "height": 415661,
        "depth": 4294858669
      },
      "62": {
        "width": 1077722,
        "height": 630106,
        "depth": 105818
      },
      "63": {
        "width": 677216,
        "height": 728179,
        "depth": 0
      },
      "64": {
        "width": 1077722,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "66": {
        "width": 980144,
        "height": 712781,
        "depth": 0
      },
      "67": {
        "width": 1004902,
        "height": 712781,
        "depth": 0
      },
      "68": {
        "width": 1052963,
        "height": 712781,
        "depth": 0
      },
      "69": {
        "width": 943734,
        "height": 712781,
        "depth": 0
      },
      "70": {
        "width": 907325,
        "height": 712781,
        "depth": 0
      },
      "71": {
        "width": 1083549,
        "height": 712781,
        "depth": 0
      },
      "72": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "73": {
        "width": 518470,
        "height": 712781,
        "depth": 0
      },
      "74": {
        "width": 725277,
        "height": 712781,
        "depth": 0
      },
      "75": {
        "width": 1064614,
        "height": 712781,
        "depth": 0
      },
      "76": {
        "width": 870915,
        "height": 712781,
        "depth": 0
      },
      "77": {
        "width": 1246662,
        "height": 712781,
        "depth": 0
      },
      "78": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "79": {
        "width": 1077722,
        "height": 712781,
        "depth": 0
      },
      "80": {
        "width": 943734,
        "height": 712781,
        "depth": 0
      },
      "81": {
        "width": 1077722,
        "height": 712781,
        "depth": 203891
      },
      "82": {
        "width": 1016554,
        "height": 712781,
        "depth": 0
      },
      "83": {
        "width": 786445,
        "height": 712781,
        "depth": 0
      },
      "84": {
        "width": 1004902,
        "height": 712781,
        "depth": 0
      },
      "85": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "86": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "87": {
        "width": 1392301,
        "height": 712781,
        "depth": 0
      },
      "88": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "89": {
        "width": 1028205,
        "height": 712781,
        "depth": 0
      },
      "90": {
        "width": 859264,
        "height": 712781,
        "depth": 0
      },
      "91": {
        "width": 422349,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "93": {
        "width": 422349,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "95": {
        "width": 422349,
        "height": 712781,
        "depth": 0
      },
      "96": {
        "width": 422349,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 713626,
        "height": 451469,
        "depth": 0
      },
      "98": {
        "width": 786445,
        "height": 728179,
        "depth": 0
      },
      "99": {
        "width": 640806,
        "height": 451469,
        "depth": 0
      },
      "100": {
        "width": 786445,
        "height": 728179,
        "depth": 0
      },
      "101": {
        "width": 640806,
        "height": 451469,
        "depth": 0
      },
      "102": {
        "width": 458758,
        "height": 728179,
        "depth": 0
      },
      "103": {
        "width": 713626,
        "height": 451469,
        "depth": 203891
      },
      "104": {
        "width": 786445,
        "height": 728179,
        "depth": 0
      },
      "105": {
        "width": 422349,
        "height": 712781,
        "depth": 0
      },
      "106": {
        "width": 458758,
        "height": 712781,
        "depth": 203891
      },
      "107": {
        "width": 750035,
        "height": 728179,
        "depth": 0
      },
      "108": {
        "width": 422349,
        "height": 728179,
        "depth": 0
      },
      "109": {
        "width": 1150541,
        "height": 451469,
        "depth": 0
      },
      "110": {
        "width": 786445,
        "height": 451469,
        "depth": 0
      },
      "111": {
        "width": 713626,
        "height": 451469,
        "depth": 0
      },
      "112": {
        "width": 786445,
        "height": 451469,
        "depth": 203891
      },
      "113": {
        "width": 750035,
        "height": 451469,
        "depth": 203891
      },
      "114": {
        "width": 567987,
        "height": 451469,
        "depth": 0
      },
      "115": {
        "width": 575270,
        "height": 451469,
        "depth": 0
      },
      "116": {
        "width": 567987,
        "height": 644957,
        "depth": 0
      },
      "117": {
        "width": 786445,
        "height": 451469,
        "depth": 0
      },
      "118": {
        "width": 750035,
        "height": 451469,
        "depth": 0
      },
      "119": {
        "width": 1004902,
        "height": 451469,
        "depth": 0
      },
      "120": {
        "width": 750035,
        "height": 451469,
        "depth": 0
      },
      "121": {
        "width": 750035,
        "height": 451469,
        "depth": 203891
      },
      "122": {
        "width": 640806,
        "height": 451469,
        "depth": 0
      },
      "123": {
        "width": 713626,
        "height": 451469,
        "depth": 0
      },
      "124": {
        "width": 1427251,
        "height": 451469,
        "depth": 0
      },
      "125": {
        "width": 713626,
        "height": 728179,
        "depth": 0
      },
      "126": {
        "width": 713626,
        "height": 712781,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 5242880
  },
  "cmr6": {
    "characters": {
      "0": {
        "width": 790307,
        "height": 716528,
        "depth": 0
      },
      "1": {
        "width": 1048565,
        "height": 716528,
        "depth": 0
      },
      "2": {
        "width": 980603,
        "height": 716528,
        "depth": 0
      },
      "3": {
        "width": 871861,
        "height": 716528,
        "depth": 0
      },
      "4": {
        "width": 844677,
        "height": 716528,
        "depth": 0
      },
      "5": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "6": {
        "width": 912640,
        "height": 716528,
        "depth": 0
      },
      "7": {
        "width": 980603,
        "height": 716528,
        "depth": 0
      },
      "8": {
        "width": 912640,
        "height": 716528,
        "depth": 0
      },
      "9": {
        "width": 980603,
        "height": 716528,
        "depth": 0
      },
      "10": {
        "width": 912640,
        "height": 716528,
        "depth": 0
      },
      "11": {
        "width": 771859,
        "height": 728179,
        "depth": 0
      },
      "12": {
        "width": 737877,
        "height": 728179,
        "depth": 0
      },
      "13": {
        "width": 737877,
        "height": 728179,
        "depth": 0
      },
      "14": {
        "width": 1106816,
        "height": 728179,
        "depth": 0
      },
      "15": {
        "width": 1106816,
        "height": 728179,
        "depth": 0
      },
      "16": {
        "width": 368939,
        "height": 451469,
        "depth": 0
      },
      "17": {
        "width": 402920,
        "height": 451469,
        "depth": 203891
      },
      "18": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "19": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "20": {
        "width": 640789,
        "height": 659003,
        "depth": 0
      },
      "21": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "22": {
        "width": 640789,
        "height": 606813,
        "depth": 0
      },
      "23": {
        "width": 939824,
        "height": 728179,
        "depth": 0
      },
      "24": {
        "width": 572827,
        "height": 0,
        "depth": 178405
      },
      "25": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "26": {
        "width": 912640,
        "height": 451469,
        "depth": 0
      },
      "27": {
        "width": 980603,
        "height": 451469,
        "depth": 0
      },
      "28": {
        "width": 640789,
        "height": 553416,
        "depth": 101947
      },
      "29": {
        "width": 1130120,
        "height": 716528,
        "depth": 0
      },
      "30": {
        "width": 1266045,
        "height": 716528,
        "depth": 0
      },
      "31": {
        "width": 980603,
        "height": 767501,
        "depth": 50973
      },
      "32": {
        "width": 368939,
        "height": 451469,
        "depth": 0
      },
      "33": {
        "width": 368939,
        "height": 728179,
        "depth": 0
      },
      "34": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "35": {
        "width": 1048565,
        "height": 728179,
        "depth": 203888
      },
      "36": {
        "width": 640789,
        "height": 786432,
        "depth": 58253
      },
      "37": {
        "width": 1048565,
        "height": 786432,
        "depth": 58253
      },
      "38": {
        "width": 980603,
        "height": 728179,
        "depth": 0
      },
      "39": {
        "width": 368939,
        "height": 728179,
        "depth": 0
      },
      "40": {
        "width": 504864,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 504864,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 640789,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 980603,
        "height": 669920,
        "depth": 145632
      },
      "44": {
        "width": 368939,
        "height": 126219,
        "depth": 203891
      },
      "45": {
        "width": 436901,
        "height": 451469,
        "depth": 0
      },
      "46": {
        "width": 368939,
        "height": 126219,
        "depth": 0
      },
      "47": {
        "width": 640789,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 640789,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 368939,
        "height": 451469,
        "depth": 0
      },
      "59": {
        "width": 368939,
        "height": 451469,
        "depth": 203891
      },
      "60": {
        "width": 368939,
        "height": 524288,
        "depth": 203891
      },
      "61": {
        "width": 980603,
        "height": 408872,
        "depth": 4294851880
      },
      "62": {
        "width": 606808,
        "height": 524288,
        "depth": 203891
      },
      "63": {
        "width": 606808,
        "height": 728179,
        "depth": 0
      },
      "64": {
        "width": 980603,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 892251,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 912640,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 960213,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 858269,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 824288,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 987400,
        "height": 716528,
        "depth": 0
      },
      "72": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 464085,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 654381,
        "height": 716528,
        "depth": 0
      },
      "75": {
        "width": 973805,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 790307,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1143712,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 980603,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 858269,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 980603,
        "height": 716528,
        "depth": 203891
      },
      "82": {
        "width": 926232,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 708752,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 912640,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1279637,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 939824,
        "height": 716528,
        "depth": 0
      },
      "90": {
        "width": 776715,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 368939,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "93": {
        "width": 368939,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "95": {
        "width": 368939,
        "height": 708067,
        "depth": 0
      },
      "96": {
        "width": 368939,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 640789,
        "height": 451469,
        "depth": 0
      },
      "98": {
        "width": 708752,
        "height": 728179,
        "depth": 0
      },
      "99": {
        "width": 572827,
        "height": 451469,
        "depth": 0
      },
      "100": {
        "width": 708752,
        "height": 728179,
        "depth": 0
      },
      "101": {
        "width": 572829,
        "height": 451469,
        "depth": 0
      },
      "102": {
        "width": 402920,
        "height": 728179,
        "depth": 0
      },
      "103": {
        "width": 640789,
        "height": 451469,
        "depth": 203891
      },
      "104": {
        "width": 708752,
        "height": 728179,
        "depth": 0
      },
      "105": {
        "width": 368939,
        "height": 708067,
        "depth": 0
      },
      "106": {
        "width": 402920,
        "height": 708067,
        "depth": 203891
      },
      "107": {
        "width": 674771,
        "height": 728179,
        "depth": 0
      },
      "108": {
        "width": 368939,
        "height": 728179,
        "depth": 0
      },
      "109": {
        "width": 1048565,
        "height": 451469,
        "depth": 0
      },
      "110": {
        "width": 708752,
        "height": 451469,
        "depth": 0
      },
      "111": {
        "width": 640789,
        "height": 451469,
        "depth": 0
      },
      "112": {
        "width": 708752,
        "height": 451469,
        "depth": 203891
      },
      "113": {
        "width": 674771,
        "height": 451469,
        "depth": 203891
      },
      "114": {
        "width": 504864,
        "height": 451469,
        "depth": 0
      },
      "115": {
        "width": 511661,
        "height": 451469,
        "depth": 0
      },
      "116": {
        "width": 504864,
        "height": 644957,
        "depth": 0
      },
      "117": {
        "width": 708752,
        "height": 451469,
        "depth": 0
      },
      "118": {
        "width": 674771,
        "height": 451469,
        "depth": 0
      },
      "119": {
        "width": 912640,
        "height": 451469,
        "depth": 0
      },
      "120": {
        "width": 674771,
        "height": 451469,
        "depth": 0
      },
      "121": {
        "width": 674771,
        "height": 451469,
        "depth": 203891
      },
      "122": {
        "width": 572827,
        "height": 451469,
        "depth": 0
      },
      "123": {
        "width": 640789,
        "height": 451469,
        "depth": 0
      },
      "124": {
        "width": 1281579,
        "height": 451469,
        "depth": 0
      },
      "125": {
        "width": 640789,
        "height": 728179,
        "depth": 0
      },
      "126": {
        "width": 640789,
        "height": 708067,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 6291456
  },
  "cmr7": {
    "characters": {
      "0": {
        "width": 740665,
        "height": 716528,
        "depth": 0
      },
      "1": {
        "width": 984085,
        "height": 716528,
        "depth": 0
      },
      "2": {
        "width": 919589,
        "height": 716528,
        "depth": 0
      },
      "3": {
        "width": 819726,
        "height": 716528,
        "depth": 0
      },
      "4": {
        "width": 790597,
        "height": 716528,
        "depth": 0
      },
      "5": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "6": {
        "width": 855093,
        "height": 716528,
        "depth": 0
      },
      "7": {
        "width": 919589,
        "height": 716528,
        "depth": 0
      },
      "8": {
        "width": 855093,
        "height": 716528,
        "depth": 0
      },
      "9": {
        "width": 919589,
        "height": 716528,
        "depth": 0
      },
      "10": {
        "width": 855093,
        "height": 716528,
        "depth": 0
      },
      "11": {
        "width": 710498,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 678249,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 678249,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1017374,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1017374,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 339125,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 371374,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 597109,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 597109,
        "height": 602096,
        "depth": 0
      },
      "23": {
        "width": 884222,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 532613,
        "height": 0,
        "depth": 178405
      },
      "25": {
        "width": 597111,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 855093,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 919589,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 597109,
        "height": 553415,
        "depth": 101945
      },
      "29": {
        "width": 1063145,
        "height": 716528,
        "depth": 0
      },
      "30": {
        "width": 1192137,
        "height": 716528,
        "depth": 0
      },
      "31": {
        "width": 919589,
        "height": 767502,
        "depth": 50974
      },
      "32": {
        "width": 339125,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 339125,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 984085,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 597109,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 984085,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 919589,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 339125,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 468117,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 468117,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 597109,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 919589,
        "height": 649120,
        "depth": 124832
      },
      "44": {
        "width": 339125,
        "height": 120670,
        "depth": 203890
      },
      "45": {
        "width": 403621,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 339125,
        "height": 120670,
        "depth": 0
      },
      "47": {
        "width": 597109,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 597109,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 339125,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 339125,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 339125,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 919589,
        "height": 402736,
        "depth": 4294845744
      },
      "62": {
        "width": 564862,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 564862,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 919589,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 837410,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 855093,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 901906,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 805161,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 772914,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 926871,
        "height": 716528,
        "depth": 0
      },
      "72": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 432750,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 611673,
        "height": 716528,
        "depth": 0
      },
      "75": {
        "width": 916471,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 740665,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1077710,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 919589,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 805161,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 919589,
        "height": 716528,
        "depth": 203890
      },
      "82": {
        "width": 869657,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 661605,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 855093,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1206702,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 884222,
        "height": 716528,
        "depth": 0
      },
      "90": {
        "width": 726101,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 339125,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 339125,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 339125,
        "height": 705294,
        "depth": 0
      },
      "96": {
        "width": 339125,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 597109,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 661605,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 532613,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 661605,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 532613,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 371374,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 597109,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 661605,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 339125,
        "height": 705294,
        "depth": 0
      },
      "106": {
        "width": 371374,
        "height": 705294,
        "depth": 203890
      },
      "107": {
        "width": 629358,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 339125,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 984085,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 661605,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 597109,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 661605,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 629355,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 468117,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 474567,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 468117,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 661605,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 629358,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 855093,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 629358,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 629358,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 532613,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 597109,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1194217,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 597109,
        "height": 705294,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmr8": {
    "characters": {
      "0": {
        "width": 695784,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 928440,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 772608,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 742752,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 649908,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 928440,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 928440,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 309480,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 340428,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 557064,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 557064,
        "height": 601474,
        "depth": 0
      },
      "23": {
        "width": 834504,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 495168,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 804648,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 866544,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 557064,
        "height": 553416,
        "depth": 101946
      },
      "29": {
        "width": 1005264,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1129056,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 866544,
        "height": 767498,
        "depth": 50972
      },
      "32": {
        "width": 309480,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 928440,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 557064,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 928440,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 557064,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "44": {
        "width": 309480,
        "height": 116508,
        "depth": 203890
      },
      "45": {
        "width": 371376,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 309480,
        "height": 116508,
        "depth": 0
      },
      "47": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "49": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "50": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "51": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "52": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "53": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "54": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "56": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "58": {
        "width": 309480,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 309480,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 309480,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 866544,
        "height": 396348,
        "depth": 4294839356
      },
      "62": {
        "width": 526116,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 526116,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 788628,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 850524,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 757680,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 726732,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 874008,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 401232,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 571992,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 865452,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 695784,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1020192,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 757680,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 866544,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 819576,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 618960,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1143984,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 680856,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 309480,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 309480,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 309480,
        "height": 703212,
        "depth": 0
      },
      "96": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 340428,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 557064,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 309480,
        "height": 703212,
        "depth": 0
      },
      "106": {
        "width": 340428,
        "height": 703212,
        "depth": 203890
      },
      "107": {
        "width": 588012,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 928440,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 618960,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 618960,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 588012,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 434182,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 439462,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 433272,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 618960,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 588012,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 804648,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 588012,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 588012,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1114128,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 557064,
        "height": 703212,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmr9": {
    "characters": {
      "0": {
        "width": 673479,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 898080,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 748238,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 718464,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 628656,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 898080,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 898080,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 299360,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 329296,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 538848,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 538848,
        "height": 598398,
        "depth": 0
      },
      "23": {
        "width": 808110,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 478976,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 778336,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 838208,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 538848,
        "height": 553415,
        "depth": 101945
      },
      "29": {
        "width": 972839,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1092583,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 838208,
        "height": 767499,
        "depth": 50972
      },
      "32": {
        "width": 299360,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 898080,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 538848,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 898080,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 538848,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "44": {
        "width": 299360,
        "height": 113273,
        "depth": 203890
      },
      "45": {
        "width": 359232,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 299360,
        "height": 113273,
        "depth": 0
      },
      "47": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 299360,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 299360,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 299360,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 838208,
        "height": 390377,
        "depth": 4294833385
      },
      "62": {
        "width": 508912,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 508912,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 763287,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 823159,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 733351,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 703415,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 845652,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 389006,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 553735,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 838046,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 673479,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 987726,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 733351,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 838208,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 793223,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 598720,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1107470,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 658592,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 299360,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 299360,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 299360,
        "height": 701595,
        "depth": 0
      },
      "96": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 478976,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 479381,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 329296,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 538848,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 299360,
        "height": 701595,
        "depth": 0
      },
      "106": {
        "width": 329296,
        "height": 701595,
        "depth": 203890
      },
      "107": {
        "width": 568784,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 898080,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 598720,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 598720,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 568784,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 421532,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 425092,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 419104,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 598720,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 568784,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 778336,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 568784,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 568784,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 478976,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1077696,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 538848,
        "height": 701595,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmsl10": {
    "characters": {
      "0": {
        "width": 655362,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 873816,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 728179,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 699053,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 611672,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 320400,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 524290,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 524290,
        "height": 595357,
        "depth": 0
      },
      "23": {
        "width": 847926,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 466035,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 524291,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 757307,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 815562,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 524290,
        "height": 553416,
        "depth": 101946
      },
      "29": {
        "width": 946634,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1063142,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 815562,
        "height": 767499,
        "depth": 50973
      },
      "32": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 873816,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 524290,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 873816,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 524290,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "44": {
        "width": 291272,
        "height": 110683,
        "depth": 203890
      },
      "45": {
        "width": 349526,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 291272,
        "height": 110683,
        "depth": 0
      },
      "47": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 524290,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 291272,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 291272,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 291272,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 815562,
        "height": 384696,
        "depth": 4294827704
      },
      "62": {
        "width": 495163,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 495163,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 742744,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 800998,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 713616,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 684490,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 822843,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 378653,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 538853,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 655362,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 961197,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 815562,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 713616,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 815562,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 771870,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 582544,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 757307,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1077706,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 640798,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 291272,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 291272,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 291272,
        "height": 700301,
        "depth": 0
      },
      "96": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 524290,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 291272,
        "height": 700301,
        "depth": 0
      },
      "106": {
        "width": 320400,
        "height": 700301,
        "depth": 203890
      },
      "107": {
        "width": 553418,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 873816,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 582544,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 582544,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 553416,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 410694,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 413606,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 407781,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 582544,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 553418,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 757307,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 553418,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 553418,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 466035,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 524290,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1048579,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 524290,
        "height": 700301,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmsl12": {
    "characters": {
      "0": {
        "width": 641519,
        "height": 716527,
        "depth": 0
      },
      "1": {
        "width": 855600,
        "height": 716527,
        "depth": 0
      },
      "2": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "3": {
        "width": 712637,
        "height": 716527,
        "depth": 0
      },
      "4": {
        "width": 684480,
        "height": 716527,
        "depth": 0
      },
      "5": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "6": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "7": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "8": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "9": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "10": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "11": {
        "width": 598920,
        "height": 728177,
        "depth": 0
      },
      "12": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "13": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "14": {
        "width": 855600,
        "height": 728177,
        "depth": 0
      },
      "15": {
        "width": 855600,
        "height": 728177,
        "depth": 0
      },
      "16": {
        "width": 285200,
        "height": 451471,
        "depth": 0
      },
      "17": {
        "width": 313720,
        "height": 451471,
        "depth": 203889
      },
      "18": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "19": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "20": {
        "width": 513360,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "22": {
        "width": 513360,
        "height": 592249,
        "depth": 0
      },
      "23": {
        "width": 831169,
        "height": 728177,
        "depth": 0
      },
      "24": {
        "width": 456320,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "26": {
        "width": 741520,
        "height": 451471,
        "depth": 0
      },
      "27": {
        "width": 798560,
        "height": 451471,
        "depth": 0
      },
      "28": {
        "width": 513360,
        "height": 553416,
        "depth": 101945
      },
      "29": {
        "width": 926719,
        "height": 716527,
        "depth": 0
      },
      "30": {
        "width": 1040799,
        "height": 716527,
        "depth": 0
      },
      "31": {
        "width": 798560,
        "height": 767499,
        "depth": 50972
      },
      "32": {
        "width": 285200,
        "height": 451471,
        "depth": 0
      },
      "33": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "34": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "35": {
        "width": 855600,
        "height": 728177,
        "depth": 203888
      },
      "36": {
        "width": 513360,
        "height": 786432,
        "depth": 58255
      },
      "37": {
        "width": 855600,
        "height": 786432,
        "depth": 58255
      },
      "38": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "39": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "40": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 513360,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 798560,
        "height": 604384,
        "depth": 80096
      },
      "44": {
        "width": 285200,
        "height": 101945,
        "depth": 203889
      },
      "45": {
        "width": 342240,
        "height": 451471,
        "depth": 0
      },
      "46": {
        "width": 285200,
        "height": 101945,
        "depth": 0
      },
      "47": {
        "width": 513360,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 513360,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 285200,
        "height": 451471,
        "depth": 0
      },
      "59": {
        "width": 285200,
        "height": 451471,
        "depth": 203889
      },
      "60": {
        "width": 285200,
        "height": 524288,
        "depth": 203889
      },
      "61": {
        "width": 798560,
        "height": 378433,
        "depth": 4294821441
      },
      "62": {
        "width": 484840,
        "height": 524288,
        "depth": 203889
      },
      "63": {
        "width": 484840,
        "height": 728177,
        "depth": 0
      },
      "64": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "65": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "66": {
        "width": 727079,
        "height": 716527,
        "depth": 0
      },
      "67": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "68": {
        "width": 784119,
        "height": 716527,
        "depth": 0
      },
      "69": {
        "width": 698559,
        "height": 716527,
        "depth": 0
      },
      "70": {
        "width": 670039,
        "height": 716527,
        "depth": 0
      },
      "71": {
        "width": 805600,
        "height": 716527,
        "depth": 0
      },
      "72": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "73": {
        "width": 370397,
        "height": 716527,
        "depth": 0
      },
      "74": {
        "width": 527439,
        "height": 716527,
        "depth": 0
      },
      "75": {
        "width": 798197,
        "height": 716527,
        "depth": 0
      },
      "76": {
        "width": 641519,
        "height": 716527,
        "depth": 0
      },
      "77": {
        "width": 940797,
        "height": 716527,
        "depth": 0
      },
      "78": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "79": {
        "width": 798560,
        "height": 716527,
        "depth": 0
      },
      "80": {
        "width": 698559,
        "height": 716527,
        "depth": 0
      },
      "81": {
        "width": 798560,
        "height": 716527,
        "depth": 203889
      },
      "82": {
        "width": 755599,
        "height": 716527,
        "depth": 0
      },
      "83": {
        "width": 570400,
        "height": 716527,
        "depth": 0
      },
      "84": {
        "width": 741520,
        "height": 716527,
        "depth": 0
      },
      "85": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "86": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "87": {
        "width": 1054877,
        "height": 716527,
        "depth": 0
      },
      "88": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "89": {
        "width": 769677,
        "height": 716527,
        "depth": 0
      },
      "90": {
        "width": 627440,
        "height": 716527,
        "depth": 0
      },
      "91": {
        "width": 285200,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "93": {
        "width": 285200,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "95": {
        "width": 285200,
        "height": 695932,
        "depth": 0
      },
      "96": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "97": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "98": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "99": {
        "width": 456320,
        "height": 451471,
        "depth": 0
      },
      "100": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "101": {
        "width": 456320,
        "height": 451471,
        "depth": 0
      },
      "102": {
        "width": 313720,
        "height": 728177,
        "depth": 0
      },
      "103": {
        "width": 513360,
        "height": 451471,
        "depth": 203889
      },
      "104": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "105": {
        "width": 285200,
        "height": 695932,
        "depth": 0
      },
      "106": {
        "width": 313720,
        "height": 695932,
        "depth": 203889
      },
      "107": {
        "width": 541880,
        "height": 728177,
        "depth": 0
      },
      "108": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "109": {
        "width": 855600,
        "height": 451471,
        "depth": 0
      },
      "110": {
        "width": 570400,
        "height": 451471,
        "depth": 0
      },
      "111": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "112": {
        "width": 570400,
        "height": 451471,
        "depth": 203889
      },
      "113": {
        "width": 541880,
        "height": 451471,
        "depth": 203889
      },
      "114": {
        "width": 399280,
        "height": 451471,
        "depth": 0
      },
      "115": {
        "width": 404984,
        "height": 451471,
        "depth": 0
      },
      "116": {
        "width": 399280,
        "height": 644959,
        "depth": 0
      },
      "117": {
        "width": 570400,
        "height": 451471,
        "depth": 0
      },
      "118": {
        "width": 541880,
        "height": 451471,
        "depth": 0
      },
      "119": {
        "width": 741520,
        "height": 451471,
        "depth": 0
      },
      "120": {
        "width": 541880,
        "height": 451471,
        "depth": 0
      },
      "121": {
        "width": 541880,
        "height": 451471,
        "depth": 203889
      },
      "122": {
        "width": 456320,
        "height": 451471,
        "depth": 0
      },
      "123": {
        "width": 513360,
        "height": 451471,
        "depth": 0
      },
      "124": {
        "width": 1026720,
        "height": 451471,
        "depth": 0
      },
      "125": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "126": {
        "width": 513360,
        "height": 695932,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 12582912
  },
  "cmsl8": {
    "characters": {
      "0": {
        "width": 695784,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 928440,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 772608,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 742752,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 649908,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 928440,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 928440,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 309480,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 340428,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 557064,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 557064,
        "height": 601474,
        "depth": 0
      },
      "23": {
        "width": 895996,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 495168,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 804648,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 866544,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 557064,
        "height": 553416,
        "depth": 101946
      },
      "29": {
        "width": 1005264,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1129056,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 866544,
        "height": 767498,
        "depth": 50972
      },
      "32": {
        "width": 309480,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 928440,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 557064,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 928440,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 557064,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "44": {
        "width": 309480,
        "height": 116508,
        "depth": 203890
      },
      "45": {
        "width": 371376,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 309480,
        "height": 116508,
        "depth": 0
      },
      "47": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "49": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "50": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "51": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "52": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "53": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "54": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "56": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 557064,
        "height": 675750,
        "depth": 0
      },
      "58": {
        "width": 309480,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 309480,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 309480,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 866544,
        "height": 396348,
        "depth": 4294839356
      },
      "62": {
        "width": 526116,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 526116,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 788628,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 850524,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 757680,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 726732,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 874008,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 401232,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 571992,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 865452,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 695784,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1020192,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 866544,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 757680,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 866544,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 819576,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 618960,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 804648,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1143984,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 680856,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 309480,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 309480,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 309480,
        "height": 703212,
        "depth": 0
      },
      "96": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 340428,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 557064,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 309480,
        "height": 703212,
        "depth": 0
      },
      "106": {
        "width": 340428,
        "height": 703212,
        "depth": 203890
      },
      "107": {
        "width": 588012,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 928440,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 618960,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 618960,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 588012,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 434182,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 439462,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 433272,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 618960,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 588012,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 804648,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 588012,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 588012,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1114128,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 557064,
        "height": 703212,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmsl9": {
    "characters": {
      "0": {
        "width": 673479,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 898080,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 748238,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 718464,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 628656,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 898080,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 898080,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 299360,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 329296,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 538848,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 538848,
        "height": 598398,
        "depth": 0
      },
      "23": {
        "width": 869602,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 478976,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 778336,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 838208,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 538848,
        "height": 553415,
        "depth": 101945
      },
      "29": {
        "width": 972839,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1092583,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 838208,
        "height": 767499,
        "depth": 50972
      },
      "32": {
        "width": 299360,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 898080,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 538848,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 898080,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 538848,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "44": {
        "width": 299360,
        "height": 113273,
        "depth": 203890
      },
      "45": {
        "width": 359232,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 299360,
        "height": 113273,
        "depth": 0
      },
      "47": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "53": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "56": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 538848,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 299360,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 299360,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 299360,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 838208,
        "height": 390377,
        "depth": 4294833385
      },
      "62": {
        "width": 508912,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 508912,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 763287,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 823159,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 733351,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 703415,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 845652,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 389006,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 553735,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 838046,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 673479,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 987726,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 838208,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 733351,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 838208,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 793223,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 598720,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 778336,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1107470,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 658592,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 299360,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 299360,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 299360,
        "height": 701595,
        "depth": 0
      },
      "96": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 478976,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 479381,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 329296,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 538848,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 299360,
        "height": 701595,
        "depth": 0
      },
      "106": {
        "width": 329296,
        "height": 701595,
        "depth": 203890
      },
      "107": {
        "width": 568784,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 898080,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 598720,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 598720,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 568784,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 421532,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 425092,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 419104,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 598720,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 568784,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 778336,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 568784,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 568784,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 478976,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 538848,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1077696,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 538848,
        "height": 701595,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmsltt10": {
    "characters": {
      "0": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "1": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "2": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "3": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "4": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "5": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "6": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "7": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "8": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "9": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "10": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "11": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "12": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "13": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "14": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "15": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "16": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "18": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "19": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "20": {
        "width": 550498,
        "height": 593466,
        "depth": 0
      },
      "21": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "22": {
        "width": 550498,
        "height": 593027,
        "depth": 0
      },
      "23": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "24": {
        "width": 550498,
        "height": 0,
        "depth": 203891
      },
      "25": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "26": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 550498,
        "height": 567979,
        "depth": 116509
      },
      "29": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "30": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "31": {
        "width": 550498,
        "height": 699051,
        "depth": 58254
      },
      "32": {
        "width": 550498,
        "height": 230104,
        "depth": 116509
      },
      "33": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "34": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "35": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "36": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "37": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "38": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "39": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "40": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "41": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "42": {
        "width": 550498,
        "height": 546134,
        "depth": 0
      },
      "43": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "44": {
        "width": 550498,
        "height": 131072,
        "depth": 145635
      },
      "45": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "46": {
        "width": 550498,
        "height": 131072,
        "depth": 0
      },
      "47": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "48": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "49": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "50": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "51": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "52": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "53": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "54": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "55": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "56": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "57": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "58": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 550498,
        "height": 451470,
        "depth": 145635
      },
      "60": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "61": {
        "width": 550498,
        "height": 435813,
        "depth": 4294762312
      },
      "62": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "63": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "64": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "65": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "66": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "67": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "68": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "69": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "70": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "71": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "72": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "73": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "74": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "75": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "76": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "77": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "78": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "79": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "80": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "81": {
        "width": 550498,
        "height": 640797,
        "depth": 145635
      },
      "82": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "83": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "84": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "85": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "86": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "87": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "88": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "89": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "90": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "91": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "92": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "93": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "94": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "95": {
        "width": 550498,
        "height": 0,
        "depth": 99757
      },
      "96": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "97": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "99": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "101": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "103": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "104": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "105": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "106": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "107": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "108": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "109": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "113": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "114": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 550498,
        "height": 580466,
        "depth": 0
      },
      "117": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "122": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "124": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "125": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "126": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmss10": {
    "characters": {
      "0": {
        "width": 567981,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 640800,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 699053,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 742746,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 611672,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 562155,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 562155,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 853427,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 853427,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 250494,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 279622,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 524290,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 524290,
        "height": 638464,
        "depth": 0
      },
      "23": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 466035,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 503902,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 757307,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 815562,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 524290,
        "height": 567979,
        "depth": 101946
      },
      "29": {
        "width": 902944,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 1019453,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 815562,
        "height": 779150,
        "depth": 50973
      },
      "32": {
        "width": 250494,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 334963,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 873816,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 524290,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 873816,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 795173,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 524290,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "44": {
        "width": 291272,
        "height": 87381,
        "depth": 131072
      },
      "45": {
        "width": 349526,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 291272,
        "height": 87381,
        "depth": 0
      },
      "47": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "49": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "50": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "51": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "52": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "53": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "54": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "55": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "56": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "57": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "58": {
        "width": 291272,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 291272,
        "height": 466034,
        "depth": 131072
      },
      "60": {
        "width": 334963,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 815562,
        "height": 387973,
        "depth": 4294830981
      },
      "62": {
        "width": 495163,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 495163,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 699053,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 669926,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 757309,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 626235,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 699053,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 742746,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 291274,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 495163,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 728182,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 567981,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 917509,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 742746,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 771870,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 669926,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 771870,
        "height": 728178,
        "depth": 131072
      },
      "82": {
        "width": 677208,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 713616,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 720901,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 990326,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 640798,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 302923,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 302923,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 291272,
        "height": 712366,
        "depth": 0
      },
      "96": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 503901,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 541766,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 466035,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 541766,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 466035,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 524290,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 541766,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 250494,
        "height": 712366,
        "depth": 0
      },
      "106": {
        "width": 279622,
        "height": 712366,
        "depth": 203890
      },
      "107": {
        "width": 512640,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 250494,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 833038,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 541766,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 524290,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 541766,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 541766,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 358266,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 401955,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 378653,
        "height": 599189,
        "depth": 0
      },
      "117": {
        "width": 541766,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 483512,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 716530,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 483512,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 483512,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 455840,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 524290,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1048579,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 524290,
        "height": 709454,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmss12": {
    "characters": {
      "0": {
        "width": 554621,
        "height": 728177,
        "depth": 0
      },
      "1": {
        "width": 855600,
        "height": 728177,
        "depth": 0
      },
      "2": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "3": {
        "width": 621371,
        "height": 728177,
        "depth": 0
      },
      "4": {
        "width": 684480,
        "height": 728177,
        "depth": 0
      },
      "5": {
        "width": 719675,
        "height": 728177,
        "depth": 0
      },
      "6": {
        "width": 741520,
        "height": 728177,
        "depth": 0
      },
      "7": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "8": {
        "width": 741520,
        "height": 728177,
        "depth": 0
      },
      "9": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "10": {
        "width": 741520,
        "height": 728177,
        "depth": 0
      },
      "11": {
        "width": 598920,
        "height": 728177,
        "depth": 0
      },
      "12": {
        "width": 548555,
        "height": 728177,
        "depth": 0
      },
      "13": {
        "width": 548555,
        "height": 728177,
        "depth": 0
      },
      "14": {
        "width": 833755,
        "height": 728177,
        "depth": 0
      },
      "15": {
        "width": 833755,
        "height": 728177,
        "depth": 0
      },
      "16": {
        "width": 241509,
        "height": 466033,
        "depth": 0
      },
      "17": {
        "width": 270029,
        "height": 466033,
        "depth": 203889
      },
      "18": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "19": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "20": {
        "width": 513360,
        "height": 662641,
        "depth": 0
      },
      "21": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "22": {
        "width": 513360,
        "height": 637397,
        "depth": 0
      },
      "23": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "24": {
        "width": 456320,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 491515,
        "height": 728177,
        "depth": 0
      },
      "26": {
        "width": 741520,
        "height": 466033,
        "depth": 0
      },
      "27": {
        "width": 798560,
        "height": 466033,
        "depth": 0
      },
      "28": {
        "width": 513360,
        "height": 567979,
        "depth": 101945
      },
      "29": {
        "width": 881085,
        "height": 728177,
        "depth": 0
      },
      "30": {
        "width": 995165,
        "height": 728177,
        "depth": 0
      },
      "31": {
        "width": 798560,
        "height": 779149,
        "depth": 50972
      },
      "32": {
        "width": 241509,
        "height": 466033,
        "depth": 0
      },
      "33": {
        "width": 326464,
        "height": 728177,
        "depth": 0
      },
      "34": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "35": {
        "width": 855600,
        "height": 728177,
        "depth": 203888
      },
      "36": {
        "width": 513360,
        "height": 786432,
        "depth": 58255
      },
      "37": {
        "width": 855600,
        "height": 786432,
        "depth": 58255
      },
      "38": {
        "width": 776715,
        "height": 728177,
        "depth": 0
      },
      "39": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "40": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 513360,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 798560,
        "height": 604384,
        "depth": 80096
      },
      "44": {
        "width": 285200,
        "height": 84955,
        "depth": 131072
      },
      "45": {
        "width": 342240,
        "height": 466033,
        "depth": 0
      },
      "46": {
        "width": 285200,
        "height": 84955,
        "depth": 0
      },
      "47": {
        "width": 513360,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "49": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "50": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "51": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "52": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "53": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "54": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "55": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "56": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "57": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "58": {
        "width": 285200,
        "height": 466033,
        "depth": 0
      },
      "59": {
        "width": 285200,
        "height": 466033,
        "depth": 131072
      },
      "60": {
        "width": 326464,
        "height": 524288,
        "depth": 203889
      },
      "61": {
        "width": 798560,
        "height": 381564,
        "depth": 4294824572
      },
      "62": {
        "width": 484840,
        "height": 524288,
        "depth": 203889
      },
      "63": {
        "width": 484840,
        "height": 728177,
        "depth": 0
      },
      "64": {
        "width": 684480,
        "height": 728177,
        "depth": 0
      },
      "65": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "66": {
        "width": 681445,
        "height": 728177,
        "depth": 0
      },
      "67": {
        "width": 655960,
        "height": 728177,
        "depth": 0
      },
      "68": {
        "width": 738485,
        "height": 728177,
        "depth": 0
      },
      "69": {
        "width": 611661,
        "height": 728177,
        "depth": 0
      },
      "70": {
        "width": 583141,
        "height": 728177,
        "depth": 0
      },
      "71": {
        "width": 684480,
        "height": 728177,
        "depth": 0
      },
      "72": {
        "width": 719675,
        "height": 728177,
        "depth": 0
      },
      "73": {
        "width": 279131,
        "height": 728177,
        "depth": 0
      },
      "74": {
        "width": 481805,
        "height": 728177,
        "depth": 0
      },
      "75": {
        "width": 706931,
        "height": 728177,
        "depth": 0
      },
      "76": {
        "width": 554621,
        "height": 728177,
        "depth": 0
      },
      "77": {
        "width": 890795,
        "height": 728177,
        "depth": 0
      },
      "78": {
        "width": 719675,
        "height": 728177,
        "depth": 0
      },
      "79": {
        "width": 757296,
        "height": 728177,
        "depth": 0
      },
      "80": {
        "width": 652925,
        "height": 728177,
        "depth": 0
      },
      "81": {
        "width": 757296,
        "height": 728177,
        "depth": 131072
      },
      "82": {
        "width": 660813,
        "height": 728177,
        "depth": 0
      },
      "83": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "84": {
        "width": 700256,
        "height": 728177,
        "depth": 0
      },
      "85": {
        "width": 699043,
        "height": 728177,
        "depth": 0
      },
      "86": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "87": {
        "width": 963611,
        "height": 728177,
        "depth": 0
      },
      "88": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "89": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "90": {
        "width": 627440,
        "height": 728177,
        "depth": 0
      },
      "91": {
        "width": 296731,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "93": {
        "width": 296731,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "95": {
        "width": 285200,
        "height": 710667,
        "depth": 0
      },
      "96": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "97": {
        "width": 491515,
        "height": 466033,
        "depth": 0
      },
      "98": {
        "width": 526709,
        "height": 728177,
        "depth": 0
      },
      "99": {
        "width": 456320,
        "height": 466033,
        "depth": 0
      },
      "100": {
        "width": 526709,
        "height": 728177,
        "depth": 0
      },
      "101": {
        "width": 456320,
        "height": 466033,
        "depth": 0
      },
      "102": {
        "width": 313720,
        "height": 728177,
        "depth": 0
      },
      "103": {
        "width": 513360,
        "height": 466033,
        "depth": 203889
      },
      "104": {
        "width": 526709,
        "height": 728177,
        "depth": 0
      },
      "105": {
        "width": 241509,
        "height": 710667,
        "depth": 0
      },
      "106": {
        "width": 270029,
        "height": 710667,
        "depth": 203889
      },
      "107": {
        "width": 498189,
        "height": 728177,
        "depth": 0
      },
      "108": {
        "width": 241509,
        "height": 728177,
        "depth": 0
      },
      "109": {
        "width": 811909,
        "height": 466033,
        "depth": 0
      },
      "110": {
        "width": 526709,
        "height": 466033,
        "depth": 0
      },
      "111": {
        "width": 513360,
        "height": 466033,
        "depth": 0
      },
      "112": {
        "width": 526709,
        "height": 466033,
        "depth": 203889
      },
      "113": {
        "width": 526709,
        "height": 466033,
        "depth": 203889
      },
      "114": {
        "width": 348915,
        "height": 466033,
        "depth": 0
      },
      "115": {
        "width": 393576,
        "height": 466033,
        "depth": 0
      },
      "116": {
        "width": 370760,
        "height": 599189,
        "depth": 0
      },
      "117": {
        "width": 526709,
        "height": 466033,
        "depth": 0
      },
      "118": {
        "width": 469669,
        "height": 466033,
        "depth": 0
      },
      "119": {
        "width": 697829,
        "height": 466033,
        "depth": 0
      },
      "120": {
        "width": 469669,
        "height": 466033,
        "depth": 0
      },
      "121": {
        "width": 469669,
        "height": 466033,
        "depth": 203889
      },
      "122": {
        "width": 445397,
        "height": 466033,
        "depth": 0
      },
      "123": {
        "width": 513360,
        "height": 466033,
        "depth": 0
      },
      "124": {
        "width": 1026720,
        "height": 466033,
        "depth": 0
      },
      "125": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "126": {
        "width": 513360,
        "height": 708240,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 12582912
  },
  "cmss17": {
    "characters": {
      "0": {
        "width": 533486,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 821722,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 766941,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 600067,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 657378,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 695303,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 712159,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 766941,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 712159,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 766941,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 712159,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 575206,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 527588,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 527588,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 801495,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 801495,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 233454,
        "height": 451403,
        "depth": 0
      },
      "17": {
        "width": 260844,
        "height": 451403,
        "depth": 203957
      },
      "18": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 493033,
        "height": 658984,
        "depth": 0
      },
      "21": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 493033,
        "height": 629198,
        "depth": 0
      },
      "23": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 438252,
        "height": 0,
        "depth": 178463
      },
      "25": {
        "width": 472807,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 712159,
        "height": 451403,
        "depth": 0
      },
      "27": {
        "width": 766941,
        "height": 451403,
        "depth": 0
      },
      "28": {
        "width": 493033,
        "height": 553382,
        "depth": 101979
      },
      "29": {
        "width": 847848,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 957411,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 766941,
        "height": 779168,
        "depth": 50990
      },
      "32": {
        "width": 233454,
        "height": 451403,
        "depth": 0
      },
      "33": {
        "width": 314362,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 821722,
        "height": 728178,
        "depth": 203957
      },
      "36": {
        "width": 493033,
        "height": 785488,
        "depth": 57310
      },
      "37": {
        "width": 821722,
        "height": 785488,
        "depth": 57310
      },
      "38": {
        "width": 746714,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 273907,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 383470,
        "height": 785488,
        "depth": 261267
      },
      "41": {
        "width": 383470,
        "height": 785488,
        "depth": 261267
      },
      "42": {
        "width": 493033,
        "height": 785488,
        "depth": 0
      },
      "43": {
        "width": 766941,
        "height": 590799,
        "depth": 66579
      },
      "44": {
        "width": 273907,
        "height": 84280,
        "depth": 131477
      },
      "45": {
        "width": 328689,
        "height": 451403,
        "depth": 0
      },
      "46": {
        "width": 273907,
        "height": 84280,
        "depth": 0
      },
      "47": {
        "width": 493033,
        "height": 785488,
        "depth": 261267
      },
      "48": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "49": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "50": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "51": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "52": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "53": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "54": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "55": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "56": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "57": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "58": {
        "width": 273907,
        "height": 451403,
        "depth": 0
      },
      "59": {
        "width": 273907,
        "height": 451403,
        "depth": 131477
      },
      "60": {
        "width": 314362,
        "height": 524220,
        "depth": 203957
      },
      "61": {
        "width": 766941,
        "height": 372851,
        "depth": 4294815927
      },
      "62": {
        "width": 465643,
        "height": 524220,
        "depth": 203957
      },
      "63": {
        "width": 465643,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 657378,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 656113,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 629987,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 710894,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 588268,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 560877,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 657378,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 695303,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 271378,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 464378,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 682239,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 533486,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 859647,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 695303,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 726486,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 628722,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 726486,
        "height": 728178,
        "depth": 131477
      },
      "82": {
        "width": 635885,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 547815,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 671705,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 675076,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 928756,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 602596,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 286129,
        "height": 785488,
        "depth": 261267
      },
      "92": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 286129,
        "height": 785488,
        "depth": 261267
      },
      "94": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 273907,
        "height": 688688,
        "depth": 0
      },
      "96": {
        "width": 273907,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 472807,
        "height": 451403,
        "depth": 0
      },
      "98": {
        "width": 507361,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 438252,
        "height": 451403,
        "depth": 0
      },
      "100": {
        "width": 507361,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 438252,
        "height": 451403,
        "depth": 0
      },
      "102": {
        "width": 301298,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 493033,
        "height": 451403,
        "depth": 203957
      },
      "104": {
        "width": 507361,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 233454,
        "height": 688688,
        "depth": 0
      },
      "106": {
        "width": 260844,
        "height": 688688,
        "depth": 203957
      },
      "107": {
        "width": 479970,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 233454,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 781269,
        "height": 451403,
        "depth": 0
      },
      "110": {
        "width": 507361,
        "height": 451403,
        "depth": 0
      },
      "111": {
        "width": 493033,
        "height": 451403,
        "depth": 0
      },
      "112": {
        "width": 507361,
        "height": 451403,
        "depth": 203957
      },
      "113": {
        "width": 507361,
        "height": 451403,
        "depth": 203957
      },
      "114": {
        "width": 335853,
        "height": 451403,
        "depth": 0
      },
      "115": {
        "width": 377993,
        "height": 451403,
        "depth": 0
      },
      "116": {
        "width": 356080,
        "height": 580378,
        "depth": 0
      },
      "117": {
        "width": 507361,
        "height": 451403,
        "depth": 0
      },
      "118": {
        "width": 452580,
        "height": 451403,
        "depth": 0
      },
      "119": {
        "width": 671706,
        "height": 451403,
        "depth": 0
      },
      "120": {
        "width": 452580,
        "height": 451403,
        "depth": 0
      },
      "121": {
        "width": 452580,
        "height": 451403,
        "depth": 203957
      },
      "122": {
        "width": 428138,
        "height": 451403,
        "depth": 0
      },
      "123": {
        "width": 493033,
        "height": 451403,
        "depth": 0
      },
      "124": {
        "width": 986067,
        "height": 451403,
        "depth": 0
      },
      "125": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 493033,
        "height": 687002,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 18119392
  },
  "cmss8": {
    "characters": {
      "0": {
        "width": 606218,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 928440,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 680856,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 786442,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 649908,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 597114,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 597114,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 906594,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 906594,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 265788,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 296736,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 557064,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 557064,
        "height": 644436,
        "depth": 0
      },
      "23": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 495168,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 535218,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 804648,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 866544,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 557064,
        "height": 567980,
        "depth": 101946
      },
      "29": {
        "width": 959388,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 1083180,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 866544,
        "height": 779150,
        "depth": 50972
      },
      "32": {
        "width": 265788,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 353170,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 928440,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 557064,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 928440,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 844698,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 557064,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "44": {
        "width": 309480,
        "height": 94664,
        "depth": 134712
      },
      "45": {
        "width": 371376,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 309480,
        "height": 94664,
        "depth": 0
      },
      "47": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "49": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "50": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "51": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "52": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "53": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "54": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "55": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "56": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "57": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "58": {
        "width": 309480,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 309480,
        "height": 466034,
        "depth": 134712
      },
      "60": {
        "width": 353170,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 866544,
        "height": 399916,
        "depth": 4294842924
      },
      "62": {
        "width": 526116,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 526116,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 711804,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 668114,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 637166,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 786442,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 526116,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 773700,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 606218,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 972130,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 786442,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 822854,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 711804,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 822854,
        "height": 728178,
        "depth": 134712
      },
      "82": {
        "width": 720906,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 760958,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 764598,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 1052232,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 680856,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 324044,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 324044,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 309480,
        "height": 718558,
        "depth": 0
      },
      "96": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 535218,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 575268,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 495168,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 575268,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 495168,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 340428,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 557064,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 575268,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 265788,
        "height": 718558,
        "depth": 0
      },
      "106": {
        "width": 296736,
        "height": 718558,
        "depth": 203890
      },
      "107": {
        "width": 544320,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 265788,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 884748,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 575268,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 557064,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 575268,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 575268,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 380478,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 427082,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 402324,
        "height": 599190,
        "depth": 0
      },
      "117": {
        "width": 575268,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 513372,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 760956,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 513372,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 513372,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 484244,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 557064,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1114128,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 557064,
        "height": 713096,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmss9": {
    "characters": {
      "0": {
        "width": 587392,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 898080,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 660210,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 718464,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 762155,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 628656,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 577684,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 577684,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 877044,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 877044,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 257287,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 287223,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 538848,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 538848,
        "height": 642738,
        "depth": 0
      },
      "23": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 478976,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 517812,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 778336,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 838208,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 538848,
        "height": 567979,
        "depth": 101945
      },
      "29": {
        "width": 928825,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 1048569,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 838208,
        "height": 779150,
        "depth": 50972
      },
      "32": {
        "width": 257287,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 341433,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 898080,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 538848,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 898080,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 817172,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 538848,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "44": {
        "width": 299360,
        "height": 90617,
        "depth": 132690
      },
      "45": {
        "width": 359232,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 299360,
        "height": 90617,
        "depth": 0
      },
      "47": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "49": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "50": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "51": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "52": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "53": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "54": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "55": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "56": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "57": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "58": {
        "width": 299360,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 299360,
        "height": 466034,
        "depth": 132690
      },
      "60": {
        "width": 341433,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 838208,
        "height": 393799,
        "depth": 4294836807
      },
      "62": {
        "width": 508912,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 508912,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 718464,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 719273,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 688528,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 779145,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 647264,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 617328,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 718464,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 762155,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 300978,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 509721,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 750018,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 587392,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 941771,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 762155,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 796135,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 689337,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 796135,
        "height": 728178,
        "depth": 132690
      },
      "82": {
        "width": 698236,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 736263,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 741118,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 1019442,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 658592,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 313115,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 313115,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 299360,
        "height": 715927,
        "depth": 0
      },
      "96": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 517812,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 556647,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 478976,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 556647,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 478978,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 329296,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 538848,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 556647,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 257287,
        "height": 715927,
        "depth": 0
      },
      "106": {
        "width": 287223,
        "height": 715927,
        "depth": 203890
      },
      "107": {
        "width": 526711,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 257287,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 856007,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 556647,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 538848,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 556647,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 556647,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 368132,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 413118,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 389168,
        "height": 599189,
        "depth": 0
      },
      "117": {
        "width": 556647,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 496775,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 736263,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 496775,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 496775,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 468457,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 538848,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1077696,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 538848,
        "height": 711072,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmssbx10": {
    "characters": {
      "0": {
        "width": 608760,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 961200,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 897120,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 704880,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 897120,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 897120,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 672840,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 614586,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 614586,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 934986,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 934986,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 267971,
        "height": 480597,
        "depth": 0
      },
      "17": {
        "width": 300011,
        "height": 480597,
        "depth": 203890
      },
      "18": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 576720,
        "height": 666283,
        "depth": 0
      },
      "21": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 576720,
        "height": 668757,
        "depth": 0
      },
      "23": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 512640,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 592739,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 833040,
        "height": 480597,
        "depth": 0
      },
      "27": {
        "width": 897120,
        "height": 480597,
        "depth": 0
      },
      "28": {
        "width": 576720,
        "height": 582542,
        "depth": 101946
      },
      "29": {
        "width": 993240,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 1121400,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 897120,
        "height": 779150,
        "depth": 50973
      },
      "32": {
        "width": 267971,
        "height": 480597,
        "depth": 0
      },
      "33": {
        "width": 384480,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 585458,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 961200,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 576720,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 1079109,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 870906,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 448560,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 448560,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 576720,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 897120,
        "height": 646624,
        "depth": 122336
      },
      "44": {
        "width": 320400,
        "height": 136898,
        "depth": 110683
      },
      "45": {
        "width": 384480,
        "height": 480597,
        "depth": 0
      },
      "46": {
        "width": 320400,
        "height": 136898,
        "depth": 0
      },
      "47": {
        "width": 576720,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "49": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "50": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "51": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "52": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "53": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "54": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "55": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "56": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 320400,
        "height": 480597,
        "depth": 0
      },
      "59": {
        "width": 320400,
        "height": 480597,
        "depth": 110683
      },
      "60": {
        "width": 384480,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 897120,
        "height": 425984,
        "depth": 4294868992
      },
      "62": {
        "width": 544680,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 544680,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 736920,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 672840,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 640800,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 346614,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 544680,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 801000,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 608760,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 1025280,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 833040,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 736920,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 833040,
        "height": 728178,
        "depth": 110683
      },
      "82": {
        "width": 736920,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 640800,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 801000,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 1089360,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 768960,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 704880,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 359722,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 585458,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 359722,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 550506,
        "height": 480597,
        "depth": 0
      },
      "98": {
        "width": 588371,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 512640,
        "height": 480597,
        "depth": 0
      },
      "100": {
        "width": 588371,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 535942,
        "height": 480597,
        "depth": 0
      },
      "102": {
        "width": 352440,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 576720,
        "height": 480597,
        "depth": 203890
      },
      "104": {
        "width": 588371,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 267971,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 300011,
        "height": 728178,
        "depth": 203890
      },
      "107": {
        "width": 556331,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 267971,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 908771,
        "height": 480597,
        "depth": 0
      },
      "110": {
        "width": 588371,
        "height": 480597,
        "depth": 0
      },
      "111": {
        "width": 576720,
        "height": 480597,
        "depth": 0
      },
      "112": {
        "width": 588371,
        "height": 480597,
        "depth": 203890
      },
      "113": {
        "width": 588371,
        "height": 480597,
        "depth": 203890
      },
      "114": {
        "width": 390306,
        "height": 480597,
        "depth": 0
      },
      "115": {
        "width": 442152,
        "height": 480597,
        "depth": 0
      },
      "116": {
        "width": 423802,
        "height": 617914,
        "depth": 0
      },
      "117": {
        "width": 588371,
        "height": 480597,
        "depth": 0
      },
      "118": {
        "width": 524291,
        "height": 480597,
        "depth": 0
      },
      "119": {
        "width": 780611,
        "height": 480597,
        "depth": 0
      },
      "120": {
        "width": 524291,
        "height": 480597,
        "depth": 0
      },
      "121": {
        "width": 524291,
        "height": 480597,
        "depth": 203890
      },
      "122": {
        "width": 499533,
        "height": 480597,
        "depth": 0
      },
      "123": {
        "width": 576720,
        "height": 480597,
        "depth": 0
      },
      "124": {
        "width": 1153440,
        "height": 480597,
        "depth": 0
      },
      "125": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 576720,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmssdc10": {
    "characters": {
      "0": {
        "width": 538843,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 830112,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 774771,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 611661,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 664090,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 710693,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 719430,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 774771,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 719430,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 774771,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 719430,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 581078,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 530106,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 530106,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 806810,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 806810,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 230099,
        "height": 495162,
        "depth": 0
      },
      "17": {
        "width": 257770,
        "height": 495162,
        "depth": 174763
      },
      "18": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 498067,
        "height": 669923,
        "depth": 0
      },
      "21": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 498067,
        "height": 665262,
        "depth": 0
      },
      "23": {
        "width": 667002,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 442726,
        "height": 0,
        "depth": 152918
      },
      "25": {
        "width": 490789,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 719430,
        "height": 495162,
        "depth": 0
      },
      "27": {
        "width": 774771,
        "height": 495162,
        "depth": 0
      },
      "28": {
        "width": 498067,
        "height": 582544,
        "depth": 87382
      },
      "29": {
        "width": 859238,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 969920,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 774771,
        "height": 771869,
        "depth": 43691
      },
      "32": {
        "width": 230099,
        "height": 495162,
        "depth": 0
      },
      "33": {
        "width": 320395,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 498069,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 830112,
        "height": 728178,
        "depth": 174762
      },
      "36": {
        "width": 498067,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 922162,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 751469,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 276704,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 387386,
        "height": 786432,
        "depth": 233018
      },
      "41": {
        "width": 387386,
        "height": 786432,
        "depth": 233018
      },
      "42": {
        "width": 498067,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 774771,
        "height": 608752,
        "depth": 55338
      },
      "44": {
        "width": 276704,
        "height": 110683,
        "depth": 116509
      },
      "45": {
        "width": 332045,
        "height": 495162,
        "depth": 0
      },
      "46": {
        "width": 276704,
        "height": 110683,
        "depth": 0
      },
      "47": {
        "width": 498067,
        "height": 786432,
        "depth": 233018
      },
      "48": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "49": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "50": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "51": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "52": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "53": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "54": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "55": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "56": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 276704,
        "height": 495162,
        "depth": 0
      },
      "59": {
        "width": 276704,
        "height": 495162,
        "depth": 116509
      },
      "60": {
        "width": 320395,
        "height": 553414,
        "depth": 174763
      },
      "61": {
        "width": 774771,
        "height": 431955,
        "depth": 4294845837
      },
      "62": {
        "width": 470397,
        "height": 553414,
        "depth": 174763
      },
      "63": {
        "width": 470397,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 664090,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 667002,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 665546,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 636419,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 720886,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 594184,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 566514,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 664090,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 710693,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 297094,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 471853,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 694672,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 538843,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 876715,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 710693,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 731080,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 637875,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 731080,
        "height": 728178,
        "depth": 116509
      },
      "82": {
        "width": 643699,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 553408,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 675739,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 688848,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 667002,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 943706,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 667002,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 667002,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 608749,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 307288,
        "height": 786432,
        "depth": 233018
      },
      "92": {
        "width": 498069,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 307288,
        "height": 786432,
        "depth": 233018
      },
      "94": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 276704,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 276704,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 474765,
        "height": 495162,
        "depth": 0
      },
      "98": {
        "width": 506803,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 442726,
        "height": 495162,
        "depth": 0
      },
      "100": {
        "width": 506803,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 459475,
        "height": 495162,
        "depth": 0
      },
      "102": {
        "width": 304374,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 506803,
        "height": 495162,
        "depth": 174763
      },
      "104": {
        "width": 506803,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 230099,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 257770,
        "height": 728178,
        "depth": 174763
      },
      "107": {
        "width": 479133,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 230099,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 783507,
        "height": 495162,
        "depth": 0
      },
      "110": {
        "width": 506803,
        "height": 495162,
        "depth": 0
      },
      "111": {
        "width": 498067,
        "height": 495162,
        "depth": 0
      },
      "112": {
        "width": 506803,
        "height": 495162,
        "depth": 174763
      },
      "113": {
        "width": 506803,
        "height": 495162,
        "depth": 174763
      },
      "114": {
        "width": 336413,
        "height": 495162,
        "depth": 0
      },
      "115": {
        "width": 381853,
        "height": 495162,
        "depth": 0
      },
      "116": {
        "width": 362629,
        "height": 636640,
        "depth": 0
      },
      "117": {
        "width": 506803,
        "height": 495162,
        "depth": 0
      },
      "118": {
        "width": 451462,
        "height": 495162,
        "depth": 0
      },
      "119": {
        "width": 672826,
        "height": 495162,
        "depth": 0
      },
      "120": {
        "width": 451462,
        "height": 495162,
        "depth": 0
      },
      "121": {
        "width": 451462,
        "height": 495162,
        "depth": 174763
      },
      "122": {
        "width": 431075,
        "height": 495162,
        "depth": 0
      },
      "123": {
        "width": 498067,
        "height": 495162,
        "depth": 0
      },
      "124": {
        "width": 996134,
        "height": 495162,
        "depth": 0
      },
      "125": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 498067,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmssi10": {
    "characters": {
      "0": {
        "width": 567981,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 640800,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 699053,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 742746,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 611672,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 562155,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 562155,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 853427,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 853427,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 250494,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 279622,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 524290,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 524290,
        "height": 638464,
        "depth": 0
      },
      "23": {
        "width": 773347,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 466035,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 503902,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 757307,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 815562,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 524290,
        "height": 567979,
        "depth": 101946
      },
      "29": {
        "width": 902944,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 1019453,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 815562,
        "height": 779150,
        "depth": 50973
      },
      "32": {
        "width": 250494,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 334963,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 873816,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 524290,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 873816,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 795173,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 524290,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "44": {
        "width": 291272,
        "height": 87381,
        "depth": 131072
      },
      "45": {
        "width": 349526,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 291272,
        "height": 87381,
        "depth": 0
      },
      "47": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "49": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "50": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "51": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "52": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "53": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "54": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "55": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "56": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "57": {
        "width": 524290,
        "height": 687400,
        "depth": 0
      },
      "58": {
        "width": 291272,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 291272,
        "height": 466034,
        "depth": 131072
      },
      "60": {
        "width": 334963,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 815562,
        "height": 387973,
        "depth": 4294830981
      },
      "62": {
        "width": 495163,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 495163,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 699053,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 669926,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 757309,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 626235,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 597109,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 699053,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 742746,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 291274,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 495163,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 728182,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 567981,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 917509,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 742746,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 771870,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 669926,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 771870,
        "height": 728178,
        "depth": 131072
      },
      "82": {
        "width": 677208,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 713616,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 720901,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 990326,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 699054,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 640798,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 302923,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 302923,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 291272,
        "height": 712366,
        "depth": 0
      },
      "96": {
        "width": 291272,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 503901,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 541766,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 466035,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 541766,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 466035,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 320400,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 524290,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 541766,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 250494,
        "height": 712366,
        "depth": 0
      },
      "106": {
        "width": 279622,
        "height": 712366,
        "depth": 203890
      },
      "107": {
        "width": 512640,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 250494,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 833038,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 541766,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 524290,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 541766,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 541766,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 358266,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 401955,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 378653,
        "height": 599189,
        "depth": 0
      },
      "117": {
        "width": 541766,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 483512,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 716530,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 483512,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 483512,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 455840,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 524290,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1048579,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 524290,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 524290,
        "height": 709454,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmssi12": {
    "characters": {
      "0": {
        "width": 554621,
        "height": 728177,
        "depth": 0
      },
      "1": {
        "width": 855600,
        "height": 728177,
        "depth": 0
      },
      "2": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "3": {
        "width": 621371,
        "height": 728177,
        "depth": 0
      },
      "4": {
        "width": 684480,
        "height": 728177,
        "depth": 0
      },
      "5": {
        "width": 719675,
        "height": 728177,
        "depth": 0
      },
      "6": {
        "width": 741520,
        "height": 728177,
        "depth": 0
      },
      "7": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "8": {
        "width": 741520,
        "height": 728177,
        "depth": 0
      },
      "9": {
        "width": 798560,
        "height": 728177,
        "depth": 0
      },
      "10": {
        "width": 741520,
        "height": 728177,
        "depth": 0
      },
      "11": {
        "width": 598920,
        "height": 728177,
        "depth": 0
      },
      "12": {
        "width": 548555,
        "height": 728177,
        "depth": 0
      },
      "13": {
        "width": 548555,
        "height": 728177,
        "depth": 0
      },
      "14": {
        "width": 833755,
        "height": 728177,
        "depth": 0
      },
      "15": {
        "width": 833755,
        "height": 728177,
        "depth": 0
      },
      "16": {
        "width": 241509,
        "height": 466033,
        "depth": 0
      },
      "17": {
        "width": 270029,
        "height": 466033,
        "depth": 203889
      },
      "18": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "19": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "20": {
        "width": 513360,
        "height": 662641,
        "depth": 0
      },
      "21": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "22": {
        "width": 513360,
        "height": 637397,
        "depth": 0
      },
      "23": {
        "width": 752704,
        "height": 728177,
        "depth": 0
      },
      "24": {
        "width": 456320,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 491515,
        "height": 728177,
        "depth": 0
      },
      "26": {
        "width": 741520,
        "height": 466033,
        "depth": 0
      },
      "27": {
        "width": 798560,
        "height": 466033,
        "depth": 0
      },
      "28": {
        "width": 513360,
        "height": 567979,
        "depth": 101945
      },
      "29": {
        "width": 881085,
        "height": 728177,
        "depth": 0
      },
      "30": {
        "width": 995165,
        "height": 728177,
        "depth": 0
      },
      "31": {
        "width": 798560,
        "height": 779149,
        "depth": 50972
      },
      "32": {
        "width": 241509,
        "height": 466033,
        "depth": 0
      },
      "33": {
        "width": 326464,
        "height": 728177,
        "depth": 0
      },
      "34": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "35": {
        "width": 855600,
        "height": 728177,
        "depth": 203888
      },
      "36": {
        "width": 513360,
        "height": 786432,
        "depth": 58255
      },
      "37": {
        "width": 855600,
        "height": 786432,
        "depth": 58255
      },
      "38": {
        "width": 776715,
        "height": 728177,
        "depth": 0
      },
      "39": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "40": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 399280,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 513360,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 798560,
        "height": 604384,
        "depth": 80096
      },
      "44": {
        "width": 285200,
        "height": 84955,
        "depth": 131072
      },
      "45": {
        "width": 342240,
        "height": 466033,
        "depth": 0
      },
      "46": {
        "width": 285200,
        "height": 84955,
        "depth": 0
      },
      "47": {
        "width": 513360,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "49": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "50": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "51": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "52": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "53": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "54": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "55": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "56": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "57": {
        "width": 513360,
        "height": 686915,
        "depth": 0
      },
      "58": {
        "width": 285200,
        "height": 466033,
        "depth": 0
      },
      "59": {
        "width": 285200,
        "height": 466033,
        "depth": 131072
      },
      "60": {
        "width": 326464,
        "height": 524288,
        "depth": 203889
      },
      "61": {
        "width": 798560,
        "height": 381564,
        "depth": 4294824572
      },
      "62": {
        "width": 484840,
        "height": 524288,
        "depth": 203889
      },
      "63": {
        "width": 484840,
        "height": 728177,
        "depth": 0
      },
      "64": {
        "width": 684480,
        "height": 728177,
        "depth": 0
      },
      "65": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "66": {
        "width": 681445,
        "height": 728177,
        "depth": 0
      },
      "67": {
        "width": 655960,
        "height": 728177,
        "depth": 0
      },
      "68": {
        "width": 738485,
        "height": 728177,
        "depth": 0
      },
      "69": {
        "width": 611661,
        "height": 728177,
        "depth": 0
      },
      "70": {
        "width": 583141,
        "height": 728177,
        "depth": 0
      },
      "71": {
        "width": 684480,
        "height": 728177,
        "depth": 0
      },
      "72": {
        "width": 719675,
        "height": 728177,
        "depth": 0
      },
      "73": {
        "width": 279131,
        "height": 728177,
        "depth": 0
      },
      "74": {
        "width": 481805,
        "height": 728177,
        "depth": 0
      },
      "75": {
        "width": 706931,
        "height": 728177,
        "depth": 0
      },
      "76": {
        "width": 554621,
        "height": 728177,
        "depth": 0
      },
      "77": {
        "width": 890795,
        "height": 728177,
        "depth": 0
      },
      "78": {
        "width": 719675,
        "height": 728177,
        "depth": 0
      },
      "79": {
        "width": 757296,
        "height": 728177,
        "depth": 0
      },
      "80": {
        "width": 652925,
        "height": 728177,
        "depth": 0
      },
      "81": {
        "width": 757296,
        "height": 728177,
        "depth": 131072
      },
      "82": {
        "width": 660813,
        "height": 728177,
        "depth": 0
      },
      "83": {
        "width": 570400,
        "height": 728177,
        "depth": 0
      },
      "84": {
        "width": 700256,
        "height": 728177,
        "depth": 0
      },
      "85": {
        "width": 699043,
        "height": 728177,
        "depth": 0
      },
      "86": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "87": {
        "width": 963611,
        "height": 728177,
        "depth": 0
      },
      "88": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "89": {
        "width": 678411,
        "height": 728177,
        "depth": 0
      },
      "90": {
        "width": 627440,
        "height": 728177,
        "depth": 0
      },
      "91": {
        "width": 296731,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "93": {
        "width": 296731,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "95": {
        "width": 285200,
        "height": 710667,
        "depth": 0
      },
      "96": {
        "width": 285200,
        "height": 728177,
        "depth": 0
      },
      "97": {
        "width": 491515,
        "height": 466033,
        "depth": 0
      },
      "98": {
        "width": 526709,
        "height": 728177,
        "depth": 0
      },
      "99": {
        "width": 456320,
        "height": 466033,
        "depth": 0
      },
      "100": {
        "width": 526709,
        "height": 728177,
        "depth": 0
      },
      "101": {
        "width": 456320,
        "height": 466033,
        "depth": 0
      },
      "102": {
        "width": 313720,
        "height": 728177,
        "depth": 0
      },
      "103": {
        "width": 513360,
        "height": 466033,
        "depth": 203889
      },
      "104": {
        "width": 526709,
        "height": 728177,
        "depth": 0
      },
      "105": {
        "width": 241509,
        "height": 710667,
        "depth": 0
      },
      "106": {
        "width": 270029,
        "height": 710667,
        "depth": 203889
      },
      "107": {
        "width": 498189,
        "height": 728177,
        "depth": 0
      },
      "108": {
        "width": 241509,
        "height": 728177,
        "depth": 0
      },
      "109": {
        "width": 811909,
        "height": 466033,
        "depth": 0
      },
      "110": {
        "width": 526709,
        "height": 466033,
        "depth": 0
      },
      "111": {
        "width": 513360,
        "height": 466033,
        "depth": 0
      },
      "112": {
        "width": 526709,
        "height": 466033,
        "depth": 203889
      },
      "113": {
        "width": 526709,
        "height": 466033,
        "depth": 203889
      },
      "114": {
        "width": 348915,
        "height": 466033,
        "depth": 0
      },
      "115": {
        "width": 393576,
        "height": 466033,
        "depth": 0
      },
      "116": {
        "width": 370760,
        "height": 599189,
        "depth": 0
      },
      "117": {
        "width": 526709,
        "height": 466033,
        "depth": 0
      },
      "118": {
        "width": 469669,
        "height": 466033,
        "depth": 0
      },
      "119": {
        "width": 697829,
        "height": 466033,
        "depth": 0
      },
      "120": {
        "width": 469669,
        "height": 466033,
        "depth": 0
      },
      "121": {
        "width": 469669,
        "height": 466033,
        "depth": 203889
      },
      "122": {
        "width": 445397,
        "height": 466033,
        "depth": 0
      },
      "123": {
        "width": 513360,
        "height": 466033,
        "depth": 0
      },
      "124": {
        "width": 1026720,
        "height": 466033,
        "depth": 0
      },
      "125": {
        "width": 513360,
        "height": 728177,
        "depth": 0
      },
      "126": {
        "width": 513360,
        "height": 708240,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 12582912
  },
  "cmssi17": {
    "characters": {
      "0": {
        "width": 533486,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 821722,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 766941,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 600067,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 657378,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 695303,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 712159,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 766941,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 712159,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 766941,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 712159,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 575206,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 527588,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 527588,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 801495,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 801495,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 233454,
        "height": 451403,
        "depth": 0
      },
      "17": {
        "width": 260844,
        "height": 451403,
        "depth": 203957
      },
      "18": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 493033,
        "height": 658984,
        "depth": 0
      },
      "21": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 493033,
        "height": 629198,
        "depth": 0
      },
      "23": {
        "width": 733288,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 438252,
        "height": 0,
        "depth": 178463
      },
      "25": {
        "width": 472807,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 712159,
        "height": 451403,
        "depth": 0
      },
      "27": {
        "width": 766941,
        "height": 451403,
        "depth": 0
      },
      "28": {
        "width": 493033,
        "height": 553382,
        "depth": 101979
      },
      "29": {
        "width": 847848,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 957411,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 766941,
        "height": 779168,
        "depth": 50990
      },
      "32": {
        "width": 233454,
        "height": 451403,
        "depth": 0
      },
      "33": {
        "width": 314362,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 821722,
        "height": 728178,
        "depth": 203957
      },
      "36": {
        "width": 493033,
        "height": 785488,
        "depth": 57310
      },
      "37": {
        "width": 821722,
        "height": 785488,
        "depth": 57310
      },
      "38": {
        "width": 746714,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 273907,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 383470,
        "height": 785488,
        "depth": 261267
      },
      "41": {
        "width": 383470,
        "height": 785488,
        "depth": 261267
      },
      "42": {
        "width": 493033,
        "height": 785488,
        "depth": 0
      },
      "43": {
        "width": 766941,
        "height": 590799,
        "depth": 66579
      },
      "44": {
        "width": 273907,
        "height": 84280,
        "depth": 131477
      },
      "45": {
        "width": 328689,
        "height": 451403,
        "depth": 0
      },
      "46": {
        "width": 273907,
        "height": 84280,
        "depth": 0
      },
      "47": {
        "width": 493033,
        "height": 785488,
        "depth": 261267
      },
      "48": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "49": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "50": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "51": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "52": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "53": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "54": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "55": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "56": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "57": {
        "width": 493033,
        "height": 687723,
        "depth": 0
      },
      "58": {
        "width": 273907,
        "height": 451403,
        "depth": 0
      },
      "59": {
        "width": 273907,
        "height": 451403,
        "depth": 131477
      },
      "60": {
        "width": 314362,
        "height": 524220,
        "depth": 203957
      },
      "61": {
        "width": 766941,
        "height": 372851,
        "depth": 4294815927
      },
      "62": {
        "width": 465643,
        "height": 524220,
        "depth": 203957
      },
      "63": {
        "width": 465643,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 657378,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 656113,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 629987,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 710894,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 588268,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 560877,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 657378,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 695303,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 271378,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 464378,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 682239,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 533486,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 859647,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 695303,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 726486,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 628722,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 726486,
        "height": 728178,
        "depth": 131477
      },
      "82": {
        "width": 635885,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 547815,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 671705,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 675076,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 928756,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 654848,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 602596,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 286129,
        "height": 785488,
        "depth": 261267
      },
      "92": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 286129,
        "height": 785488,
        "depth": 261267
      },
      "94": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 273907,
        "height": 688688,
        "depth": 0
      },
      "96": {
        "width": 273907,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 472807,
        "height": 451403,
        "depth": 0
      },
      "98": {
        "width": 507361,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 438252,
        "height": 451403,
        "depth": 0
      },
      "100": {
        "width": 507361,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 438252,
        "height": 451403,
        "depth": 0
      },
      "102": {
        "width": 301298,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 493033,
        "height": 451403,
        "depth": 203957
      },
      "104": {
        "width": 507361,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 233454,
        "height": 688688,
        "depth": 0
      },
      "106": {
        "width": 260844,
        "height": 688688,
        "depth": 203957
      },
      "107": {
        "width": 479970,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 233454,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 781269,
        "height": 451403,
        "depth": 0
      },
      "110": {
        "width": 507361,
        "height": 451403,
        "depth": 0
      },
      "111": {
        "width": 493033,
        "height": 451403,
        "depth": 0
      },
      "112": {
        "width": 507361,
        "height": 451403,
        "depth": 203957
      },
      "113": {
        "width": 507361,
        "height": 451403,
        "depth": 203957
      },
      "114": {
        "width": 335853,
        "height": 451403,
        "depth": 0
      },
      "115": {
        "width": 377993,
        "height": 451403,
        "depth": 0
      },
      "116": {
        "width": 356080,
        "height": 580378,
        "depth": 0
      },
      "117": {
        "width": 507361,
        "height": 451403,
        "depth": 0
      },
      "118": {
        "width": 452580,
        "height": 451403,
        "depth": 0
      },
      "119": {
        "width": 671706,
        "height": 451403,
        "depth": 0
      },
      "120": {
        "width": 452580,
        "height": 451403,
        "depth": 0
      },
      "121": {
        "width": 452580,
        "height": 451403,
        "depth": 203957
      },
      "122": {
        "width": 428138,
        "height": 451403,
        "depth": 0
      },
      "123": {
        "width": 493033,
        "height": 451403,
        "depth": 0
      },
      "124": {
        "width": 986067,
        "height": 451403,
        "depth": 0
      },
      "125": {
        "width": 493033,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 493033,
        "height": 687002,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 18119392
  },
  "cmssi8": {
    "characters": {
      "0": {
        "width": 606218,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 928440,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 680856,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 786442,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 649908,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 597114,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 597114,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 906594,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 906594,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 265788,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 296736,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 557064,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 557064,
        "height": 644436,
        "depth": 0
      },
      "23": {
        "width": 817046,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 495168,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 535218,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 804648,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 866544,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 557064,
        "height": 567980,
        "depth": 101946
      },
      "29": {
        "width": 959388,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 1083180,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 866544,
        "height": 779150,
        "depth": 50972
      },
      "32": {
        "width": 265788,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 353170,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 928440,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 557064,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 928440,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 844698,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 557064,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "44": {
        "width": 309480,
        "height": 94664,
        "depth": 134712
      },
      "45": {
        "width": 371376,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 309480,
        "height": 94664,
        "depth": 0
      },
      "47": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "49": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "50": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "51": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "52": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "53": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "54": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "55": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "56": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "57": {
        "width": 557064,
        "height": 688128,
        "depth": 0
      },
      "58": {
        "width": 309480,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 309480,
        "height": 466034,
        "depth": 134712
      },
      "60": {
        "width": 353170,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 866544,
        "height": 399916,
        "depth": 4294842924
      },
      "62": {
        "width": 526116,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 526116,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 711804,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 668114,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 637166,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 786442,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 526116,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 773700,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 606218,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 972130,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 786442,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 822854,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 711804,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 822854,
        "height": 728178,
        "depth": 134712
      },
      "82": {
        "width": 720906,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 760958,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 764598,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 1052232,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 742752,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 680856,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 324044,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 324044,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 309480,
        "height": 718558,
        "depth": 0
      },
      "96": {
        "width": 309480,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 535218,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 575268,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 495168,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 575268,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 495168,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 340428,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 557064,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 575268,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 265788,
        "height": 718558,
        "depth": 0
      },
      "106": {
        "width": 296736,
        "height": 718558,
        "depth": 203890
      },
      "107": {
        "width": 544320,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 265788,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 884748,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 575268,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 557064,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 575268,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 575268,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 380478,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 427082,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 402324,
        "height": 599190,
        "depth": 0
      },
      "117": {
        "width": 575268,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 513372,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 760956,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 513372,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 513372,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 484244,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 557064,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1114128,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 557064,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 557064,
        "height": 713096,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmssi9": {
    "characters": {
      "0": {
        "width": 587392,
        "height": 728178,
        "depth": 0
      },
      "1": {
        "width": 898080,
        "height": 728178,
        "depth": 0
      },
      "2": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "3": {
        "width": 660210,
        "height": 728178,
        "depth": 0
      },
      "4": {
        "width": 718464,
        "height": 728178,
        "depth": 0
      },
      "5": {
        "width": 762155,
        "height": 728178,
        "depth": 0
      },
      "6": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "7": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "8": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "9": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "10": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "11": {
        "width": 628656,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 577684,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 577684,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 877044,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 877044,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 257287,
        "height": 466034,
        "depth": 0
      },
      "17": {
        "width": 287223,
        "height": 466034,
        "depth": 203890
      },
      "18": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 538848,
        "height": 662642,
        "depth": 0
      },
      "21": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 538848,
        "height": 642738,
        "depth": 0
      },
      "23": {
        "width": 794375,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 478976,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 517812,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 778336,
        "height": 466034,
        "depth": 0
      },
      "27": {
        "width": 838208,
        "height": 466034,
        "depth": 0
      },
      "28": {
        "width": 538848,
        "height": 567979,
        "depth": 101945
      },
      "29": {
        "width": 928825,
        "height": 728178,
        "depth": 0
      },
      "30": {
        "width": 1048569,
        "height": 728178,
        "depth": 0
      },
      "31": {
        "width": 838208,
        "height": 779150,
        "depth": 50972
      },
      "32": {
        "width": 257287,
        "height": 466034,
        "depth": 0
      },
      "33": {
        "width": 341433,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 898080,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 538848,
        "height": 786432,
        "depth": 58254
      },
      "37": {
        "width": 898080,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 817172,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 538848,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "44": {
        "width": 299360,
        "height": 90617,
        "depth": 132690
      },
      "45": {
        "width": 359232,
        "height": 466034,
        "depth": 0
      },
      "46": {
        "width": 299360,
        "height": 90617,
        "depth": 0
      },
      "47": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "49": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "50": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "51": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "52": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "53": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "54": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "55": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "56": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "57": {
        "width": 538848,
        "height": 686105,
        "depth": 0
      },
      "58": {
        "width": 299360,
        "height": 466034,
        "depth": 0
      },
      "59": {
        "width": 299360,
        "height": 466034,
        "depth": 132690
      },
      "60": {
        "width": 341433,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 838208,
        "height": 393799,
        "depth": 4294836807
      },
      "62": {
        "width": 508912,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 508912,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 718464,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "66": {
        "width": 719273,
        "height": 728178,
        "depth": 0
      },
      "67": {
        "width": 688528,
        "height": 728178,
        "depth": 0
      },
      "68": {
        "width": 779145,
        "height": 728178,
        "depth": 0
      },
      "69": {
        "width": 647264,
        "height": 728178,
        "depth": 0
      },
      "70": {
        "width": 617328,
        "height": 728178,
        "depth": 0
      },
      "71": {
        "width": 718464,
        "height": 728178,
        "depth": 0
      },
      "72": {
        "width": 762155,
        "height": 728178,
        "depth": 0
      },
      "73": {
        "width": 300978,
        "height": 728178,
        "depth": 0
      },
      "74": {
        "width": 509721,
        "height": 728178,
        "depth": 0
      },
      "75": {
        "width": 750018,
        "height": 728178,
        "depth": 0
      },
      "76": {
        "width": 587392,
        "height": 728178,
        "depth": 0
      },
      "77": {
        "width": 941771,
        "height": 728178,
        "depth": 0
      },
      "78": {
        "width": 762155,
        "height": 728178,
        "depth": 0
      },
      "79": {
        "width": 796135,
        "height": 728178,
        "depth": 0
      },
      "80": {
        "width": 689337,
        "height": 728178,
        "depth": 0
      },
      "81": {
        "width": 796135,
        "height": 728178,
        "depth": 132690
      },
      "82": {
        "width": 698236,
        "height": 728178,
        "depth": 0
      },
      "83": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "84": {
        "width": 736263,
        "height": 728178,
        "depth": 0
      },
      "85": {
        "width": 741118,
        "height": 728178,
        "depth": 0
      },
      "86": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "87": {
        "width": 1019442,
        "height": 728178,
        "depth": 0
      },
      "88": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "89": {
        "width": 720082,
        "height": 728178,
        "depth": 0
      },
      "90": {
        "width": 658592,
        "height": 728178,
        "depth": 0
      },
      "91": {
        "width": 313115,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 313115,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 299360,
        "height": 715927,
        "depth": 0
      },
      "96": {
        "width": 299360,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 517812,
        "height": 466034,
        "depth": 0
      },
      "98": {
        "width": 556647,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 478976,
        "height": 466034,
        "depth": 0
      },
      "100": {
        "width": 556647,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 478978,
        "height": 466034,
        "depth": 0
      },
      "102": {
        "width": 329296,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 538848,
        "height": 466034,
        "depth": 203890
      },
      "104": {
        "width": 556647,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 257287,
        "height": 715927,
        "depth": 0
      },
      "106": {
        "width": 287223,
        "height": 715927,
        "depth": 203890
      },
      "107": {
        "width": 526711,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 257287,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 856007,
        "height": 466034,
        "depth": 0
      },
      "110": {
        "width": 556647,
        "height": 466034,
        "depth": 0
      },
      "111": {
        "width": 538848,
        "height": 466034,
        "depth": 0
      },
      "112": {
        "width": 556647,
        "height": 466034,
        "depth": 203890
      },
      "113": {
        "width": 556647,
        "height": 466034,
        "depth": 203890
      },
      "114": {
        "width": 368132,
        "height": 466034,
        "depth": 0
      },
      "115": {
        "width": 413118,
        "height": 466034,
        "depth": 0
      },
      "116": {
        "width": 389168,
        "height": 599189,
        "depth": 0
      },
      "117": {
        "width": 556647,
        "height": 466034,
        "depth": 0
      },
      "118": {
        "width": 496775,
        "height": 466034,
        "depth": 0
      },
      "119": {
        "width": 736263,
        "height": 466034,
        "depth": 0
      },
      "120": {
        "width": 496775,
        "height": 466034,
        "depth": 0
      },
      "121": {
        "width": 496775,
        "height": 466034,
        "depth": 203890
      },
      "122": {
        "width": 468457,
        "height": 466034,
        "depth": 0
      },
      "123": {
        "width": 538848,
        "height": 466034,
        "depth": 0
      },
      "124": {
        "width": 1077696,
        "height": 466034,
        "depth": 0
      },
      "125": {
        "width": 538848,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 538848,
        "height": 711072,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmssq8": {
    "characters": {
      "0": {
        "width": 742744,
        "height": 717256,
        "depth": 0
      },
      "1": {
        "width": 1092270,
        "height": 717256,
        "depth": 0
      },
      "2": {
        "width": 1019452,
        "height": 717256,
        "depth": 0
      },
      "3": {
        "width": 771872,
        "height": 717256,
        "depth": 0
      },
      "4": {
        "width": 873816,
        "height": 717256,
        "depth": 0
      },
      "5": {
        "width": 851972,
        "height": 717256,
        "depth": 0
      },
      "6": {
        "width": 946634,
        "height": 717256,
        "depth": 0
      },
      "7": {
        "width": 1019452,
        "height": 717256,
        "depth": 0
      },
      "8": {
        "width": 946634,
        "height": 717256,
        "depth": 0
      },
      "9": {
        "width": 1019452,
        "height": 717256,
        "depth": 0
      },
      "10": {
        "width": 946634,
        "height": 717256,
        "depth": 0
      },
      "11": {
        "width": 764590,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 691772,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 691772,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1055862,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1055862,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 291274,
        "height": 546134,
        "depth": 0
      },
      "17": {
        "width": 327684,
        "height": 546134,
        "depth": 145636
      },
      "18": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 655362,
        "height": 682668,
        "depth": 0
      },
      "21": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 655362,
        "height": 688126,
        "depth": 0
      },
      "23": {
        "width": 844690,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 582544,
        "height": 0,
        "depth": 127432
      },
      "25": {
        "width": 618956,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 946634,
        "height": 546134,
        "depth": 0
      },
      "27": {
        "width": 1019452,
        "height": 546134,
        "depth": 0
      },
      "28": {
        "width": 655362,
        "height": 618952,
        "depth": 72818
      },
      "29": {
        "width": 1114116,
        "height": 717256,
        "depth": 0
      },
      "30": {
        "width": 1259752,
        "height": 717256,
        "depth": 0
      },
      "31": {
        "width": 1019452,
        "height": 753666,
        "depth": 36410
      },
      "32": {
        "width": 291274,
        "height": 546134,
        "depth": 0
      },
      "33": {
        "width": 371372,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1092270,
        "height": 728178,
        "depth": 145632
      },
      "36": {
        "width": 655362,
        "height": 800996,
        "depth": 72818
      },
      "37": {
        "width": 1092270,
        "height": 800996,
        "depth": 72818
      },
      "38": {
        "width": 983044,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 364090,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 509726,
        "height": 800996,
        "depth": 218452
      },
      "41": {
        "width": 509726,
        "height": 800996,
        "depth": 218452
      },
      "42": {
        "width": 655362,
        "height": 800996,
        "depth": 0
      },
      "43": {
        "width": 1019452,
        "height": 728180,
        "depth": 145636
      },
      "44": {
        "width": 364090,
        "height": 101944,
        "depth": 145636
      },
      "45": {
        "width": 436908,
        "height": 546134,
        "depth": 0
      },
      "46": {
        "width": 364090,
        "height": 101944,
        "depth": 0
      },
      "47": {
        "width": 655362,
        "height": 800996,
        "depth": 218452
      },
      "48": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "49": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "50": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "51": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "52": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "53": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "54": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "55": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "56": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "57": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "58": {
        "width": 364090,
        "height": 546134,
        "depth": 0
      },
      "59": {
        "width": 364090,
        "height": 546134,
        "depth": 145636
      },
      "60": {
        "width": 371372,
        "height": 582542,
        "depth": 145636
      },
      "61": {
        "width": 1019452,
        "height": 450560,
        "depth": 4294835312
      },
      "62": {
        "width": 618954,
        "height": 582542,
        "depth": 145636
      },
      "63": {
        "width": 618954,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "66": {
        "width": 859254,
        "height": 717256,
        "depth": 0
      },
      "67": {
        "width": 837408,
        "height": 717256,
        "depth": 0
      },
      "68": {
        "width": 932072,
        "height": 717256,
        "depth": 0
      },
      "69": {
        "width": 815562,
        "height": 717256,
        "depth": 0
      },
      "70": {
        "width": 779154,
        "height": 717256,
        "depth": 0
      },
      "71": {
        "width": 873816,
        "height": 717256,
        "depth": 0
      },
      "72": {
        "width": 851972,
        "height": 717256,
        "depth": 0
      },
      "73": {
        "width": 334964,
        "height": 717256,
        "depth": 0
      },
      "74": {
        "width": 604390,
        "height": 717256,
        "depth": 0
      },
      "75": {
        "width": 881100,
        "height": 717256,
        "depth": 0
      },
      "76": {
        "width": 742744,
        "height": 717256,
        "depth": 0
      },
      "77": {
        "width": 1070426,
        "height": 717256,
        "depth": 0
      },
      "78": {
        "width": 851972,
        "height": 717256,
        "depth": 0
      },
      "79": {
        "width": 1012170,
        "height": 717256,
        "depth": 0
      },
      "80": {
        "width": 822844,
        "height": 717256,
        "depth": 0
      },
      "81": {
        "width": 1012170,
        "height": 717256,
        "depth": 145636
      },
      "82": {
        "width": 855612,
        "height": 717256,
        "depth": 0
      },
      "83": {
        "width": 728180,
        "height": 717256,
        "depth": 0
      },
      "84": {
        "width": 939352,
        "height": 717256,
        "depth": 0
      },
      "85": {
        "width": 848332,
        "height": 717256,
        "depth": 0
      },
      "86": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "87": {
        "width": 1208780,
        "height": 717256,
        "depth": 0
      },
      "88": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "89": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "90": {
        "width": 800998,
        "height": 717256,
        "depth": 0
      },
      "91": {
        "width": 369552,
        "height": 800996,
        "depth": 218452
      },
      "92": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 369552,
        "height": 800996,
        "depth": 218452
      },
      "94": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 364090,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 364090,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 618954,
        "height": 546134,
        "depth": 0
      },
      "98": {
        "width": 655364,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 582544,
        "height": 546134,
        "depth": 0
      },
      "100": {
        "width": 655364,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 582544,
        "height": 546134,
        "depth": 0
      },
      "102": {
        "width": 400500,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 655364,
        "height": 546134,
        "depth": 145636
      },
      "104": {
        "width": 655364,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 291274,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 327684,
        "height": 728178,
        "depth": 145636
      },
      "107": {
        "width": 618956,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 291274,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1019454,
        "height": 546134,
        "depth": 0
      },
      "110": {
        "width": 655364,
        "height": 546134,
        "depth": 0
      },
      "111": {
        "width": 655362,
        "height": 546134,
        "depth": 0
      },
      "112": {
        "width": 655364,
        "height": 546134,
        "depth": 145636
      },
      "113": {
        "width": 655364,
        "height": 546134,
        "depth": 145636
      },
      "114": {
        "width": 436910,
        "height": 546134,
        "depth": 0
      },
      "115": {
        "width": 502444,
        "height": 546134,
        "depth": 0
      },
      "116": {
        "width": 473316,
        "height": 702176,
        "depth": 0
      },
      "117": {
        "width": 655364,
        "height": 546134,
        "depth": 0
      },
      "118": {
        "width": 582546,
        "height": 546134,
        "depth": 0
      },
      "119": {
        "width": 873818,
        "height": 546134,
        "depth": 0
      },
      "120": {
        "width": 582546,
        "height": 546134,
        "depth": 0
      },
      "121": {
        "width": 582546,
        "height": 546134,
        "depth": 145636
      },
      "122": {
        "width": 564340,
        "height": 546134,
        "depth": 0
      },
      "123": {
        "width": 655362,
        "height": 546134,
        "depth": 0
      },
      "124": {
        "width": 1310724,
        "height": 546134,
        "depth": 0
      },
      "125": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmssqi8": {
    "characters": {
      "0": {
        "width": 742744,
        "height": 717256,
        "depth": 0
      },
      "1": {
        "width": 1092270,
        "height": 717256,
        "depth": 0
      },
      "2": {
        "width": 1019452,
        "height": 717256,
        "depth": 0
      },
      "3": {
        "width": 771872,
        "height": 717256,
        "depth": 0
      },
      "4": {
        "width": 873816,
        "height": 717256,
        "depth": 0
      },
      "5": {
        "width": 851972,
        "height": 717256,
        "depth": 0
      },
      "6": {
        "width": 946634,
        "height": 717256,
        "depth": 0
      },
      "7": {
        "width": 1019452,
        "height": 717256,
        "depth": 0
      },
      "8": {
        "width": 946634,
        "height": 717256,
        "depth": 0
      },
      "9": {
        "width": 1019452,
        "height": 717256,
        "depth": 0
      },
      "10": {
        "width": 946634,
        "height": 717256,
        "depth": 0
      },
      "11": {
        "width": 764590,
        "height": 728178,
        "depth": 0
      },
      "12": {
        "width": 691772,
        "height": 728178,
        "depth": 0
      },
      "13": {
        "width": 691772,
        "height": 728178,
        "depth": 0
      },
      "14": {
        "width": 1055862,
        "height": 728178,
        "depth": 0
      },
      "15": {
        "width": 1055862,
        "height": 728178,
        "depth": 0
      },
      "16": {
        "width": 291274,
        "height": 546134,
        "depth": 0
      },
      "17": {
        "width": 327684,
        "height": 546134,
        "depth": 145636
      },
      "18": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 655362,
        "height": 682668,
        "depth": 0
      },
      "21": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 655362,
        "height": 688126,
        "depth": 0
      },
      "23": {
        "width": 896282,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 582544,
        "height": 0,
        "depth": 127432
      },
      "25": {
        "width": 618956,
        "height": 728178,
        "depth": 0
      },
      "26": {
        "width": 946634,
        "height": 546134,
        "depth": 0
      },
      "27": {
        "width": 1019452,
        "height": 546134,
        "depth": 0
      },
      "28": {
        "width": 655362,
        "height": 618952,
        "depth": 72818
      },
      "29": {
        "width": 1114116,
        "height": 717256,
        "depth": 0
      },
      "30": {
        "width": 1259752,
        "height": 717256,
        "depth": 0
      },
      "31": {
        "width": 1019452,
        "height": 753666,
        "depth": 36410
      },
      "32": {
        "width": 291274,
        "height": 546134,
        "depth": 0
      },
      "33": {
        "width": 371372,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1092270,
        "height": 728178,
        "depth": 145632
      },
      "36": {
        "width": 655362,
        "height": 800996,
        "depth": 72818
      },
      "37": {
        "width": 1092270,
        "height": 800996,
        "depth": 72818
      },
      "38": {
        "width": 983044,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 364090,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 509726,
        "height": 800996,
        "depth": 218452
      },
      "41": {
        "width": 509726,
        "height": 800996,
        "depth": 218452
      },
      "42": {
        "width": 655362,
        "height": 800996,
        "depth": 0
      },
      "43": {
        "width": 1019452,
        "height": 728180,
        "depth": 145636
      },
      "44": {
        "width": 364090,
        "height": 101944,
        "depth": 145636
      },
      "45": {
        "width": 436908,
        "height": 546134,
        "depth": 0
      },
      "46": {
        "width": 364090,
        "height": 101944,
        "depth": 0
      },
      "47": {
        "width": 655362,
        "height": 800996,
        "depth": 218452
      },
      "48": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "49": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "50": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "51": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "52": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "53": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "54": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "55": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "56": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "57": {
        "width": 655362,
        "height": 691768,
        "depth": 0
      },
      "58": {
        "width": 364090,
        "height": 546134,
        "depth": 0
      },
      "59": {
        "width": 364090,
        "height": 546134,
        "depth": 145636
      },
      "60": {
        "width": 371372,
        "height": 582542,
        "depth": 145636
      },
      "61": {
        "width": 1019452,
        "height": 450560,
        "depth": 4294835312
      },
      "62": {
        "width": 618954,
        "height": 582542,
        "depth": 145636
      },
      "63": {
        "width": 618954,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 873816,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "66": {
        "width": 859254,
        "height": 717256,
        "depth": 0
      },
      "67": {
        "width": 837408,
        "height": 717256,
        "depth": 0
      },
      "68": {
        "width": 932072,
        "height": 717256,
        "depth": 0
      },
      "69": {
        "width": 815562,
        "height": 717256,
        "depth": 0
      },
      "70": {
        "width": 779154,
        "height": 717256,
        "depth": 0
      },
      "71": {
        "width": 873816,
        "height": 717256,
        "depth": 0
      },
      "72": {
        "width": 851972,
        "height": 717256,
        "depth": 0
      },
      "73": {
        "width": 334964,
        "height": 717256,
        "depth": 0
      },
      "74": {
        "width": 604390,
        "height": 717256,
        "depth": 0
      },
      "75": {
        "width": 881100,
        "height": 717256,
        "depth": 0
      },
      "76": {
        "width": 742744,
        "height": 717256,
        "depth": 0
      },
      "77": {
        "width": 1070426,
        "height": 717256,
        "depth": 0
      },
      "78": {
        "width": 851972,
        "height": 717256,
        "depth": 0
      },
      "79": {
        "width": 1012170,
        "height": 717256,
        "depth": 0
      },
      "80": {
        "width": 822844,
        "height": 717256,
        "depth": 0
      },
      "81": {
        "width": 1012170,
        "height": 717256,
        "depth": 145636
      },
      "82": {
        "width": 855612,
        "height": 717256,
        "depth": 0
      },
      "83": {
        "width": 728180,
        "height": 717256,
        "depth": 0
      },
      "84": {
        "width": 939352,
        "height": 717256,
        "depth": 0
      },
      "85": {
        "width": 848332,
        "height": 717256,
        "depth": 0
      },
      "86": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "87": {
        "width": 1208780,
        "height": 717256,
        "depth": 0
      },
      "88": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "89": {
        "width": 844690,
        "height": 717256,
        "depth": 0
      },
      "90": {
        "width": 800998,
        "height": 717256,
        "depth": 0
      },
      "91": {
        "width": 369552,
        "height": 800996,
        "depth": 218452
      },
      "92": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 369552,
        "height": 800996,
        "depth": 218452
      },
      "94": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 364090,
        "height": 728178,
        "depth": 0
      },
      "96": {
        "width": 364090,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 618954,
        "height": 546134,
        "depth": 0
      },
      "98": {
        "width": 655364,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 582544,
        "height": 546134,
        "depth": 0
      },
      "100": {
        "width": 655364,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 582544,
        "height": 546134,
        "depth": 0
      },
      "102": {
        "width": 400500,
        "height": 728178,
        "depth": 0
      },
      "103": {
        "width": 655364,
        "height": 546134,
        "depth": 145636
      },
      "104": {
        "width": 655364,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 291274,
        "height": 728178,
        "depth": 0
      },
      "106": {
        "width": 327684,
        "height": 728178,
        "depth": 145636
      },
      "107": {
        "width": 618956,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 291274,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1019454,
        "height": 546134,
        "depth": 0
      },
      "110": {
        "width": 655364,
        "height": 546134,
        "depth": 0
      },
      "111": {
        "width": 655362,
        "height": 546134,
        "depth": 0
      },
      "112": {
        "width": 655364,
        "height": 546134,
        "depth": 145636
      },
      "113": {
        "width": 655364,
        "height": 546134,
        "depth": 145636
      },
      "114": {
        "width": 436910,
        "height": 546134,
        "depth": 0
      },
      "115": {
        "width": 502444,
        "height": 546134,
        "depth": 0
      },
      "116": {
        "width": 473316,
        "height": 702176,
        "depth": 0
      },
      "117": {
        "width": 655364,
        "height": 546134,
        "depth": 0
      },
      "118": {
        "width": 582546,
        "height": 546134,
        "depth": 0
      },
      "119": {
        "width": 873818,
        "height": 546134,
        "depth": 0
      },
      "120": {
        "width": 582546,
        "height": 546134,
        "depth": 0
      },
      "121": {
        "width": 582546,
        "height": 546134,
        "depth": 145636
      },
      "122": {
        "width": 564340,
        "height": 546134,
        "depth": 0
      },
      "123": {
        "width": 655362,
        "height": 546134,
        "depth": 0
      },
      "124": {
        "width": 1310724,
        "height": 546134,
        "depth": 0
      },
      "125": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 655362,
        "height": 728178,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmsy10": {
    "characters": {
      "0": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "1": {
        "width": 291272,
        "height": 466035,
        "depth": 4294909043
      },
      "2": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "3": {
        "width": 524290,
        "height": 487880,
        "depth": 4294930888
      },
      "4": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "5": {
        "width": 524290,
        "height": 466035,
        "depth": 4294909043
      },
      "6": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "7": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "8": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "9": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "10": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "11": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "12": {
        "width": 815562,
        "height": 611670,
        "depth": 87382
      },
      "13": {
        "width": 1048579,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 524290,
        "height": 466035,
        "depth": 4294909043
      },
      "15": {
        "width": 524290,
        "height": 466035,
        "depth": 4294909043
      },
      "16": {
        "width": 815562,
        "height": 486275,
        "depth": 4294929283
      },
      "17": {
        "width": 815562,
        "height": 486275,
        "depth": 4294929283
      },
      "18": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "19": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "20": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "21": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "22": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "23": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "24": {
        "width": 815562,
        "height": 384696,
        "depth": 4294827704
      },
      "25": {
        "width": 815562,
        "height": 506590,
        "depth": 4294949598
      },
      "26": {
        "width": 815562,
        "height": 565285,
        "depth": 40997
      },
      "27": {
        "width": 815562,
        "height": 565285,
        "depth": 40997
      },
      "28": {
        "width": 1048579,
        "height": 565285,
        "depth": 40997
      },
      "29": {
        "width": 1048579,
        "height": 565285,
        "depth": 40997
      },
      "30": {
        "width": 815562,
        "height": 565285,
        "depth": 40997
      },
      "31": {
        "width": 815562,
        "height": 565285,
        "depth": 40997
      },
      "32": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "33": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "34": {
        "width": 524290,
        "height": 728178,
        "depth": 203888
      },
      "35": {
        "width": 524290,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "37": {
        "width": 1048579,
        "height": 728178,
        "depth": 203888
      },
      "38": {
        "width": 1048579,
        "height": 728178,
        "depth": 203888
      },
      "39": {
        "width": 815562,
        "height": 486275,
        "depth": 4294929283
      },
      "40": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "41": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "42": {
        "width": 640798,
        "height": 728178,
        "depth": 203888
      },
      "43": {
        "width": 640798,
        "height": 728178,
        "depth": 203888
      },
      "44": {
        "width": 1048579,
        "height": 384696,
        "depth": 4294827704
      },
      "45": {
        "width": 1048579,
        "height": 728178,
        "depth": 203888
      },
      "46": {
        "width": 1048579,
        "height": 728178,
        "depth": 203888
      },
      "47": {
        "width": 815562,
        "height": 451470,
        "depth": 0
      },
      "48": {
        "width": 288358,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1048579,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 699053,
        "height": 565285,
        "depth": 40997
      },
      "51": {
        "width": 699053,
        "height": 565285,
        "depth": 40997
      },
      "52": {
        "width": 932070,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 932070,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203888
      },
      "55": {
        "width": 0,
        "height": 384696,
        "depth": 4294827704
      },
      "56": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 582544,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 699053,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 524290,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 757307,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 815562,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 640798,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 837258,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 688715,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 552106,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 808864,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 553419,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 753662,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 623762,
        "height": 716526,
        "depth": 101946
      },
      "72": {
        "width": 885541,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 570966,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 710704,
        "height": 716526,
        "depth": 101946
      },
      "75": {
        "width": 798963,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 723229,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1259235,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 860347,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 834786,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 729347,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 856341,
        "height": 716526,
        "depth": 101946
      },
      "82": {
        "width": 888672,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 634974,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 571101,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 656232,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 642549,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1035766,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 747946,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 700802,
        "height": 716526,
        "depth": 101946
      },
      "90": {
        "width": 759930,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 699053,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 699053,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 699053,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 699053,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 699053,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 640798,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 640798,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 466035,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 466035,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 466035,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 466035,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 407781,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 291272,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 640798,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 524290,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 291272,
        "height": 728178,
        "depth": 203888
      },
      "112": {
        "width": 873816,
        "height": 41942,
        "depth": 1006634
      },
      "113": {
        "width": 786434,
        "height": 716526,
        "depth": 0
      },
      "114": {
        "width": 873816,
        "height": 716526,
        "depth": 0
      },
      "115": {
        "width": 436909,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 699053,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 699053,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "119": {
        "width": 815562,
        "height": 666864,
        "depth": 142576
      },
      "120": {
        "width": 466037,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 466035,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 466035,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 640798,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 815562,
        "height": 728178,
        "depth": 135926
      },
      "125": {
        "width": 815562,
        "height": 728178,
        "depth": 135926
      },
      "126": {
        "width": 815562,
        "height": 728178,
        "depth": 135926
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmsy5": {
    "characters": {
      "0": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "1": {
        "width": 480602,
        "height": 517011,
        "depth": 4294960019
      },
      "2": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "3": {
        "width": 771878,
        "height": 487878,
        "depth": 4294930886
      },
      "4": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "5": {
        "width": 771878,
        "height": 517011,
        "depth": 4294960019
      },
      "6": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "7": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "8": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "9": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "10": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "11": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "12": {
        "width": 1135974,
        "height": 699059,
        "depth": 174771
      },
      "13": {
        "width": 1427251,
        "height": 728179,
        "depth": 203891
      },
      "14": {
        "width": 771878,
        "height": 517011,
        "depth": 4294960019
      },
      "15": {
        "width": 771878,
        "height": 517011,
        "depth": 4294960019
      },
      "16": {
        "width": 1135974,
        "height": 539814,
        "depth": 15526
      },
      "17": {
        "width": 1135974,
        "height": 539814,
        "depth": 15526
      },
      "18": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "19": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "20": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "21": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "22": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "23": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "24": {
        "width": 1135974,
        "height": 415661,
        "depth": 4294858669
      },
      "25": {
        "width": 1135974,
        "height": 564646,
        "depth": 40358
      },
      "26": {
        "width": 1135974,
        "height": 630106,
        "depth": 105818
      },
      "27": {
        "width": 1135974,
        "height": 630106,
        "depth": 105818
      },
      "28": {
        "width": 1427251,
        "height": 630106,
        "depth": 105818
      },
      "29": {
        "width": 1427251,
        "height": 630106,
        "depth": 105818
      },
      "30": {
        "width": 1135974,
        "height": 630106,
        "depth": 105818
      },
      "31": {
        "width": 1135974,
        "height": 630106,
        "depth": 105818
      },
      "32": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "33": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "34": {
        "width": 771878,
        "height": 728179,
        "depth": 203891
      },
      "35": {
        "width": 771878,
        "height": 728179,
        "depth": 203891
      },
      "36": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "37": {
        "width": 1427251,
        "height": 728179,
        "depth": 203891
      },
      "38": {
        "width": 1427251,
        "height": 728179,
        "depth": 203891
      },
      "39": {
        "width": 1135974,
        "height": 539814,
        "depth": 15526
      },
      "40": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "41": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "42": {
        "width": 917517,
        "height": 728179,
        "depth": 203891
      },
      "43": {
        "width": 917517,
        "height": 728179,
        "depth": 203891
      },
      "44": {
        "width": 1427251,
        "height": 415661,
        "depth": 4294858669
      },
      "45": {
        "width": 1427251,
        "height": 728179,
        "depth": 203891
      },
      "46": {
        "width": 1427251,
        "height": 728179,
        "depth": 203891
      },
      "47": {
        "width": 1135974,
        "height": 451469,
        "depth": 0
      },
      "48": {
        "width": 462397,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1427251,
        "height": 451469,
        "depth": 0
      },
      "50": {
        "width": 990336,
        "height": 630106,
        "depth": 105818
      },
      "51": {
        "width": 990336,
        "height": 630106,
        "depth": 105818
      },
      "52": {
        "width": 1281613,
        "height": 728179,
        "depth": 203891
      },
      "53": {
        "width": 1281613,
        "height": 728179,
        "depth": 203891
      },
      "54": {
        "width": 0,
        "height": 728179,
        "depth": 203891
      },
      "55": {
        "width": 0,
        "height": 415661,
        "depth": 4294858669
      },
      "56": {
        "width": 844698,
        "height": 728179,
        "depth": 0
      },
      "57": {
        "width": 844698,
        "height": 728179,
        "depth": 0
      },
      "58": {
        "width": 990336,
        "height": 451469,
        "depth": 0
      },
      "59": {
        "width": 771878,
        "height": 786432,
        "depth": 58253
      },
      "60": {
        "width": 1063155,
        "height": 728179,
        "depth": 0
      },
      "61": {
        "width": 1063155,
        "height": 728179,
        "depth": 0
      },
      "62": {
        "width": 1135974,
        "height": 728179,
        "depth": 0
      },
      "63": {
        "width": 1135974,
        "height": 728179,
        "depth": 0
      },
      "64": {
        "width": 917517,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1167574,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 963974,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 820083,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 1136560,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 808288,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 1036208,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 891741,
        "height": 716528,
        "depth": 101946
      },
      "72": {
        "width": 1252560,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 859338,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 1004899,
        "height": 716528,
        "depth": 101946
      },
      "75": {
        "width": 1106269,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 1011600,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1644022,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 1180758,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 1142090,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 1046118,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 1186947,
        "height": 716528,
        "depth": 101946
      },
      "82": {
        "width": 1254237,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 910234,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 862384,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 916202,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 942570,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1434099,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 1045053,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 974608,
        "height": 716528,
        "depth": 101946
      },
      "90": {
        "width": 1039562,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 990336,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 990336,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 990336,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 990336,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 990336,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 917517,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 917517,
        "height": 728179,
        "depth": 0
      },
      "98": {
        "width": 699059,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 699059,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 699059,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 699059,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 771878,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 771878,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 626240,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 626240,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 480602,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 771878,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 771878,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 917517,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 771878,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 480602,
        "height": 728179,
        "depth": 203891
      },
      "112": {
        "width": 1150541,
        "height": 58720,
        "depth": 989856
      },
      "113": {
        "width": 1086458,
        "height": 716528,
        "depth": 0
      },
      "114": {
        "width": 1208794,
        "height": 716528,
        "depth": 0
      },
      "115": {
        "width": 662650,
        "height": 728179,
        "depth": 203891
      },
      "116": {
        "width": 990336,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 990336,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "119": {
        "width": 1135974,
        "height": 754262,
        "depth": 229974
      },
      "120": {
        "width": 699059,
        "height": 728179,
        "depth": 203891
      },
      "121": {
        "width": 699059,
        "height": 728179,
        "depth": 203891
      },
      "122": {
        "width": 699059,
        "height": 728179,
        "depth": 203891
      },
      "123": {
        "width": 917517,
        "height": 728179,
        "depth": 203891
      },
      "124": {
        "width": 1135974,
        "height": 728179,
        "depth": 135926
      },
      "125": {
        "width": 1135974,
        "height": 728179,
        "depth": 135926
      },
      "126": {
        "width": 1135974,
        "height": 728179,
        "depth": 135926
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 5242880
  },
  "cmsy6": {
    "characters": {
      "0": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "1": {
        "width": 398069,
        "height": 500013,
        "depth": 4294943021
      },
      "2": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "3": {
        "width": 669920,
        "height": 487880,
        "depth": 4294930888
      },
      "4": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "5": {
        "width": 669920,
        "height": 500013,
        "depth": 4294943021
      },
      "6": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "7": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "8": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "9": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "10": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "11": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "12": {
        "width": 1009733,
        "height": 669920,
        "depth": 145632
      },
      "13": {
        "width": 1281584,
        "height": 728179,
        "depth": 203891
      },
      "14": {
        "width": 669920,
        "height": 500013,
        "depth": 4294943021
      },
      "15": {
        "width": 669920,
        "height": 500013,
        "depth": 4294943021
      },
      "16": {
        "width": 1009733,
        "height": 528512,
        "depth": 4224
      },
      "17": {
        "width": 1009733,
        "height": 528512,
        "depth": 4224
      },
      "18": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "19": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "20": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "21": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "22": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "23": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "24": {
        "width": 1009733,
        "height": 408872,
        "depth": 4294851880
      },
      "25": {
        "width": 1009733,
        "height": 552440,
        "depth": 28152
      },
      "26": {
        "width": 1009733,
        "height": 616549,
        "depth": 92261
      },
      "27": {
        "width": 1009733,
        "height": 616549,
        "depth": 92261
      },
      "28": {
        "width": 1281584,
        "height": 616549,
        "depth": 92261
      },
      "29": {
        "width": 1281584,
        "height": 616549,
        "depth": 92261
      },
      "30": {
        "width": 1009733,
        "height": 616549,
        "depth": 92261
      },
      "31": {
        "width": 1009733,
        "height": 616549,
        "depth": 92261
      },
      "32": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "33": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "34": {
        "width": 669920,
        "height": 728179,
        "depth": 203888
      },
      "35": {
        "width": 669920,
        "height": 728179,
        "depth": 203888
      },
      "36": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "37": {
        "width": 1281584,
        "height": 728179,
        "depth": 203888
      },
      "38": {
        "width": 1281584,
        "height": 728179,
        "depth": 203888
      },
      "39": {
        "width": 1009733,
        "height": 528512,
        "depth": 4224
      },
      "40": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "41": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "42": {
        "width": 805845,
        "height": 728179,
        "depth": 203888
      },
      "43": {
        "width": 805845,
        "height": 728179,
        "depth": 203888
      },
      "44": {
        "width": 1281584,
        "height": 408872,
        "depth": 4294851880
      },
      "45": {
        "width": 1281584,
        "height": 728179,
        "depth": 203888
      },
      "46": {
        "width": 1281584,
        "height": 728179,
        "depth": 203888
      },
      "47": {
        "width": 1009733,
        "height": 451469,
        "depth": 0
      },
      "48": {
        "width": 383507,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1281584,
        "height": 451469,
        "depth": 0
      },
      "50": {
        "width": 873808,
        "height": 616549,
        "depth": 92261
      },
      "51": {
        "width": 873808,
        "height": 616549,
        "depth": 92261
      },
      "52": {
        "width": 1145659,
        "height": 728179,
        "depth": 203891
      },
      "53": {
        "width": 1145659,
        "height": 728179,
        "depth": 203891
      },
      "54": {
        "width": 0,
        "height": 728179,
        "depth": 203888
      },
      "55": {
        "width": 0,
        "height": 408872,
        "depth": 4294851880
      },
      "56": {
        "width": 737883,
        "height": 728179,
        "depth": 0
      },
      "57": {
        "width": 737883,
        "height": 728179,
        "depth": 0
      },
      "58": {
        "width": 873808,
        "height": 451469,
        "depth": 0
      },
      "59": {
        "width": 669920,
        "height": 786432,
        "depth": 58253
      },
      "60": {
        "width": 941771,
        "height": 728179,
        "depth": 0
      },
      "61": {
        "width": 941771,
        "height": 728179,
        "depth": 0
      },
      "62": {
        "width": 1009733,
        "height": 728179,
        "depth": 0
      },
      "63": {
        "width": 1009733,
        "height": 728179,
        "depth": 0
      },
      "64": {
        "width": 805845,
        "height": 728179,
        "depth": 0
      },
      "65": {
        "width": 1038029,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 852789,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 711328,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 1007888,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 703901,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 922592,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 782979,
        "height": 716528,
        "depth": 101947
      },
      "72": {
        "width": 1110776,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 743779,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 887400,
        "height": 716528,
        "depth": 101947
      },
      "75": {
        "width": 984397,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 896043,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1496560,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 1054517,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 1020219,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 921091,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 1057307,
        "height": 716528,
        "depth": 101947
      },
      "82": {
        "width": 1112939,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 799048,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 745856,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 812053,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 823371,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1282117,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 926581,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 863909,
        "height": 716528,
        "depth": 101947
      },
      "90": {
        "width": 926917,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 873808,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 873808,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 873808,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 873808,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 873808,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 805845,
        "height": 728179,
        "depth": 0
      },
      "97": {
        "width": 805845,
        "height": 728179,
        "depth": 0
      },
      "98": {
        "width": 601957,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 601957,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 601957,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 601957,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 669920,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 669920,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 533995,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 533995,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 398069,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 669920,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 669920,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 805845,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 669920,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 398069,
        "height": 728179,
        "depth": 203888
      },
      "112": {
        "width": 1048568,
        "height": 54176,
        "depth": 994400
      },
      "113": {
        "width": 968955,
        "height": 716528,
        "depth": 0
      },
      "114": {
        "width": 1077696,
        "height": 716528,
        "depth": 0
      },
      "115": {
        "width": 567979,
        "height": 728179,
        "depth": 203891
      },
      "116": {
        "width": 873808,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 873808,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "119": {
        "width": 1009733,
        "height": 736189,
        "depth": 211901
      },
      "120": {
        "width": 601957,
        "height": 728179,
        "depth": 203891
      },
      "121": {
        "width": 601957,
        "height": 728179,
        "depth": 203891
      },
      "122": {
        "width": 601957,
        "height": 728179,
        "depth": 203891
      },
      "123": {
        "width": 805845,
        "height": 728179,
        "depth": 203891
      },
      "124": {
        "width": 1009733,
        "height": 728179,
        "depth": 135928
      },
      "125": {
        "width": 1009733,
        "height": 728179,
        "depth": 135928
      },
      "126": {
        "width": 1009733,
        "height": 728179,
        "depth": 135928
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 6291456
  },
  "cmsy7": {
    "characters": {
      "0": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "1": {
        "width": 355769,
        "height": 487881,
        "depth": 4294930889
      },
      "2": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "3": {
        "width": 613753,
        "height": 487879,
        "depth": 4294930887
      },
      "4": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "5": {
        "width": 613753,
        "height": 487881,
        "depth": 4294930889
      },
      "6": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "7": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "8": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "9": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "10": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "11": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "12": {
        "width": 936233,
        "height": 649120,
        "depth": 124832
      },
      "13": {
        "width": 1194217,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 613753,
        "height": 487881,
        "depth": 4294930889
      },
      "15": {
        "width": 613753,
        "height": 487881,
        "depth": 4294930889
      },
      "16": {
        "width": 936233,
        "height": 517861,
        "depth": 4294960869
      },
      "17": {
        "width": 936233,
        "height": 517861,
        "depth": 4294960869
      },
      "18": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "19": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "20": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "21": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "22": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "23": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "24": {
        "width": 936233,
        "height": 402736,
        "depth": 4294845744
      },
      "25": {
        "width": 936233,
        "height": 540885,
        "depth": 16597
      },
      "26": {
        "width": 936233,
        "height": 603639,
        "depth": 79351
      },
      "27": {
        "width": 936233,
        "height": 603639,
        "depth": 79351
      },
      "28": {
        "width": 1194217,
        "height": 603639,
        "depth": 79351
      },
      "29": {
        "width": 1194217,
        "height": 603639,
        "depth": 79351
      },
      "30": {
        "width": 936233,
        "height": 603639,
        "depth": 79351
      },
      "31": {
        "width": 936233,
        "height": 603639,
        "depth": 79351
      },
      "32": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "33": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "34": {
        "width": 613753,
        "height": 728178,
        "depth": 203890
      },
      "35": {
        "width": 613753,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "37": {
        "width": 1194217,
        "height": 728178,
        "depth": 203890
      },
      "38": {
        "width": 1194217,
        "height": 728178,
        "depth": 203890
      },
      "39": {
        "width": 936233,
        "height": 517861,
        "depth": 4294960869
      },
      "40": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "41": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "42": {
        "width": 742745,
        "height": 728178,
        "depth": 203890
      },
      "43": {
        "width": 742745,
        "height": 728178,
        "depth": 203890
      },
      "44": {
        "width": 1194217,
        "height": 402736,
        "depth": 4294845744
      },
      "45": {
        "width": 1194217,
        "height": 728178,
        "depth": 203890
      },
      "46": {
        "width": 1194217,
        "height": 728178,
        "depth": 203890
      },
      "47": {
        "width": 936233,
        "height": 451470,
        "depth": 0
      },
      "48": {
        "width": 345365,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1194217,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 807241,
        "height": 603639,
        "depth": 79351
      },
      "51": {
        "width": 807241,
        "height": 603639,
        "depth": 79351
      },
      "52": {
        "width": 1065225,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 1065225,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203890
      },
      "55": {
        "width": 0,
        "height": 402736,
        "depth": 4294845744
      },
      "56": {
        "width": 678249,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 678249,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 807241,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 613753,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 871737,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 871737,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 936233,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 936233,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 742745,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 962176,
        "height": 716528,
        "depth": 0
      },
      "66": {
        "width": 790037,
        "height": 716528,
        "depth": 0
      },
      "67": {
        "width": 650311,
        "height": 716528,
        "depth": 0
      },
      "68": {
        "width": 932656,
        "height": 716528,
        "depth": 0
      },
      "69": {
        "width": 646005,
        "height": 716528,
        "depth": 0
      },
      "70": {
        "width": 858107,
        "height": 716528,
        "depth": 0
      },
      "71": {
        "width": 721963,
        "height": 716528,
        "depth": 101945
      },
      "72": {
        "width": 1026183,
        "height": 716528,
        "depth": 0
      },
      "73": {
        "width": 677906,
        "height": 716528,
        "depth": 0
      },
      "74": {
        "width": 820142,
        "height": 716528,
        "depth": 101945
      },
      "75": {
        "width": 914021,
        "height": 716528,
        "depth": 0
      },
      "76": {
        "width": 830171,
        "height": 716528,
        "depth": 0
      },
      "77": {
        "width": 1407911,
        "height": 716528,
        "depth": 0
      },
      "78": {
        "width": 981017,
        "height": 716528,
        "depth": 0
      },
      "79": {
        "width": 949840,
        "height": 716528,
        "depth": 0
      },
      "80": {
        "width": 848457,
        "height": 716528,
        "depth": 0
      },
      "81": {
        "width": 981383,
        "height": 716528,
        "depth": 101945
      },
      "82": {
        "width": 1028690,
        "height": 716528,
        "depth": 0
      },
      "83": {
        "width": 736297,
        "height": 716528,
        "depth": 0
      },
      "84": {
        "width": 679289,
        "height": 716528,
        "depth": 0
      },
      "85": {
        "width": 752667,
        "height": 716528,
        "depth": 0
      },
      "86": {
        "width": 754898,
        "height": 716528,
        "depth": 0
      },
      "87": {
        "width": 1190247,
        "height": 716528,
        "depth": 0
      },
      "88": {
        "width": 858633,
        "height": 716528,
        "depth": 0
      },
      "89": {
        "width": 801502,
        "height": 716528,
        "depth": 101945
      },
      "90": {
        "width": 863127,
        "height": 716528,
        "depth": 0
      },
      "91": {
        "width": 807241,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 807241,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 807241,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 807241,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 807241,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 742745,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 742745,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 549257,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 549257,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 549257,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 549257,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 613753,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 613753,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 484761,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 484761,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 355769,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 613753,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 613753,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 742745,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 613753,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 355769,
        "height": 728178,
        "depth": 203890
      },
      "112": {
        "width": 984085,
        "height": 50930,
        "depth": 997646
      },
      "113": {
        "width": 900866,
        "height": 716528,
        "depth": 0
      },
      "114": {
        "width": 1000729,
        "height": 716528,
        "depth": 0
      },
      "115": {
        "width": 517531,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 807241,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 807241,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "119": {
        "width": 936233,
        "height": 718766,
        "depth": 194478
      },
      "120": {
        "width": 549259,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 549257,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 549257,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 742745,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 936233,
        "height": 728178,
        "depth": 135927
      },
      "125": {
        "width": 936233,
        "height": 728178,
        "depth": 135927
      },
      "126": {
        "width": 936233,
        "height": 728178,
        "depth": 135927
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmsy8": {
    "characters": {
      "0": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "1": {
        "width": 309480,
        "height": 478780,
        "depth": 4294921788
      },
      "2": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "3": {
        "width": 557064,
        "height": 487880,
        "depth": 4294930888
      },
      "4": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "5": {
        "width": 557064,
        "height": 478780,
        "depth": 4294921788
      },
      "6": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "7": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "8": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "9": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "10": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "11": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "12": {
        "width": 866544,
        "height": 633520,
        "depth": 109232
      },
      "13": {
        "width": 1114128,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 557064,
        "height": 478780,
        "depth": 4294921788
      },
      "15": {
        "width": 557064,
        "height": 478780,
        "depth": 4294921788
      },
      "16": {
        "width": 866544,
        "height": 506958,
        "depth": 4294949966
      },
      "17": {
        "width": 866544,
        "height": 506958,
        "depth": 4294949966
      },
      "18": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "19": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "20": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "21": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "22": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "23": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "24": {
        "width": 866544,
        "height": 396348,
        "depth": 4294839356
      },
      "25": {
        "width": 866544,
        "height": 529078,
        "depth": 4790
      },
      "26": {
        "width": 866544,
        "height": 590480,
        "depth": 66192
      },
      "27": {
        "width": 866544,
        "height": 590480,
        "depth": 66192
      },
      "28": {
        "width": 1114128,
        "height": 590480,
        "depth": 66192
      },
      "29": {
        "width": 1114128,
        "height": 590480,
        "depth": 66192
      },
      "30": {
        "width": 866544,
        "height": 590480,
        "depth": 66192
      },
      "31": {
        "width": 866544,
        "height": 590480,
        "depth": 66192
      },
      "32": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "33": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "34": {
        "width": 557064,
        "height": 728178,
        "depth": 203888
      },
      "35": {
        "width": 557064,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "37": {
        "width": 1114128,
        "height": 728178,
        "depth": 203888
      },
      "38": {
        "width": 1114128,
        "height": 728178,
        "depth": 203888
      },
      "39": {
        "width": 866544,
        "height": 506958,
        "depth": 4294949966
      },
      "40": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "41": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "42": {
        "width": 680856,
        "height": 728178,
        "depth": 203888
      },
      "43": {
        "width": 680856,
        "height": 728178,
        "depth": 203888
      },
      "44": {
        "width": 1114128,
        "height": 396348,
        "depth": 4294839356
      },
      "45": {
        "width": 1114128,
        "height": 728178,
        "depth": 203888
      },
      "46": {
        "width": 1114128,
        "height": 728178,
        "depth": 203888
      },
      "47": {
        "width": 866544,
        "height": 451470,
        "depth": 0
      },
      "48": {
        "width": 302194,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1114128,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 742752,
        "height": 590480,
        "depth": 66192
      },
      "51": {
        "width": 742752,
        "height": 590480,
        "depth": 66192
      },
      "52": {
        "width": 990336,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 990336,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203888
      },
      "55": {
        "width": 0,
        "height": 396348,
        "depth": 4294839356
      },
      "56": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 618960,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 742752,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 557064,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 804648,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 866544,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 680856,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 890716,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 728406,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 589978,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 861666,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 588012,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 795176,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 661632,
        "height": 716526,
        "depth": 101946
      },
      "72": {
        "width": 948174,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 613934,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 755132,
        "height": 716526,
        "depth": 101946
      },
      "75": {
        "width": 846668,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 766198,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1325946,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 911326,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 882490,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 779418,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 909872,
        "height": 716526,
        "depth": 101946
      },
      "82": {
        "width": 950942,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 674666,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 614800,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 695014,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 688068,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1105866,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 793100,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 740130,
        "height": 716526,
        "depth": 101946
      },
      "90": {
        "width": 800716,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 742752,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 742752,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 742752,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 742752,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 742752,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 680856,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 680856,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 495168,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 495168,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 495168,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 495168,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 433272,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 309480,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 680856,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 557064,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 309480,
        "height": 728178,
        "depth": 203888
      },
      "112": {
        "width": 928440,
        "height": 47186,
        "depth": 1001390
      },
      "113": {
        "width": 834504,
        "height": 716526,
        "depth": 0
      },
      "114": {
        "width": 928440,
        "height": 716526,
        "depth": 0
      },
      "115": {
        "width": 465130,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 742752,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 742752,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "119": {
        "width": 866544,
        "height": 701090,
        "depth": 176802
      },
      "120": {
        "width": 495168,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 495168,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 495168,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 680856,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 866544,
        "height": 728178,
        "depth": 135926
      },
      "125": {
        "width": 866544,
        "height": 728178,
        "depth": 135926
      },
      "126": {
        "width": 866544,
        "height": 728178,
        "depth": 135926
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmsy9": {
    "characters": {
      "0": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "1": {
        "width": 299360,
        "height": 471696,
        "depth": 4294914704
      },
      "2": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "3": {
        "width": 538848,
        "height": 487879,
        "depth": 4294930887
      },
      "4": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "5": {
        "width": 538848,
        "height": 471696,
        "depth": 4294914704
      },
      "6": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "7": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "8": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "9": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "10": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "11": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "12": {
        "width": 838208,
        "height": 621376,
        "depth": 97088
      },
      "13": {
        "width": 1077696,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 538848,
        "height": 471696,
        "depth": 4294914704
      },
      "15": {
        "width": 538848,
        "height": 471696,
        "depth": 4294914704
      },
      "16": {
        "width": 838208,
        "height": 496471,
        "depth": 4294939479
      },
      "17": {
        "width": 838208,
        "height": 496471,
        "depth": 4294939479
      },
      "18": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "19": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "20": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "21": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "22": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "23": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "24": {
        "width": 838208,
        "height": 390377,
        "depth": 4294833385
      },
      "25": {
        "width": 838208,
        "height": 517689,
        "depth": 4294960697
      },
      "26": {
        "width": 838208,
        "height": 577737,
        "depth": 53449
      },
      "27": {
        "width": 838208,
        "height": 577737,
        "depth": 53449
      },
      "28": {
        "width": 1077696,
        "height": 577737,
        "depth": 53449
      },
      "29": {
        "width": 1077696,
        "height": 577737,
        "depth": 53449
      },
      "30": {
        "width": 838208,
        "height": 577737,
        "depth": 53449
      },
      "31": {
        "width": 838208,
        "height": 577737,
        "depth": 53449
      },
      "32": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "33": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "34": {
        "width": 538848,
        "height": 728178,
        "depth": 203890
      },
      "35": {
        "width": 538848,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "37": {
        "width": 1077696,
        "height": 728178,
        "depth": 203890
      },
      "38": {
        "width": 1077696,
        "height": 728178,
        "depth": 203890
      },
      "39": {
        "width": 838208,
        "height": 496471,
        "depth": 4294939479
      },
      "40": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "41": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "42": {
        "width": 658592,
        "height": 728178,
        "depth": 203890
      },
      "43": {
        "width": 658592,
        "height": 728178,
        "depth": 203890
      },
      "44": {
        "width": 1077696,
        "height": 390377,
        "depth": 4294833385
      },
      "45": {
        "width": 1077696,
        "height": 728178,
        "depth": 203890
      },
      "46": {
        "width": 1077696,
        "height": 728178,
        "depth": 203890
      },
      "47": {
        "width": 838208,
        "height": 451470,
        "depth": 0
      },
      "48": {
        "width": 292889,
        "height": 582544,
        "depth": 0
      },
      "49": {
        "width": 1077696,
        "height": 451470,
        "depth": 0
      },
      "50": {
        "width": 718464,
        "height": 577737,
        "depth": 53449
      },
      "51": {
        "width": 718464,
        "height": 577737,
        "depth": 53449
      },
      "52": {
        "width": 957952,
        "height": 728178,
        "depth": 203890
      },
      "53": {
        "width": 957952,
        "height": 728178,
        "depth": 203890
      },
      "54": {
        "width": 0,
        "height": 728178,
        "depth": 203890
      },
      "55": {
        "width": 0,
        "height": 390377,
        "depth": 4294833385
      },
      "56": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "57": {
        "width": 598720,
        "height": 728178,
        "depth": 0
      },
      "58": {
        "width": 718464,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 538848,
        "height": 786432,
        "depth": 58254
      },
      "60": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "61": {
        "width": 778336,
        "height": 728178,
        "depth": 0
      },
      "62": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "63": {
        "width": 838208,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 658592,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 861003,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 706345,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 568928,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 832318,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 568784,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 772103,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 640583,
        "height": 716526,
        "depth": 101945
      },
      "72": {
        "width": 913360,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 590053,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 730437,
        "height": 716526,
        "depth": 101945
      },
      "75": {
        "width": 820153,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 742316,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1288868,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 882990,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 855975,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 751588,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 880117,
        "height": 716526,
        "depth": 101945
      },
      "82": {
        "width": 916331,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 652604,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 590512,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 673460,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 662768,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1066903,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 768004,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 718274,
        "height": 716526,
        "depth": 101945
      },
      "90": {
        "width": 778046,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 718464,
        "height": 582544,
        "depth": 0
      },
      "92": {
        "width": 718464,
        "height": 582544,
        "depth": 0
      },
      "93": {
        "width": 718464,
        "height": 582544,
        "depth": 0
      },
      "94": {
        "width": 718464,
        "height": 582544,
        "depth": 0
      },
      "95": {
        "width": 718464,
        "height": 582544,
        "depth": 0
      },
      "96": {
        "width": 658592,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 658592,
        "height": 728178,
        "depth": 0
      },
      "98": {
        "width": 478976,
        "height": 786432,
        "depth": 262144
      },
      "99": {
        "width": 478976,
        "height": 786432,
        "depth": 262144
      },
      "100": {
        "width": 478976,
        "height": 786432,
        "depth": 262144
      },
      "101": {
        "width": 478976,
        "height": 786432,
        "depth": 262144
      },
      "102": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "103": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "104": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "105": {
        "width": 419104,
        "height": 786432,
        "depth": 262144
      },
      "106": {
        "width": 299360,
        "height": 786432,
        "depth": 262144
      },
      "107": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "108": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "109": {
        "width": 658592,
        "height": 786432,
        "depth": 262144
      },
      "110": {
        "width": 538848,
        "height": 786432,
        "depth": 262144
      },
      "111": {
        "width": 299360,
        "height": 728178,
        "depth": 203890
      },
      "112": {
        "width": 898080,
        "height": 44274,
        "depth": 1004302
      },
      "113": {
        "width": 808110,
        "height": 716526,
        "depth": 0
      },
      "114": {
        "width": 898080,
        "height": 716526,
        "depth": 0
      },
      "115": {
        "width": 449042,
        "height": 728178,
        "depth": 203890
      },
      "116": {
        "width": 718464,
        "height": 582544,
        "depth": 0
      },
      "117": {
        "width": 718464,
        "height": 582544,
        "depth": 0
      },
      "118": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "119": {
        "width": 838208,
        "height": 683831,
        "depth": 159543
      },
      "120": {
        "width": 478976,
        "height": 728178,
        "depth": 203890
      },
      "121": {
        "width": 478976,
        "height": 728178,
        "depth": 203890
      },
      "122": {
        "width": 478976,
        "height": 728178,
        "depth": 203890
      },
      "123": {
        "width": 658592,
        "height": 728178,
        "depth": 203890
      },
      "124": {
        "width": 838208,
        "height": 728178,
        "depth": 135927
      },
      "125": {
        "width": 838208,
        "height": 728178,
        "depth": 135927
      },
      "126": {
        "width": 838208,
        "height": 728178,
        "depth": 135927
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmtcsc10": {
    "characters": {
      "0": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "1": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "2": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "3": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "4": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "5": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "6": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "7": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "8": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "9": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "10": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "11": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "12": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "13": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "14": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "15": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "16": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "17": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "18": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "19": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "20": {
        "width": 550498,
        "height": 593466,
        "depth": 0
      },
      "21": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "22": {
        "width": 550498,
        "height": 593027,
        "depth": 0
      },
      "23": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "24": {
        "width": 550498,
        "height": 0,
        "depth": 203891
      },
      "25": {
        "width": 1100995,
        "height": 495162,
        "depth": 0
      },
      "26": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "27": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "28": {
        "width": 550498,
        "height": 553416,
        "depth": 58254
      },
      "29": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "30": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "31": {
        "width": 550498,
        "height": 699051,
        "depth": 58254
      },
      "32": {
        "width": 550498,
        "height": 230104,
        "depth": 116509
      },
      "33": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "34": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "35": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "36": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "37": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "38": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "39": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "40": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "41": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "42": {
        "width": 550498,
        "height": 546134,
        "depth": 0
      },
      "43": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "44": {
        "width": 550498,
        "height": 131072,
        "depth": 145635
      },
      "45": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "46": {
        "width": 550498,
        "height": 131072,
        "depth": 0
      },
      "47": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "48": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "49": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "50": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "51": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "52": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "53": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "54": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "55": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "56": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "57": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "58": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 550498,
        "height": 451470,
        "depth": 145635
      },
      "60": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "61": {
        "width": 550498,
        "height": 435813,
        "depth": 4294762312
      },
      "62": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "63": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "64": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "65": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "66": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "67": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "68": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "69": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "70": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "71": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "72": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "73": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "74": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "75": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "76": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "77": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "78": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "79": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "80": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "81": {
        "width": 550498,
        "height": 640797,
        "depth": 145635
      },
      "82": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "83": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "84": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "85": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "86": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "87": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "88": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "89": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "90": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "91": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "92": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "93": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "94": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "95": {
        "width": 550498,
        "height": 0,
        "depth": 99757
      },
      "96": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "97": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "98": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "99": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "100": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "101": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "102": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "103": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "104": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "105": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "106": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "107": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "108": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "109": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "110": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "111": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "112": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "113": {
        "width": 550498,
        "height": 495162,
        "depth": 110683
      },
      "114": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "115": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "116": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "117": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "118": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "119": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "120": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "121": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "122": {
        "width": 550498,
        "height": 495162,
        "depth": 0
      },
      "123": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "124": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "125": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "126": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmtex10": {
    "characters": {
      "0": {
        "width": 550498,
        "height": 534482,
        "depth": 4294860981
      },
      "1": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "2": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "3": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "4": {
        "width": 550498,
        "height": 512640,
        "depth": 0
      },
      "5": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "6": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "7": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "8": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "9": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "10": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "11": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "12": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "13": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "14": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "15": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "16": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "17": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "18": {
        "width": 550498,
        "height": 512640,
        "depth": 0
      },
      "19": {
        "width": 550498,
        "height": 512640,
        "depth": 0
      },
      "20": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "21": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "22": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "23": {
        "width": 550498,
        "height": 628053,
        "depth": 4294954552
      },
      "24": {
        "width": 550498,
        "height": 435813,
        "depth": 4294762312
      },
      "25": {
        "width": 550498,
        "height": 435813,
        "depth": 4294762312
      },
      "26": {
        "width": 550498,
        "height": 672827,
        "depth": 32030
      },
      "27": {
        "width": 550498,
        "height": 534482,
        "depth": 4294860981
      },
      "28": {
        "width": 550498,
        "height": 661547,
        "depth": 20750
      },
      "29": {
        "width": 550498,
        "height": 661547,
        "depth": 20750
      },
      "30": {
        "width": 550498,
        "height": 514818,
        "depth": 4294841317
      },
      "31": {
        "width": 550498,
        "height": 512640,
        "depth": 0
      },
      "32": {
        "width": 550498,
        "height": 0,
        "depth": 0
      },
      "33": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "34": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "35": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "36": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "37": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "38": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "39": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "40": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "41": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "42": {
        "width": 550498,
        "height": 546134,
        "depth": 0
      },
      "43": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "44": {
        "width": 550498,
        "height": 131072,
        "depth": 145635
      },
      "45": {
        "width": 550498,
        "height": 556326,
        "depth": 4294882826
      },
      "46": {
        "width": 550498,
        "height": 131072,
        "depth": 0
      },
      "47": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "48": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "49": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "50": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "51": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "52": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "53": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "54": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "55": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "56": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "57": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "58": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 550498,
        "height": 451470,
        "depth": 145635
      },
      "60": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "61": {
        "width": 550498,
        "height": 435813,
        "depth": 4294762312
      },
      "62": {
        "width": 550498,
        "height": 582542,
        "depth": 4294909042
      },
      "63": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "64": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "65": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "66": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "67": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "68": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "69": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "70": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "71": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "72": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "73": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "74": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "75": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "76": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "77": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "78": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "79": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "80": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "81": {
        "width": 550498,
        "height": 640797,
        "depth": 145635
      },
      "82": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "83": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "84": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "85": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "86": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "87": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "88": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "89": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "90": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "91": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "92": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "93": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "94": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "95": {
        "width": 550498,
        "height": 0,
        "depth": 99757
      },
      "96": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "97": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "99": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "101": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "103": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "104": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "105": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "106": {
        "width": 550498,
        "height": 640797,
        "depth": 233018
      },
      "107": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "108": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "109": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "113": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "114": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 550498,
        "height": 580466,
        "depth": 0
      },
      "117": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "122": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "124": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "125": {
        "width": 550498,
        "height": 728178,
        "depth": 87379
      },
      "126": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmtex8": {
    "characters": {
      "0": {
        "width": 557064,
        "height": 537034,
        "depth": 4294863534
      },
      "1": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "2": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "3": {
        "width": 557064,
        "height": 640796,
        "depth": 233016
      },
      "4": {
        "width": 557064,
        "height": 512638,
        "depth": 0
      },
      "5": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "6": {
        "width": 557064,
        "height": 586184,
        "depth": 4294912684
      },
      "7": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "8": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "9": {
        "width": 557064,
        "height": 451470,
        "depth": 233016
      },
      "10": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "11": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "12": {
        "width": 557064,
        "height": 559140,
        "depth": 4294885640
      },
      "13": {
        "width": 557064,
        "height": 559140,
        "depth": 4294885640
      },
      "14": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "15": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "16": {
        "width": 557064,
        "height": 586184,
        "depth": 4294912684
      },
      "17": {
        "width": 557064,
        "height": 586184,
        "depth": 4294912684
      },
      "18": {
        "width": 557064,
        "height": 512638,
        "depth": 0
      },
      "19": {
        "width": 557064,
        "height": 512638,
        "depth": 0
      },
      "20": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "21": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "22": {
        "width": 557064,
        "height": 559140,
        "depth": 4294885640
      },
      "23": {
        "width": 557064,
        "height": 631694,
        "depth": 4294958194
      },
      "24": {
        "width": 557064,
        "height": 439454,
        "depth": 4294765954
      },
      "25": {
        "width": 557064,
        "height": 439454,
        "depth": 4294765954
      },
      "26": {
        "width": 557064,
        "height": 676468,
        "depth": 35672
      },
      "27": {
        "width": 557064,
        "height": 537034,
        "depth": 4294863534
      },
      "28": {
        "width": 557064,
        "height": 665188,
        "depth": 24392
      },
      "29": {
        "width": 557064,
        "height": 665188,
        "depth": 24392
      },
      "30": {
        "width": 557064,
        "height": 518458,
        "depth": 4294844958
      },
      "31": {
        "width": 557064,
        "height": 512638,
        "depth": 0
      },
      "32": {
        "width": 557064,
        "height": 0,
        "depth": 0
      },
      "33": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "34": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "35": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "36": {
        "width": 557064,
        "height": 728178,
        "depth": 87382
      },
      "37": {
        "width": 557064,
        "height": 728178,
        "depth": 87382
      },
      "38": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "39": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "40": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "41": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "42": {
        "width": 557064,
        "height": 546134,
        "depth": 0
      },
      "43": {
        "width": 557064,
        "height": 559140,
        "depth": 4294885640
      },
      "44": {
        "width": 557064,
        "height": 141086,
        "depth": 145636
      },
      "45": {
        "width": 557064,
        "height": 559140,
        "depth": 4294885640
      },
      "46": {
        "width": 557064,
        "height": 141086,
        "depth": 0
      },
      "47": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "48": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "49": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "50": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "51": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "52": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "53": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "54": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "55": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "56": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "57": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "58": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 557064,
        "height": 451470,
        "depth": 145636
      },
      "60": {
        "width": 557064,
        "height": 586184,
        "depth": 4294912684
      },
      "61": {
        "width": 557064,
        "height": 439454,
        "depth": 4294765954
      },
      "62": {
        "width": 557064,
        "height": 586184,
        "depth": 4294912684
      },
      "63": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "64": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "65": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "66": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "67": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "68": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "69": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "70": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "71": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "72": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "73": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "74": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "75": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "76": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "77": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "78": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "79": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "80": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "81": {
        "width": 557064,
        "height": 640796,
        "depth": 145636
      },
      "82": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "83": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "84": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "85": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "86": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "87": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "88": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "89": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "90": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "91": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "92": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "93": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "94": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "95": {
        "width": 557064,
        "height": 0,
        "depth": 103396
      },
      "96": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "97": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "99": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "101": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "103": {
        "width": 557064,
        "height": 451470,
        "depth": 233016
      },
      "104": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "105": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "106": {
        "width": 557064,
        "height": 640796,
        "depth": 233016
      },
      "107": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "108": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      },
      "109": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 557064,
        "height": 451470,
        "depth": 233016
      },
      "113": {
        "width": 557064,
        "height": 451470,
        "depth": 233016
      },
      "114": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 557064,
        "height": 580464,
        "depth": 0
      },
      "117": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 557064,
        "height": 451470,
        "depth": 233016
      },
      "122": {
        "width": 557064,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "124": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "125": {
        "width": 557064,
        "height": 728178,
        "depth": 87380
      },
      "126": {
        "width": 557064,
        "height": 640796,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmtex9": {
    "characters": {
      "0": {
        "width": 550496,
        "height": 534480,
        "depth": 4294860980
      },
      "1": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "2": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "3": {
        "width": 550496,
        "height": 640796,
        "depth": 233017
      },
      "4": {
        "width": 550496,
        "height": 512640,
        "depth": 0
      },
      "5": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "6": {
        "width": 550496,
        "height": 584971,
        "depth": 4294911470
      },
      "7": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "8": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "9": {
        "width": 550496,
        "height": 451470,
        "depth": 233017
      },
      "10": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "11": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "12": {
        "width": 550496,
        "height": 556325,
        "depth": 4294882825
      },
      "13": {
        "width": 550496,
        "height": 556325,
        "depth": 4294882825
      },
      "14": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "15": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "16": {
        "width": 550496,
        "height": 584971,
        "depth": 4294911470
      },
      "17": {
        "width": 550496,
        "height": 584971,
        "depth": 4294911470
      },
      "18": {
        "width": 550496,
        "height": 512640,
        "depth": 0
      },
      "19": {
        "width": 550496,
        "height": 512640,
        "depth": 0
      },
      "20": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "21": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "22": {
        "width": 550496,
        "height": 556325,
        "depth": 4294882825
      },
      "23": {
        "width": 550496,
        "height": 630482,
        "depth": 4294956981
      },
      "24": {
        "width": 550496,
        "height": 438242,
        "depth": 4294764741
      },
      "25": {
        "width": 550496,
        "height": 438242,
        "depth": 4294764741
      },
      "26": {
        "width": 550496,
        "height": 675260,
        "depth": 34464
      },
      "27": {
        "width": 550496,
        "height": 534480,
        "depth": 4294860980
      },
      "28": {
        "width": 550496,
        "height": 663977,
        "depth": 23180
      },
      "29": {
        "width": 550496,
        "height": 663977,
        "depth": 23180
      },
      "30": {
        "width": 550496,
        "height": 517248,
        "depth": 4294843748
      },
      "31": {
        "width": 550496,
        "height": 512640,
        "depth": 0
      },
      "32": {
        "width": 550496,
        "height": 0,
        "depth": 0
      },
      "33": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "34": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "35": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "36": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "37": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "38": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "39": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "40": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "41": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "42": {
        "width": 550496,
        "height": 546133,
        "depth": 0
      },
      "43": {
        "width": 550496,
        "height": 556325,
        "depth": 4294882825
      },
      "44": {
        "width": 550496,
        "height": 133500,
        "depth": 145636
      },
      "45": {
        "width": 550496,
        "height": 556325,
        "depth": 4294882825
      },
      "46": {
        "width": 550496,
        "height": 133500,
        "depth": 0
      },
      "47": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "48": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "49": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "50": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "51": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "52": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "53": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "54": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "55": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "56": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "57": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "58": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 550496,
        "height": 451470,
        "depth": 145636
      },
      "60": {
        "width": 550496,
        "height": 584971,
        "depth": 4294911470
      },
      "61": {
        "width": 550496,
        "height": 438242,
        "depth": 4294764741
      },
      "62": {
        "width": 550496,
        "height": 584971,
        "depth": 4294911470
      },
      "63": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "64": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "65": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "66": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "67": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "68": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "69": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "70": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "71": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "72": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "73": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "74": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "75": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "76": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "77": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "78": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "79": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "80": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "81": {
        "width": 550496,
        "height": 640796,
        "depth": 145636
      },
      "82": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "83": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "84": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "85": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "86": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "87": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "88": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "89": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "90": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "91": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "92": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "93": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "94": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "95": {
        "width": 550496,
        "height": 0,
        "depth": 102187
      },
      "96": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "97": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "99": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "101": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "103": {
        "width": 550496,
        "height": 451470,
        "depth": 233017
      },
      "104": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "105": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "106": {
        "width": 550496,
        "height": 640796,
        "depth": 233017
      },
      "107": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "108": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      },
      "109": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 550496,
        "height": 451470,
        "depth": 233017
      },
      "113": {
        "width": 550496,
        "height": 451470,
        "depth": 233017
      },
      "114": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 550496,
        "height": 580464,
        "depth": 0
      },
      "117": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 550496,
        "height": 451470,
        "depth": 233017
      },
      "122": {
        "width": 550496,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "124": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "125": {
        "width": 550496,
        "height": 728178,
        "depth": 87381
      },
      "126": {
        "width": 550496,
        "height": 640796,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmti10": {
    "characters": {
      "0": {
        "width": 657686,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 857498,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 803904,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 725843,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 696717,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 750310,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 803904,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 750310,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 803904,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 750310,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 643123,
        "height": 728178,
        "depth": 203890
      },
      "12": {
        "width": 589530,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 616326,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 924490,
        "height": 728178,
        "depth": 203890
      },
      "15": {
        "width": 937888,
        "height": 728178,
        "depth": 203890
      },
      "16": {
        "width": 321562,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 348358,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 535936,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 535936,
        "height": 588949,
        "depth": 0
      },
      "23": {
        "width": 871672,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 482342,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 562733,
        "height": 728178,
        "depth": 203890
      },
      "26": {
        "width": 750310,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 750310,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 535936,
        "height": 553416,
        "depth": 101946
      },
      "29": {
        "width": 925654,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1032842,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 803904,
        "height": 767499,
        "depth": 50973
      },
      "32": {
        "width": 267968,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 321562,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 539432,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 857498,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 806453,
        "height": 728178,
        "depth": 0
      },
      "37": {
        "width": 857498,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 803904,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 321562,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 428749,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 428749,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 535936,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 803904,
        "height": 588949,
        "depth": 59418
      },
      "44": {
        "width": 321562,
        "height": 110683,
        "depth": 203890
      },
      "45": {
        "width": 375155,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 321562,
        "height": 110683,
        "depth": 0
      },
      "47": {
        "width": 535936,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 535936,
        "height": 675749,
        "depth": 203890
      },
      "53": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 535936,
        "height": 675749,
        "depth": 203890
      },
      "56": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 535936,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 321562,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 321562,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 321562,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 803904,
        "height": 384696,
        "depth": 4294827704
      },
      "62": {
        "width": 535936,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 803904,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 738077,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 750310,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 791670,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 711280,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 684483,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 811186,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 404282,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 550499,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 806234,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 657686,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 940218,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 803904,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 711280,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 803904,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 764874,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 589530,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 750310,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1047405,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 779437,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 643123,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 321562,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 539432,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 321562,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 321562,
        "height": 700301,
        "depth": 0
      },
      "96": {
        "width": 321562,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 535936,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 482342,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 482342,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 482342,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 321562,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 482342,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 321562,
        "height": 687194,
        "depth": 0
      },
      "106": {
        "width": 321562,
        "height": 687194,
        "depth": 203890
      },
      "107": {
        "width": 482342,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 267968,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 857498,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 589530,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 535936,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 535936,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 482342,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 442147,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 428749,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 348358,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 562733,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 482342,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 696717,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 486421,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 509139,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 428749,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 535936,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1071872,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 535936,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 535936,
        "height": 700301,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 10485760
  },
  "cmti12": {
    "characters": {
      "0": {
        "width": 643231,
        "height": 716527,
        "depth": 0
      },
      "1": {
        "width": 838869,
        "height": 716527,
        "depth": 0
      },
      "2": {
        "width": 786440,
        "height": 716527,
        "depth": 0
      },
      "3": {
        "width": 709739,
        "height": 716527,
        "depth": 0
      },
      "4": {
        "width": 681581,
        "height": 716527,
        "depth": 0
      },
      "5": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "6": {
        "width": 734011,
        "height": 716527,
        "depth": 0
      },
      "7": {
        "width": 786440,
        "height": 716527,
        "depth": 0
      },
      "8": {
        "width": 734011,
        "height": 716527,
        "depth": 0
      },
      "9": {
        "width": 786440,
        "height": 716527,
        "depth": 0
      },
      "10": {
        "width": 734011,
        "height": 716527,
        "depth": 0
      },
      "11": {
        "width": 629152,
        "height": 728177,
        "depth": 203889
      },
      "12": {
        "width": 576724,
        "height": 728177,
        "depth": 203889
      },
      "13": {
        "width": 602939,
        "height": 728177,
        "depth": 203889
      },
      "14": {
        "width": 904408,
        "height": 728177,
        "depth": 203889
      },
      "15": {
        "width": 917515,
        "height": 728177,
        "depth": 203889
      },
      "16": {
        "width": 314576,
        "height": 451471,
        "depth": 0
      },
      "17": {
        "width": 340791,
        "height": 451471,
        "depth": 203889
      },
      "18": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "19": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "20": {
        "width": 524293,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "22": {
        "width": 524293,
        "height": 591280,
        "depth": 0
      },
      "23": {
        "width": 854404,
        "height": 728177,
        "depth": 0
      },
      "24": {
        "width": 471864,
        "height": 0,
        "depth": 178403
      },
      "25": {
        "width": 550508,
        "height": 728177,
        "depth": 203889
      },
      "26": {
        "width": 734011,
        "height": 451471,
        "depth": 0
      },
      "27": {
        "width": 734011,
        "height": 451471,
        "depth": 0
      },
      "28": {
        "width": 524293,
        "height": 553416,
        "depth": 101945
      },
      "29": {
        "width": 905377,
        "height": 716527,
        "depth": 0
      },
      "30": {
        "width": 1010236,
        "height": 716527,
        "depth": 0
      },
      "31": {
        "width": 786440,
        "height": 767499,
        "depth": 50972
      },
      "32": {
        "width": 262147,
        "height": 451471,
        "depth": 0
      },
      "33": {
        "width": 314576,
        "height": 728177,
        "depth": 0
      },
      "34": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "35": {
        "width": 838869,
        "height": 728177,
        "depth": 203888
      },
      "36": {
        "width": 791900,
        "height": 728177,
        "depth": 0
      },
      "37": {
        "width": 838869,
        "height": 786432,
        "depth": 58255
      },
      "38": {
        "width": 786440,
        "height": 728177,
        "depth": 0
      },
      "39": {
        "width": 314576,
        "height": 728177,
        "depth": 0
      },
      "40": {
        "width": 419435,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 419435,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 524293,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 786440,
        "height": 576720,
        "depth": 52432
      },
      "44": {
        "width": 314576,
        "height": 101945,
        "depth": 203889
      },
      "45": {
        "width": 367005,
        "height": 451471,
        "depth": 0
      },
      "46": {
        "width": 314576,
        "height": 101945,
        "depth": 0
      },
      "47": {
        "width": 524293,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "49": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "50": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "51": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "52": {
        "width": 524293,
        "height": 679772,
        "depth": 203889
      },
      "53": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "54": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "55": {
        "width": 524293,
        "height": 679772,
        "depth": 203889
      },
      "56": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "57": {
        "width": 524293,
        "height": 679772,
        "depth": 0
      },
      "58": {
        "width": 314576,
        "height": 451471,
        "depth": 0
      },
      "59": {
        "width": 314576,
        "height": 451471,
        "depth": 203889
      },
      "60": {
        "width": 314576,
        "height": 524288,
        "depth": 203889
      },
      "61": {
        "width": 786440,
        "height": 378433,
        "depth": 4294821441
      },
      "62": {
        "width": 524293,
        "height": 524288,
        "depth": 203889
      },
      "63": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "64": {
        "width": 786440,
        "height": 728177,
        "depth": 0
      },
      "65": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "66": {
        "width": 721875,
        "height": 716527,
        "depth": 0
      },
      "67": {
        "width": 734011,
        "height": 716527,
        "depth": 0
      },
      "68": {
        "width": 774304,
        "height": 716527,
        "depth": 0
      },
      "69": {
        "width": 695660,
        "height": 716527,
        "depth": 0
      },
      "70": {
        "width": 669445,
        "height": 716527,
        "depth": 0
      },
      "71": {
        "width": 793480,
        "height": 716527,
        "depth": 0
      },
      "72": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "73": {
        "width": 395163,
        "height": 716527,
        "depth": 0
      },
      "74": {
        "width": 538372,
        "height": 716527,
        "depth": 0
      },
      "75": {
        "width": 788383,
        "height": 716527,
        "depth": 0
      },
      "76": {
        "width": 643231,
        "height": 716527,
        "depth": 0
      },
      "77": {
        "width": 919456,
        "height": 716527,
        "depth": 0
      },
      "78": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "79": {
        "width": 786440,
        "height": 716527,
        "depth": 0
      },
      "80": {
        "width": 695660,
        "height": 716527,
        "depth": 0
      },
      "81": {
        "width": 786440,
        "height": 716527,
        "depth": 203889
      },
      "82": {
        "width": 748089,
        "height": 716527,
        "depth": 0
      },
      "83": {
        "width": 576723,
        "height": 716527,
        "depth": 0
      },
      "84": {
        "width": 734011,
        "height": 716527,
        "depth": 0
      },
      "85": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "86": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "87": {
        "width": 1024315,
        "height": 716527,
        "depth": 0
      },
      "88": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "89": {
        "width": 762168,
        "height": 716527,
        "depth": 0
      },
      "90": {
        "width": 629152,
        "height": 716527,
        "depth": 0
      },
      "91": {
        "width": 314576,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "93": {
        "width": 314576,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "95": {
        "width": 314576,
        "height": 695932,
        "depth": 0
      },
      "96": {
        "width": 314576,
        "height": 728177,
        "depth": 0
      },
      "97": {
        "width": 524293,
        "height": 451471,
        "depth": 0
      },
      "98": {
        "width": 471864,
        "height": 728177,
        "depth": 0
      },
      "99": {
        "width": 471864,
        "height": 451471,
        "depth": 0
      },
      "100": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "101": {
        "width": 471864,
        "height": 451471,
        "depth": 0
      },
      "102": {
        "width": 314576,
        "height": 728177,
        "depth": 203889
      },
      "103": {
        "width": 471864,
        "height": 451471,
        "depth": 203889
      },
      "104": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "105": {
        "width": 314576,
        "height": 679772,
        "depth": 0
      },
      "106": {
        "width": 314576,
        "height": 679772,
        "depth": 203889
      },
      "107": {
        "width": 471864,
        "height": 728177,
        "depth": 0
      },
      "108": {
        "width": 262147,
        "height": 728177,
        "depth": 0
      },
      "109": {
        "width": 838869,
        "height": 451471,
        "depth": 0
      },
      "110": {
        "width": 576723,
        "height": 451471,
        "depth": 0
      },
      "111": {
        "width": 524293,
        "height": 451471,
        "depth": 0
      },
      "112": {
        "width": 524293,
        "height": 451471,
        "depth": 203889
      },
      "113": {
        "width": 471864,
        "height": 451471,
        "depth": 203889
      },
      "114": {
        "width": 432543,
        "height": 451471,
        "depth": 0
      },
      "115": {
        "width": 419436,
        "height": 451471,
        "depth": 0
      },
      "116": {
        "width": 340791,
        "height": 644959,
        "depth": 0
      },
      "117": {
        "width": 550508,
        "height": 451471,
        "depth": 0
      },
      "118": {
        "width": 471864,
        "height": 451471,
        "depth": 0
      },
      "119": {
        "width": 681581,
        "height": 451471,
        "depth": 0
      },
      "120": {
        "width": 471864,
        "height": 451471,
        "depth": 0
      },
      "121": {
        "width": 498079,
        "height": 451471,
        "depth": 203889
      },
      "122": {
        "width": 419435,
        "height": 451471,
        "depth": 0
      },
      "123": {
        "width": 524293,
        "height": 451471,
        "depth": 0
      },
      "124": {
        "width": 1048587,
        "height": 451471,
        "depth": 0
      },
      "125": {
        "width": 524293,
        "height": 728177,
        "depth": 0
      },
      "126": {
        "width": 524293,
        "height": 695932,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 12582912
  },
  "cmti7": {
    "characters": {
      "0": {
        "width": 770215,
        "height": 710910,
        "depth": 0
      },
      "1": {
        "width": 1001986,
        "height": 710910,
        "depth": 0
      },
      "2": {
        "width": 940402,
        "height": 710910,
        "depth": 0
      },
      "3": {
        "width": 846363,
        "height": 710910,
        "depth": 0
      },
      "4": {
        "width": 817234,
        "height": 710910,
        "depth": 0
      },
      "5": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "6": {
        "width": 878818,
        "height": 710910,
        "depth": 0
      },
      "7": {
        "width": 940402,
        "height": 710910,
        "depth": 0
      },
      "8": {
        "width": 878818,
        "height": 710910,
        "depth": 0
      },
      "9": {
        "width": 940402,
        "height": 710910,
        "depth": 0
      },
      "10": {
        "width": 878818,
        "height": 710910,
        "depth": 0
      },
      "11": {
        "width": 772295,
        "height": 728178,
        "depth": 203890
      },
      "12": {
        "width": 710713,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 741504,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 1112254,
        "height": 728178,
        "depth": 203890
      },
      "15": {
        "width": 1127650,
        "height": 728178,
        "depth": 203890
      },
      "16": {
        "width": 386146,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 416939,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 632482,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 632482,
        "height": 600848,
        "depth": 0
      },
      "23": {
        "width": 1000183,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 570898,
        "height": 0,
        "depth": 178405
      },
      "25": {
        "width": 663278,
        "height": 728178,
        "depth": 203890
      },
      "26": {
        "width": 878818,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 878818,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 632482,
        "height": 553415,
        "depth": 101945
      },
      "29": {
        "width": 1078135,
        "height": 710910,
        "depth": 0
      },
      "30": {
        "width": 1201303,
        "height": 710910,
        "depth": 0
      },
      "31": {
        "width": 940402,
        "height": 767502,
        "depth": 50974
      },
      "32": {
        "width": 324562,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 386146,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 1001986,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 922976,
        "height": 728178,
        "depth": 0
      },
      "37": {
        "width": 1001986,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 940402,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 386146,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 509314,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 509314,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 632482,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 940402,
        "height": 631648,
        "depth": 107360
      },
      "44": {
        "width": 386146,
        "height": 120670,
        "depth": 203890
      },
      "45": {
        "width": 447730,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 386146,
        "height": 120670,
        "depth": 0
      },
      "47": {
        "width": 632482,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 632482,
        "height": 675749,
        "depth": 203890
      },
      "53": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 632482,
        "height": 675749,
        "depth": 203890
      },
      "56": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 632482,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 386146,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 386146,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 386146,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 940402,
        "height": 402736,
        "depth": 4294845744
      },
      "62": {
        "width": 632482,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 940402,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "66": {
        "width": 862592,
        "height": 710910,
        "depth": 0
      },
      "67": {
        "width": 878818,
        "height": 710910,
        "depth": 0
      },
      "68": {
        "width": 924176,
        "height": 710910,
        "depth": 0
      },
      "69": {
        "width": 831799,
        "height": 710910,
        "depth": 0
      },
      "70": {
        "width": 801008,
        "height": 710910,
        "depth": 0
      },
      "71": {
        "width": 947685,
        "height": 710910,
        "depth": 0
      },
      "72": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "73": {
        "width": 476859,
        "height": 710910,
        "depth": 0
      },
      "74": {
        "width": 647047,
        "height": 710910,
        "depth": 0
      },
      "75": {
        "width": 938741,
        "height": 710910,
        "depth": 0
      },
      "76": {
        "width": 770215,
        "height": 710910,
        "depth": 0
      },
      "77": {
        "width": 1092699,
        "height": 710910,
        "depth": 0
      },
      "78": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "79": {
        "width": 940402,
        "height": 710910,
        "depth": 0
      },
      "80": {
        "width": 831799,
        "height": 710910,
        "depth": 0
      },
      "81": {
        "width": 940402,
        "height": 710910,
        "depth": 203890
      },
      "82": {
        "width": 893383,
        "height": 710910,
        "depth": 0
      },
      "83": {
        "width": 694066,
        "height": 710910,
        "depth": 0
      },
      "84": {
        "width": 878818,
        "height": 710910,
        "depth": 0
      },
      "85": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "86": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "87": {
        "width": 1215867,
        "height": 710910,
        "depth": 0
      },
      "88": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "89": {
        "width": 907947,
        "height": 710910,
        "depth": 0
      },
      "90": {
        "width": 755650,
        "height": 710910,
        "depth": 0
      },
      "91": {
        "width": 386146,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 386146,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 386146,
        "height": 710910,
        "depth": 0
      },
      "96": {
        "width": 386146,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 632482,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 570898,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 570898,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 570898,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 386149,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 570898,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 386146,
        "height": 688649,
        "depth": 0
      },
      "106": {
        "width": 386146,
        "height": 688649,
        "depth": 203890
      },
      "107": {
        "width": 570898,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 324562,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 1001986,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 694066,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 632482,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 632482,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 570898,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 524711,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 509314,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 416939,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 663275,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 570898,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 817234,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 570901,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 601691,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 509317,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 632482,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1264965,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 632482,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 632482,
        "height": 710910,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 7340032
  },
  "cmti8": {
    "characters": {
      "0": {
        "width": 705242,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 920418,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 862892,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 777696,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 747840,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 805366,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 862892,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 805366,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 862892,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 805366,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 690318,
        "height": 728178,
        "depth": 203890
      },
      "12": {
        "width": 632794,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 661556,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 992332,
        "height": 728178,
        "depth": 203890
      },
      "15": {
        "width": 1006714,
        "height": 728178,
        "depth": 203890
      },
      "16": {
        "width": 345158,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 373922,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 575262,
        "height": 659002,
        "depth": 0
      },
      "21": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 575262,
        "height": 603658,
        "depth": 0
      },
      "23": {
        "width": 927458,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 517736,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 604028,
        "height": 728178,
        "depth": 203890
      },
      "26": {
        "width": 805366,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 805366,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 575262,
        "height": 553416,
        "depth": 101946
      },
      "29": {
        "width": 992872,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1107924,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 862892,
        "height": 767498,
        "depth": 50972
      },
      "32": {
        "width": 287632,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 345158,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 576718,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 920418,
        "height": 728178,
        "depth": 203888
      },
      "36": {
        "width": 855610,
        "height": 728178,
        "depth": 0
      },
      "37": {
        "width": 920418,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 862892,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 345158,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 460210,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 460210,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 575262,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 862892,
        "height": 603658,
        "depth": 83012
      },
      "44": {
        "width": 345158,
        "height": 116508,
        "depth": 203890
      },
      "45": {
        "width": 402684,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 345158,
        "height": 116508,
        "depth": 0
      },
      "47": {
        "width": 575262,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "49": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "50": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "51": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "52": {
        "width": 575262,
        "height": 675750,
        "depth": 203890
      },
      "53": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "54": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "55": {
        "width": 575262,
        "height": 675750,
        "depth": 203890
      },
      "56": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "57": {
        "width": 575262,
        "height": 675750,
        "depth": 0
      },
      "58": {
        "width": 345158,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 345158,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 345158,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 862892,
        "height": 396348,
        "depth": 4294839356
      },
      "62": {
        "width": 575262,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 862892,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 791532,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 805366,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 849058,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 762768,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 734006,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 870356,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 432540,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 590190,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 863986,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 705242,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 1007800,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 862892,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 762768,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 862892,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 820294,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 632788,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 805366,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1122852,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 835222,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 690314,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 345158,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 576718,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 345158,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 345158,
        "height": 703212,
        "depth": 0
      },
      "96": {
        "width": 345158,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 575262,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 517736,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 517736,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 517736,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 345160,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 517736,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 345158,
        "height": 688650,
        "depth": 0
      },
      "106": {
        "width": 345158,
        "height": 688650,
        "depth": 203890
      },
      "107": {
        "width": 517736,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 287632,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 920418,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 632788,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 575262,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 575262,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 517736,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 474592,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 460210,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 373922,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 604026,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 517736,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 747840,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 518830,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 546500,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 460212,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 575262,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1150524,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 575262,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 575262,
        "height": 703212,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 8388608
  },
  "cmti9": {
    "characters": {
      "0": {
        "width": 675111,
        "height": 716526,
        "depth": 0
      },
      "1": {
        "width": 880299,
        "height": 716526,
        "depth": 0
      },
      "2": {
        "width": 825280,
        "height": 716526,
        "depth": 0
      },
      "3": {
        "width": 745017,
        "height": 716526,
        "depth": 0
      },
      "4": {
        "width": 715243,
        "height": 716526,
        "depth": 0
      },
      "5": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "6": {
        "width": 770261,
        "height": 716526,
        "depth": 0
      },
      "7": {
        "width": 825280,
        "height": 716526,
        "depth": 0
      },
      "8": {
        "width": 770261,
        "height": 716526,
        "depth": 0
      },
      "9": {
        "width": 825280,
        "height": 716526,
        "depth": 0
      },
      "10": {
        "width": 770261,
        "height": 716526,
        "depth": 0
      },
      "11": {
        "width": 660224,
        "height": 728178,
        "depth": 203890
      },
      "12": {
        "width": 605205,
        "height": 728178,
        "depth": 203890
      },
      "13": {
        "width": 632715,
        "height": 728178,
        "depth": 203890
      },
      "14": {
        "width": 949072,
        "height": 728178,
        "depth": 203890
      },
      "15": {
        "width": 962827,
        "height": 728178,
        "depth": 203890
      },
      "16": {
        "width": 330112,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 357621,
        "height": 451470,
        "depth": 203890
      },
      "18": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "19": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "20": {
        "width": 550187,
        "height": 659001,
        "depth": 0
      },
      "21": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "22": {
        "width": 550187,
        "height": 594679,
        "depth": 0
      },
      "23": {
        "width": 892272,
        "height": 728178,
        "depth": 0
      },
      "24": {
        "width": 495168,
        "height": 0,
        "depth": 178404
      },
      "25": {
        "width": 577696,
        "height": 728178,
        "depth": 203890
      },
      "26": {
        "width": 770261,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 770261,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 550187,
        "height": 553415,
        "depth": 101945
      },
      "29": {
        "width": 950204,
        "height": 716526,
        "depth": 0
      },
      "30": {
        "width": 1060242,
        "height": 716526,
        "depth": 0
      },
      "31": {
        "width": 825280,
        "height": 767499,
        "depth": 50972
      },
      "32": {
        "width": 275093,
        "height": 451470,
        "depth": 0
      },
      "33": {
        "width": 330112,
        "height": 728178,
        "depth": 0
      },
      "34": {
        "width": 553422,
        "height": 728178,
        "depth": 0
      },
      "35": {
        "width": 880299,
        "height": 728178,
        "depth": 203890
      },
      "36": {
        "width": 824267,
        "height": 728178,
        "depth": 0
      },
      "37": {
        "width": 880299,
        "height": 786432,
        "depth": 58254
      },
      "38": {
        "width": 825280,
        "height": 728178,
        "depth": 0
      },
      "39": {
        "width": 330112,
        "height": 728178,
        "depth": 0
      },
      "40": {
        "width": 440149,
        "height": 786432,
        "depth": 262144
      },
      "41": {
        "width": 440149,
        "height": 786432,
        "depth": 262144
      },
      "42": {
        "width": 550187,
        "height": 786432,
        "depth": 0
      },
      "43": {
        "width": 825280,
        "height": 594679,
        "depth": 67968
      },
      "44": {
        "width": 330112,
        "height": 113273,
        "depth": 203890
      },
      "45": {
        "width": 385131,
        "height": 451470,
        "depth": 0
      },
      "46": {
        "width": 330112,
        "height": 113273,
        "depth": 0
      },
      "47": {
        "width": 550187,
        "height": 786432,
        "depth": 262144
      },
      "48": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "49": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "50": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "51": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "52": {
        "width": 550187,
        "height": 675749,
        "depth": 203890
      },
      "53": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "54": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "55": {
        "width": 550187,
        "height": 675749,
        "depth": 203890
      },
      "56": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "57": {
        "width": 550187,
        "height": 675749,
        "depth": 0
      },
      "58": {
        "width": 330112,
        "height": 451470,
        "depth": 0
      },
      "59": {
        "width": 330112,
        "height": 451470,
        "depth": 203890
      },
      "60": {
        "width": 330112,
        "height": 524288,
        "depth": 203890
      },
      "61": {
        "width": 825280,
        "height": 390377,
        "depth": 4294833385
      },
      "62": {
        "width": 550187,
        "height": 524288,
        "depth": 203890
      },
      "63": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "64": {
        "width": 825280,
        "height": 728178,
        "depth": 0
      },
      "65": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "66": {
        "width": 757639,
        "height": 716526,
        "depth": 0
      },
      "67": {
        "width": 770261,
        "height": 716526,
        "depth": 0
      },
      "68": {
        "width": 812658,
        "height": 716526,
        "depth": 0
      },
      "69": {
        "width": 730130,
        "height": 716526,
        "depth": 0
      },
      "70": {
        "width": 702620,
        "height": 716526,
        "depth": 0
      },
      "71": {
        "width": 832724,
        "height": 716526,
        "depth": 0
      },
      "72": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "73": {
        "width": 414905,
        "height": 716526,
        "depth": 0
      },
      "74": {
        "width": 565074,
        "height": 716526,
        "depth": 0
      },
      "75": {
        "width": 827545,
        "height": 716526,
        "depth": 0
      },
      "76": {
        "width": 675111,
        "height": 716526,
        "depth": 0
      },
      "77": {
        "width": 965092,
        "height": 716526,
        "depth": 0
      },
      "78": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "79": {
        "width": 825280,
        "height": 716526,
        "depth": 0
      },
      "80": {
        "width": 730130,
        "height": 716526,
        "depth": 0
      },
      "81": {
        "width": 825280,
        "height": 716526,
        "depth": 203890
      },
      "82": {
        "width": 785148,
        "height": 716526,
        "depth": 0
      },
      "83": {
        "width": 605205,
        "height": 716526,
        "depth": 0
      },
      "84": {
        "width": 770261,
        "height": 716526,
        "depth": 0
      },
      "85": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "86": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "87": {
        "width": 1075129,
        "height": 716526,
        "depth": 0
      },
      "88": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "89": {
        "width": 800036,
        "height": 716526,
        "depth": 0
      },
      "90": {
        "width": 660224,
        "height": 716526,
        "depth": 0
      },
      "91": {
        "width": 330112,
        "height": 786432,
        "depth": 262144
      },
      "92": {
        "width": 553422,
        "height": 728178,
        "depth": 0
      },
      "93": {
        "width": 330112,
        "height": 786432,
        "depth": 262144
      },
      "94": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "95": {
        "width": 330112,
        "height": 701595,
        "depth": 0
      },
      "96": {
        "width": 330112,
        "height": 728178,
        "depth": 0
      },
      "97": {
        "width": 550187,
        "height": 451470,
        "depth": 0
      },
      "98": {
        "width": 495168,
        "height": 728178,
        "depth": 0
      },
      "99": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "100": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "101": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "102": {
        "width": 330112,
        "height": 728178,
        "depth": 203890
      },
      "103": {
        "width": 495168,
        "height": 451470,
        "depth": 203890
      },
      "104": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "105": {
        "width": 330112,
        "height": 687031,
        "depth": 0
      },
      "106": {
        "width": 330112,
        "height": 687031,
        "depth": 203890
      },
      "107": {
        "width": 495168,
        "height": 728178,
        "depth": 0
      },
      "108": {
        "width": 275093,
        "height": 728178,
        "depth": 0
      },
      "109": {
        "width": 880299,
        "height": 451470,
        "depth": 0
      },
      "110": {
        "width": 605205,
        "height": 451470,
        "depth": 0
      },
      "111": {
        "width": 550187,
        "height": 451470,
        "depth": 0
      },
      "112": {
        "width": 550187,
        "height": 451470,
        "depth": 203890
      },
      "113": {
        "width": 495168,
        "height": 451470,
        "depth": 203890
      },
      "114": {
        "width": 453904,
        "height": 451470,
        "depth": 0
      },
      "115": {
        "width": 440149,
        "height": 451470,
        "depth": 0
      },
      "116": {
        "width": 357621,
        "height": 644958,
        "depth": 0
      },
      "117": {
        "width": 577696,
        "height": 451470,
        "depth": 0
      },
      "118": {
        "width": 495168,
        "height": 451470,
        "depth": 0
      },
      "119": {
        "width": 715243,
        "height": 451470,
        "depth": 0
      },
      "120": {
        "width": 496786,
        "height": 451470,
        "depth": 0
      },
      "121": {
        "width": 522677,
        "height": 451470,
        "depth": 203890
      },
      "122": {
        "width": 440149,
        "height": 451470,
        "depth": 0
      },
      "123": {
        "width": 550187,
        "height": 451470,
        "depth": 0
      },
      "124": {
        "width": 1100373,
        "height": 451470,
        "depth": 0
      },
      "125": {
        "width": 550187,
        "height": 728178,
        "depth": 0
      },
      "126": {
        "width": 550187,
        "height": 701595,
        "depth": 0
      }
    },
    "smallest_character_code": 0,
    "largest_character_code": 127,
    "design_size": 9437184
  },
  "cmtt10": {
    "characters": {
      "0": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "1": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "2": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "3": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "4": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "5": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "6": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "7": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "8": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "9": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "10": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "11": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "12": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "13": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "14": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "15": {
        "width": 550498,
        "height": 407779,
        "depth": 233018
      },
      "16": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "17": {
        "width": 550498,
        "height": 451470,
        "depth": 233018
      },
      "18": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "19": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "20": {
        "width": 550498,
        "height": 593466,
        "depth": 0
      },
      "21": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "22": {
        "width": 550498,
        "height": 593027,
        "depth": 0
      },
      "23": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "24": {
        "width": 550498,
        "height": 0,
        "depth": 203891
      },
      "25": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "26": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "27": {
        "width": 550498,
        "height": 451470,
        "depth": 0
      },
      "28": {
        "width": 550498,
        "height": 567979,
        "depth": 116509
      },
      "29": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "30": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "31": {
        "width": 550498,
        "height": 699051,
        "depth": 58254
      },
      "32": {
        "width": 550498,
        "height": 230104,
        "depth": 116509
      },
      "33": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "34": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "35": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "36": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "37": {
        "width": 550498,
        "height": 728178,
        "depth": 87381
      },
      "38": {
        "width": 550498,
        "height": 640797,
        "depth": 0
      },
      "39": {
        "width": 550498,
        "depth": 0