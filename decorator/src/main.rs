// Interface for operators and decorators
pub trait Operator {
    fn add(&self, a: i32, b: i32) -> i32;
    fn sub(&self, a: i32, b: i32) -> i32;
}

// A logging decorator
struct OpLogger<'a> {
    op: &'a Operator,
}

impl<'a> OpLogger<'a> {
    // Factory method for OpLogger
    pub fn new(op: &'a Operator) -> OpLogger {
        OpLogger { op }
    }
}

// An actual operator that can be decorated
struct Op {}

// Logger decorator implementation
impl<'a> Operator for OpLogger<'a> {
    fn add(&self, a: i32, b: i32) -> i32 {
        println!("{} + {}", a, b);
        self.op.add(a, b)
    }

    fn sub(&self, a: i32, b: i32) -> i32 {
        println!("{} - {}", a, b);
        self.op.add(a, b)
    }
}

// Actual operator implementation
impl Operator for Op {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn sub(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let op = Op {};
    let lop = OpLogger::new(&op);
    let lop2 = OpLogger::new(&lop);
    lop2.add(3, 3);
    lop2.sub(3, 3);
}
