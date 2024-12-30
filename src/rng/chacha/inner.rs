use core::fmt;

const CONSTANT: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

#[derive(Clone)]
#[repr(C)]
pub struct ChaChaCore {
	seed: [u32; 8],
	counter: [u32; 2],
	stream: [u32; 2],
}

unsafe impl dataview::Pod for ChaChaCore {}

impl ChaChaCore {
	#[inline]
	pub fn new(seed: [u32; 8], counter: u64, stream: u64) -> ChaChaCore {
		ChaChaCore {
			seed,
			counter: [counter as u32, (counter >> 32) as u32],
			stream: [stream as u32, (stream >> 32) as u32],
		}
	}
	#[inline]
	pub fn get_state(&self) -> [[u32; 4]; 4] {
		[
			CONSTANT,
			[self.seed[0], self.seed[1], self.seed[2], self.seed[3]],
			[self.seed[4], self.seed[5], self.seed[6], self.seed[7]],
			[self.counter[0], self.counter[1], self.stream[0], self.stream[1]],
		]
	}
	#[inline]
	pub fn get_counter(&self) -> u64 {
		(self.counter[1] as u64) << 32 | self.counter[0] as u64
	}
	#[inline]
	pub fn set_counter(&mut self, counter: u64) {
		self.counter[0] = counter as u32;
		self.counter[1] = (counter >> 32) as u32;
	}
	#[inline]
	pub fn add_counter(&self, counter: u64) -> ChaChaCore {
		let mut this = self.clone();
		this.set_counter(self.get_counter().wrapping_add(counter));
		this
	}
	#[inline]
	pub fn get_stream(&self) -> u64 {
		(self.stream[1] as u64) << 32 | self.stream[0] as u64
	}
	#[inline]
	pub fn set_stream(&mut self, stream: u64) {
		self.stream[0] = stream as u32;
		self.stream[1] = (stream >> 32) as u32;
	}
}

impl fmt::Debug for ChaChaCore {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ChaChaCore")
			.field("seed", &format_args!("{:x?}", self.seed))
			.field("counter", &self.get_counter())
			.field("stream", &self.get_stream())
			.finish()
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for ChaChaCore {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		[
			self.seed[0], self.seed[1], self.seed[2], self.seed[3],
			self.seed[4], self.seed[5], self.seed[6], self.seed[7],
			self.counter[0], self.counter[1], self.stream[0], self.stream[1],
		].serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ChaChaCore {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let values = <[u32; 12]>::deserialize(deserializer)?;
		Ok(ChaChaCore {
			seed: [values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7]],
			counter: [values[8], values[9]],
			stream: [values[10], values[11]],
		})
	}
}
