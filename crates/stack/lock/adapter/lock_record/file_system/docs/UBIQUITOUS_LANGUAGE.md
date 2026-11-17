# The ubiquitous language

| Word | What it means |
|---|---|
| Compose Device File | To turn the record of a device file into a device file. |
| Compose Input Error | To turn an error of a read into an error of the record. |
| Compose Text | To write an error as a text. |
| Configuration | The record configuration of the adapter. |
| Demand Document | A demand as the record of its lock holds it. |
| Document | The error of a record that holds no lock. |
| Entries | The entries of the record directory. |
| Entry | An entry of the record directory. |
| Extension | The extension of the path of an entry of the record directory. |
| File Document | A device file as the record of its lock holds it. |
| Graphics Locks | The locks of every record. |
| Input | The error of a read of a record. |
| Lease Failure | The error of a lease of the records that failed. |
| Lease File | The open file the record lease locks with flock. |
| Lease Name | The name of the file of the lease. |
| Lease Path | The path of the file of the lease, lease in the record directory. |
| Lock Document | The record of a lock as the file of JSON holds it. |
| Lock Record | The adapter of the port LockRecord, which keeps a file of JSON for every lock in the record directory and leases them through flock on a file of its own. |
| Lock Record Port | The port LockRecord as the adapter imports it. |
| Output | The error of a write of a record. |
| Path | The path of a file of the records. |
| Record Configuration | The directory of the records of the locks. |
| Record Directory | The directory of the records of the locks, /run/baton unless an option names another. |
| Record Extension | The extension json of a record. |
| Record Lease | The exclusive lease of the records, an open file locked with flock, which the kernel ends when the file closes. |
| Record Name | The name of the file of the record of a lock. |
| Record Path | The path of the record of a lock, its name with the extension json in the record directory. |
| Record Paths | The paths of the records in the record directory. |
| Record Text | The text of the record of a lock. |
| Stack Lock Adapter Lock Record File System | The crate stack_lock_adapter_lock_record_file_system, the adapter of the port LockRecord of stack_lock_application over a file of JSON for every lock and a lease of flock. |
| Transfer Extension | The extension of the file a record is first written to before it replaces the record. |
| Transfer Path | The path of the file a record is first written to before it replaces the record. |
| Try Open Lease | To open the file of the lease and lock it exclusively, waiting for another lease to end. |
| Try Parse Lock | To read a lock from the text of its record. |
| User Document | A console or a framebuffer an acquisition unbound, as the record of its lock holds it. |
