# up

> Hey, you up?

`up` checks if a local server is healthy by performing an HTTP GET request at the specified path and port. If the server responds with a 200 OK, `up` exits with a 0 status code. Otherwise, `up` exits with a 1 status code, indicating the health check has failed.

Why is this useful? Amazon ECS provides support for health checking of containers defined in ECS Tasks, but it does not provide a client, nor the facilities, to perform the health check request. Instead, users must add a client, such as `curl`, to their container image in order to support the health check. `curl` is an amazing tool, but quite a bit more than whats necessary for a simple health check. Therefore, `up` was created - to simply ask "Hey, you up?"

## Usage

By default, `up` checks for a HTTP 200 OK response on localhost port `80` at the path `/healthz`. The port and/or path, can be changed via environment variables or arguments.

### Environment Variables

`UP_PATH` and `UP_PORT` environment variables will modify the path and port that `up` checks.

```sh
UP_PATH=/healthy UP_PORT=3000 up
```

### Arguments

Using arguments, the first argument is the path, and the second argument is the port:

```sh
up --port 3000 --path /healthy
```

You can mix and match environment variables and arguments - environment variables will take precedence over arguments.
