# Intention tester
- Test intention/category detection api against test files (in the `./data` folder).

- You can run intention_tester either manually or with Docker.

## Install & Run the binary (Recommended method)
- if you have `rust` and `cargo` installed on your machine, you can install the `intention_tester` tool as follows:
```
cargo install intention_tester
```
- it will by default download the crate, compile the binary target (in ''release'' mode, so it might take a while) and copy them into the `~/.cargo/bin/` directory.
- then you can simply run the tool as follows:
```
intention_tester -c <api-url> -i <data folder path to csv test files>
```

## Run with Docker (if you don't have rust or cargo)
- first you have to add some `csv` files in the `./data` folder if you want more tests, with the same schema as the `basic.csv` file test.
- then build & run the intention_tester image:
```
docker build -t intention_tester:1.0 . & docker run --rm intention_tester:1.0 <api-url>
```

