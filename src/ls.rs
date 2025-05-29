/// Usage
/// ```
/// let file_kind =  CrustFileKind::from_mode(st_mode);
/// println!("Type: {}", file_kind.as_str());
/// ```
enum CrustFileKind {
    Regular,
    Directory,
    Symlink,
    CharDevice,
    BlockDevice,
    Fifo,
    Socket,
    Unknown
}

impl CrustFileKind {
    pub fn from_mode(mode: libc::mode_t) -> Self {
        match mode & libc::S_IFMT {
            libc::S_IFREG => CrustFileKind::Regular,
            libc::S_IFDIR => CrustFileKind::Directory,
            libc::S_IFLNK => CrustFileKind::Symlink,
            libc::S_IFCHR => CrustFileKind::CharDevice,
            libc::S_IFBLK => CrustFileKind::BlockDevice,
            libc::S_IFIFO => CrustFileKind::Fifo,
            libc::S_IFSOCK => CrustFileKind::Socket,
            _ => CrustFileKind::Unknown,
        }    
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CrustFileKind::Regular => "Regular File",
            CrustFileKind::Directory => "Directory",
            CrustFileKind::Symlink => "Symbolic Link",
            CrustFileKind::CharDevice => "Character Device",
            CrustFileKind::BlockDevice => "Block Device",
            CrustFileKind::Fifo => "FIFO (Named Pipe)",
            CrustFileKind::Socket => "Socket",
            CrustFileKind::Unknown => "Unknown",
        }
    }
}

#[derive(Debug)]
struct CrustFilePermissions {
    owner_read: bool,
    owner_write: bool,
    owner_exec: bool,
    group_read: bool,
    group_write: bool,
    group_exec: bool,
    others_read: bool,
    others_write: bool,
    others_exec: bool
}

impl CrustFilePermissions {
    pub fn from_mode(mode: libc::mode_t) -> Self {
        Self {
            owner_read:  mode & libc::S_IRUSR != 0,
            owner_write: mode & libc::S_IWUSR != 0,
            owner_exec: mode & libc::S_IXUSR != 0,
            group_read:  mode & libc::S_IRGRP != 0,
            group_write: mode & libc::S_IWGRP != 0,
            group_exec: mode & libc::S_IXGRP != 0,
            others_read:  mode & libc::S_IROTH != 0,
            others_write: mode & libc::S_IWOTH != 0,
            others_exec: mode & libc::S_IXOTH != 0,
        }
    }

    pub fn as_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            if self.owner_read { "r" } else { "-" },
            if self.owner_write { "w" } else { "-" },
            if self.owner_exec { "x" } else { "-" },
            if self.group_read { "r" } else { "-" },
            if self.group_write { "w" } else { "-" },
            if self.group_exec { "x" } else { "-" },
            if self.others_read { "r" } else { "-" },
            if self.others_write { "w" } else { "-" },
            if self.others_exec { "x" } else { "-" },
        )
    }
}


struct CrustFileMetadata {
    /// File name 
    name: String,
    /// File type
    kind: CrustFileKind,
    /// File size
    size: u128,
    /// File permissions
    permissions: CrustFilePermissions,
    /// The UID of the owner 
    owner: u32,
    /// epoch timesamp
    last_modified: i64 
}

impl CrustFileMetadata {
    fn new(name: String, kind: libc::mode_t, size: u128, permissions: libc::mode_t, owner: u32, last_modified: i64) -> Self {
        CrustFileMetadata {
            name,
            kind: CrustFileKind::from_mode(kind),
            size,
            permissions: CrustFilePermissions::from_mode(permissions),
            owner,
            last_modified
        }
    }
}

unsafe extern "C" {
    pub fn opendir(name: *const libc::c_char) -> *mut libc::DIR;
    pub fn readdir(dirent: *mut libc::DIR) -> *mut libc::dirent;
    pub fn closedir(dirent: *mut libc::DIR) -> libc::c_int;
}

pub fn list_directory(path: &str) {
    let mut files : Vec<CrustFileMetadata> = Vec::new();

    let c_path = std::ffi::CString::new(path).expect("CString::new failed");

    unsafe {
        let dir = opendir(c_path.as_ptr());
        
        if dir.is_null() {
            eprintln!("Failed to open directory");
            return;
        }

        loop {
            let entry = readdir(dir);

            if entry.is_null() {
                break;
            }

            let directory_name = std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()).to_string_lossy();

            if directory_name == "." || directory_name == ".." {
                continue;
            }

            let full_path = format!("{}/{}",path, directory_name);
            let c_full_path = std::ffi::CString::new(full_path.clone()).expect("Failed to create full path string");

            let mut stat_buffer: libc::stat = std::mem::zeroed();

            if libc::stat(c_full_path.as_ptr(), &mut stat_buffer) != 0 {
                eprintln!("Failed to stat file: {}", full_path);
                continue;
            }

            let file_meta = CrustFileMetadata::new(directory_name.into_owned(), stat_buffer.st_mode, stat_buffer.st_size as u128, stat_buffer.st_mode, stat_buffer.st_uid, stat_buffer.st_mtime);

            files.push(file_meta);
        }

        closedir(dir);
    }
}

/// Sorts the file names alphabetically (base command)
pub fn sort_filenames(filenames: &mut Vec<&str>) {
    todo!() 
}

/// This will output in colour (base command)
pub fn colour_code(filenames: &Vec<&str>) {
}

/// Skip any hidden files in the dir (base command)
/// Should also output in coloumns and rows
pub fn skip_hidden(filenames: &mut Vec<&str>) {
}

pub fn sort_file_size() {
}

/* -a shows all files including hidden files (all files)
 -l shows permissions, owner, size, modification time (long format)
 -h human readable sizes ???
 -R recursive list subdirs too
 -S sort by file size
 -t sort by modification time
 --colour colourize the output
*/
