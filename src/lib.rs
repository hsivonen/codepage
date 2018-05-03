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

pub fn encoding_for_code_page(_code_page: u16) -> Option<&'static Encoding> {
	return Some(UTF_8);
}

pub fn code_page_for_encoding(_encoding: &'static Encoding) -> Option<u16> {
	return Some(65001);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
