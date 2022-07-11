use storage_node;

struct LSMStorageNode {

}

impl<T> LSMStorageNode<T> for StorageNode<T> {
    fn get(&self, key: T) -> Option<T> {
        unimplemented!();
    }

    fn put(&self, key: T, value: T) {
        unimplemented!();
    }

    fn delete(&self, key: T) {
        unimplemented!();
    }

    fn list(&self) -> Option<Vec<T>> {
        unimplemented!();
    }

    fn start(&mut self) {
        unimplemented!();
    }

    fn stop(&mut self) {
        unimplemented!();
    }
}
