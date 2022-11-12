use std::{io::ErrorKind,  time::Duration, thread};
#[allow(dead_code)]
use super::Com;
use async_trait::async_trait;
use tokio::{io::{self}, net::TcpStream, sync::oneshot, time::Timeout};

#[derive(Debug)]
pub struct TelnetCom
{
    tcp_stream: Option<TcpStream>,
    state: ParserState,
    buf: std::collections::VecDeque<u8>
}

#[derive(Debug)]
enum ParserState {
    Data,
    Iac,
    Will,
    Wont,
    Do,
    Dont
}

pub const IAC:u8 = 0xFF;

#[derive(Debug, Clone, Copy)]
enum TelnetCmd {
    /// End of subnegotiation parameters.
    SE = 0xF0,

    /// No operation.
    NOP = 0xF1,

    /// The data stream portion of a Synch.
    /// This should always be accompanied
    /// by a TCP Urgent notification.
    DataMark = 0xF2,

    /// NVT character BRK
    Break = 0xF3,
    
    /// The function Interrupt Process
    IP = 0xF4,

    // The function Abort output
    AO = 0xF5,

    // The function Are You There
    AYT = 0xF6,

    // The function Erase character
    EC = 0xF7,

    // The function Erase line
    EL = 0xF8,

    // The Go ahead signal.
    GA = 0xF9,

    // Indicates that what follows is subnegotiation of the indicated option.
    SB = 0xFA,

    ///  (option code)
    /// Indicates the desire to begin performing, or confirmation that you are now performing, the indicated option.
    WILL = 0xFB,

    /// (option code) 
    /// Indicates the refusal to perform, or continue performing, the indicated option.
    WONT = 0xFC, 

    /// (option code)
    /// Indicates the request that the other party perform, or confirmation that you are expecting
    /// the other party to perform, the indicated option.
    DO = 0xFD,

    /// (option code) 
    /// Indicates the demand that the other party stop performing,
    /// or confirmation that you are no longer expecting the other party
    /// to perform, the indicated option.
    DONT = 0xFE,

    /// Data Byte 255.
    IAC = 0xFF
}

impl TelnetCmd {
    pub fn get(byte: u8) -> io::Result<TelnetCmd> {
        let cmd = match byte {
            0xF0 => TelnetCmd::SE,
            0xF1 => TelnetCmd::NOP,
            0xF2 => TelnetCmd::DataMark,
            0xF3 => TelnetCmd::Break,
            0xF4 => TelnetCmd::IP,
            0xF5 => TelnetCmd::AO,
            0xF6 => TelnetCmd::AYT,
            0xF7 => TelnetCmd::EC,
            0xF8 => TelnetCmd::EL,
            0xF9 => TelnetCmd::GA,
            0xFA => TelnetCmd::SB,
            0xFB => TelnetCmd::WILL,
            0xFC => TelnetCmd::WONT, 
            0xFD => TelnetCmd::DO,
            0xFE => TelnetCmd::DONT,
            0xFF => TelnetCmd::IAC,
            _ => { return Err(io::Error::new(ErrorKind::InvalidData, format!("unknown IAC: {}/x{:02X}", byte, byte))); }
        };
        Ok(cmd)
    }
    pub fn to_bytes(&self) -> [u8;2] {
        [IAC, *self as u8]
    }

    pub fn to_bytes_opt(&self, opt: TelnetOption) -> [u8;3] {
        [IAC, *self as u8, opt as u8]
    }
}


