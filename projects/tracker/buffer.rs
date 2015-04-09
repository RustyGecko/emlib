macro_rules! def_fixed_size_buffer(
    ($name:ident, $size:expr) => (
        pub struct $name<T> {
            pub index: usize,
            pub data: [T; $size]
        }

        impl<T> $name<T> {

            pub fn push(&mut self, val: T) -> bool {
                self.data[self.index % $size] = val;
                self.index = self.index + 1;

                if self.index >= $size {
                    self.index = 0;
                    true
                } else {
                    false
                }
            }
        }
    )
);

def_fixed_size_buffer!(FixedSizeBuffer8, 8);
def_fixed_size_buffer!(FixedSizeBuffer128, 128);
def_fixed_size_buffer!(FixedSizeBuffer512, 512);
