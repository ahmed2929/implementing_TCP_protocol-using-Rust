use std::io;

 fn main()->io::Result<()> {
 let nic=tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
 let mut buffer=[0u8,150];
 let Number_of_bytes=nic.recv(&mut buffer[..])?;
 eprintln!("read {} bytes {:x?}",Number_of_bytes,&buffer[..Number_of_bytes]);
 Ok(())

 }
