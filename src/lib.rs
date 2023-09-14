// localizer-rs
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

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use serde_json;


///////////////////
// CONFIG OBJECT //
///////////////////

/// Localization config object.
///
/// Use [`Config::new()`] to create config objects instead of using this struct.
///
/// # Parameters
///
/// - `path`: The directory containing the translation files. The directory is relative to the path the executable was executed from.
/// - `language`: The language to translate to.
///
/// # Returns
///
/// A new `Config` object with the specified path and language.
///
/// # Examples
///
/// ```rust
/// # use localizer_rs;
/// localizer_rs::Config {
///     path: "path".to_owned(),
///     language: "language".to_owned()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config {
	pub path: String,
	pub language: String
}


//////////////////////
// CONFIG FUNCTIONS //
//////////////////////

impl Config {
    /// Creates a new config object.
    ///
    /// # Parameters
    ///
    /// - `path`: The directory containing the translation files. The directory is relative to the path the executable was executed from.
    /// - `language`: The language to translate to.
    ///
    /// # Returns
    ///
    /// A new `Config` object with the specified path and language.
    ///
    /// # Panics
    ///
    /// Panics if the Path provided is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use localizer_rs;
    /// localizer_rs::Config::new("examples/translations", "language");
    /// ```
    ///
    /// # See also
    ///
    /// - [`Config`]
	pub fn new(path: &str, language: &str) -> Config {
		let mut config: Config = Config {
			path: "".to_string(),
			language: "".to_string()
		}.to_owned();
		config = config.set_language(language).to_owned();
		config = config.set_path(path).to_owned();

		return config;
	}

    /// Sets the path for the config object.
    ///
    /// # Parameters
    ///
    /// - `self`: The config object.
    /// - `str_path`: The directory containing the translation files. The directory is relative to the path the executable was executed from.
    ///
    /// # Returns
    ///
    /// The modified `Config` object with the specified path.
    ///
    /// # Panics
    ///
    /// Panics if the Path provided is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use localizer_rs;
    /// # let config: localizer_rs::Config = localizer_rs::Config::new("examples/translations", "language");
    /// config.set_path("examples");
    /// ```
    ///
    /// # See also
    ///
    /// - [`Config`]
	pub fn set_path(&mut self, str_path: &str) -> &Config {
		let path: &Path = Path::new(str_path);

		match path.try_exists() {
			Ok(value) => {
				if !value {
					eprintln!("Translation path {:?} does not exist", str_path);
					exit(1);
				}
			},
			Err(error) => {
				eprintln!("Can't open translation path {:?}: {}", str_path, error);
				exit(2);
			}
		}

		self.path = String::from(path.to_owned().to_str().expect("Expected valid path"));
		return self;
	}

    /// Sets the language for the config object.
    ///
    /// # Parameters
    ///
    /// - `self`: The config object.
    /// - `language`: The language to translate to.
    ///
    /// # Returns
    ///
    /// The modified `Config` object with the specified language.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use localizer_rs;
    /// # let config: localizer_rs::Config = localizer_rs::Config::new("examples/translations", "language");
    /// config.set_language("en");
    /// ```
    ///
    /// # See also
    ///
    /// - [`Config`]
	pub fn set_language(&mut self, language: &str) -> &Config {
		self.language = language.to_string();
		return self;
	}

    /// Translates the specified key in the language specified in the config.
    ///
    /// # Parameters
    ///
    /// - `self`: The config object.
    /// - `key`: The key to translate to.
    /// - `arguments`: The arguments to replace.
    ///
    /// # Returns
    ///
    /// A `String` containing the translated value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use localizer_rs;
    /// # let config: localizer_rs::Config = localizer_rs::Config::new("examples/translations", "en");
    /// config.t("test", vec![]);
    /// ```
    ///
    /// # See also
    ///
    /// - [`Config`]
	pub fn t(&self, key: &str, arguments: Vec<(&str, &str)>) -> String {
		return self.translate::<serde_json::Value>(key, arguments);
	}

	fn translate<T>(&self, key: &str, mut arguments: Vec<(&str, &str)>) -> String
	where
		T: serde::Serialize + for<'de> serde::Deserialize<'de>
	{
		let mut colors: Vec<(&str, &str)> = vec![
			// Formatting codes
			("end", "\x1b[0m"), ("bold", "\x1b[1m"), ("italic", "\x1b[3m"), ("underline", "\x1b[4m"), ("overline", "\x1b[53m"),

			// Foreground colors
			("color.black", "\x1b[30m"), ("color.red", "\x1b[31m"), ("color.green", "\x1b[32m"), ("color.yellow", "\x1b[33m"),
			("color.blue", "\x1b[34m"), ("color.magenta", "\x1b[35m"), ("color.cyan", "\x1b[36m"), ("color.white", "\x1b[37m"),

			// Bright foreground colors
			("color.bright_black", "\x1b[90m"), ("color.bright_red", "\x1b[91m"), ("color.bright_green", "\x1b[92m"),
			("color.bright_yellow", "\x1b[93m"), ("color.bright_blue", "\x1b[94m"), ("color.bright_magenta", "\x1b[95m"),
			("color.bright_cyan", "\x1b[96m"), ("color.bright_white", "\x1b[97m"),

			// Background colors
			("back.black", "\x1b[40m"), ("back.red", "\x1b[41m"), ("back.green", "\x1b[42m"), ("back.yellow", "\x1b[43m"),
			("back.blue", "\x1b[44m"), ("back.magenta", "\x1b[45m"), ("back.cyan", "\x1b[46m"), ("back.white", "\x1b[47m"),

			// Bright background colors
			("back.bright_black", "\x1b[100m"), ("back.bright_red", "\x1b[101m"), ("back.bright_green", "\x1b[102m"),
			("back.bright_yellow", "\x1b[103m"), ("back.bright_blue", "\x1b[104m"), ("back.bright_magenta", "\x1b[105m"),
			("back.bright_cyan", "\x1b[106m"), ("back.bright_white", "\x1b[107m"),
		];
		arguments.append(&mut colors);

		let file: File = File::open(Path::new(format!("./{}/{}.json", &self.path, &self.language).as_str())).unwrap();
		let reader: BufReader<File> = BufReader::new(file);

		let json: serde_json::Value = serde_json::to_value::<T>(serde_json::from_reader::<BufReader<File>, T>(reader).unwrap()).unwrap().to_owned();
		let mut result: String = match json[key].as_str() {
			Some(value) => value.to_string(),
			None => "".to_string()
		};

		for (key, value) in arguments {
			result = result.replace(("{{".to_owned() + key + "}}").as_str(), value);
		}

		return result;
	}
}
