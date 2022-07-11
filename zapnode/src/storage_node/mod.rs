pub mod lsm_storage_node;

pub trait StorageNode<T> {
    fn get(&self, key: T) -> Option<T>;
    fn put(&self, key: T, value: T);
    fn delete(&self, key: T);
    fn list(&self);

    fn start(&mut self);
    fn stop(&mut self);
}
