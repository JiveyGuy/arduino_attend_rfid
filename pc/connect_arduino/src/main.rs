// this is a rust file that sends and recieves text 
// from an arduino over serial
// this program is ran on a linux PC with a usb serial 
// connection to the arduino
use serialport::*;
use std::io::prelude::*;
// sudo dnf install rust-libudev-devel
fn main() 
{
    // send "ping" to the arduino until we get pong
    loop
    {
        // send the message
        let response = match send_message(String::from("ping\n"))
        {
            // if we get a response, print it
            Some(s) => 
            {
                println!("Got \"{}\" from Arduino for ping", s);
                s
            },
            // if we don't get a response, print an error
            None => 
            {
                println!("No response from Arduino for ping");
                continue;
            },
        };

        // if the response hs pong, break the loop
        if response.contains("pong")
        {
            println!("Got pong, breaking loop");
            break;
        }
    } 

    // send "read" command which will either respond with "no card"
    // or the id of a rfid card
    loop
    {
        // send the message
        let response = send_message(String::from("read"));

        // if we get a response, print it
        if response.is_some()
        {
            let mut id = response.unwrap();

            // check if "no card" or an id in the form of "A5 3F 2D 1E"
            if id.contains("no card")
            {
                println!("No card found");
            }
            else
            {
                println!("Card found with id: {}", id);
                
                // remove the \n from the end of the id
                let mut id = id.replace("\n", "");
                // remove the spaces from the id
                id = id.replace(" ", "");
                // remove carriage return
                id = id.replace("\r", "");

                // print
                println!("Card id b4 conversion: {}", id);

                // convert the id to a u32 from "A5C89800" format
                let id = match u32::from_str_radix(&id, 16)
                {
                    Ok(t) => t,
                    Err(e) => 
                    {
                        println!("Error converting id: {}", e);
                        // if invalid digit erro print which
                        // use a for to loop through each char
                        // and print the char if it is invalid
                        for c in id.chars()
                        {
                            if c.is_digit(16) == false
                            {
                                // print the invalid char as an ascii code
                                println!("Invalid char: {}", c as u32);
                            }
                        }
                        return ();
                    },
                };

                // print the id
                println!("Card id: {}", id);
            }
        }
        else 
        {
            println!("No response from Arduino for read");
        }
    }
}

// convert above main code to a function that sends 
// a message and returns true if a response is recieved
fn send_message(message: String) -> Option<String>
{
    // get a list of all serial ports
    let ports = serialport::available_ports().expect("No ports found!");
    // find the port that is connected to the arduino
    let mut port_name = String::new();
    for p in ports
    {
        if p.port_name.contains("ttyACM")
        {
            port_name = p.port_name;
        }
    }
    // if no port is found, return None
    if port_name == ""
    {
        return None;
    }
    // open the port
    let mut port = serialport::new(port_name, 9600)
        .timeout(std::time::Duration::from_millis(5000))
        .open()
        .expect("Failed to open port");

    // sleep 3 seconds
    std::thread::sleep(std::time::Duration::from_millis(3000));

    // write the message to the port
    port.write_all(message.as_bytes()).expect("Write failed");

    // flush the port
    port.flush().expect("Flush failed");

    // read the response from the port until \n is recieved
    let mut serial_buf: Vec<u8> = vec![0; 100];
    let mut response = String::new();
    loop
    {
        match port.read(serial_buf.as_mut_slice())
        {
            Ok(t) => 
            {
                // convert the bytes to a string
                let s = String::from_utf8(serial_buf[0..t].to_vec()).unwrap();
                // add the string to the response
                response.push_str(&s);
                // if the response contains \n, break the loop
                if response.contains("\n")
                {
                    return Some(response);
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => 
            {
                // if the response times out, return None
                return None;
            },
            Err(e) => eprintln!("{:?}", e),
        }
    }
}