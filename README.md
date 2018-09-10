Skitty
======

Sketch + Git + I love kitties! 😻

Use git to version sketch files

> **WARNING:** This is an early release, back up your sketch files before use!


Installation
------------

If you're into Rust, you can install with Cargo, otherwise download the binaries from the
[releases page](https://github.com/Gisleburt/Skitty/releases).

```shell
$ cargo install skitty
```

Usage
-----

**Important:** This is likely to change prior to a 1.0 release depending on usage feedback.

There are three functions:

**Extract:**

```shell
$ sketch extract my-designs.sketch
```

Will extract all of the files in your sketch file and place them into a folder of the same name,
here it would be `my-designs/`.

**Combine**

```
$ sketch combine my-design
```

Will combine all of the files in the `my-design` directory into a `my-design.sketch` file.

**Watch**

```shell
$ skitty watch my-design.sketch
```

Will watch for changes in your sketch file and automatically extract them when it changes.

Recommended usage:
------------------

Create a little workspace for your sketch files, initialise a git repository, and add `*.sketch` to
a `.gitignore`.

Copy sketch files into your workspace, it should look something like this

```
my-workspace/
  .git/
  .gitignore
  my-amazing-signup-flow.sketch
  my-awesome-login-flow.sketch
```

You can then use skitty to manage files that will be version controlled. From inside your workspace:

```shell
my-workspace $ skitty extact my-amazing-signup-flow
```

> Note, regardless of which direction you're going, whether you include the `.sketch` or not doesn't
matter.

```
my-workspace/
  .git/
  .gitignore
  my-amazing-signup-flow/
    pages/
      BEBD1391-55E7-4B33-9715-6C38F25EF254.json
    previews/
      preview.png
    document.json
    meta.json
    user.json
  my-amazing-signup-flow.sketch
  my-awesome-login-flow.sketch
```

You can now version control the extracted files!
