set -e

cd kernel
cargo build --target x86_64-unknown-none
cd ..
cargo run -- target/x86_64-unknown-none/debug/kernel os.img

qemu-system-x86_64 -enable-kvm -drive format=raw,file=os.img -display none -serial stdio "$@"
