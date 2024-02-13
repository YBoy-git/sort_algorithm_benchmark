pub fn is_sorted<T: PartialEq + PartialOrd>(collection: impl AsRef<[T]>) -> bool {
    collection.as_ref().windows(2).all(|w| w[0] <= w[1])
}