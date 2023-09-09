use std::env::args;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread::{sleep, spawn};
use std::time::Duration;
use anyhow::anyhow;
use edge_tts::{build_ssml, request_audio};

pub fn serve(mut conn: TcpStream, _addr: SocketAddr) -> anyhow::Result<()> {
    let mut buf = [0u8; 8192];
    let mut data_len = 0;
    loop {
        if data_len > buf.len() {
            conn.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes())?;
            conn.flush()?;
            return Err(anyhow!("Bad Request: header too long"));
        }
        let n = conn.read(&mut buf[data_len..])?;
        data_len += n;
        // 只读取 HTTP 请求的第一行
        // GET /tts?voice=zh-CN-XiaoxiaoNeural&text=$TTSTEXT HTTP/1.1
        if let Some(position) = buf[0..data_len].windows(2).position(|p| p[0] == b'\r' && p[1] == b'\n') {
            match from_utf8(&buf[0..position]) {
                Ok(line) => {
                    if line.starts_with("GET ") && line.ends_with(" HTTP/1.1") {
                        let path = &line["GET ".len()..(line.len() - " HTTP/1.1".len())];
                        if path.starts_with("/tts") {
                            if path.starts_with("/tts?") {
                                let query_string = &path["/tts?".len()..];
                                let mut text = "".to_string();
                                let mut voice = "zh-CN-XiaoxiaoNeural".to_string();
                                let mut pitch = "medium".to_string();
                                let mut rate = "medium".to_string();
                                let mut volume = "medium".to_string();
                                let mut output_format = "audio-24khz-48kbitrate-mono-mp3".to_string();
                                for (k, v) in form_urlencoded::parse(query_string.as_bytes()) {
                                    if k == "voice" {
                                        voice = v.into_owned();
                                    } else if k == "pitch" {
                                        pitch = v.into_owned();
                                    } else if k == "rate" {
                                        rate = v.into_owned();
                                    } else if k == "volume" {
                                        volume = v.into_owned();
                                    } else if k == "output_format" {
                                        output_format = v.into_owned();
                                    } else if k == "text" {
                                        text = v.into_owned();
                                    }
                                }
                                if text.len() == 0 {
                                    conn.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes())?;
                                    conn.flush()?;
                                    return Err(anyhow!("Bad Request: no text"));
                                }
                                match request_audio(&build_ssml(&text, &voice, &pitch, &rate, &volume), &output_format) {
                                    Ok(data) => {
                                        conn.write(format!("HTTP/1.1 200 OK\r\nContent-Type: audio/mpeg\r\nContent-Length: {}\r\n\r\n", data.len()).as_bytes())?;
                                        conn.write(&data)?;
                                        conn.flush()?;
                                        break;
                                    }
                                    Err(e) => {
                                        conn.write("HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes())?;
                                        conn.flush()?;
                                        return Err(anyhow!("500 Internal Server Error: request_audio error: {:?}", e));
                                    }
                                }
                            } else {
                                conn.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes())?;
                                conn.flush()?;
                                return Err(anyhow!("Bad Request: no query"));
                            }
                        } else {
                            conn.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?;
                            conn.flush()?;
                            return Err(anyhow!("Not Found: Unknown request path {}", path));
                        }
                    } else {
                        conn.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes())?;
                        conn.flush()?;
                        return Err(anyhow!("Bad Request: HTTP request header not GET HTTP/1.1"));
                    }
                }
                Err(e) => {
                    conn.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes())?;
                    conn.flush()?;
                    return Err(anyhow!("Bad Request: parse HTTP request header utf8 {:?}", e));
                }
            }
        } else {
            // 请求还未完全接收
        }
    }
    Ok(())
}

fn main() {
    let addr = args().skip(1).next().unwrap_or("127.0.0.1:23456".to_string());
    match TcpListener::bind(addr) {
        Ok(listener) => {
            loop {
                match listener.accept() {
                    Ok((conn, addr)) => {
                        spawn(move || {
                            println!("Accept conn from {}", addr);
                            match serve(conn, addr) {
                                Ok(_) => println!("Handle conn from {} OK", addr),
                                Err(e) => println!("Handle conn from {} error: {:?}", addr, e),
                            };
                        });
                    }
                    Err(e) => {
                        println!("Accept error: {:?}", e);
                        sleep(Duration::from_secs(1));
                    }
                }
            }
        }
        Err(e) => {
            println!("Bind error: {:?}", e);
            sleep(Duration::from_secs(1));
        }
    }
}
