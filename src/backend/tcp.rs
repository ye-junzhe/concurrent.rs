use std::{
    net::{TcpListener, TcpStream},
    io::{BufRead, BufReader, Write},
    fs,
    thread,
    time::Duration
};

use super::thread::ThreadPool;

const FAIL_RESPOND: &str = "HTTP/1.1 404 NOT FOUND";
const OK_RESPOND: &str = "HTTP/1.1 200 OK";

pub fn start_listener() -> Result<TcpListener, ()> {
    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(&addr)
        .map_err(|err| {
        eprintln!("[ERROR] FAILED STARTING THE TCP SERVER ON {}: {}", addr, err);
    })?;

    println!("SERVER STARTS AT: {}", addr);

    Ok(listener)
}

pub fn receiving_stream(listener: TcpListener) {
    let tp = ThreadPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => tp.execute(|| if let Err(_) = handle_connection(stream) { return } ),
            Err(e) => eprintln!("[ERROR] FAILED RECEIVING STREAM: {}", e),
        };
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) -> Result<String, ()> {
    let buf_reader = BufReader::new(&mut stream);
    let bad_result: Result<String, std::io::Error> = Result::<String, std::io::Error>::Ok(FAIL_RESPOND.to_string());

    let request_line = buf_reader.lines().next().unwrap_or_else(|| bad_result).unwrap_or_else(|_| FAIL_RESPOND.to_string());
    let (status_line, filename) = routes(&request_line);

    let contents = fs::read_to_string(filename).map_err(|err| {
        eprintln!("[ERROR] FAILED FINDING FILE {}: {}", filename, err);
    })?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).map_err(|err| { eprintln!("[ERROR] FAILED WRITING RESPONSE TO STREAM: {}", err); })?;

    Ok(response)
}

fn routes(request_line: &String) -> (&str, &str) {
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => (OK_RESPOND, "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (OK_RESPOND, "index.html")
        }
        "GET /chat HTTP/1.1" => (OK_RESPOND, "chat.html"),
        _ => (FAIL_RESPOND, "404.html"),
    };

    (status_line, filename)
}

