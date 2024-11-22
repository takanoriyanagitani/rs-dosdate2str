pub fn dosdate_to_dosyear(dosdate: u16) -> u8 {
    (dosdate >> 9) as u8
}

pub fn dosdate_to_year(dosdate: u16) -> u16 {
    let dosyear: u8 = dosdate_to_dosyear(dosdate);
    let year: u16 = dosyear.into();
    year + 1980
}

#[cfg(target_family = "wasm")]
#[cfg(feature = "dosdate2year")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn dosdate2year(dosdate: u16) -> u16 {
    dosdate_to_year(dosdate)
}

pub fn dosdate_to_month(dosdate: u16) -> u8 {
    let month: u16 = dosdate >> 5;
    let m: u16 = month & 0x0f;
    m as u8
}

#[cfg(target_family = "wasm")]
#[cfg(feature = "dosdate2month")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn dosdate2month(dosdate: u16) -> u8 {
    dosdate_to_month(dosdate)
}

pub fn dosdate_to_day(dosdate: u16) -> u8 {
    let day: u16 = dosdate & 0x1f;
    day as u8
}

#[cfg(target_family = "wasm")]
#[cfg(feature = "dosdate2day")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn dosdate2day(dosdate: u16) -> u8 {
    dosdate_to_day(dosdate)
}

pub fn dosdate_to_str(dosdate: u16) -> u64 {
    let m: u8 = dosdate_to_month(dosdate);
    let d: u8 = dosdate_to_day(dosdate);
    let y: u8 = dosdate_to_dosyear(dosdate);

    let day: u16 = rs_day2str::day_to_string(d);
    let mon: u16 = rs_month2str::month_to_string(m);
    let yer: u32 = rs_dosyear2str::dosyear_to_string128(y);

    let bd: [u8; 2] = day.to_be_bytes();
    let bm: [u8; 2] = mon.to_be_bytes();
    let by: [u8; 4] = yer.to_be_bytes();

    let bymd: [u8; 8] = [by[0], by[1], by[2], by[3], bm[0], bm[1], bd[0], bd[1]];
    u64::from_be_bytes(bymd)
}

#[cfg(target_family = "wasm")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn dosdate2str(dosdate: u16) -> u64 {
    dosdate_to_str(dosdate)
}

pub fn dosymd2dosdate(dosyear: u8, month: u8, day: u8) -> u16 {
    let y: u16 = dosyear.into();
    let m: u16 = month.into();
    let d: u16 = day.into();
    (y << 9) | (m << 5) | d
}

pub fn ymd2dosdate(year: u16, month: u8, day: u8) -> u16 {
    let bound: u16 = year.clamp(1980, 2107);
    let sub: u16 = bound - 1980;
    let s: u8 = sub as u8;
    dosymd2dosdate(s, month, day)
}

pub fn dostime_to_str(dostime: u32) -> u64 {
    let dosdate: u32 = dostime >> 16;
    dosdate_to_str(dosdate as u16)
}

#[cfg(target_family = "wasm")]
#[cfg(feature = "dostime2str")]
#[allow(unsafe_code)]
#[no_mangle]
pub fn dostime2str(dostime: u32) -> u64 {
    dostime_to_str(dostime)
}

#[cfg(test)]
mod test_dosdate2str {

    mod dosdate_to_str {
        macro_rules! create_test_dosdate2str {
            ($fname: ident, $year: expr, $month: expr, $day: expr, $expected: expr) => {
                #[test]
                fn $fname() {
                    let year: u16 = $year;
                    let month: u8 = $month;
                    let day: u8 = $day;

                    let input: u16 = crate::ymd2dosdate(year, month, day);
                    let expected: &[u8] = $expected;

                    let got: u64 = crate::dosdate_to_str(input);
                    let b: [u8; 8] = got.to_be_bytes();

                    assert_eq!(&b, expected);
                }
            };
        }

        create_test_dosdate2str!(test19800101, 1980, 1, 1, b"19800101");
        create_test_dosdate2str!(test19800102, 1980, 1, 2, b"19800102");
        create_test_dosdate2str!(test19800103, 1980, 1, 3, b"19800103");

        create_test_dosdate2str!(test19800228, 1980, 2, 28, b"19800228");
        create_test_dosdate2str!(test19800331, 1980, 3, 31, b"19800331");
        create_test_dosdate2str!(test19800430, 1980, 4, 30, b"19800430");
        create_test_dosdate2str!(test19800531, 1980, 5, 31, b"19800531");
        create_test_dosdate2str!(test19800630, 1980, 6, 30, b"19800630");
        create_test_dosdate2str!(test19800731, 1980, 7, 31, b"19800731");
        create_test_dosdate2str!(test19800831, 1980, 8, 31, b"19800831");
        create_test_dosdate2str!(test19800930, 1980, 9, 30, b"19800930");

        create_test_dosdate2str!(test19811031, 1981, 10, 31, b"19811031");
        create_test_dosdate2str!(test19811130, 1981, 11, 30, b"19811130");
        create_test_dosdate2str!(test19811231, 1981, 12, 31, b"19811231");

        create_test_dosdate2str!(test19941230, 1994, 12, 30, b"19941230");
        create_test_dosdate2str!(test20041230, 2004, 12, 30, b"20041230");
        create_test_dosdate2str!(test20241230, 2024, 12, 30, b"20241230");
        create_test_dosdate2str!(test20841230, 2084, 12, 30, b"20841230");
        create_test_dosdate2str!(test21071231, 2107, 12, 31, b"21071231");
    }
}
