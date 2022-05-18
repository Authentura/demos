Vulnerable systems and exploit demos for teaching.
==================================================


This repository holds various vulnerable applications and live demos that we use to demonstrate security concepts.





SQL Injection
-------------
This demo demonstrates a very simple SQL injection example.


Build and run Dockerfile:
```
docker build -t sqldemo .
```
```
podman run -p 3000:3000 sqldemo
```
