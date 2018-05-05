// Copyright 2018 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate encoding_rs;

use encoding_rs::*;

static CODE_PAGES_SORTED: [u16; 2] = [1252, 65001];

static ENCODINGS_IN_CODE_PAGE_SORT: [&'static Encoding; 2] = [&WINDOWS_1252_INIT, &UTF_8_INIT];

pub fn to_encoding(code_page: u16) -> Option<&'static Encoding> {
    if let Ok(i) = CODE_PAGES_SORTED.binary_search(&code_page) {
        Some(ENCODINGS_IN_CODE_PAGE_SORT[i])
    } else {
        None
    }
}

pub fn to_encoding_no_replacement(code_page: u16) -> Option<&'static Encoding> {
    let opt_encoding = to_encoding(code_page);
    if opt_encoding == Some(REPLACEMENT) {
        None
    } else {
        opt_encoding
    }
}

pub fn from_encoding(encoding: &'static Encoding) -> Option<u16> {
    if encoding == REPLACEMENT {
        None
    } else {
        ENCODINGS_IN_CODE_PAGE_SORT
            .iter()
            .position(|&x| x == encoding)
            .map(|i| CODE_PAGES_SORTED[i])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
