#[derive(Debug, PartialEq, Eq)]
struct MyStruct {
    my_string: String,
    my_bool: bool,
    my_number: u64,
}

fn main() {
    println!("main")
}

#[test]
fn test_struct() {
    let test1 = MyStruct {
        my_string: "string1".to_string(),
        my_bool: false,
        my_number: 1,
    };
    let test11 = MyStruct {
        my_string: "string1".to_string(),
        my_bool: false,
        my_number: 1,
    };
    let test2 = MyStruct {
        my_string: "string1".to_string(),
        my_bool: false,
        my_number: 2,
    };
    assert_eq!(test1, test2);
    assert_eq!(test1, test11);
}