/// http://www.iana.org/assignments/telnet-options/telnet-options.xhtml
#[derive(Debug, Clone, Copy, PartialEq)]
enum TelnetOption {
    /// https://www.rfc-editor.org/rfc/rfc856
    TransmitBinary = 0x00,
    /// https://www.rfc-editor.org/rfc/rfc857
    Echo = 0x01,
    /// ???
    Reconnection = 0x02,
    /// https://www.rfc-editor.org/rfc/rfc858
    SuppressGoAhead = 0x03,
    /// https://www.rfc-editor.org/rfc/rfc859
    Status = 0x05,
    /// https://www.rfc-editor.org/rfc/rfc860
    TimingMark = 0x06,
    /// https://www.rfc-editor.org/rfc/rfc726.html
    RemoteControlledTransAndEcho = 0x07,
    /// ???
    OutputLineWidth = 0x08,
    /// ???
    OutputPageSize = 0x09,
    ///https://www.rfc-editor.org/rfc/RFC652
    OutputCarriageReturnDisposition = 10,
    ///https://www.rfc-editor.org/rfc/RFC653
    OutputHorizontalTabStops = 11,
    ///https://www.rfc-editor.org/rfc/RFC654
    OutputHorizontalTabDisposition = 12,
    ///https://www.rfc-editor.org/rfc/RFC655
    OutputFormfeedDisposition = 13,
    ///https://www.rfc-editor.org/rfc/RFC656
    OutputVerticalTabstops = 14,
    ///https://www.rfc-editor.org/rfc/RFC657
    OutputVerticalTabDisposition = 15,
    ///https://www.rfc-editor.org/rfc/RFC658
    OutputLinefeedDisposition = 16,
    ///https://www.rfc-editor.org/rfc/RFC698
    ExtendedASCII = 17,
    ///https://www.rfc-editor.org/rfc/RFC727
    Logout = 18,
    ///https://www.rfc-editor.org/rfc/RFC735
    ByteMacro = 19,
    ///https://www.rfc-editor.org/rfc/RFC1043][RFC732
    DataEntryTerminal = 20,
    ///https://www.rfc-editor.org/rfc/RFC736][RFC734
    SUPDUP = 21,
    ///https://www.rfc-editor.org/rfc/RFC749
    SUPDUPOutput = 22,
    ///https://www.rfc-editor.org/rfc/RFC779
    SendLocation = 23,
    /// https://www.rfc-editor.org/rfc/rfc1091
    TerminalType = 24,
    /// https://www.rfc-editor.org/rfc/rfc885
    EndOfRecord = 25,
    /// https://www.rfc-editor.org/rfc/rfc1073
    NegotiateAboutWindowSize = 31,
    /// https://www.rfc-editor.org/rfc/rfc1079
    TerminalSpeed = 32,
    /// https://www.rfc-editor.org/rfc/rfc1372
    ToggleFlowControl = 33,
    /// https://www.rfc-editor.org/rfc/rfc1184
    LineMode = 34,
    /// https://www.rfc-editor.org/rfc/rfc1096
    XDisplayLocation = 35,
    /// https://www.rfc-editor.org/rfc/rfc1408
    EnvironmentOption = 36,
    /// https://www.rfc-editor.org/rfc/rfc2941
    Authentication = 37,
    /// https://www.rfc-editor.org/rfc/rfc2946
    Encrypt = 38,
    /// https://www.rfc-editor.org/rfc/rfc1572
    NewEnviron = 39,
    ///https://www.rfc-editor.org/rfc/RFC2355
    TN3270E = 40,
    ///https://www.rfc-editor.org/rfc/Rob_Earhart
    XAuth = 41,
    ///https://www.rfc-editor.org/rfc/RFC2066
    CharSet = 42,
    ///https://www.rfc-editor.org/rfc/Robert_Barnes
    TelnetRemoteSerialPortRSP = 43,
    ///https://www.rfc-editor.org/rfc/RFC2217
    ComPortControlOption = 44,
    ///https://www.rfc-editor.org/rfc/Wirt_Atmar
    TelnetSuppressLocalEcho = 45,
    ///https://www.rfc-editor.org/rfc/Michael_Boe
    TelnetStartTLS = 46,
    ///https://www.rfc-editor.org/rfc/RFC2840
    Kermit = 47,
    ///https://www.rfc-editor.org/rfc/David_Croft
    SendURL = 48,
    ///https://www.rfc-editor.org/rfc/Jeffrey_Altman
    ForwardX = 49,
    // 50-137 	Unassigned
    TelOptPragmaLogon = 138,///https://www.rfc-editor.org/rfc/Steve_McGregory
    TelOptSSPILogon = 139,///https://www.rfc-editor.org/rfc/Steve_McGregory
    TelOptPragmaHeartbeat = 140,///https://www.rfc-editor.org/rfc/Steve_McGregory
    // 141-254 	Unassigned
    /// https://www.rfc-editor.org/rfc/rfc861
    ExtendedOptionsList = 0xFF,
}

