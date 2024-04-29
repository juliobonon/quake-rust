# Description

This is a simple rust program that parses a Quake 3 Arena log file. The program reads the log file and outputs the following information:
- Total kills
- Players' names
- Players' kills
- Means of death

## Usage

To run the program, you need to have Rust installed. You can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

After installing Rust, you can run the program by executing the following command in the terminal:

```bash
make setup
make build
make run
```

Also, you can run the tests by executing the following command:

```bash
make test
```

### Docker Containers

You can also run the program using Docker. Just execute the following command:

```bash
make docker-build
make docker-run
```