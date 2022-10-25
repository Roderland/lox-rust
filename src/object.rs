use std::fmt::{self, Formatter};
use crate::object::Object::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Num(x) => write!(f, "{x}"),
            Str(x) => write!(f, "\"{x}\""),
            Nil => write!(f, "nil"),
            True => write!(f, "true"),
            False => write!(f, "false"),
        }
    }
}

impl Object {
    /// 获取数值类型的object的值
    pub fn num(&self) -> f64 {
        match self {
            Num(x) => *x,
            _ => 0f64
        }
    }

    /// 检查object是否为数值类型
    pub fn is_num(&self) -> bool {
        match self {
            Num(_) => true,
            _ => false
        }
    }

    /// 获取字符串类型的object的值
    pub fn str(&self) -> &str {
        match self {
            Str(x) => x,
            _ => ""
        }
    }

    /// 检查object是否为字符串类型
    pub fn is_str(&self) -> bool {
        match self {
            Str(_) => true,
            _ => false
        }
    }

    /// 检查object是否为true
    pub fn is_true(&self) -> bool {
        match self {
            Nil => false,
            False => false,
            _ => true
        }
    }

    /// 创建一个bool类型的object
    pub fn new_bool(x :bool) -> Object {
        if x { True } else { False }
    }

    // 判断是否与另一个object相等 @PartialEq
    // fn eq(&self, other: &Self) -> bool {
    //     match self {
    //         Num(x) => {
    //             if let Num(y) = other { return x == y; }
    //             false
    //         },
    //         Str(x) => {
    //             if let Str(y) = other { return x == y; }
    //             false
    //         },
    //         True => if let True = other { true } else { false },
    //         False => if let False = other { true } else { false },
    //         Nil => if let Nil = other { true } else { false },
    //     }
    // }


}

#[cfg(test)]
mod tests {
    use crate::object::Object::{Num, Str};

    #[test]
    fn test_equal() {
        let x = &1f64;
        let y = &1f64;
        assert_eq!(*x, *y);
        assert_eq!(x, y);
        println!("{} {}", x, y);

        let x = &"123".to_string();
        let y = &"123".to_string();
        assert_eq!(*x, *y);
        assert_eq!(x, y);
        println!("{} {}", x, y);

        let x = &Num(1.0f64);
        let y = &Num(1f64);
        assert_eq!(*x, *y);
        assert_eq!(x, y);
        println!("{} {}", x, y);

        let x = &Str("hello".to_string());
        let y = &Str("hello".to_string());
        assert_eq!(*x, *y);
        assert_eq!(x, y);
        println!("{} {}", x, y);

        let x = &Str("hello".to_string());
        let y = &Str("hello ".to_string());
        assert_ne!(*x, *y);
        assert_ne!(x, y);
        println!("{} {}", x, y);
    }
}