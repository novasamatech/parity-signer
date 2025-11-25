# Fuzzing Signer

This directory contains a fuzzing harness for ``parser::decode_call`` and an
accompanying grammar-fuzzer ``grammar-fuzzer``.

## To run the fuzzer
1. Build the fuzzing harness.
``` bash
cd ../decode-call
cargo ziggy build --no-honggfuzz
```
2. Build the grammar fuzzer
```
cd ../grammar-fuzzer
cargo build --release
```
3. Run the grammar-fuzzer
We use 16 cores ``0-16``, with a timeout of ``3 seconds``. The ``-r`` will tell
the fuzzer to render the input in ``rendered_corpus`` and
``rendered_crashes``.
```
# Run the workspace root (/rust)
./target/release/grammar-fuzzer -o ./output -c0-16 -r -t3 ./fuzz/decode-call/target/afl/debug/decode-call
```
4. Debugging a crash
If a fuzzer finds a crash or a timeout, it will be shown as ``objective: <num_crashes>``.
If you want to run the crash to debug it.
```
cd ../decode-call
cargo ziggy run -i ../grammar-fuzzer/output/<core_id>/rendered_crashes/<entry>
```
