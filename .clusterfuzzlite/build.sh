#!/bin/bash -eu

cd $SRC/precis

# Build fuzzers for precis-core
if [ -d "precis-core/fuzz" ]; then
    cd precis-core
    cargo fuzz build --release

    # Copy fuzzers to $OUT
    for fuzzer in fuzz/target/x86_64-unknown-linux-gnu/release/*; do
        if [ -f "$fuzzer" ] && [ -x "$fuzzer" ]; then
            fuzzer_name=$(basename $fuzzer)
            # Skip build artifacts that aren't actual fuzzers
            if [[ ! "$fuzzer_name" =~ ^(build|deps|incremental|\.fingerprint)$ ]]; then
                cp $fuzzer $OUT/precis_core_${fuzzer_name}
            fi
        fi
    done

    cd $SRC/precis
fi

# Build fuzzers for precis-profiles
if [ -d "precis-profiles/fuzz" ]; then
    cd precis-profiles
    cargo fuzz build --release

    # Copy fuzzers to $OUT
    for fuzzer in fuzz/target/x86_64-unknown-linux-gnu/release/*; do
        if [ -f "$fuzzer" ] && [ -x "$fuzzer" ]; then
            fuzzer_name=$(basename $fuzzer)
            # Skip build artifacts that aren't actual fuzzers
            if [[ ! "$fuzzer_name" =~ ^(build|deps|incremental|\.fingerprint)$ ]]; then
                cp $fuzzer $OUT/precis_profiles_${fuzzer_name}
            fi
        fi
    done
fi
