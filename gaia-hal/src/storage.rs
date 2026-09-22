//! Storage subsystem — block device trait and file-backed implementation.

use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;

/// Errors from block device operations.
#[derive(Debug, Error)]
pub enum StorageError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// The requested block is out of range.
    #[error("block {0} out of range (device has {1} blocks)")]
    OutOfRange(u64, u64),
    /// Buffer size does not match block size.
    #[error("buffer size {0} != block size {1}")]
    BadBufferSize(usize, usize),
}

/// Metadata about a block device.
#[derive(Debug, Clone)]
pub struct BlockDeviceInfo {
    /// Human-readable device name.
    pub name: String,
    /// Size of each block in bytes.
    pub block_size: usize,
    /// Total number of blocks.
    pub block_count: u64,
}

impl BlockDeviceInfo {
    /// Total capacity in bytes.
    pub fn capacity_bytes(&self) -> u64 {
        self.block_count * self.block_size as u64
    }
}

/// Trait for a block-addressable storage device.
pub trait BlockDevice: Send + Sync {
    /// Return device metadata.
    fn info(&self) -> &BlockDeviceInfo;

    /// Read `buf.len()` bytes starting at `block_index * block_size`.
    ///
    /// `buf` must be exactly `block_size` bytes.
    fn read_block(&self, block_index: u64, buf: &mut [u8]) -> Result<(), StorageError>;

    /// Write `buf.len()` bytes starting at `block_index * block_size`.
    ///
    /// `buf` must be exactly `block_size` bytes.
    fn write_block(&self, block_index: u64, buf: &[u8]) -> Result<(), StorageError>;
}

/// A [`BlockDevice`] backed by a regular file on the host filesystem.
///
/// Useful for testing, loopback devices, and Tier 0/1 deployments.
pub struct FileBackedDevice {
    info: BlockDeviceInfo,
    path: PathBuf,
    file: Mutex<File>,
}

impl FileBackedDevice {
    /// Open or create a file-backed block device.
    ///
    /// If `path` does not exist it will be created and zeroed to
    /// `block_size * block_count` bytes.
    pub fn open(
        path: impl AsRef<Path>,
        block_size: usize,
        block_count: u64,
        name: impl Into<String>,
    ) -> Result<Self, StorageError> {
        let path = path.as_ref().to_path_buf();
        let capacity = block_size as u64 * block_count;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        // Ensure the file is at least `capacity` bytes.
        if file.metadata()?.len() < capacity {
            file.set_len(capacity)?;
        }
        Ok(Self {
            info: BlockDeviceInfo {
                name: name.into(),
                block_size,
                block_count,
            },
            path,
            file: Mutex::new(file),
        })
    }

    /// Path of the backing file.
    pub fn path(&self) -> &Path { &self.path }
}

impl BlockDevice for FileBackedDevice {
    fn info(&self) -> &BlockDeviceInfo { &self.info }

    fn read_block(&self, block_index: u64, buf: &mut [u8]) -> Result<(), StorageError> {
        if buf.len() != self.info.block_size {
            return Err(StorageError::BadBufferSize(buf.len(), self.info.block_size));
        }
        if block_index >= self.info.block_count {
            return Err(StorageError::OutOfRange(block_index, self.info.block_count));
        }
        let offset = block_index * self.info.block_size as u64;
        let mut f = self.file.lock().unwrap();
        f.seek(SeekFrom::Start(offset))?;
        f.read_exact(buf)?;
        Ok(())
    }

    fn write_block(&self, block_index: u64, buf: &[u8]) -> Result<(), StorageError> {
        if buf.len() != self.info.block_size {
            return Err(StorageError::BadBufferSize(buf.len(), self.info.block_size));
        }
        if block_index >= self.info.block_count {
            return Err(StorageError::OutOfRange(block_index, self.info.block_count));
        }
        let offset = block_index * self.info.block_size as u64;
        let mut f = self.file.lock().unwrap();
        f.seek(SeekFrom::Start(offset))?;
        f.write_all(buf)?;
        f.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn file_backed_read_write_round_trip() {
        let dir  = tempdir().unwrap();
        let path = dir.path().join("test.img");
        let dev  = FileBackedDevice::open(&path, 512, 16, "test").unwrap();

        let write_buf: Vec<u8> = (0..512).map(|i| (i % 256) as u8).collect();
        dev.write_block(3, &write_buf).unwrap();

        let mut read_buf = vec![0u8; 512];
        dev.read_block(3, &mut read_buf).unwrap();
        assert_eq!(write_buf, read_buf);
    }

    #[test]
    fn out_of_range_block_is_rejected() {
        let dir  = tempdir().unwrap();
        let path = dir.path().join("oor.img");
        let dev  = FileBackedDevice::open(&path, 512, 4, "oor").unwrap();
        assert!(matches!(dev.read_block(4, &mut vec![0u8; 512]), Err(StorageError::OutOfRange(4, 4))));
    }

    #[test]
    fn bad_buffer_size_is_rejected() {
        let dir  = tempdir().unwrap();
        let path = dir.path().join("bad.img");
        let dev  = FileBackedDevice::open(&path, 512, 4, "bad").unwrap();
        assert!(matches!(dev.read_block(0, &mut vec![0u8; 256]), Err(StorageError::BadBufferSize(256, 512))));
    }
}
