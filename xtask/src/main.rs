mod flags;

fn main() {
    let _xtask_flags = flags::Xtask::from_env_or_exit();

    println!("Running xtask");
}
