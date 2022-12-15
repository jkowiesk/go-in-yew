# Go w architekturze klient-serwer

Projekt składa się z dwóch komponentów - klienta (kod źródłowy w katalogu *rust_go-client*) oraz serwera (kod źródłowy w katalogu *rust_go-server*). 

## Uruchamianie aplikacji

Możliwe jest uruchomienie osobno klienta i serwera, jak i obu komponentów naraz. Budowanie i uruchamianie projektu zostało zautomatyzowane przy użyciu [`docker compose`](https://docs.docker.com/compose/install/). Aby uruchomić oba komponenty, znajdując się w katalogu głównym repozytorium należy wykonać komendę

```
docker compose up
```

W celu uruchomienia tylko jednego komponentu, należy przejść do odpowiadającemgo mu podkatalogu poprzez wykonanie komendy 

```
cd rust_go-client
```

lub 

```
cd rust_go-server
``` 

i z poziomu danego podkatalogu wykonać

```
docker compose up
```

Możliwe jest również uruchomienie komponentów bez użycia Dockera. W tym przypadku należy po pierwsze zainstalować narzędzie [`rustup`](https://www.rust-lang.org/tools/install). Dalsze kroki jakie trzeba wykonać różnią się dla poszczególnych komponentów.

W celu uruchomienia serwera wystarczy przejść do podkatalogu `rust_go-server` i wykonać komendę

```
cargo run
```

która zbuduje i uruchomi program.  

W przypadku klienta przed uruchomieniem należy wykonać kilka dodatkowych kroków. Po pierwsze należy dodać WASM jako cel budowania dla Rusta poprzez komendę

```
rustup target add wasm32-unknown-unknown
```

Następnie należy zainstalować [`trunk`](https://trunkrs.dev/) - bundler aplikacji wykorzysujących WASM dla Rusta.

```
cargo install trunk
```

Po wykonaniu tych kroków możemy uruchomić program wykonując komendę

```
trunk serve
```

## Dokumentacja projektu

W celu utworzenia dokumentacji technicznej korzystamy z `cargo`, które posiada możliwość automatycznej generacji dokumentacji. W celu skorzystania z niej należy przejść do podfolderu odpowiadającego komponentowi którego dokumentację chcemy wygenerować i wykonać komendę

```
cargo doc --no-deps --open
```

która zbuduje dokumentację, a następnie otworzy ją w nowej karcie przeglądarki.
