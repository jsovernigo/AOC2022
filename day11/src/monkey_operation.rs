use crate::item::ItemWorry;

pub enum MonkeyOperation {
    PlusValue(i32),
    PlusSelf,
    MinusValue(i32),
    MinusSelf,
    MultiplyValue(i32),
    MultiplySelf,
    DivideValue(i32),
    DivideSelf
}

impl std::fmt::Display for MonkeyOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MonkeyOperation::PlusSelf               => write!(f, "new = old + old"),
            MonkeyOperation::MinusSelf              => write!(f, "new = old - old"),
            MonkeyOperation::MultiplySelf           => write!(f, "new = old * old"),
            MonkeyOperation::DivideSelf             => write!(f, "new = old / old"),
            MonkeyOperation::PlusValue(v)           => write!(f, "new = old + {}", v),  
            MonkeyOperation::MinusValue(v)          => write!(f, "new = old - {}", v),
            MonkeyOperation::MultiplyValue(v)       => write!(f, "new = old * {}", v),
            MonkeyOperation::DivideValue(v)         => write!(f, "new = old / {}", v),
        }
    }
}

impl MonkeyOperation {
    pub fn new(operator: &str, operand: &str) -> MonkeyOperation {
        if operand == "old" {
            match operator {
                "+" => MonkeyOperation::PlusSelf,
                "-" => MonkeyOperation::MinusSelf,
                "*" => MonkeyOperation::MultiplySelf,
                "/" => MonkeyOperation::DivideSelf,
                _ => panic!("Couldn't parse operator {}", operator)
            }
        } else {
            let value = operand
                .parse::<i32>()
                .expect("Couldn't parse operand.");

            match operator {
                "+" => MonkeyOperation::PlusValue(value),
                "-" => MonkeyOperation::MinusValue(value),
                "*" => MonkeyOperation::MultiplyValue(value),
                "/" => MonkeyOperation::DivideValue(value),
                _ => panic!("Couldn't parse operator {}", operator)
            }
        }
    }
    pub fn apply(&self, item: ItemWorry) -> ItemWorry {
        match *self {
            MonkeyOperation::PlusSelf => item + item,
            MonkeyOperation::MinusSelf => item - item,
            MonkeyOperation::MultiplySelf => item * item,
            MonkeyOperation::DivideSelf => item / item,
            MonkeyOperation::PlusValue(v) => item + v,
            MonkeyOperation::MinusValue(v) => item - v,
            MonkeyOperation::MultiplyValue(v) => item * v,
            MonkeyOperation::DivideValue(v) => item / v
        }
    }
}