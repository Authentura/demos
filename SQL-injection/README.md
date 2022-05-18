SQL Injection
=============
This demo demonstrates a very simple SQL injection example.

Build and run the Dockerfile:

```bash
$ podman build -t sqldemo .
$ podman run -p 3000:3000 sqldemo
```

Alternatively, you can pull the container from the Authentura GHCR using the
following command:

```bash
$ podman pull ghcr.io/authentura/sql-injection-demo:latest

```
