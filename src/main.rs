fn main() {
    let ans = sum(1, 3);
    println!("Ans is {}", ans);
    // println!("Hello, world!");

    println!("{}", is_even(12));
    println!("String is : {}", string_create());
    let name = String::from("Nishant");
    println!("String length is : {} ", string_len(name));
    println!("Float is : {}", add_floats(33, 42.2));
    println!("Is odd : {}", is_odd(-98));
    println!("Find char: {:?}", find_char(String::from("Nishant"), 'k'));

    println!("Array is : {:?}", create_arr_static_arr());

    println!("Dynamic arr is : {:?}", create_dynamic_arr());
    println!(
        "Dynamic  update arr is : {:?}",
        update_arr(create_dynamic_arr())
    )
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}

fn string_create() -> String {
    const HELLO: &str = "hello world";

    let name = String::from("Nishant");

    return name;
}

fn string_len(s: String) -> usize {
    return s.len();
}

fn add_floats(a: u32, b: f32) -> f32 {
    return a as f32 + b; //converting u32 to f32
}

fn is_odd(num: i32) -> bool {
    return num % 2 != 0;
}

fn find_char(s: String, c: char) -> Result<usize, String> {
    let mut letter: char = 'a';

    letter = 'd';

    println!("Letter is : {}", letter);

    let find_s = s.find(c);
    println!("Find char is : {:?}", find_s);

    // return find_s.unwrap_or(usize::MAX);

    match find_s {
        Some(i) => Ok(i),
        None => Err("Character not found".to_string()),
    }
}

fn create_arr_static_arr() -> [u32; 5] {
    let arr: [u32; 5] = [1, 2, 4, 5, 42949672]; //fixed sized arr

    // let mut dyna_arr: Vec<u32> = vec![13, 4, 56, 66];

    // dyna_arr.push(21);
    // return dyna_arr.as_slice();
    return arr;
}

fn create_dynamic_arr() -> Vec<u32> {
    let mut darr: Vec<u32> = vec![34, 46, 767, 876, 87876, 22];

    darr.push(23);

    println!("Dynamic array is : {:?}", darr);

    darr.insert(0, 108);

    println!("Dynamic array is : {:?}", darr);

    darr.pop();

    println!("Dynamic array is : {:?}", darr);

    darr.remove(1);

    println!("Dynamic array is : {:?}", darr);

    return darr;
}

fn update_arr(mut arr: Vec<u32>) -> Vec<u32> {
    arr.push(12);

    //update index 3 with 10008

    arr[3] = 10008;

    return arr;
}

/*
*
*   Data Types:
*   - Integers
*   - Floats
*   - Booleans
*   - Characters
*   - Strings
*   - Arrays
*   - Tuples
*   - Structs
*   - Enums
*

*/
