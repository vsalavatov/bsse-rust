#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comparison {
    Equal,     // список `a` равен списку `b`
    Sublist,   // список `a` является подсписком `b`
    Superlist, // список `b` является подсписком `a`
    Other,     // списки не равны и не являются подсписками друг друга
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let mut j = 0;
    for lhs in a {
        while j < b.len() && *lhs != b[j] {
            j += 1;
        }
        if j == b.len() {
            return false;
        }
    }
    return true;
}

pub fn compare<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    let a_b = is_sublist(&a, &b);
    let b_a = is_sublist(&b, &a);
    if a_b && b_a {
        return Comparison::Equal;
    }
    if a_b {
        return Comparison::Sublist;
    }
    if b_a {
        return Comparison::Superlist;
    }
    return Comparison::Other;
}

#[cfg(test)]
mod tests {
    use crate::{compare, Comparison};
    #[test]
    fn equal() {
        assert_eq!(compare(&[1, 2], &[1, 2]), Comparison::Equal);
        assert_eq!(
            compare(&["aboba", "abuba"], &["aboba", "abuba"]),
            Comparison::Equal
        );
    }

    #[test]
    fn sublist() {
        assert_eq!(compare(&[1, 2], &[1, 2, 3]), Comparison::Sublist);
        assert_eq!(compare(&[2, 3], &[1, 2, 3, 4]), Comparison::Sublist);
        assert_eq!(compare(&[2, 3, 4], &[1, 2, 3, 4]), Comparison::Sublist);
    }

    #[test]
    fn superlist() {
        assert_eq!(compare(&[1, 2, 3], &[1, 2]), Comparison::Superlist);
        assert_eq!(compare(&[1, 2, 3, 4], &[2, 3]), Comparison::Superlist);
        assert_eq!(compare(&[1, 2, 3, 4], &[2, 3, 4]), Comparison::Superlist);
    }

    #[test]
    fn other() {
        assert_eq!(compare(&[1, 2], &[2, 1]), Comparison::Other);
        assert_eq!(compare(&[1, 2, 3, 4], &[2, 3, 4, 5]), Comparison::Other);
    }
}
