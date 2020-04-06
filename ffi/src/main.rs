use crate::rsync::compute_file_signature;

mod rsync {
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int, c_longlong, c_uchar};
    use std::path::{Path, PathBuf};

    use libc::{fclose, FILE, fopen, size_t};

    #[repr(C)]
    #[derive(Debug)]
    enum RsSyncResult {
        Done = 0,
    }

    #[repr(C)]
    enum MagicNumber {
        Delta = 0x72730236,
        MD4 = 0x72730136,
        Blake2Hash = 0x72730137,
    }

    #[repr(C)]
    // TODO: Unimplemented!
    pub struct RsyncStats {}

    #[link(name = "rsync")]
    extern "C" {
        fn rs_sig_file(
            old_file: *const FILE,
            sig_file: *const FILE,
            block_len: size_t,
            strong_len: size_t,
            sig_magic: MagicNumber,
            stats: *mut RsyncStats,
        ) -> RsSyncResult;
    }

    pub fn compute_file_signature<T: AsRef<Path>>(target_file: T) {
        let path = target_file.as_ref().to_str().unwrap();
        let output_path = {
            let mut path = PathBuf::from(target_file.as_ref());
            path.set_file_name(format!(
                "{}.sig",
                path.file_name()
                    .expect("Unable to get file name")
                    .to_str()
                    .expect("Unable to deserialize file")
            ));
            path.to_str().unwrap().to_owned()
        };

        unsafe {
            let target = CString::new(path).expect("unable to build path");
            let output = CString::new(output_path).expect("Unable to build output path");
            let input = fopen(
                target.as_ptr() as *const c_char,
                "rb".as_ptr() as *const c_char,
            );
            let output = fopen(
                output.as_ptr() as *const c_char,
                "wb".as_ptr() as *const c_char,
            );

            let result = rs_sig_file(
                input,
                output,
                16 as size_t,
                8 as size_t,
                MagicNumber::MD4,
                std::ptr::null_mut(),
            );

            dbg!(result);

            fclose(input);
            fclose(output);
        }
    }
}
fn main() {
    let mut sys_dir = std::env::current_dir().unwrap();
    sys_dir.push("Cargo.toml");

    compute_file_signature(sys_dir);
}
