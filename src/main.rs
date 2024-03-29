use pnet::datalink::ChannelType;
use pnet::datalink::{linux, Channel};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use std::io::Read;

fn main() {
    // get a network interface that is running and is not the loopback
    let interface = linux::interfaces()
        .into_iter()
        .find(|ni| ni.is_running() && ni.name != "lo")
        .unwrap();

    let config = linux::Config {
        write_buffer_size: 4096,
        read_buffer_size: 4096,
        read_timeout: None,
        write_timeout: None,
        channel_type: ChannelType::Layer2,
        fanout: None,
        promiscuous: true,
    };

    // create a new channel dealing with layer 2 packets
    let (_, mut rx) = match linux::channel(&interface, config) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandeld channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start Listening on: {:?}", interface.name);

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                let mut packet_packet = packet.packet();
                let mut some_string = String::new();

                /*
                packet_packet.read_to_string(&mut some_string).unwrap();
                println!("{}", some_string);
                */

                for byte in packet_packet {
                    print!("{:02X}", byte);
                }
                //println!("{}", std::str::from_utf8(packet_packet).unwrap())
            }
            Err(e) => panic!("An error occurred while reading: {}", e),
        }
    }
}
