use std::thread::sleep;
use std::time::Duration;
use rusb::{ConfigDescriptor, Context, Device, DeviceHandle, UsbContext};

mod libusb_info;
mod hotplug;
mod list_devices;
mod read_device;

const VID: u16 = 0xae0c;
const PID: u16 = 0x001e;

// fn main() {
//     rusb::set_log_level(rusb::LogLevel::Debug);
//     // for device in rusb::devices().unwrap().iter() {
//     //     let device_desc = device.device_descriptor().unwrap();
//     //
//     //     println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
//     //              device.bus_number(),
//     //              device.address(),
//     //              device_desc.vendor_id(),
//     //              device_desc.product_id());
//     // }
//
//     libusb_info::main();
//     //hotplug::main();
//     //list_devices::main();
//     let args: Vec<String>;
//
//     args = vec!["read_device".to_string(), VID.to_string(), PID.to_string()];
//     read_device::main(args);
// }

fn main() -> rusb::Result<()> {
    let mut context = Context::new()?;
    let (mut device, mut handle) =
        open_device(&mut context, VID, PID).expect("Failed to open USB device");

    print_device_info(&mut handle)?;
    handle.claim_interface(0).expect("Failed to claim interface 0");

    //test_with_color(&mut handle)?;


    //send_start_signal(&mut handle).expect("Failed to send start signal");
    //set_color(&mut handle, 20, 20, 20)?;
    //std::thread::sleep(Duration::from_secs(20));
    //send_stop_signal(&mut handle).expect("Failed to send stop signal");

    // let name = "TESTNAME - Yes a test name";
    // set_name(&mut handle, name)?;

    // let name = match request_name(&mut handle) {
    //     Ok(n) => {
    //         println!("Name: {}", n); n
    //     },
    //     Err(e) => {
    //         println!("Error reading name: {}", e);
    //         return Ok(());
    //     }
    // };


    // let file_name = match request_file_name(&mut handle) {
    //     Ok(n) => {
    //         println!("File Name: {}", n); n
    //     },
    //     Err(e) => {
    //         println!("Error reading file name: {}", e);
    //         return Ok(());
    //     }
    // };

    //set_group_name(&mut handle, "TEST")?;

    // let group_name = match request_group_name(&mut handle) {
    //     Ok(n) => {
    //         println!("Group Name: {}", n); n
    //     },
    //     Err(e) => {
    //         println!("Error reading group name: {}", e);
    //         return Ok(());
    //     }
    // };


    // let mut program_line = [0u8; 6+32];
    // get_program_line(&mut handle, &[0xc0, 0x49], &mut program_line)?;
    // println!("Program line: {:?}", program_line);
    // print_as_hex(&program_line);
    // print_as_hex(&program_line[6..]);


    // let mut program_line = [0u8; 16];
    // for i in 0..16{
    //     program_line[i] = 0xff;
    // }
    // program_line[0] = 0x01;
    // program_line[1] = 0x20;
    // program_line[2] = 0x20;
    // program_line[3] = 0x00;
    //
    // program_line[4] = 0x02;
    // program_line[5] = 0xc8; //2 seconds delay
    //
    // program_line[6] = 0x01;
    // program_line[7] = 0x00;
    // program_line[8] = 0x20;
    // program_line[9] = 0x00;
    //
    // program_line[10] = 0x02;
    // program_line[11] = 0xc8; //2 seconds delay
    //
    // program_line[12] = 0x01;
    // program_line[13] = 0x00;
    // program_line[14] = 0x00;
    // program_line[15] = 0x20;
    //
    // print_as_hex(&program_line);
    // set_program_line(&mut handle, &[0x00, 0x40], &program_line, true)?;
    //
    // for i in 0..16{
    //     program_line[i] = 0xff;
    // }
    //
    // program_line[0] = 0x02;
    // program_line[1] = 0xc8; //2 seconds delay
    //
    // program_line[2] = 0x01;
    // program_line[3] = 0x20;
    // program_line[4] = 0x00;
    // program_line[5] = 0x20;
    //
    // program_line[6] = 0x02;
    // program_line[7] = 0xc8; //2 seconds delay
    //
    // program_line[8] = 0xff;
    //
    // print_as_hex(&program_line);
    // set_program_line(&mut handle, &[0x10, 0x40], &program_line, false)?;
    //
    // sleep(Duration::from_secs(1));
    // send_start_signal(&mut handle)?;
    // sleep(Duration::from_secs(7));
    // send_stop_signal(&mut handle)?;


    set_file_name(&mut handle, "TestWriteFileName")?;
    sleep(Duration::from_secs(1));
    let file_name = request_file_name(&mut handle)?;
    println!("File Name: {}", file_name);

    handle.release_interface(0).expect("Failed to release interface 0");
    Ok(())
}

