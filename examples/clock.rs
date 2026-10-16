use bcd_time_stamp::BcdTimeStamp;

fn main() {
    let mut clock = BcdTimeStamp::from_bcd(0x2359).unwrap();
    clock.increment();
    println!("{:04x}", clock.timestamp);
    assert_eq!(clock.timestamp, 0);
}
