# HTTP request and diff tools

There're two separate CLIs provided:

- xdiff: A diff tool for comparing HTTP requests. It could be used to compare the difference between production staging or two versions of the same API.
- xreq: A tool to build HTTP requests based on predefined profiles. It could be used to replace curl/httpie for building complicated HTTP requests.

## xdiff

### Configuration

You can configure multiple profiles for xdiff. Each profile is identified by a name. Inside a profile you can define the details of the two requests (method, url, query params, request headers, request body), and also what part of the response should be skipped for comparison (currently only headers could be skipped).

```yaml
---
rust:
  request1:
    method: GET
    url: https://www.rust-lang.org/
    headers:
        user-agent: Aloha
    params:
      hello: world
  request2:
    method: GET
    url: https://www.rust-lang.org/
    params: {}
  response:
    skip_headers:
      - set-cookie
      - date
      - via
      - x-amz-cf-id
```

### How to use xdiff?

You can use `cargo install xdiff-live` to install it (need help to [install rust toolchain](https://rustup.rs/)?). Once finished you shall be able to use it.

```trycmd
$ xdiff-live --help
xdiff-live [..]
Diff two http requests and compare the difference of the responses

USAGE:
    xdiff-live <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help     Print this message or the help of the given subcommand(s)
    parse    Parse URLs to generate a profile
    run      Diff two API responses based on given profile

```

```trycmd
$ xdiff-live run --help
xdiff-live-run 
Diff two API responses based on given profile

USAGE:
    xdiff-live run [OPTIONS] --profile <PROFILE>

OPTIONS:
    -c, --config <CONFIG>
            Configuration to use

    -e, --extra-params <EXTRA_PARAMS>
            Overrides args. Could be used to override the query, headers and body of the request.
            For query params, use `-e key=value`. For headers, use `-e %key=value`. For body, use
            `-e @key=value`

    -h, --help
            Print help information

    -p, --profile <PROFILE>
            Profile name

```
