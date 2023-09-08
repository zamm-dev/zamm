# Downloading files from remote servers

## scp

You can do

```bash
$ scp user@remote_host:/path/to/remote/file /path/to/local/file
```

## rsync

You can do

```bash
$ rsync -P -e ssh user@remote_host:/path/to/remote/file /path/to/local/file
```

The advantage of this over `scp` is that this allows for resumable downloads over slow internet connections.
