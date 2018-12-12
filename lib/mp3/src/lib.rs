extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};

mod id3{
	use super::{ParserContext, ParseResult, Parsable};
	
	#[derive(Debug)]
	pub struct Frame{
		pub id: String,
		pub size: u32,
		pub flags: Vec<u8>,
		pub encoding: u8,
		pub data: String
	}
	
	impl Parsable for Frame {
		fn parse<'a>(ctx: &'a mut ParserContext) -> ParseResult<'a, Frame>{
			let id = String::from_utf8(ctx.read_bytes(4).to_vec()).unwrap();
			let size = ctx.read_u32().unwrap();
			let flags = ctx.read_bytes(2).to_vec();
			let encoding = ctx.read_byte();
			let data = String::from_utf8(ctx.read_bytes(size as usize - 2).to_vec()).unwrap();
			//let terminator = ctx.read_byte();
			
			while ctx.peek_byte() == 0 {
				let _ = ctx.read_byte();
			}
			
			//assert_eq!(terminator, 0);
			return Ok(Frame {
				id,
				size,
				flags,
				encoding,
				data,
			});
		}
	}
}

pub fn parse(data: &[u8]){
	let mut ctx = ParserContext::new(data);
	
	let magic = ctx.read_bytes(3);
	assert_eq!(&magic, b"ID3");
	
	let version = &ctx.read_bytes(2);
	println!("Version: {:?}", &version);
	
	let flags = ctx.read_byte();
	println!("Flags: {:?}", &flags);
	
	let size = ctx.read_u32().unwrap();
	println!("Size: {}", &size);
	
	while ctx.byte_index < (size - 10) as usize && ctx.peek_byte() != 0xFF {
		let frame: id3::Frame = ctx.read().unwrap();
		println!("{:#?}", frame);
	}
	
	//let data = std::str::from_utf8(ctx.read_bytes(5)).unwrap();
	//println!("{:?}", data);
	
	let b = ctx.peek_byte();
	println!("Next Hex: {:x}", b);
	println!("As char: {}", b as char);
}

#[derive(Debug)]
enum ParseError<'a>{
	Custom(&'a str)
}

type ParseResult<'a, T> = Result<T, ParseError<'a>>;
struct ParserContext<'a>{
	slice: &'a [u8],
	byte_index: usize,
	bit_index: usize,
}

impl<'a> ParserContext<'a> {
	pub fn new(data: &[u8]) -> ParserContext{
		return ParserContext {
			slice: data,
			byte_index: 0,
			bit_index: 0
		};
	}
	
	pub fn read_byte(&mut self) -> u8{
		let data = self.slice[self.byte_index];
		self.byte_index += 1;
		self.bit_index = 0;
		return data;
	}
	
	pub fn read_bytes(&mut self, n: usize) -> &[u8]{
		let new_index = self.byte_index + n;
		let data = &self.slice[self.byte_index..new_index];
		self.byte_index = new_index;
		self.bit_index = 0;
		return data;
	}
	
	pub fn read_bit(&mut self) -> bool{
		if self.bit_index == 7 {
			self.bit_index = 0;
			self.byte_index += 1;
		}
		return (self.peek_byte() & (1 >> self.bit_index)) != 0;
	}
	
	pub fn read_bits(&mut self, n: usize) -> Vec<bool>{
		
	}
	
	pub fn read_u32(&mut self) -> ParseResult<u32>{
		return self.read_bytes(4).read_u32::<BigEndian>().map_err(|_| ParseError::Custom("Error Parsing u32"));
	}
	
	pub fn read<T: Parsable>(&mut self) -> ParseResult<T>{
		return T::parse(self);
	}
	
	pub fn peek_byte(&self) -> u8{
		return self.slice[self.byte_index];
	}
}

trait Parsable: Sized{
	fn parse<'a>(ctx: &'a mut ParserContext) -> ParseResult<'a, Self>;
}
