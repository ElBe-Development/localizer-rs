// localizer-rs error tests
// Version: 1.0.0

// Copyright (c) 2023-present ElBe Development.

// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the 'Software'),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

////////////////////////////////
// IMPORTS AND USE STATEMENTS //
////////////////////////////////

#[allow(unused_imports)]
use localizer_rs;


///////////
// TESTS //
///////////

#[cfg(test)]
mod tests {
    #[test]
    fn test_error() {
        let error: localizer_rs::errors::Error =
            localizer_rs::errors::Error::new("name", "description", 1);

        assert_eq!(
            error,
            localizer_rs::errors::Error {
                name: "name".to_owned(),
                description: "description".to_owned(),
                exit_code: 1
            }
        );
    }

    #[test]
    #[ignore]
    fn raise_helper() {
        let error: localizer_rs::errors::Error =
            localizer_rs::errors::Error::new("name", "description", 1);
        error.raise("details");
    }

    #[test]
    fn test_raise() {
        let status = std::process::Command::new("cargo")
            .args(&["test", "--", "--ignored"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .expect("Unable to run program");

        assert_eq!(Some(1), status.code())
    }
}
