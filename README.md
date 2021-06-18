# andrew's cli

This is a little CLI to help automate myself.

It is also a way for me to learn and experiment with Rust.

## usage

```bash
ac
```

## git

Flip the remote to either HTTPS or SSH

```bash
ac git remote --flip
```

Change the remote to SSH

```bash
ac git remote --to-ssh
```

Change the remote to HTTPS

```bash
ac git remote --to-https
```

Check the git status of all repos in the specified `dir`. If the `--modified` or
`-m` flag is passed in only repos that have changes in them will be displayed.
Otherwise, all repos will be displayed.

```bash
ac git status --dir ~/Github [--modified]
```

## permissions

A Unix file permission calculator.

To symbolic representation:

```bash
ac permissions convert --permission 640 --symbolic
```

To octal representation:

```bash
ac permissions convert --permission drwxrwxr-x --octal
```

> Note: the symbolic representation should always start with a letter or else
> it will be parsed as a command. e.g. `-rw-rw-r--` will error.
