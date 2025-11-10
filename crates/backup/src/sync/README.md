# **Cover – A Simple Backup Sync Tool**

Cover is a command-line tool to help you keep two folders in sync. It copies new files, updates modified ones, and optionally deletes files that only exist in the destination. You can run it normally, preview changes with a dry-run, or enable detailed logging.

## **Features**

* **Sync files** between a source and destination folder
* **Copy only changed files**
* **Detect modified files** and update them
* **Detect new files**
* **Optionally delete extra files** from the destination
* **Dry run mode** to preview what will happen
* **Verbose mode** for full detailed logs

## **Basic Usage**

```
cover sync --source <path> --destination <path> <FLAG>
```

You must pick **exactly one** of the following flags:

| Flag             | Meaning                                |
| ---------------- | -------------------------------------- |
| `--changed-only` | Sync only new or modified files        |
| `--delete`       | Remove all files from destination      |
| `--dry-run`      | Show what would happen, but do nothing |
| `--verbose`      | Show detailed logs of the sync process |

## **Examples**

### **Copy only changed files**

```
cover sync -s src -d dest --changed-only
```

### **Preview changes without modifying anything**

```
cover sync -s src -d dest --dry-run
```

### **Delete all destination files**

```
cover sync -s src -d dest --delete
```

### **Verbose logs for debugging**

```
cover sync -s src -d dest --verbose
```

## **How It Works (Simple Explanation)**

* The tool scans both source and destination folders.
* It checks:

  * Which files exist only in the source
  * Which files exist only in the destination
  * Which files were modified
* Based on the flag you choose, it performs the appropriate action:

  * Copy new files
  * Update modified files
  * Delete unmatched files
  * Or simply show logs (dry-run / verbose)

It uses file timestamps to decide whether a file is “modified” or not.

## **Notes**

* Source and destination must both be valid folders.
* Only one flag can be used at a time.
* Errors appear when required directories or files are missing.
