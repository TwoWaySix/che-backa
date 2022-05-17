# **Che-Backa**: **Ch**ecked **Back**up **A**utomation 

An automated simple yet powerful backup solution that checks the integrity of the backups by using **MD5 checksums** and **meta data**. 

# THE PROJECT IS STILL IN EARLY CONCEPT PHASE


## Concept
- incremental backups with hard links
- MD5 hashes are compared
- There are three possible outcomes:
  1. MD5 and meta data **the same** -> No action required, because it is the same file.
  2. MD5 and meta data **different** -> File will be versioned, because some changes have happened to it.
  3. MD5 **different** and meta data **the same** -> Possible hard drive corruption. File will be versioned. Since it might be a drive issue, it will be reported.
- The meta data used are **file size** and **edit date**
