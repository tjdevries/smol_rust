use std::collections::HashMap;

pub struct MyStruct {
    a: HashMap<usize, usize>,
}

pub fn my_func() -> MyStruct {
    MyStruct { a: HashMap::new() }
}

pub fn print_struct(s: MyStruct) {
    println!("My Struct: {:?}", s.a);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::my_func;

    #[test]
    fn it_works() {
        let result = my_func();
        assert_eq!(result.a, HashMap::new());
    }
}
