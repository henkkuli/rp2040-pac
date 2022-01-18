#!/usr/bin/env bash

set -ex

# NOTE: Last executed using Rust 1.49.0

cargo install --version 0.21.0 svd2rust
cargo install --version 0.8.0  form
rustup component add rustfmt
pip3 install --upgrade --user "svdtools>=0.1.21"

rm -rf src
mkdir src

svd patch svd/rp2040.yaml

# These values are from datasheet, Section 2.2. Address Map
range_marks=()
range_marks+=(--mark_range AtomicMarker 0x40000000 0x40004000) # SYSINF
range_marks+=(--mark_range AtomicMarker 0x40004000 0x40008000) # SYSCFG
range_marks+=(--mark_range AtomicMarker 0x40008000 0x4000c000) # CLOCKS
range_marks+=(--mark_range AtomicMarker 0x4000c000 0x40010000) # RESETS
range_marks+=(--mark_range AtomicMarker 0x40010000 0x40014000) # PSM
range_marks+=(--mark_range AtomicMarker 0x40014000 0x40018000) # IO_BANK0
range_marks+=(--mark_range AtomicMarker 0x40018000 0x4001c000) # IO_QSPI
range_marks+=(--mark_range AtomicMarker 0x4001c000 0x40020000) # PADS_BANK0
range_marks+=(--mark_range AtomicMarker 0x40020000 0x40024000) # PADS_QSPI
range_marks+=(--mark_range AtomicMarker 0x40024000 0x40028000) # XOSC
range_marks+=(--mark_range AtomicMarker 0x40028000 0x4002c000) # PLL_SYS
range_marks+=(--mark_range AtomicMarker 0x4002c000 0x40030000) # PLL_USB
range_marks+=(--mark_range AtomicMarker 0x40030000 0x40034000) # BUSCTRL
range_marks+=(--mark_range AtomicMarker 0x40034000 0x40038000) # UART0
range_marks+=(--mark_range AtomicMarker 0x40038000 0x4003c000) # UART1
range_marks+=(--mark_range AtomicMarker 0x4003c000 0x40040000) # SPI0
range_marks+=(--mark_range AtomicMarker 0x40040000 0x40044000) # SPI1
range_marks+=(--mark_range AtomicMarker 0x40044000 0x40048000) # I2C0
range_marks+=(--mark_range AtomicMarker 0x40048000 0x4004c000) # I2C1
range_marks+=(--mark_range AtomicMarker 0x4004c000 0x40050000) # ADC
range_marks+=(--mark_range AtomicMarker 0x40050000 0x40054000) # PWM
range_marks+=(--mark_range AtomicMarker 0x40054000 0x40058000) # TIMER
range_marks+=(--mark_range AtomicMarker 0x40058000 0x4005c000) # WATCHDOG
range_marks+=(--mark_range AtomicMarker 0x4005c000 0x40060000) # RTC
range_marks+=(--mark_range AtomicMarker 0x40060000 0x40064000) # ROSC
range_marks+=(--mark_range AtomicMarker 0x40064000 0x4006c000) # VREG_AND_CHIP_RESET
range_marks+=(--mark_range AtomicMarker 0x4006c000 0x40070000) # TBMAN
range_marks+=(--mark_range AtomicMarker 0x50200000 0x50204000) # PIO0
range_marks+=(--mark_range AtomicMarker 0x50300000 0x50304000) # PIO1
range_marks+=(--mark_range AtomicMarker 0x50400000 0x50404000) # XIP_AUX

svd2rust -i svd/rp2040.svd.patched --generic_mod "${range_marks[@]}"

form -i lib.rs -o src
cat generic.rs generic_extension.rs > src/generic.rs
rm lib.rs generic.rs

cargo fmt

# Original svd has \n (two chars) in it, which gets converted to "\n" by svd2rust
# If we convert them to newline characters in the SVD, they don't turn up in markdown so docs suffers
# So, convert \n to [spc] [spc] [newline], then strip the spaces out if there are consecutive [newlines]
# This means that by the time we're in markdown \n\n becomes new paragraph, and \n becomes a new line
if [ "$(uname)" == "Darwin" ]; then
    find src -name '*.rs' -exec sed -i '' -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
else
    find src -name '*.rs' -exec sed -i -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
fi

# Sort specified fields alphanumerically for easier consumption in docs.rs
./sortFieldsAlphaNum.sh
