use std::collections::HashMap;

pub struct MyStruct {
    a: u64,
}

pub fn my_func() -> MyStruct {
    MyStruct { a: 5 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
