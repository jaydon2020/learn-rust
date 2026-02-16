//! Mock storage.
//!
//! Hint: Consult <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#a-use-case-for-interior-mutability-mock-objects>.
//!
//! Refer `mock_storage_grade.rs` for test cases.

use std::cell::{Ref, RefCell};
use std::collections::HashMap;

/// Mock storage.
#[derive(Debug)]
pub struct MockStorage {
    /// Files stored in the storage.
    ///
    /// Each entry of the hashmap represents the `(name, size)` of the file.
    files: RefCell<HashMap<String, usize>>,

    /// Capacity of the storage.
    ///
    /// The total size of files stored on the storage cannot exceed the capacity.
    capacity: usize,
}

impl MockStorage {
    /// Creates a new mock storage.
    pub fn new(capacity: usize) -> Self {
        Self {
            files: RefCell::new(HashMap::new()),
            capacity,
        }
    }
}

/// Trait for storage object.
pub trait Storage {
    /// Uploads a file. If a file with the same name already exists in the storage, overwrite it.
    ///
    /// Returns `Err` with insufficient memory size if there is no free space to upload a file.
    fn upload(&self, name: &str, size: usize) -> Result<(), usize>;

    /// Returns the used memory size of the storage.
    fn used(&self) -> usize;

    /// Returns the capacity of the storage.
    fn capacity(&self) -> usize;
}

impl Storage for MockStorage {
    fn upload(&self, _name: &str, _size: usize) -> Result<(), usize> {
        // We cannot call self.used() here because it tries to borrow() again (panic!).
        let mut files = self.files.borrow_mut();

        // 2. Calculate current usage manually from our mutable reference
        let current_used: usize = files.values().sum();

        // 3. Check if we are overwriting a file
        // If it exists, we get its old size. If not, old size is 0.
        let old_size = files.get(_name).copied().unwrap_or(0);

        // 4. Calculate what the NEW total would be
        // We subtract the old file (reclaiming space) and add the new one.
        let new_total = (current_used - old_size) + _size;

        // 5. Check Capacity
        if new_total > self.capacity {
            return Err(new_total - self.capacity); // Return how much we are over
        }

        // 6. Insert (Overwrite)
        // insert() automatically overwrites if the key exists.
        let _unused = files.insert(_name.to_string(), _size);

        Ok(())
    }

    fn used(&self) -> usize {
        let files = self.files.borrow();

        files.values().sum()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

/// File uploader.
///
/// It uploads files to the internal storage.
#[derive(Debug)]
pub struct FileUploader<'a, T: Storage> {
    storage: &'a T,
}

impl<'a, T: Storage> FileUploader<'a, T> {
    /// Creates a new file uploader with given internal storage.
    pub fn new(storage: &'a T) -> Self {
        Self { storage }
    }

    /// Uploads a file to the internal storage.
    pub fn upload(&self, _name: &str, _size: usize) -> Result<(), usize> {
        self.storage.upload(_name, _size)
    }
}

/// Storage usage analyzer.
#[derive(Debug)]
pub struct UsageAnalyzer<'a, T: Storage> {
    storage: &'a T,
    bound: f64,
}

impl<'a, T: Storage> UsageAnalyzer<'a, T> {
    /// Creates a new usage analyzer.
    pub fn new(storage: &'a T, bound: f64) -> Self {
        Self { storage, bound }
    }

    /// Returns `true` if the usage of the internal storage is under the bound.
    pub fn is_usage_under_bound(&self) -> bool {
        let used_ratio = self.storage.used() as f64 / self.storage.capacity() as f64;
        used_ratio < self.bound
    }
}
