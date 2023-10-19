extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::net::{ TcpStream };
use std::io::{Read, Write};
use std::str::from_utf8;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern "C" {
    fn enclave_init_ra(
        b_pse: i32,
        p_context: &mut sgx_ra_context_t
    ) -> sgx_status_t;
}

extern "C" {
    fn enclave_ra_close(
        context: &mut sgx_ra_context_t
    ) -> sgx_status_t;
}

extern "C" {
    fn verify_att_result_mac(
        context: sgx_ra_context_t,
        message: *const u8,
        msg_size: size_t,
        mac: *const u8,
        mac_size: size_t,
    ) -> sgx_status_t;
}

extern "C" {
    fn verify_secret_data(
        context: sgx_ra_context_t,
        p_secret: *const u8,
        sec_size: u32,
        gcm_mac: &[u8; 16],
        max_vlen: u32,
        p_ret: &mut [u8; 16],
    ) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;

    match TcpStream::connect("127.0.0.1:3000") {
        Ok(mut stream) => {
            println!("Successfully connected to Recording Oracle Master in port 3000");

            let msg = b"Hello!";

            stream.write(msg).unwrap();

            println!("Awaiting reply...");

            let mut data = [0 as u8; 6];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        let mut p_context: sgx_ra_context_t = 0;
                        let result = unsafe {
                            enclave_init_ra(
                                enclave.geteid(),
                                &mut p_context
                            )
                        };
                    
                        match result {
                            sgx_status_t::SGX_SUCCESS => {}
                            _ => {
                                println!("[-] ECALL Enclave Failed {}!", result.as_str());
                                return;
                            }
                        }
                    
                        println!("[+] ecall_test success...");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }

        }, 
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    enclave.destroy();
}
