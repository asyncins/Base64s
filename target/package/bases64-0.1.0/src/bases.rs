use std::collections::HashMap;

pub struct Bases{}

impl Bases{
    fn standard_code(&self) -> HashMap<i32, char>{
        //
        let standard: HashMap<i32, char> = [
            (0, 'A'),
            (1, 'B'),
            (2, 'C'),
            (3, 'D'),
            (4, 'E'),
            (5, 'F'),
            (6, 'G'),
            (7, 'H'),
            (8, 'I'),
            (9, 'J'),
            (10, 'K'),
            (11, 'L'),
            (12, 'M'),
            (13, 'N'),
            (14, 'O'),
            (15, 'P'),
            (16, 'Q'),
            (17, 'R'),
            (18, 'S'),
            (19, 'T'),
            (20, 'U'),
            (21, 'V'),
            (22, 'W'),
            (23, 'X'),
            (24, 'Y'),
            (25, 'Z'),
            (26, 'a'),
            (27, 'b'),
            (28, 'c'),
            (29, 'd'),
            (30, 'e'),
            (31, 'f'),
            (32, 'g'),
            (33, 'h'),
            (34, 'i'),
            (35, 'j'),
            (36, 'k'),
            (37, 'l'),
            (38, 'm'),
            (39, 'n'),
            (40, 'o'),
            (41, 'p'),
            (42, 'q'),
            (43, 'r'),
            (44, 's'),
            (45, 't'),
            (46, 'u'),
            (47, 'v'),
            (48, 'w'),
            (49, 'x'),
            (50, 'y'),
            (51, 'z'),
            (52, '0'),
            (53, '1'),
            (54, '2'),
            (55, '3'),
            (56, '4'),
            (57, '5'),
            (58, '6'),
            (59, '7'),
            (60, '8'),
            (61, '9'),
            (62, '+'),
            (63, '/'),
            (65, '=')
        ].iter().cloned().collect();
        standard
    }

    fn standard_code_contrary(&self) -> HashMap<char, i32>{
        //
        let standard: HashMap<char, i32> = [
            ('A', 0),
            ('B', 1),
            ('C', 2),
            ('D', 3),
            ('E', 4),
            ('F', 5),
            ('G', 6),
            ('H', 7),
            ('I', 8),
            ('J', 9),
            ('K', 10),
            ('L', 11),
            ('M', 12),
            ('N', 13),
            ('O', 14),
            ('P', 15),
            ('Q', 16),
            ('R', 17),
            ('S', 18),
            ('T', 19),
            ('U', 20),
            ('V', 21),
            ('W', 22),
            ('X', 23),
            ('Y', 24),
            ('Z', 25),
            ('a', 26),
            ('b', 27),
            ('c', 28),
            ('d', 29),
            ('e', 30),
            ('f', 31),
            ('g', 32),
            ('h', 33),
            ('i', 34),
            ('j', 35),
            ('k', 36),
            ('l', 37),
            ('m', 38),
            ('n', 39),
            ('o', 40),
            ('p', 41),
            ('q', 42),
            ('r', 43),
            ('s', 44),
            ('t', 45),
            ('u', 46),
            ('v', 47),
            ('w', 48),
            ('x', 49),
            ('y', 50),
            ('z', 51),
            ('0', 52),
            ('1', 53),
            ('2', 54),
            ('3', 55),
            ('4', 56),
            ('5', 57),
            ('6', 58),
            ('7', 59),
            ('8', 60),
            ('9', 61),
            ('+', 62),
            ('/', 63),
            ('=', 65),
        ].iter().cloned().collect();
        standard
    }

    fn standard_code_url(&self) -> HashMap<i32, char>{
        // for url
        let mut standard = self.standard_code();
        standard.insert(62, '-');
        standard.insert(63, '_');
        standard
    }

    fn standard_code_contrary_url(&self) -> HashMap<char, i32>{
        // for url
        let mut standard = self.standard_code_contrary();
        standard.remove(&'+');
        standard.remove(&'/');
        standard.insert('-', 62);
        standard.insert('_', 63);
        standard
    }

    pub fn encode(&self, value: &str) -> String{
        let value_bytes = value.as_bytes();
        let base_encode_value = self.binary_trans(value_bytes, false);
        let mut result = String::from("");
        for i in base_encode_value{
            result.push(i);
        }
        result
    }

    pub fn encode_url(&self, value: &str) -> String{
        // encode for url
        let value_bytes = value.as_bytes();
        let base_encode_value = self.binary_trans(value_bytes, true);
        let mut result = String::from("");
        for i in base_encode_value{
            result.push(i);
        }
        result
    }

    pub fn decode(&self, value: String) -> String{
        //
        let mut char_vec = self.contrary_mapping(value, false);
        let mut binary_vec: Vec<Vec<i32>> = Vec::new();
        let mut binary_vec_vec: Vec<Vec<i32>> = Vec::new();
        let mut pad_number: Vec<i32> = Vec::new();
        for i in char_vec{
            if i == 65 as i32{
                pad_number.push(1);
            }
            let chars = self.dec2binary(i, 6);
            binary_vec.push(chars);
        }
        for i in pad_number{
            binary_vec.pop();
        }
        let binary_eight = self.binary_split(binary_vec, 8);
        let original_vec = self.binary2dec(binary_eight, 2);
        let original_result = String::from_utf8(original_vec);
        let original = match original_result {
            Ok(res) => res,
            Err(err) => {
                println!("Error: vec trans to String error.");
                "".to_string()
            }
        };
        original
    }