///Requesting name stops playback on device as side effect
fn request_name(handle: &mut DeviceHandle<Context>) -> rusb::Result<(String)>{
    let mut write_buffer = [0u8; 4];
    let mut result_buffer = [0u8; 64];

    write_buffer[0] = 0x04;//
    write_buffer[1] = 0x08;//
    write_buffer[2] = 0x80;//
    write_buffer[3] = 0x00;//

    read_continuous_values(handle, &mut write_buffer, 8, &mut result_buffer)?;
    //println!("Result buffer: {:?}", result_buffer);

    //trim the null bytes
    //let result = &result_buffer[..result_buffer.iter().position(|&x| x == 0).unwrap()];

    let result = std::str::from_utf8(&result_buffer).unwrap();
    let owned_result = result.to_string();
    return Ok(owned_result);
}

fn set_name(handle: &mut DeviceHandle<Context>, name: &str) -> rusb::Result<()>{
    let mut write_buffer = [0u8; 6+64];

    write_buffer[0] = 0x05;//
    write_buffer[1] = 0x08;//
    write_buffer[2] = 0x80;//
    write_buffer[3] = 0x00;//
    write_buffer[4] = 0x00;//
    write_buffer[5] = 0x00;//

    //convert name to bytes
    let name_bytes = name.as_bytes();
    assert!(name_bytes.len() <= 64, "Name must not be longer than 64 bytes");
    for (i, byte) in name_bytes.iter().enumerate(){
        write_buffer[6+i] = *byte;
    }
    write_continuous_values(handle, &mut write_buffer, 8)?;

    Ok(())
}

///Requesting file name stops playback on device as side effect
fn request_file_name(handle: &mut DeviceHandle<Context>) -> rusb::Result<String>{
    let mut write_buffer = [0u8; 4];
    let mut result_buffer = [0u8; 64];

    write_buffer[0] = 0x04;//
    write_buffer[1] = 0x08;//
    write_buffer[2] = 0xc0;//
    write_buffer[3] = 0x00;//

    read_continuous_values(handle, &mut write_buffer, 8, &mut result_buffer)?;
    let result = std::str::from_utf8(&result_buffer).unwrap();
    let owned_result = result.to_string();
    Ok(owned_result)
}

fn set_file_name(handle: &mut DeviceHandle<Context>, name: &str) -> rusb::Result<()>{
    let mut write_buffer = [0u8; 6+64];

    write_buffer[0] = 0x05;//
    write_buffer[1] = 0x08;//
    write_buffer[2] = 0xc0;//
    write_buffer[3] = 0x00;//
    write_buffer[4] = 0x00;//
    write_buffer[5] = 0x00;//

    //convert name to bytes
    let name_bytes = name.as_bytes();
    assert!(name_bytes.len() <= 64, "Name must not be longer than 64 bytes");
    for (i, byte) in name_bytes.iter().enumerate(){
        write_buffer[6+i] = *byte;
    }
    write_continuous_values(handle, &mut write_buffer, 8)?;

    Ok(())
}

///Requesting group name stops playback on device as side effect
fn request_group_name(handle: &mut DeviceHandle<Context>) -> rusb::Result<String>{
    let mut write_buffer = [0u8; 4];
    let mut result_buffer = [0u8; 6+4];

    write_buffer[0] = 0x04;//
    write_buffer[1] = 0x04;//
    write_buffer[2] = 0x7c;//
    write_buffer[3] = 0x00;//

    write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut result_buffer)?;
    //read_continous_values(handle, &mut write_buffer, 1, &mut result_buffer)?;
    let result = std::str::from_utf8(&result_buffer[6..]).unwrap();
    let owned_result = result.to_string();
    Ok(owned_result)
}

fn set_group_name(handle: &mut DeviceHandle<Context>, name: &str) -> rusb::Result<()>{
    let mut write_buffer = [0u8; 6+4];
    let mut result_buffer = [0u8; 6+4];

    write_buffer[0] = 0x05;//
    write_buffer[1] = 0x04;//
    write_buffer[2] = 0x7c;//
    write_buffer[3] = 0x00;//
    write_buffer[5] = 0x00;//

    //convert name to bytes
    let name_bytes = name.as_bytes();
    assert!(name_bytes.len() <= 4, "Group Name must be no longer than 4 bytes");
    for (i, byte) in name_bytes.iter().enumerate(){
        write_buffer[i+6] = *byte;
    }

    write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut result_buffer)?;
    assert_eq!(result_buffer[0], 0x05, "Failed to set group name");

    //read_continous_values(handle, &mut write_buffer, 1, &mut result_buffer)?;

    Ok(())
}

