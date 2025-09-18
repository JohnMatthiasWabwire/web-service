[CLIDoc]: https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/documentation/htnet.md
[Deno]: https://deno.land/
[Greptime Database]: https://greptime.com/
[JSON]: https://www.json.org/json-en.html
[JWT]: https://jwt.io/
[MDN]: https://developer.mozilla.org/en-US/docs/Web/API
[Minio Database]: https://min.io/
[Postgres Database]: https://www.postgresql.org/
[Remote Sync]: https://rsync.samba.org/documentation.html
[Rust Language]: https://rust-lang.org
[SSH]: https://openssh.com/
[SystemD]: https://systemd.io/
[Traefik]: https://traefik.io/traefik/
[TypeScript Language]: https://www.typescriptlang.org/docs/
[WebAuthn]: https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API
[WebGL]: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API
[WebGPU]: https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API
[W3C]: https://w3.org/TR/
[YAML]: https://yaml.org/

<a href="https://github.com/HyaenaTechnologies/web-service">
  <h1>
    <picture>
      <img src="https://github.com/HyaenaTechnologies/web-service/blob/main/web/assets/ht_markdown.png" alt="">
    </picture>
  </h1>
</a>

# Web Service

Hyaena Technologies Web Service

## Features

- Environment Variables - (In Progress)
- [JavaScript Object Notation][JSON]
- [JavaScript Object Notation Web Token][JWT] - (In Progress)
- [YAML Ain't Markup Language][YAML] - (In Progress)

## Build

- [Rust][Rust Language]
- [TypeScript][TypeScript Language]
- [Deno Bundler][Deno]
- [System Daemon][SystemD]
- [Secure Shell Protocol][SSH]
- [RSync][Remote Sync]
- [Mozilla Developer Network Web Documentation][MDN]
- [Web Consortium Documentation][W3C]
- [Web Authentication][WebAuthn] - (In Progress)
- [Web Graphics Library][WebGL] - (In Progress)
- [Web GPU][WebGPU] - (In Progress)
- [Traefik Proxy][Traefik] - (In Progress)
- [Command Line Documentation][CLIDoc]

### Databases

- **_Object Storage:_** [Minio][Minio Database] - (In Progress)

- **_Relational:_** [Postgres][Postgres Database]

- **_Time Series:_** [Greptime][Greptime Database] - (In Progress)

### Compile Web Service

```shell
git clone

deno bundle ./web/src/*.ts --outdir ./web/build

cargo check

cargo test

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unkown-linux-gnu/release/htnet ./binary

./binary/htnet serve
```

### Install Web Service

```shell
sudo install ./htnet /usr/local/bin/
```
