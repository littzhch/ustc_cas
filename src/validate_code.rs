use bytes::Bytes;
use image::io::Reader as ImageReader;
use image::GrayImage;
use std::io::Cursor;
use std::string::String;

pub fn get_validatecode(raw_img: Bytes) -> String {
    let img = ImageReader::new(Cursor::new(raw_img))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let num1 = img.crop_imm(28, 4, 15, 21).into_luma8();
    let num2 = img.crop_imm(49, 4, 15, 21).into_luma8();
    let num3 = img.crop_imm(70, 4, 15, 21).into_luma8();
    let num4 = img.crop_imm(91, 4, 15, 21).into_luma8();
    let mut result = String::new();
    result += &BinaryPixels::from(&num1).get_num();
    result += &BinaryPixels::from(&num2).get_num();
    result += &BinaryPixels::from(&num3).get_num();
    result += &BinaryPixels::from(&num4).get_num();
    result
}

struct BinaryPixels([bool; 15 * 21]);

impl From<&GrayImage> for BinaryPixels {
    fn from(img: &GrayImage) -> Self {
        let mut count = 0;
        let mut arr = [false; 15 * 21];

        for pix in img.pixels() {
            if pix.0[0] < 128 {
                arr[count] = false;
            } else {
                arr[count] = true;
            }
            count += 1;
            if count > 255 {
                break;
            }
        }

        Self(arr)
    }
}

impl BinaryPixels {
    fn compare(&self, other: &Self) -> i32 {
        let mut result = 0;
        for i in 0..256 {
            if self.0[i] == other.0[i] {
                result += 1;
            }
        }
        result
    }

    fn get_num(&self) -> String {
        let mut max_cmp = 0;
        let mut max_pos = 0;
        let mut current_cmp;
        for (i, num) in NUMS.iter().enumerate() {
            current_cmp = self.compare(num);
            if current_cmp > max_cmp {
                max_cmp = current_cmp;
                max_pos = i;
            }
        }
        max_pos.to_string()
    }
}

const NUMS: [BinaryPixels; 10] = [
    NUM_0, NUM_1, NUM_2, NUM_3, NUM_4, NUM_5, NUM_6, NUM_7, NUM_8, NUM_9,
];

const NUM_0: BinaryPixels = BinaryPixels([
    true, true, true, true, true, false, false, false, false, false, false, true, true, true, true,
    true, true, true, false, false, false, false, false, false, false, false, false, false, true,
    true, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, true, true, true, false,
    false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, false, false, false, false, false, false, false, false, false, true, true, true, true,
    true, true, false, false, false, false, false, false, false, false, true, true, true, true,
    true, true, true, true, false, false, false, false, false, false, false, true, true, true,
    true, true, true, true, true, false, false, false, false, false, false, false, true, true,
    true, true, true, true, true, false, false, false, false, false, false, false, false, true,
    true, true, true, true, true, true, true, false, false, false, false, false, false, false,
    true, true, false, true, true, true, true, true, false, false, false, false, false, false,
    false, true, true, true, true, true, true, true, true, false, false, false, false, false,
    false, false, true, true, true, true, true, true, true, true, false, false, false, false,
    false, false, false, true, true, true, true, true, true, true, true, false, false, false,
    false, false, false, false, false, true, true, true, true, true, true, false, false, false,
    false, false, false, false, false, false, true, true, true, true, true, true, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false,
]);

const NUM_1: BinaryPixels = BinaryPixels([
    true, true, true, true, false, false, false, false, false, false, false, false, true, true,
    true, true, true, false, false, false, false, false, false, false, false, false, false, true,
    true, true, true, true, false, false, false, false, false, false, false, false, false, false,
    true, true, true, true, true, false, false, false, false, false, false, false, false, false,
    false, true, true, true, true, true, false, false, false, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, false, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, true, true, true, true, true, true, false, false, false,
    false, false, true, true, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false,
]);

