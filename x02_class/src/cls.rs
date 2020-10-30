pub struct ClassName {
    field: i32,
}

impl ClassName {
    pub fn new(value: i32) -> ClassName {
        ClassName {
            field: value
        }
    }

    // 加了pub才能公有暴露
    pub fn public_method(&self) {
        println!("from public method");
        self.private_method(); // 调用了私有方法
    }

    // 方法默认私有
    fn private_method(&self) {
        println!("from private method");
    }
}