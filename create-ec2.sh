set -e

cd kernel
cargo build --target x86_64-unknown-none
cd ..
cargo run -- target/x86_64-unknown-none/debug/kernel os.img

aws s3 cp os.img s3://ex2-disk-images/osos
aws ec2 import-snapshot --description osos --disk-container file://container.json
aws ec2 describe-import-snapshot-tasks --import-task-ids import-snap-0c48404d3ac31af1d