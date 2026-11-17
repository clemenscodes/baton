# The ubiquitous language

| Word | What it means |
|---|---|
| Block Device | A device file read and written in blocks, such as a disk. |
| Character Device | A device file read and written as a stream, as every device file of a GPU is. |
| Device File | A file under /dev of a GPU, named by its device kind, its major and its minor, which the device program locks. |
| Device Kind | Whether a device file is a character device or a block device. |
| Major | The major number of a device file, which names its driver, such as 195 for the files of nvidia. |
| Minor | The minor number of a device file, which names the device of its driver. |
| Stack Lock Core Device File | The crate stack_lock_core_device_file, a device file of a GPU, its kind, its major and its minor. |
| Text | The text of a device file or of a device kind. |
