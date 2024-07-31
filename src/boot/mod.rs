use crate::{
    funcs::Reader,
    vm::{Funcs, VM},
};
use std::{env, fs};
use toml::Table;

pub struct Booter<'a> {
    funcs: Funcs,
    vm: &'a mut VM<'a>,
}

impl<'a> Booter<'a> {
    pub fn new(vm: &'a mut VM<'a>) -> Self {
        Self {
            funcs: Funcs::new(),
            vm,
        }
    }

    pub fn boot(&mut self, dir_path: &str) {
        let sboot = self.read_boot_file(dir_path);
        let mut paths: Vec<String> = Vec::new();

        let main_path: String = match sboot {
            Ok(table) => match table["main"].as_str() {
                Some(main_path) => main_path.to_string(),
                None => return,
            },
            Err(()) => return,
        };

        find_func_paths(&mut paths, dir_path);

        // Parse func files
        for path in paths {
            let prefix = format!("{}/", path[dir_path.len()..].to_string());
            let funcs_string = fs::read_to_string(path);

            match funcs_string {
                Ok(funcs_string) => {
                    let mut reader = Reader::new(&mut self.funcs);

                    reader.read(Some(&prefix), &funcs_string);
                }
                _ => panic!("Cannot parse funcs file {}", prefix),
            }
        }

        self.vm.set_funcs(self.funcs.clone());
        self.vm.start(&main_path);
    }

    fn read_boot_file(&mut self, dir_path: &str) -> Result<Table, ()> {
        let boot_file_path = format!("{}/boot.toml", dir_path);

        match fs::read(boot_file_path) {
            Ok(boot_file) => match String::from_utf8(boot_file) {
                Ok(string) => match string.parse::<Table>() {
                    Ok(table) => Ok(table),
                    _ => {
                        println!("boot.toml cannot be parsed");
                        Err(())
                    }
                },
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

fn find_func_paths(file_paths: &mut Vec<String>, dir_path: &str) {
    match fs::read_dir(dir_path) {
        Ok(paths) => {
            for path in paths {
                let path = path.unwrap().path();

                if path.is_dir() {
                    find_func_paths(file_paths, &path.to_str().unwrap())
                } else if path.is_file() {
                    let path = path.to_str().unwrap().to_string();

                    if path.contains(".funcs") {
                        file_paths.push(path)
                    }
                }
            }
        }
        _ => println!("Invalid path"),
    }
}

pub fn get_main_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let main_path = args.get(1);

    match main_path {
        Some(main_path) => Some(main_path.clone()),
        _ => {
            println!("No program path were given");
            None
        }
    }
}
