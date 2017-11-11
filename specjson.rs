#![allow(dead_code)]
extern crate sysctl;


fn main() {
    let ctl = "kern.osrevision";
    let d: String = sysctl::description(ctl).unwrap();
	println!("Description: {:?}", d);

    let val_enum = sysctl::value(ctl).unwrap();
    if let sysctl::CtlValue::Int(val) = val_enum {
        println!("Value: {}", val);
    }
    
}
