# Zapnode

Single node persistent storage.

## Packages
* Memtable - 
  - In-memory key value storage with Write Ahead Logging (WAL) for crash recovery.
  - GET(key), PUT(key, value), DELETE(key), LIST
  - RESTORE - from WAL in the event of crash // sync at object creation
  - FLUSH - sort and flush memtable to sstable and add first key of sstable to the lsm tree // can happen async
* LSM Tree -
  - Sparse indexing of SSTables.
  - LOAD(sstKey), INSERT(SSTable) // first element becomes the key
  - GET_CLOSEST_SST(key) // max(sstKey <= key)
  - COMPACT() // merge sstable
* NVLog / Non-Volatile Log -
  - WAL & SSTable flush queue
  - Dumps (event, data) for recovery purposes

## Data Flow
### PUT (key, value)
(k,v) --(PUT)--> memtable --(enque_memtable)--> memtable flush queue --(deque_memtable_and_sort)--> sstable --(PUT)--> lsm

### GET (key)
(k,v) --(GET)--> memtable --(IF_NOT_EXISTS)--> lookup flush queue --(IF_NOT_EXISTS)--> lookup closest LSM key, load & lookup sstable.

### DELETE (key)
* Lazy delete, workflow similar to PUT

### LIST
* (GET memtable) U (GET memtables in flush queue) U (GET sstable in lsm)
