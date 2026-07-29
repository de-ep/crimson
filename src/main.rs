mod emulator;

fn main() {
    if let Err(err) = emulator::emulate(&"/home/Deep/Desktop/emu/temp") {
        println!("err: {:?}", err);
    }
    else {
        println!("okay");
    }
    
}