impl TelnetOption {
    pub fn get(byte: u8) -> io::Result<TelnetOption> {
        let cmd = match byte {
            0 => TelnetOption::TransmitBinary,
            1 => TelnetOption::Echo,
            2 => TelnetOption::Reconnection,
            3 => TelnetOption::SuppressGoAhead,
            5 => TelnetOption::Status,
            6 => TelnetOption::TimingMark,
            7 => TelnetOption::RemoteControlledTransAndEcho,
            8 => TelnetOption::OutputLineWidth,
            9 => TelnetOption::OutputPageSize,
            10 => TelnetOption::OutputCarriageReturnDisposition,
            11 => TelnetOption::OutputHorizontalTabStops,
            12 => TelnetOption::OutputHorizontalTabDisposition,
            13 => TelnetOption::OutputFormfeedDisposition,
            14 => TelnetOption::OutputVerticalTabstops,
            15 => TelnetOption::OutputVerticalTabDisposition,
            16 => TelnetOption::OutputLinefeedDisposition,
            17 => TelnetOption::ExtendedASCII,
            18 => TelnetOption::Logout,
            19 => TelnetOption::ByteMacro,
            20 => TelnetOption::DataEntryTerminal,
            21 => TelnetOption::SUPDUP,
            22 => TelnetOption::SUPDUPOutput,
            23 => TelnetOption::SendLocation,
            24 => TelnetOption::TerminalType,
            25 => TelnetOption::EndOfRecord,
            31 => TelnetOption::NegotiateAboutWindowSize,
            32 => TelnetOption::TerminalSpeed,
            33 => TelnetOption::ToggleFlowControl,
            34 => TelnetOption::LineMode,
            35 => TelnetOption::XDisplayLocation,
            36 => TelnetOption::EnvironmentOption,
            37 => TelnetOption::Authentication,
            38 => TelnetOption::Encrypt,
            39 => TelnetOption::NewEnviron,
            40 => TelnetOption::TN3270E,
            41 => TelnetOption::XAuth,
            42 => TelnetOption::CharSet,
            43 => TelnetOption::TelnetRemoteSerialPortRSP,
            44 => TelnetOption::ComPortControlOption,
            45 => TelnetOption::TelnetSuppressLocalEcho,
            46 => TelnetOption::TelnetStartTLS,
            47 => TelnetOption::Kermit,
            48 => TelnetOption::SendURL,
            49 => TelnetOption::ForwardX,
            // unassigned
            138 => TelnetOption::TelOptPragmaLogon,
            139 => TelnetOption::TelOptSSPILogon,
            140 => TelnetOption::TelOptPragmaHeartbeat,
            // unassigned
            255 => TelnetOption::ExtendedOptionsList,
            _ => { return Err(io::Error::new(ErrorKind::InvalidData, format!("unknown option: {}/x{:02X}", byte, byte))); }
        };
        Ok(cmd)
    }
}

impl TelnetCom 
{
    pub fn new() -> Self {
        Self { 
            tcp_stream: None,
            state: ParserState::Data,
            buf: std::collections::VecDeque::new()
        }
    }
   

