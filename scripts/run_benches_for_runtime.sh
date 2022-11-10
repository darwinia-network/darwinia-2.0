#!/bin/bash

# Runs all benchmarks for all pallets, for a given runtime, provided by $1

runtime="$1"
if [ $2 ]; then
  steps=$2
else
  steps=50
fi
if [ $3 ]; then
  repeat=$3
else
  repeat=20
fi

echo "[+] Compiling benchmarks..."
cargo build --release --features=runtime-benchmarks

# Load all pallet names in an array.
PALLETS=($(
  ./target/release/darwinia benchmark pallet --list --chain="${runtime}-dev" |
    tail -n+2 |
    cut -d',' -f1 |
    sort |
    uniq
))

echo "[+] Benchmarking ${#PALLETS[@]} pallets for runtime $runtime"

# Define the error file.
ERR_FILE="benchmarking_errors.log"
# Delete the error file before each run.
rm -f $ERR_FILE

# Benchmark each pallet.
for PALLET in "${PALLETS[@]}"; do
  echo "[+] Benchmarking $PALLET for $runtime"

  output_file=""
  if [[ $PALLET == *"::"* ]]; then
    # translates e.g. "pallet_foo::bar" to "pallet_foo_bar"
    output_file="${PALLET//::/_}.rs"
  fi

  OUTPUT=$(
    ./target/release/darwinia benchmark pallet \
      --chain="${runtime}-dev" \
      --steps=${steps} \
      --repeat=${repeat} \
      --pallet="$PALLET" \
      --extrinsic="*" \
      --execution=wasm \
      --wasm-execution=compiled \
      --header=./.maintain/license-header \
      --output="./runtime/${runtime}/src/weights/${output_file}" 2>&1
  )
  if [ $? -ne 0 ]; then
    echo "$OUTPUT" >>"$ERR_FILE"
    echo "[-] Failed to benchmark $PALLET. Error written to $ERR_FILE; continuing..."
  fi
done

# Check if the error file exists.
if [ -f "$ERR_FILE" ]; then
  echo "[-] Some benchmarks failed. See: $ERR_FILE"
else
  echo "[+] All benchmarks passed."
fi
