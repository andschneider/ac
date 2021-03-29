# andrew's cli

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

Check the git status of all repos in the specified `dir`

```bash
ac git status --dir ~/Github --verbose
```
