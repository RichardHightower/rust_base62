# Video 10 show notes for Rust Base 62

* Link to this [page](https://bit.ly/rust_base62): `https://bit.ly/rust_base62`
* Link to related [video](https://youtu.be/1kbAUob3Bi0) TODO


## Tags

```
#rust #java #functionalprogramming #scala #base62 

```



# Base62Encoder URLShortener



### Example URL shortener services:
* https://www.shorturl.at/
* https://bitly.com/
* https://tinyurl.com/

### URL shortener services and Base 62 explained:
* [WikiPedia URL Shortening](https://en.wikipedia.org/wiki/URL_shortening)
* [Base 62](https://en.wikipedia.org/wiki/Base62).

### URL shortener services and Base 62 tutorials with example code:
* [How a URL Shortening Application Works](https://dzone.com/articles/how-a-url-shortening-application-works)
* [Designing the Shortening URL system like Bit.ly, loading 6 billion clicks a month](https://itnext.io/designing-the-shortening-url-system-like-bit-ly-loading-6-billion-clicks-a-month-78b3e48eee8c)

# Videos in this series
* [First Video in Base62 language series 3rd Vlog video](https://www.youtube.com/watch?v=07Wkf9OZE3U)  [Show Notes]()
* [Second Video in Base62 language series 4th Vlog video](https://www.youtube.com/watch?v=sOhzb6OqyGA) [Show Notes](https://gist.github.com/RichardHightower/035fda0b65de540574e458dedf9dae6d)
* [Third Video in Base62 language series 5th Vlog video](https://www.youtube.com/watch?v=TlQZn9MajlY)  [Show Notes](https://gist.github.com/RichardHightower/1d64d0c958a7643c8b0b573c08138e1f)
* [Fourth Video in Base62 language series 6th Vlog video](https://www.youtube.com/watch?v=1kbAUob3Bi0) [Show notes](https://gist.github.com/RichardHightower/5b45e5162cf8295f73e71d67ad4a442d)

# Related articles
* [Comparing Basic FP support part 1 --Rick Hightower](https://www.linkedin.com/pulse/comparing-basic-fp-support-part-1-rick-hightower/)
* [Is Java a good FP language? Comparing Basic FP support part 2 --Rick Hightower](https://www.linkedin.com/pulse/java-good-fp-language-comparing-basic-support-part-2-rick-hightower/)
* [Translating to Clojure: a learning task (Part 1) --Tom Hicks](https://hickst.hashnode.dev/translating-to-clojure-a-learning-task-part-1)


# Where is Rick?
* [LinkedIn](https://www.linkedin.com/in/rickhigh/)
* [Rick's YouTube Channel](https://www.youtube.com/channel/UCgCx8XtYUGW9aSfzXhP2m6Q)
* [Where Rick works](http://www.cloudurable.com/)


## Title: Porting Base62Encoder/Decoder from Scala to Rust

Porting Base62Encoder/Decoder from Scala to Rust.

[Link To video](https://youtu.be/1kbAUob3Bi0) TODO

## Scala to Rust

### Main Method

<table>
<tr>
<th>
Scala
</th>
<th>
Rust
</th>
</tr>

<tr>
<td>
<sub>

```scala 
def main(args: Array[String]): Unit = {
  val id = 12345678910L
  val strId = convertToEncodedString(id)
  val newId = convertToLong(strId)
  println(s"$id $strId $newId")

  val longURL = "https://www.somewebiste.com/dp/..."
  val urlId = Math.abs(longURL.hashCode)
  val shortHandle = convertToEncodedString(urlId)
  println(s"$urlId $shortHandle ${convertToLong(shortHandle)}")

}
```
</sub>
</td>

<td >
<sub>

```rust
    fn main() {
    println!("Hello, world! {} {}",  convert_to_encoded_string(12345678910), convert_to_long("DTVD3O".to_string()));
    let long_url = "https://www.somewebiste.com/d=...";
    let url_id = hash(&long_url) / 10;
    println!("url_id {}", url_id);
    let short_handle = convert_to_encoded_string(url_id );
    println!("short_handle {}", short_handle);
    let short_handle_id = convert_to_long(short_handle);
    println!("short_handle_id {}", short_handle_id);
}


```

</sub>
</td>

</tr>
</table>



### convertToEncodedString

<table>
<tr>
<th>
Scala
</th>
<th>
Rust
</th>
</tr>

<tr>
<td>
<sub>

```scala 
def convertToEncodedString(id: Long): String = {
  val builder: List[String] = List()
  val placeHolder = findStartBucket(id)
  val results = accumulateDigits(placeHolder, id, builder)
  val bucketValue = powDigitsBase(1)
  val digitIndex: Int = (results._2 / bucketValue).toInt
  val acc = results._2 - (bucketValue * digitIndex)
  val newBuilder: List[String] = appendSafeToList(results._3, digitIndex)
  //Put the remainder in the ones column
  val place1DigitIndex = (acc % bucketValue).toInt
  val finalBuilder = newBuilder ++ List(DIGITS(place1DigitIndex).toString)
  finalBuilder.mkString("")
}

private def accumulateDigits(placeHolder: Int, acc: Long, 
                     builder: List[String]): (Int, Long, List[String]) = {
  if (!(placeHolder > 1)) {
    return (placeHolder, acc, builder)
  }
  val bucketValue = powDigitsBase(placeHolder)
  val digitIndex = (acc / bucketValue).toInt
  accumulateDigits(placeHolder - 1, acc - (bucketValue * digitIndex), 
       appendSafeToList(builder, digitIndex))
}
```
</sub>
</td>

<td >
<sub>

```rust
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

```

</sub>
</td>

</tr>
</table>


### findStartBucket

<table>
<tr>
<th>
Scala
</th>
<th>
Rust
</th>
</tr>

<tr>
<td>
<sub>

```scala
private def findStartBucket(value: Long): Int = {
  val i = Range(0, 15, 1).find(i => value < powDigitsBase(i.toLong))
  i.getOrElse(0)
}
```
</sub>
</td>

<td >
<sub>

```rust
fn find_start_bucket(value : u64) -> usize {
    for i in 0..15 {
        if value < pow_digits_base(i as u64) {
            return i-1;
        }
    }
    return 0
}
```

</sub>
</td>

</tr>
</table>



### powDigitsBase

<table>
<tr>
<th>
Scala
</th>
<th>
Rust
</th>
</tr>

<tr>
<td>
<sub>

```scala
private def powDigitsBase(exponent: Long): Long = 
                        doPow(exponent, DIGITS.length)

private def doPow(exponent: Long, base: Int): Long = {
  if (exponent == 0) return 1
  doPow(exponent - 1, base) * base
}

```
</sub>
</td>

<td >
<sub>

```rust 
    fn pow_digits_base( exponent : u64) -> u64 {
        return do_pow(exponent, DIGITS.len() as u64);
    }

    fn do_pow( exponent : u64, base: u64) -> u64 {
        if exponent == 0 { return 1; }
        return do_pow(exponent - 1, base) * base;
    }
```

</sub>
</td>

</tr>
</table>




### appendSafeToList

<table>
<tr>
<th>
Scala
</th>
<th>
Rust
</th>
</tr>

<tr>
<td>
<sub>

```scala

private def appendSafeToList(builder: List[String], digitIndex: Int): List[String] = {
  if (digitIndex != 0) builder ++ List((DIGITS(digitIndex)).toString)
  else if (builder.nonEmpty) builder ++ List((DIGITS(digitIndex)).toString)
  else builder
}
```
</sub>
</td>

<td >
<sub>

```rust
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
```

</sub>
</td>

</tr>
</table>




### convertToLong

<table>
<tr>
<th>
Scala
</th>
<th>
Rust
</th>
</tr>

<tr>
<td>
<sub>

```scala

def convertToLong(strId: String): Long = 
                            doConvertToLong(strId.toCharArray)

private def doConvertToLong(chars: Array[Char]): Long = {
  val (acc, _) = chars.reverse.foldLeft(0L, 0) { (pos, ch) =>
    val (acc, position) = pos
    val value = computeValue(ch, position)
    (acc + value, position + 1)
  }
  acc
}
```
</sub>
</td>

<td >
<sub>

```rust
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
```
</sub>
</td>

</tr>
</table>




### computeValue

<table>
<tr>
<th>
Scala
</th>
<th>
Rust
</th>
</tr>

<tr>
<td>
<sub>

```scala 
private def computeValue(c: Char, position: Int) = {
  val digitIndex = findIndexOfDigitInTable(c)
  val multiplier = powDigitsBase(position)
  digitIndex * multiplier
}
```
</sub>
</td>

<td >
<sub>

```rust
fn compute_value(c: char, position: usize) -> u64 {
    let digit_index = find_index_of_digit_in_table(c);
    let multiplier = pow_digits_base(position as u64);
    return digit_index as u64 * multiplier;
}
```

</sub>
</td>

</tr>
</table>






----


## Rust full example

```rust 
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
```


