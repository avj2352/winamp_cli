use anyhow::{Context, Result, ensure};
use clap::ArgMatches;
use colored::Colorize;
use crossterm::{
    cursor::{Hide, MoveToColumn, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use dialoguer::{Select, theme::ColorfulTheme};
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufReader, IsTerminal, Write},
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::util::audio_files::{display_name, music_files};

pub fn handle_play_music(args: &ArgMatches) -> Result<()> {
    let looping = args.get_one::<bool>("loop").copied().unwrap_or(false);
    let files = music_files()?;
    if files.is_empty() {
        writeln!(
            io::stdout(),
            "{}",
            "No supported audio files found in MUSIC_FOLDER.".yellow()
        )?;
        return Ok(());
    }
    ensure!(
        io::stdin().is_terminal() && io::stdout().is_terminal() && io::stderr().is_terminal(),
        "The play command requires an interactive terminal; use ls to list files in a pipe."
    );
    let names: Vec<_> = files.iter().map(|p| display_name(p)).collect();
    let Some(index) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("♫ Select an audio file".cyan().bold().to_string())
        .items(&names)
        .default(0)
        .max_length(15)
        .interact_opt()?
    else {
        return Ok(());
    };

    let file = File::open(&files[index]).context("Could not open selected audio file")?;
    let decoder = playback_source(file, looping)?;
    let duration = if looping {
        Decoder::try_from(File::open(&files[index])?)?.total_duration()
    } else {
        decoder.total_duration()
    };
    let history = Arc::new(Mutex::new(VecDeque::with_capacity(80)));
    let source = WaveformSource::new(decoder, Arc::clone(&history));
    let mut stream = OutputStreamBuilder::open_default_stream()
        .context("Could not open an audio output device")?;
    stream.log_on_drop(false);
    let sink = Sink::connect_new(stream.mixer());
    writeln!(
        io::stdout(),
        "{} {}",
        "♫ Now playing:".green().bold(),
        names[index].cyan()
    )?;
    if looping {
        writeln!(io::stdout(), "{}", "Loop: ON".green().bold())?;
    }
    writeln!(
        io::stdout(),
        "{}",
        "← / →: seek -10s / +10s • Space: pause/resume • q / Esc / Ctrl-C: stop".yellow()
    )?;
    let terminal = PlaybackTerminal::enter()?;
    sink.append(source);
    let mut stopped = false;
    let mut seek_error = None;
    while !sink.empty() {
        let width = terminal::size()?.0.saturating_sub(1) as usize;
        let samples = history.lock().unwrap_or_else(|p| p.into_inner());
        let bars: String = samples
            .iter()
            .rev()
            .take(width.min(80))
            .rev()
            .map(|&v| amplitude_bar(v))
            .collect();
        drop(samples);
        let status = if sink.is_paused() {
            "Paused"
        } else {
            "Playing"
        };
        let total = duration
            .map(format_time)
            .unwrap_or_else(|| "--:--".to_owned());
        let elapsed = format_time(sink.get_pos());
        let mut line = if looping {
            format!("Loop: ON • {status} {elapsed} elapsed  {bars}")
        } else {
            format!("{status} {elapsed} / {total}  {bars}")
        };
        if let Some(error) = &seek_error {
            line = format!("Seek failed: {error} • {line}");
        }
        // A single bounded line avoids scrolling, including on small terminals.
        let line: String = line.chars().take(width).collect();
        let mut out = io::stdout().lock();
        execute!(out, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        write!(out, "{}", line.bright_cyan())?;
        out.flush()?;
        drop(out);
        if event::poll(Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    stopped = true;
                    break;
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    stopped = true;
                    break;
                }
                KeyCode::Left | KeyCode::Right => {
                    let target = seek_target(sink.get_pos(), duration, looping, key.code);
                    seek_error = sink.try_seek(target).err().map(|error| error.to_string());
                }
                KeyCode::Char(' ') => {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                _ => {}
            }
        }
    }
    sink.stop();
    drop(terminal);
    writeln!(
        io::stdout(),
        "{}",
        if stopped {
            "Playback stopped."
        } else {
            "Playback finished."
        }
        .green()
    )?;
    Ok(())
}

fn playback_source(file: File, looping: bool) -> Result<Box<dyn Source<Item = f32> + Send>> {
    if looping {
        let len = file
            .metadata()
            .context("Could not read selected audio file metadata")?
            .len();
        let decoder = Decoder::builder()
            .with_data(BufReader::new(file))
            .with_byte_len(len)
            .with_seekable(true)
            .build_looped()
            .context("Could not decode selected audio file")?;
        Ok(Box::new(decoder))
    } else {
        Ok(Box::new(
            Decoder::try_from(file).context("Could not decode selected audio file")?,
        ))
    }
}

fn seek_target(
    position: Duration,
    duration: Option<Duration>,
    looping: bool,
    key: KeyCode,
) -> Duration {
    let position = match duration {
        Some(total) if looping && !total.is_zero() => {
            Duration::from_nanos((position.as_nanos() % total.as_nanos()) as u64)
        }
        _ => position,
    };
    let step = Duration::from_secs(10);
    let target = if key == KeyCode::Left {
        position.saturating_sub(step)
    } else {
        position.saturating_add(step)
    };
    duration.map_or(target, |total| target.min(total))
}

fn format_time(time: Duration) -> String {
    format!("{:02}:{:02}", time.as_secs() / 60, time.as_secs() % 60)
}

fn amplitude_bar(value: f32) -> char {
    const BARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    BARS[(value.clamp(0.0, 1.0) * 8.0).round() as usize]
}

struct PlaybackTerminal;
impl PlaybackTerminal {
    fn enter() -> Result<Self> {
        terminal::enable_raw_mode()?;
        let guard = Self;
        execute!(io::stdout(), Hide)?;
        Ok(guard)
    }
}
impl Drop for PlaybackTerminal {
    fn drop(&mut self) {
        let _ = execute!(
            io::stdout(),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            Show
        );
        let _ = terminal::disable_raw_mode();
    }
}

/// Collect a peak every 20 ms of decoded audio, without buffering the entire track.
struct WaveformSource<S> {
    inner: S,
    history: Arc<Mutex<VecDeque<f32>>>,
    window: usize,
    count: usize,
    peak: f32,
}
impl<S: Source> WaveformSource<S> {
    fn new(inner: S, history: Arc<Mutex<VecDeque<f32>>>) -> Self {
        let window = (inner.sample_rate() as usize * inner.channels() as usize / 50).max(1);
        Self {
            inner,
            history,
            window,
            count: 0,
            peak: 0.0,
        }
    }
}
impl<S: Source> Iterator for WaveformSource<S> {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let sample = self.inner.next()?;
        self.peak = self.peak.max(sample.abs());
        self.count += 1;
        if self.count >= self.window {
            let mut history = self.history.lock().unwrap_or_else(|p| p.into_inner());
            if history.len() == 80 {
                history.pop_front();
            }
            history.push_back(self.peak);
            self.count = 0;
            self.peak = 0.0;
        }
        Some(sample)
    }
}
impl<S: Source> Source for WaveformSource<S> {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }
    fn channels(&self) -> u16 {
        self.inner.channels()
    }
    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }
    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
    fn try_seek(&mut self, pos: Duration) -> Result<(), rodio::source::SeekError> {
        self.inner.try_seek(pos)?;
        self.count = 0;
        self.peak = 0.0;
        self.history
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rodio::buffer::SamplesBuffer;

    #[test]
    fn seek_targets_respect_track_boundaries_and_loops() {
        let total = Some(Duration::from_secs(60));
        for (position, looping, key, expected) in [
            (25, false, KeyCode::Left, 15),
            (25, false, KeyCode::Right, 35),
            (3, false, KeyCode::Left, 0),
            (55, false, KeyCode::Right, 60),
            (85, true, KeyCode::Left, 15),
            (85, true, KeyCode::Right, 35),
        ] {
            assert_eq!(
                seek_target(Duration::from_secs(position), total, looping, key),
                Duration::from_secs(expected)
            );
        }
        assert_eq!(
            seek_target(Duration::ZERO, None, false, KeyCode::Left),
            Duration::ZERO
        );
        assert_eq!(
            seek_target(Duration::from_secs(25), None, false, KeyCode::Right),
            Duration::from_secs(35)
        );
    }

    #[test]
    fn waveform_forwards_seeks_and_clears_old_peaks() {
        let history = Arc::new(Mutex::new(VecDeque::new()));
        let mut source = WaveformSource::new(
            SamplesBuffer::new(
                1,
                100,
                vec![0.5; 100]
                    .into_iter()
                    .chain(vec![0.25; 100])
                    .collect::<Vec<_>>(),
            ),
            Arc::clone(&history),
        );
        for _ in 0..3 {
            source.next();
        }
        assert!(!history.lock().unwrap().is_empty());
        source.try_seek(Duration::from_secs(1)).unwrap();
        assert!(history.lock().unwrap().is_empty());
        assert_eq!(source.count, 0);
        assert_eq!(source.peak, 0.0);
        assert_eq!(source.next(), Some(0.25));
        source.try_seek(Duration::ZERO).unwrap();
        assert_eq!(source.next(), Some(0.5));
    }

    #[test]
    fn loop_argument_controls_repetition() {
        // A short mono PCM WAV lets us verify decoding across EOF without an audio device.
        let mut file = tempfile::tempfile().unwrap();
        file.write_all(b"RIFF").unwrap();
        file.write_all(&44_u32.to_le_bytes()).unwrap();
        file.write_all(b"WAVEfmt ").unwrap();
        file.write_all(&16_u32.to_le_bytes()).unwrap();
        file.write_all(&1_u16.to_le_bytes()).unwrap();
        file.write_all(&1_u16.to_le_bytes()).unwrap();
        file.write_all(&8000_u32.to_le_bytes()).unwrap();
        file.write_all(&16000_u32.to_le_bytes()).unwrap();
        file.write_all(&2_u16.to_le_bytes()).unwrap();
        file.write_all(&16_u16.to_le_bytes()).unwrap();
        file.write_all(b"data").unwrap();
        file.write_all(&8_u32.to_le_bytes()).unwrap();
        for sample in [0_i16, 8192, -16384, 24576] {
            file.write_all(&sample.to_le_bytes()).unwrap();
        }
        for (args, looping) in [
            (vec!["play"], false),
            (vec!["play", "--loop"], true),
            (vec!["play", "--loop", "true"], true),
            (vec!["play", "--loop", "false"], false),
        ] {
            use std::io::{Seek, SeekFrom};
            file.seek(SeekFrom::Start(0)).unwrap();
            let matches = crate::features::gen_play::gen_play_command()
                .try_get_matches_from(args)
                .unwrap();
            let enabled = *matches.get_one::<bool>("loop").unwrap();
            assert_eq!(enabled, looping);
            let source = playback_source(file.try_clone().unwrap(), enabled).unwrap();
            let samples: Vec<_> = source.take(12).collect();
            let expected = [0.0, 0.25, -0.5, 0.75].repeat(if looping { 3 } else { 1 });
            assert_eq!(samples, expected);
        }
    }

    #[test]
    fn waveform_tracks_audio_and_preserves_samples() {
        let history = Arc::new(Mutex::new(VecDeque::new()));
        let samples = vec![0.0, -0.5, 1.0, 0.25];
        let source = WaveformSource::new(
            SamplesBuffer::new(1, 100, samples.clone()),
            Arc::clone(&history),
        );
        assert_eq!(source.collect::<Vec<_>>(), samples);
        assert_eq!(*history.lock().unwrap(), VecDeque::from([0.5, 1.0]));
        assert_eq!(amplitude_bar(0.0), ' ');
        assert_eq!(amplitude_bar(1.0), '█');
    }

    #[test]
    fn waveform_history_is_bounded() {
        let history = Arc::new(Mutex::new(VecDeque::new()));
        let source = WaveformSource::new(
            SamplesBuffer::new(1, 50, vec![0.5; 100]),
            Arc::clone(&history),
        );
        assert_eq!(source.count(), 100);
        assert_eq!(history.lock().unwrap().len(), 80);
    }
}
