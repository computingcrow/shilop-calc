mod data;

use std::collections::{HashMap};
use std::f64::consts::{E, PI};
use std::fs;
use std::ops::{Rem};
use std::str::FromStr;
use home::home_dir;
use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::views::{Dialog, EditView, LinearLayout, ListView, TextView};
use cursive::traits::{Nameable, Resizable};
use cursive::theme::*;
use crate::data::Data;

static BASE_WORDS: [&str; 27] = [
    "+", "-", "*", "/", "÷", "%", "mod",
    "^", "**", "pow",
    "!", "fac",
    "pi", "sin", "sine", "cos", "cosine",
    "e",
    "exp",
    "swap", "pop",
    "abs",
    "real", "re",
    "i",
    "imaginary", "im",
];

static DELTA: f64 = 0.000_000_1;

fn main() {
    let macros = home_dir()
        .map(|dir| dir.join(".shilop"))
        .filter(|file| file.exists())
        .map(|file| fs::read_to_string(file))
        .map_or(HashMap::new(), |text_maybe| {
            let mut out = HashMap::<String, Vec<String>>::new();

            if text_maybe.is_ok() {
                text_maybe.unwrap().lines().for_each(|line| {
                    let key_position = line.find(" ").expect("Invalid word format");
                    let line_string = std::string::String::from(line).clone();
                    let te = line_string.split_at(key_position);

                    let mut outs = Vec::<String>::new();
                    te.1.trim().split(" ").for_each(|w| outs.push(w.to_string()));
                    out.insert(te.0.to_string(), outs);
                });
            }

            return out;
        });

    if !check_macros(&macros) {
        println!("Error: MACROS file invalid");
        return;
    };

    let theme = Theme {
        shadow: false,
        borders: BorderStyle::None,
        palette: Palette::terminal_default(),
    };

    let mut interface = cursive::default();
    interface.set_theme(theme);
    interface.set_user_data(macros);

    interface.add_layer(
        Dialog::around(
            LinearLayout::new(Orientation::Vertical)
                .child(EditView::new().on_edit(|x, y, z| handle_input(x, y, z)).fixed_width(16))
                .child(ListView::new().with_name("out").fixed_width(16)))
            .title("Shilop")
            .button("Quit", |s| s.quit()));

    interface.run();
}