    pub fn decode_url(&self, value: String) -> String{
        //
        let mut char_vec = self.contrary_mapping(value, true);
        let mut binary_vec: Vec<Vec<i32>> = Vec::new();
        let mut binary_vec_vec: Vec<Vec<i32>> = Vec::new();
        let mut pad_number: Vec<i32> = Vec::new();
        for i in char_vec{
            if i == 65 as i32{
                pad_number.push(1);
            }
            let chars = self.dec2binary(i, 6);
            binary_vec.push(chars);
        }
        for i in pad_number{
            binary_vec.pop();
        }
        let binary_eight = self.binary_split(binary_vec, 8);
        let original_vec = self.binary2dec(binary_eight, 2);
        let original_result = String::from_utf8(original_vec);
        let original = match original_result {
            Ok(res) => res,
            Err(err) => {
                println!("Error: vec trans to String error.");
                "".to_string()
            }
        };
        original
    }

    fn contrary_mapping(&self, value: String, url: bool) -> Vec<i32>{
        // Mapping, String trans to vec
        let mut standard = self.standard_code_contrary();
        if url{
            standard = self.standard_code_contrary_url();
        }
        let mut standard_code_vec: Vec<i32> = Vec::new();
        for i in value.chars(){
            let res = match standard.get(&i) {
                Some(r) => *r,
                None => 10000
            };
            standard_code_vec.push(res);
        }
        standard_code_vec
    }

    fn binary_trans(&self, value: &[u8], url: bool) -> Vec<char>{
        //
        let mut binary_all_vec: Vec<Vec<i32>> = Vec::new();
        for i in value{
            let binary_sign_vec = self.dec2binary(*i as i32, 8);
            binary_all_vec.push(binary_sign_vec);
        }
        let (binary_full, padding_number) = self.padding(binary_all_vec);
        let binary_six = self.binary_split(binary_full,  6);
        let base64_code = self.binary2dec(binary_six, padding_number);
        let base_encode_value = self.mapping(base64_code, url);
        base_encode_value
    }

    fn dec2binary(&self, value: i32, number: i32) -> Vec<i32>{
        // ASCII value to binary
        let mut binary: Vec<i32> = Vec::new();
        let mut val = value;
        for i in 0..number{
            let result = self.mods(&mut val) as i32;
            // 如果有余数就减1，取除以2的商作为下一次取余的被除数。Python中可用地板除
            if result == 1{
                val = (val -1) / 2;
            }else{
                val = val / 2;
            }
            binary.push(result);
        }
        binary.reverse();
        binary
    }

    fn mods(&self, value: &mut i32) -> i32{
        // mods for desc to binary, so % 2
        *value % 2
    }

    fn padding(&self, mut value: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32){
        // 不足24则补0
        let mut length: i32 = 0;
        for i in 0..value.len(){
            length += value[i].len() as i32;
        }
        let remainder = length % 24;
        if remainder > 0{
            for i in 0..24 - remainder{
                value.push(vec![0]);
            }
        }

        // 是否补0会影响后续计算
        if remainder == 0{
            return (value, remainder)
        }else{
            return (value, 24 - remainder)
        }

    }

    fn binary_split(&self, value: Vec<Vec<i32>>, number: usize) -> Vec<String>{
        // Split the binary, before number trans to after number
        // [[1, 1, 1, 0, 0, 1], [0, 0, 1, 0, 1, 1], [1, 1, 0, 1, 1, 0]]
        let mut value_string = String::from("");
        for vec32 in value.iter(){
            for v in vec32{
                value_string.push_str(&v.to_string());
            }
        }
        let mut binary_vec: Vec<String> = Vec::new();
        let group_number = value_string.len() / number; // 72/8 or 72/6
        for i in 1..group_number + 1{
            binary_vec.push(value_string[(i-1)*number..i*number].to_string());
        }
        binary_vec
    }

    fn binary2dec(&self, value: Vec<String>, padding_number: i32) -> Vec<u8>{
        // binary trans to desc
        let mut pad_number = padding_number / 8; // 根据补0数计算补位数
        // String trans to Vec<i32>, because String can not slice or sort.
        let mut binary_six_vec: Vec<Vec<i32>> = Vec::new();
        for binary in value{
            let mut six_vec: Vec<i32> = Vec::new();
            // String can not for,but String.chars() or String.bytes() can for
            for i in binary.chars(){
                let mut x: i32 = 0;
                if i == '1'{
                    x = 1;
                }
                six_vec.push(x);
            }
            binary_six_vec.push(six_vec);
        }

        // algorithm for binary 011000 trans to desc:
        // 0*2.pow(5)+1*2.pow(4)+1*2.pow(3)+0*2.pow(2)+0*2.pow(1)
        const BINARY: i32 = 2;
        let mut desc_vec: Vec<u8> = Vec::new();
        for six_vec in 0..binary_six_vec.len(){
            binary_six_vec[six_vec].reverse(); // 按照计算公式，此处需要反序
            let mut desc = 0;
            for i in 0..binary_six_vec[six_vec].len(){
                desc += binary_six_vec[six_vec][i] * BINARY.pow(i as u32);
            }
            desc_vec.push(desc as u8);
        }

        // Make up according to the number of padding number
        for i in 0..pad_number{
            desc_vec.pop();
        }
        for i in 0..pad_number{
            desc_vec.push(65);
        }
        desc_vec
    }

    fn mapping(&self, numbers: Vec<u8>, url: bool) -> Vec<char>{
        // return the base64 mapping result Vec<char>
        let mut base64_encode: Vec<char> = Vec::new();

        // choose the standard code
        let mut standard_code = self.standard_code();
        if url{
            standard_code = self.standard_code_url();
        }
        for i in numbers{
            let x = i as i32;
            let chars = standard_code.get(&x);
            let res = match chars {
                Some(v) => v,
                None => &'|'
            };
            base64_encode.push(*res);
        }
        base64_encode
    }

}
