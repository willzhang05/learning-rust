#![allow(dead_code)]
extern crate libc;


fn main() {
    let ctl = "kern.osrevision";
    libc::sysctl(ctl, len(ctl), None, 0
    #let d: String = sysctl::description(ctl).unwrap();
	println!("Description: {:?}", d);

    let val_enum = sysctl::value(ctl).unwrap();
    if let sysctl::CtlValue::Int(val) = val_enum {
        println!("Value: {}", val);
    }
    
}
