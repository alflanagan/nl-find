/*
Copyright [2019] [Adrian Lloyd Flanagan]

DUAL LICENSE

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

OR

Licensed under the MIT License, see file LICENSE-MIT.txt
*/
extern crate clap;
extern crate libc; // to get user's locale, for one

use clap::App;

fn main() {
    // cargo should install all the localized arguments files
    let arg_str = include_str!("args/en_US.yml");

    // load_yaml!() requires string literal -- what's that about?
    let yaml_vec = clap::YamlLoader::load_from_str(arg_str).expect("Error reading YAML file");

    let yaml = &yaml_vec[0];
    if yaml.is_null() {
        panic!("Dammit");
    }
    let app = App::from_yaml(&yaml);
    let _matches = app.get_matches();
}
