use pnet::datalink::{Channel, linux};
use pnet::datalink::ChannelType;
fn main(){
    let interface = linux::interfaces().into_iter().find(|e| e.name == "enp42s0").unwrap();

    let config = linux::Config{
        write_buffer_size: 4096,
        read_buffer_size: 4096,
        read_timeout: None,
        write_timeout: None,
        channel_type: ChannelType::Layer2,
        fanout: None,
        promiscuous: true,
    };

    let (_, mut rx) = match linux::channel(&interface, config){
        Ok(Channel::Ethernet(tx, rx)) => (tx,rx),
        Ok(_) => panic!("Unhandeld channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    loop{
        match rx.next(){
            Ok(packet) => println!("{:?}", packet),
            Err(e) => panic!("An error occurred while reading: {}", e),
        }
    }

}