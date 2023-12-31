extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::net::{ TcpStream };
use std::io::{Read, Write};
use std::str::from_utf8;
use std::thread;
use std::net::{ TcpListener, TcpStream, Shutdown };
use std::io::{Read, Write};

enum MsgType {
    RA_MSG0,
    RA_MSG1,
    RA_MSG2,
    RA_MSG3,
    RA_VEREFICATION,
    RA_ATT_RESULT,
    RA_APP_ATT_OK
}

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern "C" {
    fn ecall_test(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        some_string: *const u8,
        len: usize,
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

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0; 512];

    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
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

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Server listening on port 3000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream)
                });
            },
            Err(e) => { 
                eprintln!("Connection error {}", e);
            }
        }
    }

    let input_string = String::from("Sending this string to the enclave then printing it\n");

    let result = unsafe {
        ecall_test(
            enclave.geteid(),
            &mut retval,
            input_string.as_ptr() as *const u8,
            input_string.len(),
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

    enclave.destroy();
    drop(listener);
}
