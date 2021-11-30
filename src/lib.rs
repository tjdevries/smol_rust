use std::collections::HashMap;

use itertools::Itertools;

pub struct MyStruct {
    a: HashMap<usize, usize>,
    b: String,
}

pub fn my_func() -> MyStruct {
    let mut i = 0..2;

    MyStruct {
        a: HashMap::new(),
        b: i.join(", "),
    }
}

pub fn print_struct(s: MyStruct) {
    println!("My Struct: {:?} {}", s.a, s.b);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::my_func;

    #[test]
    fn it_works() {
        let result = my_func();
        assert_eq!(result.a, HashMap::new());
        assert_eq!(result.b, "0, 1, 2".to_owned());
    }
}
