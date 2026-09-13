//! #197 userspace first. SFS is FUSE. Kernel inference is optional TCB.

pub fn sfs_v1() -> &'static str {
    "fuse"
}

pub fn kernel_inference() -> &'static str {
    "optional-tcb-risk"
}
