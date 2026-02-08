use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::error::Error;
use std::fmt;

// Create my own type of error
#[derive(PartialEq, Debug)]
enum ParsingError {
    Timestamp,
    Subsystem,
    SensorId,
    Value
}
// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            ParsingError::Timestamp => "cannot parse timestamp",
            ParsingError::Subsystem => "cannot parse subsystem",
            ParsingError::SensorId => "cannot parse sensor_id",
            ParsingError::Value => "cannot parse value",
        };
        f.write_str(description)
    }
}

impl Error for ParsingError {}

#[derive(Debug)] // So we can print the struct
#[repr(u8)] // Represent the enum as a single byte
#[derive(FromPrimitive)] // We can use type conversion trait from_u8() on byte values :)
enum ReadingType {
    Camera = 0,
    ZSensor = 1,
    ZHead = 2,
    PinLift = 3,
    XYTStage = 4
}
#[derive(Debug)]
struct Packet {
    timestamp: u64,
    subsystem: ReadingType,
    sensor_id: u32,
    value: f64
}
// Assuming input array is big endian
fn parse_packet(input: &[u8; 21]) -> Result<Packet, ParsingError> {
    // How do I know the byte size of the ReadingType enum?
    // A: Because I declared it as a single byte using repr(u8)

    let (ts, rest) = input.split_at(8);
    let (ss, rest) = rest.split_at(1);
    let (sid, rest) = rest.split_at(4);
    let (data, _) = rest.split_at(8);

    // Timestamp
    let timestamp = match bytes_to_u64(ts) {
        Ok(val) => val,
        Err(_) => return Err(ParsingError::Timestamp),
    };
    // Subsystem
    let subsystem = match ReadingType::from_u8(ss[0]) {
        Some(val) => val,
        None => return Err(ParsingError::Subsystem),
    };
    // Sensor ID
    let sensor_id = match bytes_to_u32(sid) {
        Ok(val) => val,
        Err(_) => return Err(ParsingError::SensorId),
    };
    // Value
    let value = match bytes_to_f64(data) {
        Ok(val) => val,
        Err(_) => return Err(ParsingError::Value),
    };
    // Return packet
    Ok(Packet {
        timestamp,
        subsystem,
        sensor_id,
        value
    })
}

fn bytes_to_u64(input: &[u8]) -> Result<u64, std::array::TryFromSliceError> {
    Ok(u64::from_be_bytes(input.try_into()?))
}

fn bytes_to_f64(input: &[u8]) -> Result<f64, std::array::TryFromSliceError> {
    Ok(f64::from_be_bytes(input.try_into()?))
}

fn bytes_to_u32(input: &[u8]) -> Result<u32, std::array::TryFromSliceError> {
    Ok(u32::from_be_bytes(input.try_into()?))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = [0x0A, 0xF, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,  // timestamp
                 0x05,                                           // reading type
                 0x00, 0x01, 0x02, 0x0F,                         // sensor id
                 0x0A, 0xF, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]; // value
    println!("{:?}", parse_packet(&bytes)?);
    Ok(())
}
