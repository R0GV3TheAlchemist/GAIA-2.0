# gaia-sfs

Semantic File System v0.1. Directory-backed store + hashed provenance +
hashed-token embeddings. POSIX tools operate on `Sfs::posix_root()`.

A FUSE mount is the same store exported through `fusermount` and is not
required for the v0.1 library tests (unprivileged CI has no /dev/fuse).
