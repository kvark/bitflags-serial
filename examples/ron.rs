#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate bitflags_serial;
extern crate ron;

bitflags_serial!{
    struct Bits: u8 {
        const ONE = 0x1;
        const TWO = 0x2;
    }
}

fn main() {
    let input = "[TWO, ONE]";
    let output: Bits = ron::de::from_str(input).unwrap();
    println!("deserialized {:?}", output);
    let input2 = ron::ser::to_string(&output).unwrap();
    println!("serialized {:?}", input2);
}