fn read_continuous_values(handle: &mut DeviceHandle<Context>, write_buffer: &mut [u8], amount_block_of_8: usize, result_buffer: &mut [u8]) -> rusb::Result<()>{
    let start = write_buffer[2];
    for i in 0..amount_block_of_8{
        write_buffer[2] =(start + (i*8) as u8);


        let mut read_buffer = [0u8; 14];
        write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut read_buffer)?;
        println!("Read buffer: {:?}", read_buffer);
        //remove the first 6 bytes and take the following 8 bytes
        let partial_result = &read_buffer[6..(6+8)];
        for (partial_buffer_index, byte) in partial_result.iter().enumerate(){
            let buffer_index = partial_buffer_index  + (i*8) as usize;
            println!("Writing byte {} with content {} to result buffer", buffer_index,byte);
            result_buffer[buffer_index] = *byte;
        }
    }
    Ok(())
}

fn write_continuous_values(handle: &mut DeviceHandle<Context>, write_buffer: &mut [u8], amount_block_of_8: usize) -> rusb::Result<()>{
    let start = write_buffer[2];
    for i in 0..amount_block_of_8 {
        write_buffer[2] = (start + (i * 8) as u8);

        let mut read_buffer = [0u8; 1];
        let partial_write_buffer = [&write_buffer[0..6], &write_buffer[(6+(i*8))..(6+(i*8)+8)]].concat();
        println!("Partial write buffer: {:?}", partial_write_buffer);
        write_read_bulk(handle, 0x01, &partial_write_buffer, 0x81, &mut read_buffer)?;
        assert_eq!(read_buffer[0], 0x05, "Failed to write continuous values")
    }
    Ok(())
}

fn test_with_color(handle: &mut DeviceHandle<Context>) -> rusb::Result<()>{
    let mut write_buffer = [0u8; 4];
    //63 00 ff 80
    write_buffer[0] = 0x63;//set_color
    write_buffer[1] = 0x00;//r
    write_buffer[2] = 0xff;//g
    write_buffer[3] = 0x80;//b
    write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut [0u8; 32])?;

    //sleep
    std::thread::sleep(Duration::from_secs(5));

    let mut write_buffer = [0u8; 4];
    write_buffer[0] = 0x63;//set_color? - without color aka 0x00 turns off

    write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut [0u8; 32])?;
    Ok(())
}

fn write_read_bulk(handle: &mut DeviceHandle<Context>, write_endpoint: u8, write_data: &[u8], read_endpoint: u8, read_data: &mut [u8]) -> rusb::Result<()> {
    write_bulk(handle, write_endpoint, write_data)?;
    read_bulk(handle, read_endpoint, read_data)?;

    //compare buffers
    if(write_data.len() != read_data.len()){
        println!("Write and read buffer sizes do not match");
        return Ok(());
    }

    for i in 0..write_data.len() {
        if write_data[i] != read_data[i] {
            println!("Mismatch at index {}", i);
        }
    }

    Ok(())
}

fn write_bulk(handle: &mut DeviceHandle<Context>, endpoint: u8, data: &[u8]) -> rusb::Result<()> {
    let timeout = Duration::from_secs(1);
    let result = handle.write_bulk(endpoint, data, timeout);
    match result {
        Ok(count) => {
            println!("Wrote {} bytes to endpoint {}", count, endpoint);
            Ok(())
        }
        Err(e) => {
            println!("Error writing to endpoint {}: {}", endpoint, e);
            Err(e)
        }
    }
}

fn read_bulk(handle: &mut DeviceHandle<Context>, endpoint: u8, data: &mut [u8]) -> rusb::Result<()> {
    let timeout = Duration::from_secs(1);
    let result = handle.read_bulk(endpoint, data, timeout);
    match result {
        Ok(count) => {
            println!("Read {} bytes from endpoint {}", count, endpoint);
            Ok(())
        }
        Err(e) => {
            println!("Error reading from endpoint {}: {}", endpoint, e);
            Err(e)
        }
    }
}

