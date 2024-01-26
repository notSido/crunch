# crunch
crunch is an archive tool built on rust. meant to package lots of single files into one comfy package

you interface with the program via cli, and it can be called using the **crunch** command

## Commands

**start:** crunch

* **compress** *(file1) (file2)* *(desired_archive_name*.crunch *<- optional)*
* **extract** *(archive name)* *(folder to extract files to <- optional)*

## usage examples

### Compress files into an archive
**crunch compress testfile1.txt testfile2.txt veryclevername.crunch <- (optional)**

### Extract files from archive
**crunch extract veryclevername.crunch targetfolder/ <- (optional)** 