    fn parse(&mut self, data: &[u8]) -> io::Result<()>
    {
        for b in data {
            match self.state {
                ParserState::Data => {
                    if *b == IAC {
                        self.state = ParserState::Iac;
                    } else {
                        self.buf.push_back(*b);
                    }
                },
                ParserState::Iac => {
                    match TelnetCmd::get(*b)? {
                        TelnetCmd::AYT => {
                            self.tcp_stream.as_mut().unwrap().try_write(&TelnetCmd::NOP.to_bytes())?;
                            self.state = ParserState::Data;
                        }
                        TelnetCmd::SE |
                        TelnetCmd::NOP |
                        TelnetCmd::GA => { self.state = ParserState::Data; }
                        TelnetCmd::IAC => {
                            self.buf.push_back(0xFF);
                            self.state = ParserState::Data;
                        }
                        TelnetCmd::WILL => {
                            self.state = ParserState::Will;
                        }
                        TelnetCmd::WONT => {
                            self.state = ParserState::Wont;
                        }
                        TelnetCmd::DO => {
                            self.state = ParserState::Do;
                        }
                        TelnetCmd::DONT => {
                            self.state = ParserState::Dont;
                        }
                        cmd => {
                            eprintln!("unsupported IAC: {:?}", cmd);
                            self.state = ParserState::Data;
                        }
                    }
                }
                ParserState::Will => {
                    let opt = TelnetOption::get(*b)?;
                    if opt != TelnetOption::TransmitBinary {
                        self.tcp_stream.as_mut().unwrap().try_write(&TelnetCmd::DONT.to_bytes_opt(opt))?;
                    } else {
                        eprintln!("unsupported will option {:?}", opt);
                        self.tcp_stream.as_mut().unwrap().try_write(&TelnetCmd::DO.to_bytes_opt(TelnetOption::TransmitBinary))?;
                    }
                    self.state = ParserState::Data;
                },
                ParserState::Wont => {
                    let opt = TelnetOption::get(*b)?;
                    eprintln!("Won't {:?}", opt);
                    self.state = ParserState::Data;
                },
                ParserState::Do => {
                    let opt = TelnetOption::get(*b)?;
                    if opt == TelnetOption::TransmitBinary {
                        self.tcp_stream.as_mut().unwrap().try_write(&TelnetCmd::WILL.to_bytes_opt(TelnetOption::TransmitBinary))?;
                    } else {
                        eprintln!("unsupported do option {:?}", opt);
                        self.tcp_stream.as_mut().unwrap().try_write(&TelnetCmd::WONT.to_bytes_opt(opt))?;
                    }
                    self.state = ParserState::Data;
                },
                ParserState::Dont => {
                    let opt = TelnetOption::get(*b)?;
                    eprintln!("Don't {:?}", opt);
                    self.state = ParserState::Data;
                },
            }
        }
        Ok(())
    }

    fn fill_buffer(&mut self) -> io::Result<()> {
        let mut buf = [0;1024 * 256];
        loop {
            match self.tcp_stream.as_mut().unwrap().try_read(&mut buf) {
                Ok(size) => {
                    return self.parse(&buf[0..size]);
                }
                Err(ref e) => {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        break;
                    }
                    return Err(io::Error::new(ErrorKind::ConnectionAborted, format!("Telnet error: {}", e)));
                }
            };
        }
        Ok(())
    }

    fn fill_buffer_wait(&mut self, _timeout: Duration) -> io::Result<()> {
        self.fill_buffer()?;
        while self.buf.len() == 0 {
            self.fill_buffer()?;
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }
}

#[async_trait]
impl Com for TelnetCom {
    fn get_name(&self) -> &'static str {
        "Telnet"
    }
    async fn connect(&mut self, addr: String) -> Result<bool, String> {

        let r = tokio::time::timeout(Duration::from_secs(5), TcpStream::connect(addr)).await;
        match r {
            Ok(tcp_stream) => {
                match tcp_stream {
                    Ok(stream) => { self.tcp_stream = Some(stream); Ok(true)}
                    Err(err) => {
                        Err(format!("{}", err))
                    }
                }
            }
            Err(err) => {
                Err(format!("{}", err))
            }
        }
    }
    fn read_char(&mut self, timeout: Duration) -> io::Result<u8> {
        if let Some(b) = self.buf.pop_front() {
            return Ok(b);
        }
        self.fill_buffer_wait(timeout)?;
        if let Some(b) = self.buf.pop_front() {
            return Ok(b);
        }
        return Err(io::Error::new(ErrorKind::TimedOut, "timed out"));
    }
    
    fn read_char_nonblocking(&mut self) -> io::Result<u8> {
        if let Some(b) = self.buf.pop_front() {
            return Ok(b);
        }
        return Err(io::Error::new(ErrorKind::TimedOut, "no data avaliable"));
    }

    fn read_exact(&mut self, duration: Duration, bytes: usize) -> io::Result<Vec<u8>> {
        while self.buf.len() < bytes {
            self.fill_buffer_wait(duration)?;
        }
        Ok(self.buf.drain(0..bytes).collect())
    }
    
    fn is_data_available(&mut self) -> io::Result<bool> {
        self.fill_buffer()?; 
        Ok(self.buf.len() > 0)
    }

    fn disconnect(&mut self) -> io::Result<()> {
       // self.tcp_stream.shutdown(std::net::Shutdown::Both)
       Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut data = Vec::with_capacity(buf.len());
        for b in buf {
            if *b == IAC {
                data.extend_from_slice(&[IAC, IAC]);
            } else {
                data.push(*b);
            }
        }
        self.tcp_stream.as_mut().unwrap().try_write(&data)
    }
}