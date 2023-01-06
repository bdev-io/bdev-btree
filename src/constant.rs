// EMPTY YET

pub const DISK_BLOCK_SIZE: usize = 512; // DOC : Disk Block Size ( 디스크 블록 사이즈 ) FIXED.
// DOC : B-Tree Magic Header [51 4C 47 4C 54 52 45 45] "QLGLTREE"
pub const MAGIC_HEADER: [u8; 8] = [0x51, 0x4C, 0x47, 0x4C, 0x54, 0x52, 0x45, 0x45];
// TODO : MAGIC HEADER를 포함하여, 파일 구조체 활성화