const NUM_2: BinaryPixels = BinaryPixels([
    true, true, false, false, false, false, false, false, false, false, false, true, true, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, true, true, true,
    true, false, false, false, false, false, false, false, true, true, true, true, true, true,
    true, true, true, false, false, false, false, false, true, true, true, true, false, true, true,
    true, true, true, true, false, false, false, false, true, true, true, true, true, true, true,
    true, true, true, true, false, false, false, false, true, true, true, true, true, true, true,
    true, true, true, true, false, false, false, false, true, true, true, true, true, true, true,
    true, true, true, false, false, false, false, false, true, true, true, true, true, true, true,
    true, true, false, false, false, false, false, false, true, true, true, true, false, true,
    true, true, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, true, true, true, true, true,
    true, false, false, false, false, false, false, false, false, true, true, true, true, true,
    false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false,
]);

const NUM_3: BinaryPixels = BinaryPixels([
    true, true, true, false, false, false, false, false, false, false, false, true, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false,
    true, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, false, false, true, true, true, true, true, true, false,
    false, false, false, false, false, true, true, true, true, true, true, true, true, true, true,
    false, false, false, false, false, true, true, true, true, true, true, true, true, true, true,
    false, false, false, false, false, true, true, true, true, false, true, true, true, true, true,
    false, false, false, false, false, true, true, true, true, true, true, true, true, true, false,
    false, false, false, false, true, true, true, true, true, false, false, false, false, false,
    false, false, false, false, false, true, true, true, true, true, false, false, false, false,
    false, false, false, false, true, true, true, true, true, true, true, false, false, false,
    false, false, false, false, false, false, false, true, true, true, true, true, false, false,
    false, false, false, false, false, false, false, false, false, true, true, true, false, true,
    true, true, true, true, false, false, false, false, false, false, true, true, true, true, true,
    true, true, true, true, true, true, false, false, false, false, true, true, true, true, true,
    true, true, true, true, true, true, false, false, false, false, false, true, true, true, true,
    true, true, true, true, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false,
]);

const NUM_4: BinaryPixels = BinaryPixels([
    true, true, true, true, true, true, true, false, false, false, false, false, false, false,
    true, true, true, true, true, true, true, true, false, false, false, false, false, false,
    false, true, true, true, true, true, true, true, false, false, false, false, false, false,
    false, false, true, true, true, true, true, true, false, false, false, false, false, false,
    false, false, false, true, true, true, true, true, true, false, false, false, true, false,
    false, false, false, false, true, true, true, true, true, false, false, false, false, true,
    false, false, false, false, false, true, true, true, true, false, false, false, false, true,
    true, false, false, false, false, false, true, true, true, false, false, false, false, true,
    true, true, false, false, false, false, false, true, true, true, false, false, false, false,
    true, true, true, false, false, false, false, false, true, true, false, false, false, false,
    true, true, true, true, false, false, false, false, false, true, true, false, false, false,
    true, true, true, true, true, false, false, false, false, false, true, false, false, false,
    false, true, true, true, true, true, false, false, false, false, false, true, false, false,
    false, true, true, true, true, true, true, false, false, false, false, false, true, false,
    false, true, true, true, true, true, true, true, false, false, false, false, false, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false,
]);

const NUM_5: BinaryPixels = BinaryPixels([
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, true, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, true, false, false, false, false, false, true, true, true,
    true, true, true, true, true, true, true, false, false, false, false, false, true, true, true,
    true, false, true, true, true, true, true, false, false, false, false, false, true, true, true,
    true, true, true, true, true, true, true, false, false, false, false, false, false, false,
    false, false, false, false, true, true, true, true, false, false, false, false, false, false,
    false, false, false, false, false, false, true, true, true, false, false, false, false, false,
    false, false, false, false, false, false, false, false, true, true, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, false, false,
    false, true, true, true, true, true, false, false, false, false, false, false, true, false,
    true, true, true, true, true, true, true, true, false, false, false, false, false, true, true,
    true, true, true, true, true, true, true, true, true, false, false, false, false, false, true,
    true, true, true, true, true, true, true, true, true, false, false, false, false, true, true,
    true, true, true, true, true, true, true, true, true, false, false, false, false, false, true,
    true, true, true, false, true, true, true, true, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false,
]);

