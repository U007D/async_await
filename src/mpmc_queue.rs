pub trait MpmcQueue: Send + Sync {
    type Item;

    fn pop(&self) -> Option<Self::Item>;
    fn push(&self, item: Self::Item) -> &Self;
}
