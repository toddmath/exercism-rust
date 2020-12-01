pub struct Luhn {
    data: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.data
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .try_fold((0, 0), |(i, checksum), c| {
                c.to_digit(10)
                    .map(|d| if i % 2 == 0 { d } else { d * 2 })
                    .map(|d| checksum + (if d > 9 { d - 9 } else { d }))
                    .map(|d| (i + 1, d))
            })
            .map_or(false, |(len, checksum)| len > 1 && checksum % 10 == 0)
    }

    fn new(data: String) -> Self {
        Self { data }
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Self::new(input.to_string())
    }
}
