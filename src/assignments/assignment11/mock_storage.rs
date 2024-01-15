//! Mock storage.
//!
//! Hint: Consult <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#a-use-case-for-interior-mutability-mock-objects>.
//!
//! Refer `mock_storage_grade.rs` for test cases.

use std::cell::RefCell;
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
    fn upload(&self, name: &str, size: usize) -> Result<(), usize> {
        let mut files = self.files.borrow_mut();
        let current_used: usize = files.values().sum();
        let space_required = if files.contains_key(name) {
            size.saturating_sub(*files.get(name).unwrap_or(&0))
        } else {
            size
        };

        if current_used + space_required > self.capacity {
            Err(space_required - (self.capacity - current_used))
        } else {
            let _ = files.insert(name.to_string(), size);
            Ok(())
        }
    }

    fn used(&self) -> usize {
        self.files.borrow().values().sum()
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
    pub fn upload(&self, name: &str, size: usize) -> Result<(), usize> {
        self.storage.upload(name, size)
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
        (self.storage.used() as f64 / self.storage.capacity() as f64) < self.bound
    }
}
