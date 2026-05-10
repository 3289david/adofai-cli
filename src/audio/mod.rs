use anyhow::Result;
use std::path::Path;

pub struct AudioEngine {
    _stream: Option<rodio::OutputStream>,
    _handle: Option<rodio::OutputStreamHandle>,
    sink: Option<rodio::Sink>,
    pub volume: f32,
    pub playing: bool,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (stream, handle) = match rodio::OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };

        Self {
            _stream: stream,
            _handle: handle,
            sink: None,
            volume: 1.0,
            playing: false,
        }
    }

    pub fn load_and_play(&mut self, path: &Path) -> Result<()> {
        let handle = match &self._handle {
            Some(h) => h,
            None => return Ok(()),
        };

        if !path.exists() {
            return Ok(());
        }

        let file = std::fs::File::open(path)?;
        let source = rodio::Decoder::new(std::io::BufReader::new(file))?;
        let sink = rodio::Sink::try_new(handle)?;
        sink.set_volume(self.volume);
        sink.append(source);

        self.sink = Some(sink);
        self.playing = true;
        Ok(())
    }

    pub fn play_hitsound(&self) {
        // Play a short beep as hitsound
        let handle = match &self._handle {
            Some(h) => h,
            None => return,
        };

        if let Ok(sink) = rodio::Sink::try_new(handle) {
            let source = rodio::source::SineWave::new(800.0);
            use rodio::Source;
            let source = source.take_duration(std::time::Duration::from_millis(30));
            let source = source.amplify(0.3);
            sink.append(source);
            sink.detach();
        }
    }

    pub fn pause(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
            self.playing = false;
        }
    }

    pub fn resume(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.play();
            self.playing = true;
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        if let Some(ref sink) = self.sink {
            sink.set_volume(volume);
        }
    }

    pub fn stop(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.stop();
        }
        self.sink = None;
        self.playing = false;
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.stop();
    }
}