fn handle_input(interface: &mut Cursive, text: &str, _length: usize) {
    let macros: &HashMap<String, Vec<String>> = interface.user_data().expect("");

    let mut arr: Vec<String> = text.split(" ").map(|w| w.to_lowercase()).collect();

    loop {
        let mut tmp = Vec::new();
        let mut has_resolved_macro = false;

        for word in &arr {
            if BASE_WORDS.contains(&word.as_str()) {
                tmp.push(String::from(word));
            } else if macros.contains_key(word) {
                macros.get(word).unwrap().iter().for_each(|w| tmp.push(String::from(w)));
                has_resolved_macro = true;
            } else if f64::from_str(&word).is_ok() {
                tmp.push(String::from(word));
            }
        }

        arr.clear();
        tmp.iter().for_each(|w| arr.push(String::from(w)));

        if !has_resolved_macro {
            break;
        }
    }

    let mut out = interface.find_name::<ListView>("out").expect("err");
    out.clear();

    let mut data = Vec::<Data>::new();
    arr.iter().for_each(|entry| {
        match entry.as_str() {
            "+" => {
                if data.len() > 1 {
                    let op2 = data.pop().unwrap();
                    let op1 = data.pop().unwrap();
                    data.push(op1 + op2);
                }
            }
            "-" => {
                if data.len() > 1 {
                    let op2 = data.pop().unwrap();
                    let op1 = data.pop().unwrap();
                    data.push(op1 - op2);
                }
            }
            "*" => {
                if data.len() > 1 {
                    let op2 = data.pop().unwrap();
                    let op1 = data.pop().unwrap();
                    data.push(op1 * op2);
                }
            }
            "/" | "÷" => {
                if data.len() > 1 {
                    let op2 = data.pop().unwrap();
                    if op2.abs_as_f64() > DELTA {
                        let op1 = data.pop().unwrap();
                        data.push(op1 / op2);
                    } else {
                        data.push(op2);
                    }
                }
            }
            "%" | "mod" => {
                if data.len() > 1 {
                    let op2 = data.pop().unwrap();
                    let op1 = data.pop().unwrap();

                    match op1 {
                        Data::Double(r) => {
                            match op2 {
                                Data::Double(m) => {
                                    if m.abs() > DELTA {
                                        data.push(Data::Double(r.rem(m)))
                                    } else {
                                        data.push(op2);
                                        data.push(op1);
                                    }
                                },
                                _ => {
                                    data.push(op2);
                                    data.push(op1);
                                }
                            }
                        },
                        _ => {
                            data.push(op2);
                            data.push(op1);
                        }
                    }
                }
            }
            "^" | "**" | "pow" => {
                if data.len() > 1 {
                    let op2 = data.pop().unwrap();
                    let op1 = data.pop().unwrap();
                    let powed = op1.pow(op2);
                    data.push(powed);
                }
            }
            "pi" => {
                data.push(Data::Double(PI));
            }
            "e" => {
                data.push(Data::Double(E));
            }
            "exp" => {
                let exponentiated = data.pop().unwrap().exp();
                data.push(exponentiated);
            }
            "real" | "re" => {
                data.pop().map(|d| d.real()).and_then(|r| { data.push(Data::Double(r)); None::<Data> });
            }
            "i" => {
                data.pop().map(|v| {
                    match v {
                        Data::Double(n) => data.push(Data::Complex(0f64, n)),
                        Data::Complex(_, _) => data.push(v)
                    }
                });
            }
            "imaginary" | "im" => {
                data.pop().map(|v| {
                    match v {
                        Data::Double(_) => data.push(v),
                        Data::Complex(_, i) => data.push(Data::Double(i))
                    }
                });
            }
            "sin" | "sine" => {
                data.pop().map(|v| {
                    data.push(v.sin().unwrap_or(v));
                });
            }
            "cos" | "cosine" => {
                data.pop().map(|v| {
                    data.push(v.cos().unwrap_or(v));
                });
            }
            "swap" => {
                if data.len() > 1 {
                    let head = data.pop().unwrap();
                    let second = data.pop().unwrap();
                    data.push(head);
                    data.push(second);
                }
            }
            "pop" => {
                let _ = data.pop();
            }
            "!" | "fac" => {
                if data.len() > 0 {
                    let n = data.pop().unwrap();
                    match n {
                        Data::Complex(_, _) => data.push(n),
                        Data::Double(d) => {
                            if d < 0f64 || (d - d.floor()) > 0f64 || d > 25.0f64 {
                                data.push(n);
                                return;
                            }

                            let mut counter = d as i32;
                            let mut accumulator = 1f64;
                            while counter > 0 {
                                accumulator = accumulator * (counter as f64);
                                counter = counter - 1;
                            }

                            data.push(Data::Double(accumulator));
                        }
                    }

                }
            }
            "abs" => {
                data.pop().map(|v| {
                    data.push(v.abs());
                });
            }
            _ => {
                let maybe_number = f64::from_str(entry);
                if maybe_number.is_ok() {
                    data.push(Data::Double(maybe_number.unwrap()));
                }
            }
        }
    });

    data.iter().for_each(|i| out.add_child("", TextView::new(i.to_string())));
    return;
}

fn check_macros(macros: &HashMap<String, Vec<String>>) -> bool {
    let mut dependencies = Vec::<String>::new();
    let mut dependencies_2 = Vec::<String>::new();
    let mut recursions = 1024;

    // Resolve all words this word depends on.
    // If we end up with an empty stack at some point,
    // everything depends on a BASE_WORD or is a number, i.e. is valid.
    // We do a lot of copying here, but I'd wager that it's okay to do so once.
    for x in macros {
        while !dependencies.is_empty() && recursions > 0 {
            recursions = 1024;

            x.1.iter().for_each(|s| { dependencies.push(String::from(s)); });

            for dependency in &dependencies {
                if !BASE_WORDS.contains(&dependency.to_lowercase().as_str()) &&
                    !f64::from_str(dependency.as_str()).is_ok() {
                    dependencies_2.push(String::from(dependency));
                }
            }

            dependencies_2.iter().for_each(|s| dependencies.push(String::from(s)));
            dependencies_2.clear();
            recursions -= 1;
        }
    }

    return dependencies.is_empty();
}
