<a name="readme-top"></a>

<div align="center">
  <a href="https://github.com/jkowiesk/go-in-yew">
    <img src="https://i.postimg.cc/qqKwfdwZ/white.png" alt="white side screen" />
  </a>

  <h3 align="center">Go In Yew</h3>

  <p align="center">
    Game of Go written in Yew framework, which is written in rust
    <br />
    <a href="https://github.com/jkowiesk/go-in-yew/issues">Report Bug</a>
  </p>
</div>

### About The Project

It consists of two applications, first is _./rust_go-client_ which is GUI for players to play game, second is _./rust_go-server_ which  is simple websocket server for accepting and processing players moves. Frontend connects to server and sends messages using websockets. Yew project compiles to WASM, which can be easily run in (Web browser)  

### Built With

- [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)][rust-url]
- [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)]

### Running the application

It is possible to run the client and server separately, as well as both components at the same time. Building and running the project has been automated using [`docker compose`](https://docs.docker.com/compose/install/). To launch both components, located in the root directory of the repository, run the command

```
docker compose up
```

> NOTE: After executing the above command, wait for a long while, as it takes up to several minutes for the container to download the crate.  

In order to run only one component, go to the corresponding subdirectory by executing the command 

```
cd rust_go-client
```

or 

```
cd rust_go-server
``` 

and from within the subdirectory in question, execute

```
docker compose up
```

It is also possible to run components without using Docker. In this case, first install the [`rustup`](https://www.rust-lang.org/tools/install) tool. The further steps you need to perform differ for each component.

To start the server, just go to the `rust_go-server` subdirectory and execute the command

```
cargo run
```

which will build and run the program.

> NOTE: If executing the `cargo run` command ends with an error `` error: failed to run custom build command for `openssl-sys v0.9.79` ``, you need to install OpenSSL packages. For example, on an Ubuntu system, this can be done via `sudo apt install libssl-dev`. 

In the case of the client, a few additional steps need to be taken before launching. First, add WASM as a build target for Rust via the command.

```
rustup target add wasm32-unknown-unknown
```

Next, install [`trunk`](https://trunkrs.dev/) - a bundler of applications that exploit WASM for Rust.

```
cargo install trunk
```

After completing these steps, we can run the program by executing the command

```
trunk serve
```

## Access the application

After the components of the application are started, to start the gameplay we need to open the `localhost:8080` page in two browser tabs. The first open tab will represent the first player, who will also have the decision on the size of the board. On the second card, the second player will be able to make moves.

## Documentation of the project

To create technical documentation we use `cargo`, which has the ability to automatically generate documentation. To use it, go to the subfolder corresponding to the component whose documentation we want to generate and execute the command

```
cargo doc --no-deps --open
```

which will build the documentation and then open it in a new browser tab.

## Unit tests

To run unit tests for a component, navigate to the appropriate subdirectory, e.g.

```
cd rust_go-client
```

and execute the command

<p align="right">(<a href="#readme-top">back to top</a>)</p>
<!-- MARKDOWN LINKS & IMAGES -->

[rust-url]: https://www.rust-lang.org
