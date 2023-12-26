# blake2b-cell-data-cycle-test

This is a simple test to get an idea of cycle consumptions for hashing data.

To build the contract, use the following command: 

```
$ clang-16 -O3 -g --target=riscv64 -march=rv64imc_zba_zbb_zbc_zbs -nostdinc -nostdlib test.c -o test -I deps/ckb-c-stdlib/libc -I deps/ckb-c-stdlib
```

Then you can run the test:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/blake2b-cell-data-cycle-test`
Seed: 1703561144006063669
Size: 51200 bytes, consume cycles: 1031288
Size: 102400 bytes, consume cycles: 2051072
Size: 204800 bytes, consume cycles: 4089748
Size: 512000 bytes, consume cycles: 10205776
```
