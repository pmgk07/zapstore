# Zapnode

Single node persistent storage.

## Components
* Memtable - 
  - In-memory key value storage with Write Ahead Logging (WAL) for crash recovery.
  - GET(key), PUT(key, value), DELETE(key), LIST
  - RESTORE - from WAL in the event of crash
* FlushQueue
  - DLL of memtable objects
  - GET(key), PUT(key, value), DELETE(key), LIST
* LSM Tree -
  - Sparse index of SSTables.
  - LOAD(sstKey), INSERT(SSTable) // first element of SSTable is sstKey
  - GET_CLOSEST_SST(key) // max(sstKey <= key) across all elements
  - COMPACT() // merge sstable
* StorageNode Interface
  - GET(key)
  - PUT(key)
  - DELETE(key)
  - LIST
* LSMStorageNode impl StorageNode
  - memtable
  - flushQueue
  - lsmTree

## Data Flow
### PUT (key, value)
(k,v) --(PUT)--> memtable --(enque_memtable)--> memtable flush queue --(deque_memtable_and_sort)--> sstable --(PUT)--> lsm

### GET (key)
(k,v) --(GET)--> memtable --(IF_NOT_EXISTS)--> lookup flush queue --(IF_NOT_EXISTS)--> lookup closest LSM key, load & lookup sstable.

### DELETE (key)
* Lazy delete via timestamps, workflow similar to PUT
* Will be implemented at a later stage of the project

### LIST
* (GET memtable) U (GET memtables in flush queue) U (GET sstable in lsm)
