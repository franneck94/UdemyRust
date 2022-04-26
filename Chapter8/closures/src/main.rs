fn calculation(val: i32) -> i32 {
    println!("Computing...");

    val
}

struct Computer<T>
where
    T: Fn(i32) -> i32,
{
    functor: T,
    value: Option<i32>,
}

impl<T> Computer<T>
where
    T: Fn(i32) -> i32,
{
    fn new(functor: T) -> Computer<T> {
        Computer {
            functor,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let res = (self.functor)(arg);
                self.value = Some(res);
                res
            }
        }
    }
}

fn main() {
    calculation(10);

    let closure = |val| {
        println!("Computing...");

        val
    };

    closure(10);
    // closure(10.0);

    let param = 10;

    let closure2 = |val| {
        println!("Computing...");

        val * param
    };

    closure2(10);

    let mut c = Computer {
        functor: closure,
        value: None,
    };

    println!("{:?}", c.value);
    c.value(10);
    println!("{:?}", c.value);
    c.value(12);
    println!("{:?}", c.value);
}
