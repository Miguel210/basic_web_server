use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {

    //Iniciar servidor
    let address = "127.0.0.1:8000"; //127.0.0.1 asta 127.0.0.7
    let listener = TcpListener::bind(&address).unwrap();

    println!("Servidor iniciado en {}",&address);
    //escuchar por conexion
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
//manejar las conexiones
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Stream recibido");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1";//127.0.0.1:8000/
    if buffer.starts_with(get) {
        //responder al cliente
        send_index(stream);
    }else {
        send_not_found(stream);
    }


}
fn build_response(content: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}
fn send_index(mut stream: TcpStream) {
    let contents = fs::read_to_string("index.html").unwrap();
    stream.write(build_response(contents).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_not_found(mut stream: TcpStream){
    let contents = fs::read_to_string("404.html").unwrap();
    stream.write(build_response(contents).as_bytes()).unwrap();
    stream.flush().unwrap();
}