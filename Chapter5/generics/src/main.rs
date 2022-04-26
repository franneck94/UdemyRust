struct GenericValue<T> {
    value: T,
}

fn generic<T>(_s: GenericValue<T>) {
    println!("called");
}

impl<T> GenericValue<T> {
    fn value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let _char: GenericValue<char> = GenericValue { value: 'a' };
    let _i32 = GenericValue { value: 96 };
    let _float = GenericValue { value: 13.37_f32 };

    generic(_i32);
    generic::<char>(_char);

    println!("{}", _float.value());
}
