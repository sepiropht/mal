# mal - Make a Lisp
I'm learning rust by trying to implement a lisp interpretor ! 
Go to the original project to see others languages implementations

```

### Rust 

The rust implementation of mal requires the rust compiler and build
tool (cargo) to build.

```
cd rust
cargo run --release --bin stepX_YYY
```

Alternative out-of-tee Rust implementations:

* [by Tim Morgan](https://github.com/seven1m/mal-rust).
* [by vi](https://github.com/vi/mal-rust-vi) - using [Pest](https://pest.rs/) grammar, not using typical Mal infrastructure (cargo-ized steps and built-in converted tests).




## Running tests

### Functional tests

The are over 600 generic functional tests (for all implementations)
in the `tests/` directory. Each step has a corresponding test file
containing tests specific to that step. The `runtest.py` test harness
launches a Mal step implementation and then feeds the tests one at
a time to the implementation and compares the output/return value to
the expected output/return value.

To simplify the process of running tests, a top level Makefile is
provided with convenient test targets.

* To run all the tests across all implementations (be prepared to wait):

```
make test
```

* To run all tests against a single implementation:

```
make "test^IMPL"

# e.g.
make "test^clojure"
make "test^js"
```

* To run tests for a single step against all implementations:

```
make "test^stepX"

# e.g.
make "test^step2"
make "test^step7"
```

* To run tests for a specific step against a single implementation:

```
make "test^IMPL^stepX"

# e.g
make "test^ruby^step3"
make "test^ps^step4"
```

### Self-hosted functional tests

* To run the functional tests in self-hosted mode, you specify `mal`
  as the test implementation and use the `MAL_IMPL` make variable
  to change the underlying host language (default is JavaScript):
```
make MAL_IMPL=IMPL "test^mal^step2"

# e.g.
make "test^mal^step2"   # js is default
make MAL_IMPL=ruby "test^mal^step2"
make MAL_IMPL=python "test^mal^step2"
```

### Starting the REPL

* To start the REPL of an implementation in a specific step:

```
make "repl^IMPL^stepX"

# e.g
make "repl^ruby^step3"
make "repl^ps^step4"
```

* If you omit the step, then `stepA` is used:

```
make "repl^IMPL"

# e.g
make "repl^ruby"
make "repl^ps"
```

* To start the REPL of the self-hosted implementation, specify `mal` as the
  REPL implementation and use the `MAL_IMPL` make variable to change the
  underlying host language (default is JavaScript):
```
make MAL_IMPL=IMPL "repl^mal^stepX"

# e.g.
make "repl^mal^step2"   # js is default
make MAL_IMPL=ruby "repl^mal^step2"
make MAL_IMPL=python "repl^mal"
```

### Performance tests

Warning: These performance tests are neither statistically valid nor
comprehensive; runtime performance is a not a primary goal of mal. If
you draw any serious conclusions from these performance tests, then
please contact me about some amazing oceanfront property in Kansas
that I'm willing to sell you for cheap.

* To run performance tests against a single implementation:
```
make "perf^IMPL"

# e.g.
make "perf^js"
```

* To run performance tests against all implementations:
```
make "perf"
```

### Generating language statistics

* To report line and byte statistics for a single implementation:
```
make "stats^IMPL"

# e.g.
make "stats^js"
```

* To report line and bytes statistics for general Lisp code (env, core
  and stepA):
```
make "stats-lisp^IMPL"

# e.g.
make "stats-lisp^js"
```

## Dockerized testing

Every implementation directory contains a Dockerfile to create
a docker image containing all the dependencies for that
implementation. In addition, the top-level Makefile contains support
for running the tests target (and perf, stats, repl, etc) within
a docker container for that implementation by passing *"DOCKERIZE=1"*
on the make command line. For example:

```
make DOCKERIZE=1 "test^js^step3"
```

Existing implementations already have docker images built and pushed
to the docker registry. However, if
you wish to build or rebuild a docker image locally, the toplevel
Makefile provides a rule for building docker images:

```
make "docker-build^IMPL"
```


**Notes**:
* Docker images are named *"kanaka/mal-test-IMPL"*
* JVM-based language implementations (Groovy, Java, Clojure, Scala):
  you will probably need to run these implementations once manually
  first (make DOCKERIZE=1 "repl^IMPL")before you can run tests because
  runtime dependencies need to be downloaded to avoid the tests timing
  out. These dependencies are download to dot-files in the /mal
  directory so they will persist between runs.

## Projects using mal

 * [malc](https://github.com/dubek/malc) - Mal (Make A Lisp) compiler. Compiles a Mal program to LLVM assembly language, then binary.
 * [frock](https://github.com/chr15m/frock) - Clojure-flavoured PHP. Uses mal/php to run programs.

## License

Mal (make-a-lisp) is licensed under the MPL 2.0 (Mozilla Public
License 2.0). See LICENSE.txt for more details.
