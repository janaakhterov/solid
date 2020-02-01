pub struct Selector {
    name: Option<String>,
    params: Vec<String>,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            name: None,
            params: Vec::new(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

#[macro_use]
macro_rules! impl_solidity_function_for_selector {
    ($ty: ty => $function: ident | $array: ident) => {
        impl Selector {
            pub fn $function(mut self) -> Self {
                self.params.push(stringify!($ty).to_owned());
                self
            }

            pub fn $array(mut self, dimensions: std::num::NonZeroUsize) -> Self {
                let param = format!(
                    "{}{}",
                    stringify!($ty).to_owned(),
                    "[]".repeat(dimensions.get())
                );
                self.params.push(param);
                self
            }
        }
    };
}

impl_solidity_function_for_selector!(i8 => add_i8 | add_i8_array);
