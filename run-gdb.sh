# INFO : virtual_address_offset: XXX
# target remote :1234
# symbol-file -o 0x8000000000 target/x86_64-unknown-none/debug/kernel

rust-gdb -ex 'target remote :1234'\
 -ex 'symbol-file -o 0x8000000000 target/x86_64-unknown-none/debug/kernel'\
 -ex 'hb kernel::fatal'