/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        value
            .to_string()
            .chars()
            .eq(value.to_string().chars().rev())
            .then(|| return Palindrome(value))
            .or(None)
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut smallest = max * max;
    let mut largest = min * min;
    (min..=max).filter(|x| x % 10 != 0).for_each(|i| {
        (min..=max)
            .filter(|x| x % 10 != 0)
            .for_each(|j| match Palindrome::new(i * j) {
                Some(v) => {
                    if v.0 < smallest {
                        smallest = v.0;
                    }
                    if v.0 > largest {
                        largest = v.0;
                    }
                }
                None => (),
            });
    });

    if smallest == max * max && largest == min * min {
        return None;
    }
    Some((Palindrome(smallest), Palindrome(largest)))
}
