mod xdf_reader;
mod xcf_reader;

use std::net::{SocketAddr, UdpSocket};

use mcb::mcb_node::{create_node_mcb, CommandType};
use mcb::IntfResult::*;
use mcb::{IntfError, IntfResult, ExtMode, PhysicalInterface, MAX_FRAME_SIZE};

use clap::Parser;
use std::fs;

use xdf_reader::xdf_from_file;
use xcf_reader::xcf_from_file;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// config file of the connected device
    #[arg(short, long)]
    xml_file: String,
    #[arg(short, long)]
    mode: String,
    #[arg(short, long)]
    virtual_xml: String,
    #[arg(short, long)]
    defaults: String,
}

struct Udp2Mcb {
    socket: UdpSocket,
    address: SocketAddr,
}

impl PhysicalInterface for Udp2Mcb {
    fn raw_write(&mut self, frame: &[u16]) -> Result<IntfResult, IntfError> {
        self.socket
            .send_to(unsafe { frame[..].align_to::<u8>().1 }, self.address)
            .unwrap();
        Ok(Success)
    }

    fn raw_read(&mut self) -> Result<IntfResult, IntfError> {
        let mut msg = [0u16; MAX_FRAME_SIZE];
        let _in_size: usize;

        (_in_size, self.address) = self
            .socket
            .recv_from(unsafe { msg[..].align_to_mut::<u8>().1 })
            .unwrap();

        Ok(Data(Box::new(msg)))
    }
}

fn main() {
    let args = Args::parse();

    let sub0_contents =
        fs::read_to_string(args.xml_file).expect("Should have been able to read the file");
    let sub1_contents =
        fs::read_to_string(args.virtual_xml).expect("Should have been able to read the file");
    let dflt_contents =
        fs::read_to_string(args.defaults).expect("Should have been able to read the file");
    let dictionary_sub0 = xdf_from_file(&sub0_contents);
    let dictionary_sub1 = xdf_from_file(&sub1_contents);
    let defaults = xcf_from_file(&dflt_contents);

    let socket = UdpSocket::bind("127.0.0.2:1061").unwrap();
    let address = socket.local_addr().unwrap();

    let bridge = Udp2Mcb { socket, address };

    let mcb_node = create_node_mcb(Some(bridge), ExtMode::Extended);
    let mut node_cfg = mcb_node.init();

    loop {
        let mut is_ready = node_cfg.listen();

        while let Ok(IntfResult::Empty) = is_ready {
            is_ready = node_cfg.listen();
        }

        let request = match node_cfg.read() {
            Ok(request) => request,
            Err(IntfError::Interface) => {
                panic!("Udp interface issue!");
            }
            Err(IntfError::WrongCommand) => {
                panic!("Unexpected command!");
            }
            Err(IntfError::Crc) => {
                panic!("Wrong CRC!");
            }
            _ => {
                panic!("Something wrong");
            }
        };
        print!(
            "Subnode: {}, Address: {}, Command: {:?} ",
            request.subnode, request.address, request.command
        );
        match dictionary_sub0.get_reg_uid(request.address){
            Ok(value) => print!("UID: {} \n", value),
            _ => (),

        }
        match dictionary_sub1.get_reg_uid(request.address){
            Ok(value) => print!("UID: {} \n", value),
            _ => (),

        }
        let _ =
            match request.command {
                CommandType::Read => {
                    match request.subnode {
                        0u8 => match request.address {
                            //0x0AAu16 => node_cfg.write_u64(request.address, 0x312E302E302E5200u64),
                            0x6E1u16 => node_cfg
                                .write_u32(request.address, dictionary_sub0.get_product_code()),
                            0x6E2u16 => node_cfg
                                .write_u32(request.address, dictionary_sub0.get_revision_number()),
                            0x6E4u16 => node_cfg.write_str(request.address, "1.0.0.000"),
                            0x6E6u16 => node_cfg.write_u32(request.address, 0x12345678),
                            _ => {
                                let uid = match dictionary_sub0.get_reg_uid(request.address) {
                                    Ok(value) => value,
                                    _ => "Hola",
                                };
                                let dflt_value: &str;
                                if uid == "hola" {
                                    dflt_value = "0"
                                }
                                else {
                                    dflt_value = defaults.get_default(uid);
                                }
                                match dictionary_sub0.data_type(request.address) {
                                "u8" => node_cfg.write_u8(request.address, dflt_value.parse().unwrap()),
                                "s8" => node_cfg.write_i8(request.address, dflt_value.parse().unwrap()),
                                "u16" => node_cfg.write_u16(request.address, dflt_value.parse().unwrap()),
                                "s16" => node_cfg.write_i16(request.address, dflt_value.parse().unwrap()),
                                "u32" => node_cfg.write_u32(request.address, dflt_value.parse().unwrap()),
                                "s32" => node_cfg.write_i32(request.address, dflt_value.parse().unwrap()),
                                "float" => node_cfg.write_f32(request.address, dflt_value.parse().unwrap()),
                                "str" => node_cfg.write_str(request.address, "1.0.0.000"),
                                _ => node_cfg.error(request.address, 0123u32),
                                }
                            }
                        },
                        1u8 => match request.address {
                            //0x0AAu16 => node_cfg.write_u64(request.address, 0x312E302E302E5200u64),
                            0x11u16 => node_cfg.write_u16(request.address, 0x250u16),
                            94u16 => node_cfg.write_f32(request.address, 24.0f32),
                            0x64Du16 => node_cfg.write_u16(request.address, 0u16),
                            0x6E1u16 => node_cfg
                                .write_u32(request.address, dictionary_sub1.get_product_code()),
                            0x6E2u16 => node_cfg
                                .write_u32(request.address, dictionary_sub1.get_revision_number()),
                            0x6E4u16 => node_cfg.write_str(request.address, "1.0.0.000"),
                            0x6E6u16 => node_cfg.write_u32(request.address, 0x12345678),
                            _ => {
                                let uid = match dictionary_sub1.get_reg_uid(request.address) {
                                    Ok(value) => value,
                                    _ => "Hola",
                                };
                                let dflt_value: &str;
                                if uid == "hola" {
                                    dflt_value = "0"
                                }
                                else {
                                    dflt_value = defaults.get_default(uid);
                                }
                                match dictionary_sub1.data_type(request.address) {
                                "u8" => node_cfg.write_u8(request.address, dflt_value.parse().unwrap()),
                                "s8" => node_cfg.write_i8(request.address, dflt_value.parse().unwrap()),
                                "u16" => node_cfg.write_u16(request.address, dflt_value.parse().unwrap()),
                                "s16" => node_cfg.write_i16(request.address, dflt_value.parse().unwrap()),
                                "u32" => node_cfg.write_u32(request.address, dflt_value.parse().unwrap()),
                                "s32" => node_cfg.write_i32(request.address, dflt_value.parse().unwrap()),
                                "float" => node_cfg.write_f32(request.address, dflt_value.parse().unwrap()),
                                "str" => node_cfg.write_str(request.address, "1.0.0.000"),
                                _ => node_cfg.error(request.address, 0123u32),
                                }
                            }
                        },
                        _ => node_cfg.error(request.address, 0123u32),
                    }
                }
                CommandType::Write => node_cfg.error(request.address, 0123u32),
                _ => node_cfg.error(request.address, 0123u32),
            };
    }
}
