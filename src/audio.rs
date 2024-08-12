use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref AUDIO_HANDLE: Arc<Mutex<Option<OutputStreamHandle>>> = {
        match OutputStream::try_default() {
            Ok((stream, handle)) => {
                // Mantener 'stream' vivo durante toda la ejecución del programa
                std::mem::forget(stream);
                Arc::new(Mutex::new(Some(handle)))
            },
            Err(e) => {
                eprintln!("Failed to obtain audio device: {}", e);
                Arc::new(Mutex::new(None))
            }
        }
    };
}

pub fn play_background_music() {
    let handle_lock = AUDIO_HANDLE.lock().unwrap();
    let handle = match handle_lock.as_ref() {
        Some(handle) => handle,
        None => {
            eprintln!("No audio device available.");
            return;
        }
    };

    let file = BufReader::new(File::open("src\\assets\\soundtrack.mp3").unwrap());
    let source = Decoder::new(file).unwrap().amplify(0.2).repeat_infinite();
    let sink = Sink::try_new(handle).unwrap();

    sink.append(source);
    sink.detach(); // Permite que el sonido continúe reproduciéndose sin bloquear otros procesos.
}
