set -e

cd kernel
cargo build --target x86_64-unknown-none
cd ..
cargo run -- target/x86_64-unknown-none/debug/kernel os.img

qemu-system-x86_64 -enable-kvm -drive format=raw,file=os.img -display none -serial stdio


# INFO : virtual_address_offset: XXX
# target remote :1234
# symbol-file -o 0x8000000000 target/x86_64-unknown-none/debug/kernel