struct Unfold<A, F> {
    base: A,
    f: F,
}

impl<A, F> Unfold<A, F> {
    fn new(base: A, f: F) -> Self {
        Unfold { base, f }
    }
}

impl<B, A, F> Iterator for Unfold<A, F>
where
    F: FnMut(&mut A) -> Option<(A, B)>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.f)(&mut self.base) {
            None => None,
            Some((a, b)) => {
                self.base = a;
                Some(b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_five() {
        let mut un = Unfold::new(
            1,
            |&mut x: &mut i32| if x > 5 { None } else { Some((x + 1, x)) },
        );

        assert_eq!(un.next(), Some(1));
        assert_eq!(un.next(), Some(2));
        assert_eq!(un.next(), Some(3));
        assert_eq!(un.next(), Some(4));
        assert_eq!(un.next(), Some(5));
        assert_eq!(un.next(), None);
        assert_eq!(un.next(), None);
    }

    #[test]
    fn sum_100() {
        let un = Unfold::new(
            1,
            |&mut x: &mut i32| if x > 100 { None } else { Some((x + 1, x)) },
        );

        assert_eq!(un.sum::<i32>(), (1..=100).sum());
    }
}
