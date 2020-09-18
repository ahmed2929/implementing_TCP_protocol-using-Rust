use std::io;

 fn main()->io::Result<()> {
     loop{
 let nic=tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
 let mut buffer = [0; 512];
 let Number_of_bytes=nic.recv(&mut buffer[..])?;
 eprintln!("read {} bytes {:x?}",Number_of_bytes,&buffer[..Number_of_bytes]);
     
     let ether_net_flags=u16::from_be_bytes([buffer[0],buffer[1]]);
     let ether_net_protocol=u16::from_be_bytes([buffer[2],buffer[3]]);
     if ether_net_protocol!=0x0800{ //check if it is not ipv4
        continue; 
     }

     match etherparse::Ipv4HeaderSlice::from_slice(&buffer[4..Number_of_bytes]){
         Ok(proto)=>{
             let source=proto.source_addr();
             let destination=proto.destination_addr();
             let protocol=proto.protocol();
             if protocol != 0x06{ //not tcp
                continue; 
             }
             match etherparse::TcpHeaderSlice:: from_slice(&buffer[4+proto.slice().len()..]){
                 Ok(proto)=>{
                    eprintln!(
                        "{} => {} b of tcp port ,{},{}",
                    source,destination,
                    proto.slice().len(),
                    proto.destination_port()
                );
               
            }
            Err(e)=>{
                eprintln!("wired tcp packets {:?}",e)
            }
        
                 }
               


         }

         Err(e)=>{
            eprintln!("wired tcp packets {:?}",e)
        }
     }



    }

 Ok(())
     
 }
