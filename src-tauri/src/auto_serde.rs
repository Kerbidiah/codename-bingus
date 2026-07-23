use serde::{Serialize, de::DeserializeOwned};
use std::fs::File;
use std::io::{Read, Write};

pub trait AutoSerde: Serialize + DeserializeOwned + Sized {
	/// Serialize to RON format
	fn to_ron(&self) -> anyhow::Result<String> {
		// this looks a little weird, but it's to convert from ron::Result to anyhow::Result
		// anyhow result is awesome and lets us mix all the errors into 1 type
		Ok(ron::to_string(self)?)
	}

	/// Deserialize from RON format
	fn from_ron(input: &str) -> anyhow::Result<Self> {
		Ok(ron::from_str(input)?)
	}

	/// Serialize to RON file
	fn to_file(&self, file: &mut File) -> anyhow::Result<()> {
		let s = self.to_ron()?;
		file.write_all(s.as_bytes())?;
		Ok(())
	}

	/// Deserialize from RON file
	fn from_file(file: &mut File) -> anyhow::Result<Self> {
		let mut s = String::new();
		file.read_to_string(&mut s)?;
		Self::from_ron(&s)
	}

	/// Open from `String` path
	fn open(path: String) -> anyhow::Result<Self> {
		let mut f = File::open(path)?;

		Self::from_file(&mut f)
	}

	/// Open from `String` path
	fn write(self, path: String) -> anyhow::Result<()> {
		let mut f = File::create(path)?;

		self.to_file(&mut f)
	}
}

// Blanket impl: every type that satisfies the bounds gets above funcs automatically
impl<T: Serialize + DeserializeOwned + Sized> AutoSerde for T {}
