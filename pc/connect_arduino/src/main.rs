// this is a rust file that sends and recieves text 
// from an arduino over serial
// this program is ran on a linux PC with a usb serial 
// connection to the arduino
use serialport::*;
// sudo dnf install rust-libudev-devel


fn main() 
{
    let ports = serialport::available_ports().expect("No ports found!");
    
    // if ACM* is found, then it is the arduino
    // select the first
    // else print "No arduino found"
    let mut port_name = String::new();
    for p in ports
    {
        if p.port_name.contains("ACM")
        {
            port_name = p.port_name;
            break;
        }
    }

    if port_name == ""
    {
        println!("No arduino found");
        return;
    }

    // open the port
    let mut port = serialport::new(port_name, 9600)
        .timeout(std::time::Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    // send a message to the arduino
    let mut buf: Vec<u8> = vec![0; 100];
    let mut message = String::new();
    println!("Enter a message to send to the arduino");
    std::io::stdin().read_line(&mut message).expect("Failed to read from stdin");
    buf = message.as_bytes().to_vec();

    // write the message to the arduino
    port.write(&buf).expect("Failed to write to port");
}
