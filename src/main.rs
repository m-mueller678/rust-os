use std::path::PathBuf;

fn main() {
    if std::env::args().count() == 1 {
        println!("{}", ovmf_prebuilt::ovmf_pure_efi().display());
    } else {
        let kernel = PathBuf::from(std::env::args_os().nth(1).unwrap());
        let image = PathBuf::from(std::env::args_os().nth(2).unwrap());
        bootloader::UefiBoot::new(&kernel)
            .create_disk_image(&image)
            .unwrap();
    }
}
