use std::hash::{Hash, Hasher};
use fasthash::{MetroHasher};
use string_builder::Builder;


fn main() {

    const DIGITS: [char; 62] =  [
//0    1    2    3    4    5    6    7    8    9
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
//10    11   12   13  14   15   16   17    18    19   20  21
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
//22    23   24   25  26   27   28   29    30    31   32  33    34  35
        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
// 36  37  38   39   40   41    42    43   44  45   46    47
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',  //Easy to add more characters if not using lookup tables.
// 48  49   50   51   52   53   54   55  56   57   58  59   60   61   // 62   63, 64
        'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];


    fn append_safe(mut builder:Builder, digit_index:usize) -> Builder{
        let ch: char = DIGITS[digit_index];

        if digit_index != 0 {
            builder.append(ch);
        } else {
            if builder.len() > 0 {
                builder.append(ch);
            }
        }
        return builder;
    }

    fn pow_digits_base( exponent : u64) -> u64 {
        return do_pow(exponent, DIGITS.len() as u64);
    }

    fn do_pow( exponent : u64, base: u64) -> u64 {
        if exponent == 0 { return 1; }
        return do_pow(exponent - 1, base) * base;
    }

    fn find_start_bucket(value : u64) -> usize {
        for i in 0..15 {
            if value < pow_digits_base(i as u64) {
                return i-1;
            }
        }
        return 0
    }

    fn convert_to_encoded_string(id :u64) -> String {
        let mut builder = Builder::default();
        let place_holder = find_start_bucket(id);
        let results = accumulate_digits(place_holder as i32, id, builder);
        let bucket_value = pow_digits_base(1);
        let digit_index = (results.1 / bucket_value) as usize;
        let acc = results.1 - (bucket_value * digit_index as u64) as u64;
        builder = append_safe(results.2, digit_index);
        //Put the remainder in the ones column
        let place1digit_index = (acc % bucket_value) as usize;
        builder = append_safe(builder, place1digit_index);
        return builder.string().unwrap();
    }

    fn accumulate_digits(place_holder: i32, acc: u64,  builder:Builder) -> (i32, u64,  Builder) {
        if !(place_holder > 1) {
            return (place_holder, acc, builder);
        }
        let bucket_value = pow_digits_base(place_holder as u64);
        let digit_index = (acc / bucket_value) as usize;
        return accumulate_digits(place_holder - 1, acc - (bucket_value * (digit_index as u64)),
                                 append_safe(builder, digit_index));
    }

    fn convert_to_long(str_id: String) -> u64 {
        let chars  : Vec<_> = str_id.chars().collect();
        return do_convert_to_long(chars);
    }

    fn do_convert_to_long(mut chars: Vec<char> ) -> u64 {
        chars.reverse();

        let (acc, _) = chars.iter().fold((0, 0), |pos, ch| {
            let (acc, position) = pos;
            let value = compute_value(*ch, position);
            return  (acc + value, position + 1);
        });
        acc
    }

    fn find_index_of_digit_in_table(c :char ) -> usize {
        for i in 0..DIGITS.len() {
            if DIGITS[i] == c {
                return  i;
            }
        }
        return 666;
    }

    fn compute_value(c: char, position: usize) -> u64 {
        let digit_index = find_index_of_digit_in_table(c);
        let multiplier = pow_digits_base(position as u64);
        return digit_index as u64 * multiplier;
    }


    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s: MetroHasher = Default::default();
        t.hash(&mut s);
        s.finish()
    }


    fn main() {
        println!("Hello, world! {} {}",  convert_to_encoded_string(12345678910), convert_to_long("DTVD3O".to_string()));
        let long_url = "https://www.somewebiste.com/dp/0201616165/?_encoding=UTF8&pd_rd_w=vwEcs&content-id=amzn1.sym.8cf3b8ef-6a74-45dc-9f0d-6409eb523603&pf_rd_p=8cf3b8ef-6a74-45dc-9f0d-6409eb523603&pf_rd_r=BQ0KD40K57XG761DBNBA&pd_rd_wg=DtkHk&pd_rd_r=f94b60b7-9080-4065-b77f-6377ec854d17&ref_=pd_gw_ci_mcx_mi";
        let url_id = hash(&long_url) / 10;
        println!("url_id {}", url_id);
        let short_handle = convert_to_encoded_string(url_id );
        println!("short_handle {}", short_handle);
        let short_handle_id = convert_to_long(short_handle);
        println!("short_handle_id {}", short_handle_id);
    }
    main();

}

