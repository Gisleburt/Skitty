Skitty
======

Use sketch files with git.

Work in Progress
----------------

The component parts work in software but the binary isn't complete.

Features / To Do:
- [x] Find sketch file and/or git folder
- [x] Convert sketch to directory
- [x] Convert directory to sketch
- [x] Detect if sketch or git is newer
- [x] Detect file system changes
- [x] Once started, automatically update dir with sketch when sketch file saves
- [ ] If directory is newer, offer to extract at start
- [ ] Prettify JSON files


Installation
------------

Once its complete you can install from cargo, or download binaries from
[releases](https://github.com/Gisleburt/Skitty/releases).

```shell
$ cargo install skitty
```

Usage
-----

**Important:** This is likely to change prior to a 1.0 release depending on usage feedback.

Skitty works by extracting the sketch file into its component files, into directory of the same name
(sans the `.sketch` so that they can be version controlled in git. To get started, navigate to the
containing directory and type:

```shell
$ skitty watch <file>
```

Example
-------

```
my-workspace/
  .git/
  .gitignore
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
