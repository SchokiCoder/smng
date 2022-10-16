fn main() {
	let man = clap_mangen::Man::new(cmd);
	let mut buffer: Vec<u8> = Default::default();
	man.render(&mut buffer)?;

	std::fs::write(out_dir.join("clap_mangen.1"), buffer)?;
}
   
