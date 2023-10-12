# PETLIK-RUST
**Rewriting Petlik in Rust, because why not**

##

The original implementation was written in C. 

The source code is located [here](https://github.com/Emilo77/SEM1-WPI-PETLIK).

### Running Rust app:
```console
cargo run
```

### Running the binary :
```console
Usage: ./petlik [OPTIONS]
        Options:
        -h, --help : printing program usage
        -g         : printing generated instructions
        -gf        : printing generated instructions ONLY, without executing them

```

### Running the binary with redirecting input:
```console
./petlik [OPTIONS] < tests_directory/chosen_test.in
```

### Running tests:
(Binary named `petlik` has to be in the same directory as `test.sh` file)
```console
bash test.sh
```



