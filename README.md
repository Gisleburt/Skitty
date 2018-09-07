Skitty
======

Use sketch files with git.

Installation
------------

Clone this repository and run `cargo install`.

Usage
-----

**Important:** This is likely to change prior to a 1.0 release depending on usage feedback.

Skitty works by extracting the sketch file into its component files, into directory of the same name
(sans the `.sketch` so that they can be version controlled in git. To get started, navigate to the
containing directory and type:

```shell
$ skitty watch <file>
```

Skitty will look for which is newer, the directory or the file. If either the directory or the file
doesn't exist, they will be created from the one that does exist.

Example
-------

```
skitty/
  my-project.sketch
  my-project/
    pages/
      BEBD1391-55E7-4B33-9715-6C38F25EF254.json
    previews/
      preview.png
    document.json
    meta.json
    user.json
```
