/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use fs2::FileExt;
use memmap2::Mmap;

use super::byte_rpr::{ByteRpr, FixedByteLen};
use super::segment::FileSegment;

mod mmap_params {
    pub const LTRS_STAMP: &str = "STAMP.ltr";
    pub const STORAGE: &str = "STORAGE.ltr";
    pub const STORAGE_LOCK: &str = "STORAGE_LOCK.ltr";
    pub const STACK: &str = "STACK.ltr";
}

///
/// Improvement points:
/// 1. unwraps
/// 2. Permission denied on windows.
///     change POP to write instead of APPEND
///

///
/// Segment Reader trait
///
pub trait SegmentReader {
    ///
    /// Read all bytes in the device
    ///
    fn read_all(&self) -> &[u8];
    ///
    /// Read a segment in the device
    ///
    fn read(&self, segment: FileSegment) -> Option<&[u8]>;
    ///
    /// Checks if the device is empty
    ///
    fn is_empty(&self) -> bool;
}

///
/// Segment Writer trait
///
pub trait SegmentWriter {
    ///
    /// Delete a file segment
    ///
    fn delete_segment(&mut self, segment: FileSegment);
    ///
    /// Insert a file segment
    ///
    fn insert(&mut self, bytes: &[u8]) -> FileSegment;
}

///
/// Stack used for persist stuff in disk.
/// Stores file segments that were deleted
///
struct DiskStack {
    stack: PathBuf,
}

impl DiskStack {
    pub fn new(path: &Path) -> DiskStack {
        std::fs::create_dir_all(&path).unwrap();
        let path = path.to_path_buf();
        DiskStack {
            stack: path.join(mmap_params::STACK),
        }
    }

    pub fn push(&self, segment: FileSegment) {
        let mut stack = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.stack)
            .unwrap();
        stack.write_all(&segment.alloc_byte_rpr()).unwrap();
        stack.flush().unwrap();
    }

    pub fn pop(&self) -> Option<FileSegment> {
        let mut stack = OpenOptions::new()
            .read(true)
            .write(true) // It fixes the Permission error.
            .create(true)
            .open(&self.stack)
            .unwrap();
        match stack.seek(SeekFrom::End(-(FileSegment::segment_len() as i64))) {
            Ok(new_len) => {
                let mut buffer = vec![0u8; FileSegment::segment_len()];
                stack.read_exact(&mut buffer).unwrap();
                stack.set_len(new_len).unwrap();
                Some(FileSegment::from_byte_rpr(&buffer))
            }
            Err(_) => None,
        }
    }

    pub fn clear(&self) {
        let stack = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.stack)
            .unwrap();
        stack.set_len(0).unwrap();
    }
}

///
/// External storage  mechanism
///
pub struct Storage {
    // Where the values are persisted.
    path_storage: PathBuf,
    // Lock file
    lock: File,
    // Stack with deleted datapoints (Useful for rewriting policies)
    deleted: DiskStack,
    // Storage interface regardiing the path_storage file
    storage: Mmap,
}

impl SegmentReader for Storage {
    fn read_all(&self) -> &[u8] {
        &self.storage[..]
    }

    fn read(&self, segment: FileSegment) -> Option<&[u8]> {
        let range = (segment.start as usize)..(segment.end as usize);
        self.storage.get(range)
    }

    fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

impl SegmentWriter for Storage {
    fn delete_segment(&mut self, segment: FileSegment) {
        self.deleted.push(segment);
    }

    fn insert(&mut self, bytes: &[u8]) -> FileSegment {
        match self.deleted.pop() {
            Some(segment) => self.update_segment(segment, bytes),
            None => self.append(bytes),
        }
    }
}

impl Storage {
    // TODO: study about mmap and finish this

    ///
    /// Creates a new storage in a given path
    ///
    pub fn new(path: &Path) -> Storage {
        std::fs::create_dir_all(&path).unwrap();

        let nuclia_stamp = path.join(mmap_params::LTRS_STAMP);
        let path_storage = path.join(mmap_params::STORAGE);
        let path_lock = path.join(mmap_params::STORAGE_LOCK);
        let path_stack = path.join(mmap_params::STACK);

        let storage = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path_storage)
            .unwrap();

        let lock = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path_lock)
            .unwrap();

