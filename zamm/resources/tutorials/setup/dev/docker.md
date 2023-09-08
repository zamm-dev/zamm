# Installing Docker on Ubuntu

Follow [these instructions](https://docs.docker.com/engine/install/ubuntu/):

```bash
$ sudo apt-get update
$ sudo apt-get install ca-certificates curl gnupg
$ sudo install -m 0755 -d /etc/apt/keyrings
$ curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
$ sudo chmod a+r /etc/apt/keyrings/docker.gpg
$ echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
$ sudo apt-get update
$ sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

Check that it works:

```bash
$ sudo docker run hello-world
Unable to find image 'hello-world:latest' locally
latest: Pulling from library/hello-world
719385e32844: Pull complete 
Digest: sha256:dcba6daec718f547568c562956fa47e1b03673dd010fe6ee58ca806767031d1c
Status: Downloaded newer image for hello-world:latest

Hello from Docker!
This message shows that your installation appears to be working correctly.

To generate this message, Docker took the following steps:
...
```

## Tips and tricks

### Image debugging

If you want to open a shell to explore a Docker image, do

```bash
$ docker run -d --name throwaway tauri-build tail -f /dev/null
$ docker exec -it throwaway /bin/bash
```

You can skip the first step if you already have the container built.

If you changed your Dockerfile and want to destroy and recreate a container based on the newer image, you can do

```bash
$ docker stop throwaway                
throwaway
$ docker rm throwaway
throwaway
```

If you see an error such as

```
Error response from daemon: Cannot kill container: throwaway: Container 9e1f033ae66fc64fa335332033b706fa77f8acf41a7e7937e9740338914f69ea is not running
```

that means the container is already stopped, and you can proceed directly to `docker rm`.

### Clean and reset

To completely nuke all existing Docker images, containers, and volumes, do

```bash
$ docker system prune -a
```

### Python installation

If you get an error such as

```
0.196 --2023-09-08 06:34:25--  https://www.python.org/ftp/python/3.11.4/Python-3.11.4.tgz
0.206 Resolving www.python.org (www.python.org)... 199.232.144.223, 2a04:4e42:64::223
0.212 Connecting to www.python.org (www.python.org)|199.232.144.223|:443... connected.
0.218 ERROR: cannot verify www.python.org's certificate, issued by 'CN=GlobalSign Atlas R3 DV TLS CA 2023 Q2,O=GlobalSign nv-sa,C=BE':
0.218   Unable to locally verify the issuer's authority.
0.218 To connect to www.python.org insecurely, use `--no-check-certificate'.
```

try installing `ca-certificates` as mentioned [here](https://unix.stackexchange.com/a/445609).
