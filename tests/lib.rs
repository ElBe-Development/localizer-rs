// localizer-rs tests
// Version: 1.2.0

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
    fn test_config() {
        let config: localizer_rs::Config = localizer_rs::Config::new("examples/translations", "en");

        assert_eq!(
            config,
            localizer_rs::Config {
                path: "examples/translations".to_owned(),
                language: "en".to_owned()
            }
        );
    }

    #[test]
    fn test_set_path() {
        let mut config: localizer_rs::Config =
            localizer_rs::Config::new("examples/translations", "en");
        config.set_path("examples");

        assert_eq!(
            config,
            localizer_rs::Config {
                path: "examples".to_owned(),
                language: "en".to_owned()
            }
        );
    }

    #[test]
    fn test_set_language() {
        let mut config: localizer_rs::Config =
            localizer_rs::Config::new("examples/translations", "en");
        config.set_language("not_en");

        assert_eq!(
            config,
            localizer_rs::Config {
                path: "examples/translations".to_owned(),
                language: "not_en".to_owned()
            }
        );
    }

    #[test]
    fn test_translate() {
        let config: localizer_rs::Config = localizer_rs::Config::new("examples/translations", "en");
        let translation: String =
            config.translate("error", vec![("details", "Something went wrong")]);

        assert_eq!(
            translation.as_str(),
            "\x1b[31m\x1b[1mError:\x1b[0m Something went wrong"
        );
    }

    #[test]
    fn test_translate_t() {
        let config: localizer_rs::Config = localizer_rs::Config::new("examples/translations", "en");
        let translation: String = config.t("error", vec![("details", "Something went wrong")]);

        assert_eq!(
            translation.as_str(),
            "\x1b[31m\x1b[1mError:\x1b[0m Something went wrong"
        );
    }

    #[test]
    fn test_translate_macro() {
        let config: localizer_rs::Config = localizer_rs::Config::new("examples/translations", "en");
        let translation: String =
            localizer_rs::t!(config, "error", "details" = "Something went wrong");

        assert_eq!(
            translation.as_str(),
            "\x1b[31m\x1b[1mError:\x1b[0m Something went wrong"
        );
    }
}
