use std::{env, path};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("{} <in_dir> <out_dir>", args[0]);
        return;
    }
    let in_dir = path::Path::new(args[1].as_str());
    if !in_dir.exists() || !in_dir.is_dir() {
        println!("invalid in_dir {:?}", in_dir);
        return;
    }
    let out_dir = path::Path::new(args[2].as_str());
    if !out_dir.exists() || !out_dir.is_dir() {
        println!("invalid out_dir {:?}", out_dir);
        return;
    }
    let protos_pat = in_dir.join("*.proto");
    let protos = protos_pat.to_str().unwrap();
    env::set_var("OUT_DIR", out_dir.to_str().unwrap());
    prost_build::compile_protos(&[protos], &[args[1].as_str()]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", env::current_dir().unwrap());
        let p = path::Path::new("src");
        assert!(p.exists());
        assert!(p.is_dir());
    }
}
