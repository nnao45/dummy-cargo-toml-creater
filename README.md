# dummy-cargo-toml-creater
dummy-cargo-toml-creater for docker cache

## Motivation

### Engilish
It's nice to have Docker build, but I just want to cache dependent packages.
But to do that, first write `COPY Cargo.toml Cargo.lock. /` To Dockerfile,
It's nice to build, but every time the version of the package itself goes up, for example, the hash of `Cargo.toml` or `Cargo.lock` changes.
Even a minor bug fix, the dependency package has to be rebuilt.

So we introduce a technique called `dummy-cargo-toml-creater`.
Just look at the source `./src/main.rs`, but it's only 70 lines of code.
We have implemented only the minimum implementation so as not to do anything extra or suspicious.

In other words, with the approach of not changing the versions of Cargo.toml and Cargo.lock,
The only fix is ​​to fix the package version with `0.1.0` only when installing dependencies.

The final image is built with the original Cargo.toml and Cargo.lock, so
There is no problem with the version of the cargo package in the binary, and if you add a dependency package, docker's cache will also work and not work.

### Japanese
Docker buildするのはいいんだが、依存パッケージだけキャッシュさせたい。
しかしそれをするためにDockerfileに最初に `COPY Cargo.toml Cargo.lock ./` とか書いて、
ビルドするのはいいんだが、例えばそのpackage自体のversionが上がるたびに `Cargo.toml` または `Cargo.lock` のハッシュが変更され、
ほんの些細なバグfixでも依存パッケのビルドし直し・・・なんて面倒なんだ。

そこで `dummy-cargo-toml-creater` という手法を紹介する。
これはソース `./src/main.rs` をみてくれればいいが、たった70行のコードだ。
余計なことや怪しいことはしてないよう、最低限の実装しかしていない。

つまり、Cargo.tomlとCargo.lockのversionを変えなければいいというアプローチで、
依存関係のインストール時のみパッケージバージョンを `0.1.0` で常に固定して解決しようと事だ。

最終的なイメージでは元のCargo.tomlとCargo.lockでビルドするので、
バイナリ内のcargoパッケージのバージョンには問題無く、仮に依存パッケージを追加した時はまたdockerのキャッシュは使用されずに上手くいく。

## How to use?

1. install

```bash
$ cargo install dummy-cargo-toml-creater
```

1-a. maybe write gitignore DummyVersion.toml/lock

```bash
$ echo DummyVersion.toml >> ./.gitignore
$ echo DummyVersion.lock >> ./.gitignore
```

2. Create DummyVersion.toml

```bash
$ ~/.cargo/bin/dummy-cargo-toml-creater
$ ls ./DummyVersion.toml
./DummyVersion.toml
$ ls ./DummyVersion.lock
./DummyVersion.lock
```

3. create Dockerfile

old
```Dockerfile
FROM ekidd/rust-musl-builder:nightly-2019-04-25 as builder

RUN mkdir /tmp/app
WORKDIR /tmp/app
COPY . .
RUN sudo chown -R rust:rust .
RUN cargo build --release
```

new
```Dockerfile
FROM ekidd/rust-musl-builder:nightly-2019-04-25 as builder

## Build Cache Dependency Library
RUN mkdir /tmp/app
WORKDIR /tmp/app
## Build Dependency Library with DummyVersion.toml/lock
COPY DummyVersion.toml ./Cargo.toml
COPY DummyVersion.lock ./Cargo.lock
RUN mkdir -p src/ && \
    touch src/lib.rs
RUN sudo chown -R rust:rust .
RUN cargo build --release
## Build Base Library with Cargo.toml/lock
COPY ./src/ ./src/
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
RUN sudo chown -R rust:rust .
RUN cargo build --release
```

