# **Che-Backa**: **Ch**ecked **Back**up **A**utomation 

An automated simple yet powerful backup solution that checks the integrity of the backups by using **MD5 checksums** and **meta data**. 

# THE PROJECT IS STILL IN EARLY CONCEPT PHASE

## Concept
- Allows daily, weekly and monthly versioned backups
- Before each backup, all the files are compared to the newest backup
- MD5 hashes and file meta data are compared
- There are three possible outcomes:
  1. MD5 and meta data **the same** -> No action required, because it is the same file.
  2. MD5 and meta data **different** -> File will be versioned, because some changes have happened to it.
  3. MD5 **different** and meta data **the same** -> Possible hard drive corruption. File will be versioned. Since it might be a drive issue, it will be reported.
- Versioning means appending "_v%0d" to the file name
- The meta data used are **file size** and **edit date**
