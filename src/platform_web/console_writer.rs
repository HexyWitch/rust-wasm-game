use std::mem;
use std::io;
use std::str;

use super::js;

#[allow(dead_code)]
pub struct ConsoleWriter(Vec<u8>);
#[allow(dead_code)]
impl ConsoleWriter {
    pub fn new() -> ConsoleWriter {
        ConsoleWriter(Vec::new())
    }
}
impl io::Write for ConsoleWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for i in buf {
            self.0.push(*i);
            if *i == '\n' as u8 {
                let buf = mem::replace(&mut self.0, Vec::new());
                js::console_log(str::from_utf8(&buf).unwrap());
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        let buf = mem::replace(&mut self.0, Vec::new());
        js::console_log(str::from_utf8(&buf).unwrap());
        Ok(())
    }
}