4. Run docker build
```bash
$ $ docker build -t nnao45/dummy .
[+] Building 119.7s (18/18) FINISHED                                                                                           
 => [internal] load .dockerignore                                                                                         0.0s
 => => transferring context: 2B                                                                                           0.0s
 => [internal] load build definition from Dockerfile                                                                      0.0s
 => => transferring dockerfile: 584B                                                                                      0.0s
 => [internal] load metadata for docker.io/ekidd/rust-musl-builder:nightly-2019-04-25                                     2.5s
 => [1/12] FROM docker.io/ekidd/rust-musl-builder:nightly-2019-04-25@sha256:e12231fc754848ccf3865d1e4e80204125c6d77baaa9  0.0s
 => CACHED [2/12] RUN mkdir /tmp/app                                                                                      0.0s
 => CACHED [internal] helper image for file operations                                                                    0.0s
 => [internal] load build context                                                                                         0.1s
 => => transferring context: 24.98kB                                                                                      0.0s
 => [3/12] COPY DummyVersion.toml ./Cargo.toml                                                                            0.7s
 => [4/12] COPY DummyVersion.lock ./Cargo.lock                                                                            0.9s
 => [5/12] RUN mkdir -p src/ &&     touch src/lib.rs                                                                      1.6s
 => [6/12] RUN sudo chown -R rust:rust .                                                                                  0.7s
 => [7/12] RUN cargo build --release                                                                                     99.0s
 => [8/12] COPY ./src/ ./src/                                                                                             0.8s
 => [9/12] COPY Cargo.toml ./Cargo.toml                                                                                   1.2s
 => [10/12] COPY Cargo.lock ./Cargo.lock                                                                                  0.8s
 => [11/12] RUN sudo chown -R rust:rust .                                                                                 4.0s
 => [12/12] RUN cargo build --release                                                                                     2.7s
 => exporting to image                                                                                                    4.2s
 => => exporting layers                                                                                                   4.1s
 => => writing image sha256:e95b11c44810ef4abc3e5a18998b4ba20df9871d5dbcfdbf7a656d992e802857                              0.0s
 => => naming to docker.io/nnao45/dummy                                                                                   0.0s
```

5. Fix Cargo.toml
```bash
$ vi Cargo.toml
- version = "0.0.1"
+ version = "0.0.2"
```

4. Reun docker build
```bash
$ docker build -t nnao45/dummy .
[+] Building 14.2s (18/18) FINISHED                                                                                            
 => [internal] load build definition from Dockerfile                                                                      0.1s
 => => transferring dockerfile: 44B                                                                                       0.0s
 => [internal] load .dockerignore                                                                                         0.0s
 => => transferring context: 2B                                                                                           0.0s
 => [internal] load metadata for docker.io/ekidd/rust-musl-builder:nightly-2019-04-25                                     2.5s
 => [1/12] FROM docker.io/ekidd/rust-musl-builder:nightly-2019-04-25@sha256:e12231fc754848ccf3865d1e4e80204125c6d77baaa9  0.0s
 => CACHED [internal] helper image for file operations                                                                    0.0s
 => [internal] load build context                                                                                         0.0s
 => => transferring context: 11.46kB                                                                                      0.0s
 => CACHED [2/12] RUN mkdir /tmp/app                                                                                      0.0s
 => CACHED [3/12] COPY DummyVersion.toml ./Cargo.toml                                                                     0.0s
 => CACHED [4/12] COPY DummyVersion.lock ./Cargo.lock                                                                     0.0s
 => CACHED [5/12] RUN mkdir -p src/ &&     touch src/lib.rs                                                               0.0s
 => CACHED [6/12] RUN sudo chown -R rust:rust .                                                                           0.0s
 => CACHED [7/12] RUN cargo build --release                                                                               0.0s
 => CACHED [8/12] COPY ./src/ ./src/                                                                                      0.0s
 => [9/12] COPY Cargo.toml ./Cargo.toml                                                                                   0.5s
 => [10/12] COPY Cargo.lock ./Cargo.lock                                                                                  0.9s
 => [11/12] RUN sudo chown -R rust:rust .                                                                                 1.7s
 => [12/12] RUN cargo build --release                                                                                     6.0s
 => exporting to image                                                                                                    1.6s
 => => exporting layers                                                                                                   1.5s
 => => writing image sha256:7904fe7a43ea67e850591d7b6c0b827a97be625b0be447fa028d0e011a2a3cb8                              0.0s
 => => naming to docker.io/nnao45/dummy                                                                                   0.0
```

cache it!!
