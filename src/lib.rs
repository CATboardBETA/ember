use std::fmt::Debug;
use std::io::{Write, Read, Seek, SeekFrom};



/// Trait for implementing interface to
pub trait StorageDrive<const size: u128>: Debug {
    /// Load bytes at location `at` from persistent memory into `into`.
    fn load(&self, at: u128, into: &mut [u8]);

    /// Save bytes at location `at` into persistent memory from `from`.
    fn save(&self, at: u128, from: &[u8]);
    
    /// Get storage device size
    fn size(&self) -> u128 {
        size
    }
}

/// This type provides an interface to the file system.
#[derive(Debug)]
pub struct FileSystem<const size: u128, D: StorageDrive<size>> {
    drive: D,
}

impl<const size: u128, D: StorageDrive<size>> FileSystem<size, D> {
    /// Load file system.
    pub const fn new(drive: D) -> FileSystem<size, D> {

        FileSystem::<size, D> { drive }
    }

    /// Check that file system is valid; `Ok()` if valid, `Err()` if not.
    pub fn check(&self) -> Result<(), ()> {
        for i in 0..size {
            let mut buf = [0u8; 1];
            self.drive.load(i, &mut buf);
            if buf[0] != 0 {
                return Err(());
            }
        }
        Ok(())
    }

    /// Initialize file system (page 0)
    pub fn init(&self, name: &str) {
        todo!()
    }

    /// Get the name of this file system.
    pub fn name(&self) -> &str {
        let mut name = &mut [0u8; 200];

        self.read(i, &mut buf);

        std::str::from_utf8_mut(name).unwrap()
    }

    pub unsafe fn read(&self, at: u128, len: usize,  into: &mut [u8]) {
        let mut buf = [0u8; 1];
        for i in 0..len {
            self.drive.load(at + i as u128, &mut buf);
            into[i] = buf[0];
        }
    }

    /// Allocate space for a new empty file (initially 1 page).
    pub fn make(&self, tags: &Tags) -> FileId {
        todo!()
    }

    /// Do a fuzzy search on `matches` for tags of type `kind` (`None` for all
    /// tag types), through files that match `filters`
    pub fn find(&self, filters: &Tags, kind: Option<TagKind>, matches: &str)
        -> FileId
    {
        todo!()
    }

    /// Load data into `data` from file `file` at location `at`.
    pub fn load(&self, file: FileId, at: u64, data: &mut [u8]) {
        todo!()
    }

    /// Save data from `data` at location `at` in file `file`.
    pub fn save(&self, file: FileId, at: u64, data: &[u8]) {
        todo!()
    }

    /// Iterate over all files in the file system.
    pub fn iter(&self) -> impl Iterator<Item = Metadata> {
        todo!();
        [].into_iter()
    }
}

/// Kind of file tag.
#[repr(u8)]
#[derive(Debug)]
pub enum TagKind {
    Filename = 0,
    Project = 1,
    MimeType = 2,
    System = 3,
}

/// A tag has two parts, a kind and a name (text)
#[derive(Debug)]
pub struct Tag {
    kind: TagKind,
    name: String,
}

/// A collection of tags.
type Tags = [Tag; 16];


/// Newtype for File index.
pub type FileId = u64;

/// Date and time (32 bit date, 32 bit time)
///
/// See https://github.com/ardaku/ardaku/blob/main/SYSCALLS.md#fn-now for spec.
pub type DateTime = u64;

/// File metadata
#[derive(Debug)]
pub struct Metadata {
    /// ID of the file
    pub index: FileId,
    /// When the file was first created
    pub creation: DateTime,
    /// Last time the file was modified
    pub modification: DateTime,
    /// Last time the file was accessed
    pub last_access: DateTime,
    /// Tag metadata
    pub tags: Tags,
}

// Byte
const B: usize = 1;
// Kibibyte
const KB: usize = 1024 * B;
// Mebibyte
const MB: usize = 1024 * KB;
// File system size
const FS: usize = 256 * MB;

/// File-Backed Storage Drive Emulator
#[derive(Debug)]
pub struct Emulator(std::fs::File);

impl Emulator {
    pub fn new(clear: bool) -> Self {
        let file = std::fs::File::create("filsystem.emb").unwrap();

        // Clear file if flag is set.        
        if clear {
            file.set_len(0);
        }
        
        // File to 256 mb
        file.set_len(FS.try_into().unwrap());

        Emulator(file)
    }
}

impl<const size: u128> StorageDrive<size> for Emulator {
    /// Load bytes at location `at` from persistent memory into `into`.
    fn load(&self, at: u128, into: &mut [u8]) {
        (&self.0).seek(SeekFrom::Start(at.try_into().unwrap())).expect("file system: out of bounds");
        (&self.0).read_exact(into).expect("file system: failed load");
        (&self.0).flush().unwrap();
    }

    /// Save bytes at location `at` into persistent memory from `from`.
    fn save(&self, at: u128, from: &[u8]) {
        (&self.0).seek(SeekFrom::Start(at.try_into().unwrap())).expect("file system: out of bounds");
        (&self.0).write_all(from).expect("file system: failed save");
        (&self.0).flush().unwrap();
    }

    /// Get storage device size
    fn size(&self) -> u128 {
        FS.try_into().unwrap()
    }
}

/// Driver to test functionality.
pub struct TestDriver;

impl TestDriver {
    fn init() {

    }
}

#[cfg(test)]
mod tests {
    use super::{FileSystem, Emulator};

    #[test]
    fn init() {
        // Open the filesystem
        let fs = FileSystem::new(Emulator::new(true /*clear*/));
        // Initialize filesystem
        fs.init("Test Filesystem");
        // Verify valid filesystem
        assert!(fs.check().is_ok());

        // Open the filesystem
        let fs = FileSystem::new(Emulator::new(false /*don't clear*/));
        // Check name persisted
        assert_eq!("Test Filesystem", fs.name());
    }
}
