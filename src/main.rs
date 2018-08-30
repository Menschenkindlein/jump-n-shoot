extern crate piston_window;
extern crate jump_n_shoot;

use piston_window::*;
use jump_n_shoot::App;
use std::net;
use std::env;
use std::thread;
use std::sync::mpsc;
use std::io::Read;
use std::io::Write;
use std::time::Duration;

fn serve(uri: String, fs: mpsc::Sender<usize>, tr: mpsc::Receiver<usize>) {
    let listener = net::TcpListener::bind(uri).unwrap();

    let mut stream = listener.incoming().next().unwrap().unwrap();
    stream.set_read_timeout(Some(Duration::from_millis(5)));

    let mut buff = [0; 1];

    loop {
        if let Ok(1) = stream.read(&mut buff) {
            match buff[0] {
                49 => fs.send(1).unwrap(),
                50 => fs.send(2).unwrap(),
                _ => (),
            }
        };
        match tr.try_recv() {
            Ok(1) => stream.write(b"1\n").unwrap(),
            Ok(2) => stream.write(b"2\n").unwrap(),
            _ => 0
        };
    }
}

fn connect(uri: String, fs: mpsc::Sender<usize>, tr: mpsc::Receiver<usize>) {
    let mut stream = net::TcpStream::connect(uri).unwrap();
    stream.set_read_timeout(Some(Duration::from_millis(5)));

    let mut buff = [0; 1];

    loop {
        if let Ok(1) = stream.read(&mut buff) {
            match buff[0] {
                49 => fs.send(1).unwrap(),
                50 => fs.send(2).unwrap(),
                _ => (),
            }
        };
        match tr.try_recv() {
            Ok(1) => stream.write(b"1\n").unwrap(),
            Ok(2) => stream.write(b"2\n").unwrap(),
            _ => 0
        };
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Jump & Shoot", [500, 250])
        .build()
        .unwrap();

    let to_sender;
    let from_receiver;

    let online = env::var("JNS_ONLINE").is_ok();
    if online {
        let server = env::var("JNS_SERVER").is_ok();
        let uri = env::var("JNS_URI").unwrap();
        let from = mpsc::channel();
        let to = mpsc::channel();

        to_sender = to.0;
        from_receiver = from.1;

        let from_sender = from.0;
        let to_receiver = to.1;

        if server {
            thread::spawn(|| serve(uri, from_sender, to_receiver));
        } else {
            thread::spawn(|| connect(uri, from_sender, to_receiver));
        };
    } else {
        let dummy = mpsc::channel();

        to_sender = dummy.0;
        from_receiver = dummy.1;
    }

    let mut app = App::new();

    while let Some(e) = window.next() {
        e.update(|args| {
            if online {
                match from_receiver.try_recv() {
                    Ok(1) => app.key(Key::Up),
                    Ok(2) => app.key(Key::Left),
                    _ => (),
                }
            }
            app.update(args.dt);
        });
        e.press(|button| {
            if let Button::Keyboard(key) = button {
                if online {
                    match key {
                        Key::W => to_sender.send(1).unwrap(),
                        Key::D => to_sender.send(2).unwrap(),
                        _ => (),
                    }
                }
                app.key(key)
            }
        });
        window.draw_2d(&e, |c, g| app.view().render(c, g));
    }
}
