use bcd_time_stamp::BcdTimeStamp;
use bcd_time_stamp::alias::TimeStamp;

#[test]
fn test_increment() {
    let mut time = BcdTimeStamp::new();
    time.increment();
    assert_eq!(time.timestamp, 1);
}

#[test]
fn test_increment_mul() {
    let mut time = BcdTimeStamp::new();

    // 23h 59m
    for _ in 0..1439 {
        time.increment();
    }

    assert_eq!(time.timestamp, 0x2359);
}

#[test]
fn test_increment_over_24h() {
    let mut time = BcdTimeStamp::new();

    // 24h 01m
    for _ in 0..1441 {
        time.increment();
    }

    assert_eq!(time.timestamp, 0x0001);
}

#[test]
fn test_from_timestamp() {
    let stamp = TimeStamp(1792132618); // UTC 06:36
    let bcd = BcdTimeStamp::from(stamp);

    assert_eq!(bcd.timestamp, 0x0636);
}
#[test]
fn timestamp_arithmetic() {
    use bcd_time_stamp::alias::MilliTimeStamp;

    macro_rules! check {
        ($stamp:ident) => {
            let mut stamp = $stamp::from(100);
            assert_eq!(stamp + 20, $stamp(120));
            assert_eq!(stamp - 20, $stamp(80));
            assert_eq!(stamp + $stamp(20), $stamp(120));
            assert_eq!(stamp - $stamp(20), $stamp(80));
            stamp += 20;
            stamp -= 10;
            stamp += $stamp(5);
            stamp -= $stamp(15);
            assert_eq!(u64::from(stamp), 100);
            assert_eq!($stamp::default(), $stamp::new());
            assert!(stamp > $stamp(99));
        };
    }

    check!(TimeStamp);
    check!(MilliTimeStamp);
}

#[test]
fn bcd_arithmetic() {
    for minute in 0u16..1440 {
        let time = BcdTimeStamp::new() + minute;
        let mut incremented = time;
        incremented.increment();
        assert_eq!(time + 1, incremented);
        assert_eq!((time + 1) - 1, time);
        assert_eq!(time + 1440, time);
        assert_eq!(time - 1440, time);
        assert_eq!((time + u16::MAX) - u16::MAX, time);
    }

    let mut time = BcdTimeStamp::from(0x2359);
    time += 2;
    assert_eq!(time.timestamp, 0x0001);
    time -= 2;
    assert_eq!(time.timestamp, 0x2359);
}

#[test]
fn validates_every_raw_encoding() {
    for raw in 0..=u16::MAX {
        let digits = [raw >> 12, (raw >> 8) & 15, (raw >> 4) & 15, raw & 15];
        let expected = digits.iter().all(|&digit| digit < 10)
            && digits[0] * 10 + digits[1] < 24
            && digits[2] * 10 + digits[3] < 60;
        let unchecked = BcdTimeStamp::from(raw);
        assert_eq!(unchecked.is_valid(), expected, "{raw:04x}");
        assert_eq!(BcdTimeStamp::from_bcd(raw), expected.then_some(unchecked));
    }
}

#[test]
fn conversions_cover_every_minute_and_extreme_seconds() {
    for minute in 0u64..1440 {
        let hour = minute / 60;
        let min = minute % 60;
        let expected = ((hour / 10) << 12 | (hour % 10) << 8 | (min / 10) << 4 | (min % 10)) as u16;
        for seconds in [minute * 60, minute * 60 + 59, minute * 60 + 86400] {
            let time = BcdTimeStamp::from(TimeStamp(seconds));
            assert_eq!(time.timestamp, expected);
            assert_eq!(time.hour(), expected >> 8);
            assert_eq!(time.minute(), expected & 255);
        }
    }
    assert_eq!(BcdTimeStamp::from(TimeStamp(u64::MAX)).timestamp, 0x0700);
}

#[test]
fn raw_access_and_defaults_remain_compatible() {
    let mut time = BcdTimeStamp::default();
    assert_eq!(time, BcdTimeStamp::new());
    *time = 0x1234;
    assert_eq!(*time, 0x1234);
    assert!(time.is_valid());
    assert_eq!(bcd_time_stamp::MINUTE_MASK, bcd_time_stamp::MINUET_MASK);
}
