# Zapstore
Experimental Key Value store being built because why not ?

# Usage
Built for the lolz. Not even meant for POCs.
You seriously want to use a key value store some random guy built on the internet ?

# Design Ramblings

- Built in Go for simplicity, might switch to Rust+C later on for performance
- APIs (for now)
  - GET(key)
  - PUT(key, value)
  - DELETE(key)
  - LIST_KEYS(void) - Useful for internal usage. Perf intensive, so ideally should work against a time snapshot of keys existing before this query was fired

Single Instance
- Choose consistency, ie, have WAL per thread
- In-memory - hashtable, write back and truncate WAL
- On Disk layout
  - Choices - LSM / B+ / B-ε / STB-ε
  - Using LSM to start off with for simplicity
  - Next iter will be using B-ε as I wanted to understand more about write buffers and how well it can trade off
  - I want to keep this pluggable so that I can switch to different implementations. Long-standing goal is to model it like MySQL storage engine as all on-disk layout trade something for some other thing

Scaling Horizontally
- Consistent Hashing to distribute/redistribute load (control+data) across instances
- Leaderless / P2P, conflict resolution via RAFT. I'll be using PAXOS if I face issues