const NUM_6: BinaryPixels = BinaryPixels([
    true, true, true, true, true, true, true, false, false, false, false, false, false, true, true,
    true, true, true, true, true, false, false, false, false, false, false, false, false, false,
    false, true, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, true, false, false, false, false, false, false, true, true, true,
    true, true, false, false, true, false, false, false, false, false, true, true, true, true,
    true, true, true, true, true, true, false, false, false, false, false, true, true, true, true,
    true, true, true, true, true, false, false, false, false, false, true, true, false, false,
    false, false, false, false, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, true, true, true, true, false, false, false, false, false,
    false, false, false, false, true, true, true, true, true, true, true, false, false, false,
    false, false, false, false, false, true, true, true, true, true, true, true, false, false,
    false, false, false, false, false, false, true, true, true, true, true, true, true, false,
    false, false, false, false, false, false, false, true, true, true, true, true, true, true,
    false, false, false, true, false, false, false, false, true, true, true, true, true, true,
    true, false, false, false, true, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
]);

const NUM_7: BinaryPixels = BinaryPixels([
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, true, true, true, true, true, true,
    true, true, true, true, false, false, false, false, false, true, true, true, true, true, true,
    true, true, true, true, false, false, false, false, false, true, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, true, true, false, false, false, false, false, true, true, true, true, true, false,
    true, true, true, false, false, false, false, false, false, true, true, true, true, true, true,
    true, true, true, false, false, false, false, false, true, true, true, true, true, true, true,
    true, true, false, false, false, false, false, false, true, true, true, true, true, true, true,
    true, true, false, false, false, false, false, true, true, true, true, true, true, true, true,
    true, false, false, false, false, false, false, true, true, true, true, true, true, true, true,
    true, false, false, false, false, false, true, true, true, true, true, true, true, true, true,
    true, false, false, false, false, false, true, true, true, true, true, true, true, true, true,
    false, false, false, false, false, false, true, true, true, true, true, true, true, true, true,
    false, false, false, false, false, true, true, true, true, true, true, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false,
]);

const NUM_8: BinaryPixels = BinaryPixels([
    true, true, true, true, true, false, false, false, false, false, false, false, true, true,
    true, true, true, true, false, false, false, false, false, false, false, false, false, false,
    false, true, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, true, false, false, false, false, false, false, true, true,
    true, false, false, false, false, false, true, false, false, false, false, false, true, true,
    true, true, true, false, false, false, false, true, false, false, false, false, false, true,
    true, true, true, true, false, false, false, false, true, false, false, false, false, false,
    false, true, true, true, false, false, false, false, false, true, true, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, true, true, false,
    false, false, false, false, false, false, false, false, false, false, true, true, true, true,
    false, false, false, false, false, false, false, false, false, false, false, true, true, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, false, false, true, true, true, true, true, false, false, false,
    false, false, false, false, false, false, true, true, true, true, true, true, true, false,
    false, false, false, false, false, false, false, true, true, true, true, true, true, true,
    false, false, false, false, false, false, false, false, true, true, true, true, true, true,
    true, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false,
]);

const NUM_9: BinaryPixels = BinaryPixels([
    true, true, true, true, true, false, false, false, false, false, false, true, true, true, true,
    true, true, true, false, false, false, false, false, false, false, false, false, false, true,
    true, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, false, false, false, false, false, true, true, true, true, true,
    false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, false, false, false, false, false, false, false, false, true, true, true, true,
    true, true, true, false, false, false, false, false, false, false, false, true, true, true,
    true, true, true, true, false, false, false, false, false, false, false, false, true, true,
    false, true, true, true, true, false, false, false, false, false, false, false, false, true,
    true, true, true, false, true, true, false, false, false, false, false, false, false, false,
    false, true, true, true, true, true, false, false, false, false, true, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, true, true, false,
    false, false, false, false, false, false, false, false, false, false, false, false, true, true,
    true, false, false, false, false, false, false, false, false, false, false, false, false, true,
    true, true, true, false, false, false, false, false, false, true, true, false, false, false,
    true, true, true, true, true, true, true, true, true, true, true, false, false, false, false,
    true, false, true, true, true, true, true, true, true, true, true, false, false, false, false,
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false,
]);
