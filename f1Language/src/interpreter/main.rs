use core::time;
use std::fs::File;
use std::io::*;
use std::thread::sleep;

fn main() {
    let file_name = "file.f1l";
    let mut input_file = File::open(file_name).expect("Couldn't find file!");
    let mut all_lines: String = String::new();
    let mut drivers: [i32; 4] = [0, 0, 0, 0]; // registers [0] is used for I/O the rest is free for whatever
                                              //altough its funny if [0] is return value since its alonso and he returns
                                              // [1] is args because its verstappen and he sent it
                                              // [2] same as above but for ricciardo
                                              // [3] is temporary since its mazepin
    let mut stack: Vec<i32> = vec![];
    input_file
        .read_to_string(&mut all_lines)
        .expect("Something is very wrong with the file. Could not read.");
    let lines: Vec<String> = all_lines
        .lines()
        .map(|x| x.to_string().to_lowercase())
        .collect();
    let mut i: usize = 0;
    while i < lines.len() {
        let words: Vec<String> = lines[i].split_whitespace().map(|x| x.to_string()).collect();
        let mut length = words.len();

        for (s, word) in words.iter().enumerate() {
            if word.starts_with("#") {
                length = s;
                break;
            }
        }

        if lines[i].contains("#") && (length == words.len()) {
            panic!("There is a comment on line {} however it isnt the after a whitespace and that fuck things up.\nPls have a space before #. As an example do:\nbox_box -5 #comment\n\nInstead of:\nbox_box -5#comment.", i)
        }

        if length == 0 {
            i = i + 1;
            continue;
        }

        if length == 1 {
            panic!("Error at line {}: {}. \nWay to few tokens!", i, lines[i]);
        }
        let match_index = {
            if length == 2 {
                0
            } else {
                1
            }
        };

        match &words[match_index] as &str {
            "target_lap" => {
                // load immidiate (li). 2 bit for register, 3 bit for op., 3 bit for imm.
                if length != 3 {
                    panic!("Wrong format for tokens at line {}: {}.\ntarget_lap should format:\ndriver target_lap <imm>\n and makes driver = <imm>.", i, lines[i]);
                }
                let imm = words[2].parse::<i32>().expect(&format!(
                    "Error at line {}: {}.\nThird token is not an immidiate.",
                    i, lines[i]
                ));
                if imm > 7 {
                    panic!(
                        "Error at line {}: {}.\nImmidiate value is not within range. Only between 0 <-> 7 is allowed.",
                        i, lines[i]
                    );
                }
                drivers[get_driver(&words[0], i)] = imm;
            }

            "target_plus" => {
                //addi. 2 bit for register, 3 bit for op., 3 bit for imm
                if length != 3 {
                    panic!("Wrong format for tokens at line {}: {}.\ntarget_plus should format:\ndriver target_plus <imm>\n and makes driver += <imm>. Only between -4 <-> 3 is allowed. ", i, lines[i]);
                }
                let imm = words[2].parse::<i32>().expect(&format!(
                    "Error at line {}: {}.\nThird token is not an immidiate.",
                    i, lines[i]
                ));
                if (imm > 3) || (imm < -4) {
                    panic!(
                        "Error at line {}: {}.\nImmidiate value is not within range. Only between -4 <-> 3 is allowed.",
                        i, lines[i]
                    );
                }
                drivers[get_driver(&words[0], i)] = drivers[get_driver(&words[0], i)] + imm;
            }

            "plan" => {
                //add driver1 plan driver2 => driver1 += driver2. 2 bit for driver1 3 for op 2 for driver2
                if length != 3 {
                    panic!("Wrong format for tokens at line {}: {}.\nplan should format:\ndriver1 target_plus drver2\n and makes driver1 += driver2.", i, lines[i]);
                }
                drivers[get_driver(&words[0], i)] =
                    drivers[get_driver(&words[0], i)] + drivers[get_driver(&words[2], i)]
            }

            "box_opposite" => {
                //box_opposite should just be branch on different. Eg jump if !=. 2 bit for register, 3 bit for op, 2 bit for comparison register, and 1 useless bit.
                //jump instructions will be hard. Plan careful with only 2 bits, where one will decide direction. //maybe just make it skip one line <- YES
                // it skips a line if they are not equal
                if length != 3 {
                    panic!("Wrong format for tokens at line {}: {}.\nbox_opposite should format:\ndriver1 box_opposite driver2\n and makes program skip a line if diver1!=driver2.", i, lines[i]);
                }
                if drivers[get_driver(&words[0], i)] != drivers[get_driver(&words[2], i)] {
                    i = i + 1;
                }
            }

            "box_box" => {
                //jump unconditionally. 3 bit for op and 5 bit for immidiate.

                if length != 2 {
                    panic!("Wrong format for tokens at line {}: {}.\nbox_box should format:\nbox_box <imm>\n and makes program jump <imm> amount of lines. ", i, lines[i]);
                }
                let imm = words[1].parse::<isize>().expect(&format!(
                    "Error at line {}: {}.\nJump destination is not an immidiate.",
                    i, lines[i]
                ));
                if (imm > 15) || (imm < -16) {
                    panic!(
                        "Error at line {}: {}.\nImmidiate value is not within range. Only between -16 <-> 15 is allowed.",
                        i, lines[i]
                    );
                }
                if (i as isize) + imm < 0 {
                    println!("WARNING: AT LINE {}: {}. \nJUMP COMMAND GOES TO NEGATIVE LINE. PROBABLY NOT SUPPOSED TO. ADJUSTING TO 0.\nNotice program will sleep for 2 seconds.", i, lines[i]);
                    sleep(time::Duration::from_secs(2));
                    i = 0; // because were adding 1 at the end // dont underflow , well more like cause an exception.
                    continue;
                } else {
                    if (i as isize) + imm > lines.len() as isize {
                        println!("WARNING: AT LINE {}: {}. \nJUMP COMMAND GOES TO LINE AFTER EOF. PROBABLY NOT SUPPOSED TO. PROGRAM WILL EXIT.", i, lines[i]);
                    }
                    i = ((i as isize) + imm) as usize;
                    continue;
                }
            }

            "quali_mode" => {
                //push value to stack //format quali_mode driver pushes driver value to stack. //so far i dont care about a stack pointer.
                if length != 2 {
                    panic!("Wrong format for tokens at line {}: {}.\nquali_mode should format:\nquali_mode driver\n and pushes driver to stack. ", i, lines[i]);
                }
                stack.push(drivers[get_driver(&words[1], i)]);
            }

            "cooldown_lap" => {
                // retrieve last value from stack into driver, format cooldown_lap driver.
                if length != 2 {
                    panic!("Wrong format for tokens at line {}: {}.\ncooldown_lap should format:\ncooldown_lap driver\n and retrieves current element from stack to driver. ", i, lines[i]);
                }
                drivers[get_driver(&words[1], i)] = stack
                    .pop()
                    .expect("Error at line {i}: {lines[i]}.\nStack is empty.");
            }

            "fia" => {
                //handles I/O and exit call
                if length != 2 {
                    panic!("Wrong format for tokens at line {}: {}.\nfia should format:\nfia <imm>\n <imm> = 0 reads input, <imm> = 1 outputs alonso, <imm> = -1 exits. ", i, lines[i]);
                }

                let imm = words[1]
                    .parse::<isize>()
                    .expect("Error at line {i}: {lines[i]}.\nIt's not an immidiate.");

                if imm == 0 {
                    drivers[0] = get_input();
                } else if imm == 1 {
                    println!("Output: {}", drivers[0]);
                } else if imm == -1 {
                    i = usize::MAX;
                    continue;
                }
            }
            _ => panic!("Error at line {}: {}.\n Unknown token", i, lines[i]),
        }
        i = i + 1;
        /* println!(
            "{}, {}, {}, {}",
            drivers[0], drivers[1], drivers[2], drivers[3]
        ); */
    }
}

fn get_driver(driver: &str, line_index: usize) -> usize {
    match driver {
        "alonso" => 0,
        "verstappen" => 1,
        "ricciardo" => 2,
        "mazepin" => 3,
        _ => panic!(
            "Error at line {}: Unkown driver (register) {}.",
            line_index, driver
        ),
    }
}

fn get_input() -> i32 {
    let input = stdin();
    let mut line_input = String::new();
    input
        .read_line(&mut line_input)
        .expect("Couldn't read any input");
    line_input
        .trim()
        .parse::<i32>()
        .expect("Input was not a number")
}