        let storage = unsafe { Mmap::map(&storage).unwrap() };
        let deleted = DiskStack::new(path_stack.as_path());
        File::create(&nuclia_stamp).unwrap();
        Storage {
            path_storage,
            lock,
            deleted,
            storage,
        }
    }

    ///
    /// Open a storage in a given  path
    ///
    pub fn open(path: &Path) -> Storage {
        if !path.join(mmap_params::LTRS_STAMP).exists() {
            panic!("Not a valid path to a Storage");
        }

        let nuclia_stamp = path.join(mmap_params::LTRS_STAMP);
        let path_storage = path.join(mmap_params::STORAGE);
        let path_lock = path.join(mmap_params::STORAGE_LOCK);
        let path_stack = path.join(mmap_params::STACK);

        let storage = OpenOptions::new().read(true).open(&path_storage).unwrap();
        let lock = OpenOptions::new().read(true).open(&path_lock).unwrap();
        let storage = unsafe { Mmap::map(&storage).unwrap() };
        let deleted = DiskStack::new(path_stack.as_path());

        File::create(&nuclia_stamp).unwrap();
        Storage {
            path_storage,
            lock,
            deleted,
            storage,
        }
    }

    ///
    /// Reloads the storage
    ///
    pub fn reload(&mut self) {
        self.lock.lock_exclusive().unwrap();
        let file = File::open(&self.path_storage).unwrap();
        self.storage = unsafe { Mmap::map(&file).unwrap() };
        self.lock.unlock().unwrap();
    }

    ///
    /// Appends an array of bytes into the storage
    ///
    pub fn append(&mut self, bytes: &[u8]) -> FileSegment {
        self.lock.lock_exclusive().unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path_storage)
            .unwrap();

        let metadata = file.metadata().unwrap();
        let segment = FileSegment {
            start: metadata.len(),
            end: metadata.len() + (bytes.len() as u64),
        };

        file.seek(SeekFrom::End(0)).unwrap();
        file.write_all(bytes).unwrap();
        file.flush().unwrap();

        self.storage = unsafe { Mmap::map(&file).unwrap() };

        self.lock.unlock().unwrap();
        segment
    }

    ///
    /// Updates a segment with  the  array of bytes
    ///
    pub fn update_segment(&mut self, segment: FileSegment, bytes: &[u8]) -> FileSegment {
        self.lock.lock_exclusive().unwrap();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path_storage)
            .unwrap();
        file.seek(SeekFrom::Start(segment.start)).unwrap();
        file.write_all(bytes).unwrap();
        file.flush().unwrap();
        self.storage = unsafe { Mmap::map(&file).unwrap() };
        self.lock.unlock().unwrap();
        segment
    }
}

#[cfg(test)]
mod deleted_stack_tests {
    use super::*;
    #[test]
    pub fn deleted_buffers_usage() {
        let dir = tempfile::tempdir().unwrap();
        let deleted_buffer = DiskStack::new(dir.path());
        let fs_0 = FileSegment { start: 0, end: 0 };
        let fs_1 = FileSegment { start: 1, end: 1 };
        let fs_2 = FileSegment { start: 2, end: 2 };
        assert_eq!(deleted_buffer.pop(), None);
        deleted_buffer.push(fs_0);
        deleted_buffer.push(fs_1);
        deleted_buffer.push(fs_2);
        deleted_buffer.clear();
        assert_eq!(deleted_buffer.pop(), None);
        deleted_buffer.push(fs_0);
        deleted_buffer.push(fs_1);
        deleted_buffer.push(fs_2);
        assert_eq!(deleted_buffer.pop(), Some(fs_2));
        assert_eq!(deleted_buffer.pop(), Some(fs_1));
        assert_eq!(deleted_buffer.pop(), Some(fs_0));
    }
}

#[cfg(test)]
mod storage_item_tests {
    use super::*;
    #[test]
    pub fn storage_item_usage() {
        let msg_0 = b"message 0";
        let msg_1 = b"message 1";
        let msg_2 = b"message 2";
        let msg_3 = b"message 3";
        let dir = tempfile::tempdir().unwrap();
        let mut segment = Storage::new(dir.path());
        let mut segment_r = Storage::open(dir.path());
        assert!(segment.is_empty());

        let fs_0 = segment.insert(msg_0);
        let fs_1 = segment.insert(msg_1);
        let fs_2 = segment.insert(msg_2);
        assert_eq!(segment.read(fs_0), Some(msg_0.as_ref()));
        assert_eq!(segment.read(fs_1), Some(msg_1.as_ref()));
        assert_eq!(segment.read(fs_2), Some(msg_2.as_ref()));
        assert_eq!(segment_r.read(fs_0), None);
        assert_eq!(segment_r.read(fs_1), None);
        assert_eq!(segment_r.read(fs_2), None);
        segment_r.reload();
        assert_eq!(segment_r.read(fs_0), Some(msg_0.as_ref()));
        assert_eq!(segment_r.read(fs_1), Some(msg_1.as_ref()));
        assert_eq!(segment_r.read(fs_2), Some(msg_2.as_ref()));
        segment.delete_segment(fs_2);
        let fs_3 = segment.insert(msg_3);
        assert_eq!(fs_3, fs_2);
        assert_eq!(segment.read(fs_3), Some(msg_3.as_ref()));
    }
}
