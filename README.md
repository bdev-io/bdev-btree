# BTree



## TODO

- [ ] 간단한 B-Tree 구현
- [ ] 확장성 있도록 DataType 넣기
- [ ] Key값에 DataType 넣기
- [ ] File Structure 넣기



## FILE Structure

MAGIC HEADER
Q  L  G  L  T  R  E  E
51 4C 47 4C 54 52 45 45



> C TYPE File Struct

```rust
type BTree struct {
  FileHeader *head;
  FIleNode **node;
}

type FileHeader struct {
  
}

type FileNode struct {
  
}
```