fn send_start_signal (handle: &mut DeviceHandle<Context>)-> rusb::Result<()>{
    let reqT = rusb::request_type(rusb::Direction::Out, rusb::RequestType::Vendor, rusb::Recipient::Endpoint);
    let r = handle.write_control(reqT, 0xd1, 0x00, 0x00, &[], Duration::from_secs(1));
    match r {
        Ok(count) => {
            println!("Wrote {} bytes to control endpoint", count);
            Ok(())
        }
        Err(e) => {
            println!("Error writing to control endpoint: {}", e);
            Err(e)
        }
    }
}

fn send_stop_signal(handle: &mut DeviceHandle<Context>)-> rusb::Result<()>{
    let mut write_buffer = [0u8; 4];
    write_buffer[0] = 0x63;//set_color? - without color aka 0x00 turns off

    let mut res_buffer = [0u8; 4];
    let r = match write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut res_buffer){
        Ok(_) => {
            println!("Wrote stop signal");
            Ok(())
        },
        Err(e) => {
            println!("Error writing stop signal: {}", e);
            Err(e)
        }
    };
    Ok(())
}

fn set_color(handle: &mut DeviceHandle<Context>, r: u8, g: u8, b: u8) -> rusb::Result<()>{
    let mut write_buffer = [0u8; 4];
    write_buffer[0] = 0x63;//set_color
    write_buffer[1] = r;
    write_buffer[2] = g;
    write_buffer[3] = b;

    let mut res_buffer = [0u8; 4];
    let r = match write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut res_buffer){
        Ok(_) => {
            println!("Wrote color signal");
            Ok(())
        },
        Err(e) => {
            println!("Error writing color signal: {}", e);
            Err(e)
        }
    };
    Ok(())
}

fn get_program_line(handle: &mut DeviceHandle<Context>, address: &[u8], data: &mut [u8]) -> rusb::Result<()>{
    assert_eq!(address.len(), 2, "Address must be 2 bytes long");
    let mut write_buffer = [0u8; 6];
    write_buffer[0] = 0x01;//get_program
    write_buffer[1] = 0x10;//packet length
    write_buffer[2] = address[0];
    write_buffer[3] = address[1];
    write_buffer[4] = 0x00;
    write_buffer[5] = 0x00;

    println!("Write buffer: {:?}", write_buffer);
    //print as hex
    print_as_hex(&write_buffer);

    let r = match write_read_bulk(handle, 0x01, &write_buffer, 0x81, data){
        Ok(_) => {
            println!("Wrote get program signal");
            Ok(())
        },
        Err(e) => {
            println!("Error writing get program signal: {}", e);
            Err(e)
        }
    };
    Ok(())

}

fn set_program_line(handle: &mut DeviceHandle<Context>, address: &[u8], data: &[u8], send03: bool) -> rusb::Result<()>{
    assert_eq!(address.len(), 2, "Address must be 2 bytes long");
    assert_eq!(data.len(), 16, "Data must be 16 bytes long");
    let mut write_buffer = [0u8; 6+16];
    write_buffer[0] = 0x02;//set_program
    write_buffer[1] = 0x10;//packet length
    write_buffer[2] = address[0];
    write_buffer[3] = address[1];
    write_buffer[4] = 0x00;
    write_buffer[5] = 0x00;

    for (i, byte) in data.iter().enumerate(){
        write_buffer[i+6] = *byte;
    }

    print_as_hex(&write_buffer);

    if send03 {
        let r = match write_read_bulk(handle, 0x01, &[0x03,0x01,address[0], address[1], 0x00, 0x00], 0x81, &mut [0u8; 32]){
            Ok(_) => {
                println!("Wrote set program signal");
                Ok(())
            },
            Err(e) => {
                println!("Error writing set program signal: {}", e);
                Err(e)
            }
        };
    }

    let r2 = match write_read_bulk(handle, 0x01, &write_buffer, 0x81, &mut [0u8; 32]){
        Ok(_) => {
            println!("Wrote set program signal");
            Ok(())
        },
        Err(e) => {
            println!("Error writing set program signal: {}", e);
            Err(e)
        }
    };
    Ok(())
}

fn print_as_hex(data: &[u8]){
    for byte in data.iter(){
        print!("{:02x} ", byte);
    }
    println!();
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };
    let mut counter = 0;
    for device in devices.iter(){
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid{
            counter += 1;
        }
    }
    println!("Found {} devices", counter);

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> rusb::Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);

    if !languages.is_empty() {
        let language = languages[0];
        println!("Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    }
    Ok(())
}
