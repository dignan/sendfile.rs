extern crate sendfile;

use std::thread;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Shutdown;

#[test]
fn test_sendfile() {
    let test_string = String::from("Hello tests");

    // Accept the connection that will come in in another thread
    let listening_thread = thread::spawn(move|| {
        return listen_for_sendfile(9009u16);
    });

    // Create a file and fill it with some data
    let mut f = File::create("/tmp/testsendfile").unwrap();
    let bytes = test_string.clone().into_bytes();
    let bytes_len = bytes.len();

    // Write our junk to the file
    let bytes_written_to_file = f.write(&bytes).unwrap();
    println!("\nWrote {} bytes to the file", bytes_written_to_file);
    f.flush().unwrap();

    // Call the Mac OS sendfile implementation
    let source = File::open("/tmp/testsendfile").unwrap();
    let mut sink_maybe = None;
    let mut connected = false;
    let mut tries = 0;

    while !connected && tries < 5{
        println!("connected={}", connected);
        match TcpStream::connect("127.0.0.1:9009") {
            Ok(stream) => {
                sink_maybe = Some(stream);
                connected = true;
            }
            Err(e) => {
                tries += 1;
                println!("Couldn't connect: {}", e);
                thread::sleep_ms(100);
            }
        }
    }

    match sink_maybe {
        Some(sink) => {
            println!("Connected to {} as {}", sink.peer_addr().unwrap(), sink.local_addr().unwrap());
            let sendfile_bytes_sent = sendfile::sendfile(&source, &sink, 0, bytes_len as i64).unwrap();
            println!("Sent {} bytes using sendfile", sendfile_bytes_sent);

            let received_string = listening_thread.join().unwrap();
            println!("Got '{}'", &received_string);
            assert_eq!(received_string, test_string);
        }
        None => panic!("Couldn't connect to the test server")
    }
}

#[test]
fn test_sendfile_with_offset() {
    let test_string = String::from("Hello tests");

    // Accept the connection that will come in in another thread
    let listening_thread = thread::spawn(move|| {
        return listen_for_sendfile(9006u16);
    });

    // Create a file and fill it with some data
    let mut f = File::create("/tmp/testsendfile").unwrap();
    let bytes = test_string.clone().into_bytes();
    let bytes_len = bytes.len();

    // Write our junk to the file
    let bytes_written_to_file = f.write(&bytes).unwrap();
    println!("\nWrote {} bytes to the file", bytes_written_to_file);
    f.flush().unwrap();

    // Call the Mac OS sendfile implementation
    let source = File::open("/tmp/testsendfile").unwrap();

    let mut sink_maybe = None;
    let mut connected = false;
    let mut tries = 0;

    while !connected && tries < 5{
        println!("connected={}", connected);
        match TcpStream::connect("127.0.0.1:9006") {
            Ok(stream) => {
                sink_maybe = Some(stream);
                connected = true;
            }
            Err(e) => {
                tries += 1;
                println!("Couldn't connect: {}", e);
                thread::sleep_ms(100);
            }
        }
    }

    match sink_maybe {
        Some(sink) => {
            println!("Connected to {} as {}", sink.peer_addr().unwrap(), sink.local_addr().unwrap());
            let sendfile_bytes_sent = sendfile::sendfile(&source, &sink, 3, bytes_len as i64).unwrap();
            println!("Sent {} bytes using sendfile", sendfile_bytes_sent);

            let received_string = listening_thread.join().unwrap();
            println!("Got '{}'", &received_string);

            assert_eq!(received_string, (&test_string[3..]));
        }
        None => panic!("Couldn't connect to the test server")
    }
}

fn listen_for_sendfile(port: u16) -> String {
    // Create the socket to listen on first
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    let stream_and_addr = listener.accept().unwrap();

    let mut stream = stream_and_addr.0;
    let addr = stream_and_addr.1;

    println!("Connection from: {}", addr);
    let mut received_data = [0; 32];

    println!("Trying to receive from stream");
    let received_bytes = stream.read(&mut received_data).unwrap();
    println!("Received {} bytes", received_bytes);

    // Close the socket
    stream.shutdown(Shutdown::Both).unwrap();
    drop(stream);
    drop(listener);

    let mut str_vec = Vec::with_capacity(received_bytes);
    str_vec.extend(received_data[0..received_bytes].iter());
    return String::from_utf8(str_vec).unwrap();
}
