use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::{env, io};

#[derive(Serialize)]
struct Digraph {
    chars: [char; 2],
    frequency: f32,
}

fn main() {
    build_digraphs();
}

fn build_digraphs() {
    let input_path = format!(
        "{}/assets/raw_bogram_Table.html",
        env!("CARGO_MANIFEST_DIR")
    );
    let output_path = format!("{}/src/digraphs.ron", env!("CARGO_MANIFEST_DIR"));
    let input_file = File::open(&input_path).expect("Failed opening input file");
    let mut output_file = File::create(&output_path).expect("Failed creating output file");

    // Parse the input file
    let lines = io::BufReader::new(input_file).lines();

    let mut digraphs: Vec<Digraph> = Vec::new();

    for raw_line in lines {
        if let Ok(line) = raw_line {
            //println!("line: {}", line);
            let start = line.find("title=");
            if let Some(start) = start {
                if let Some(end) = line.find(":") {
                    let start = start + 7;
                    let end = end;
                    let title_slice = &line[start..end];

                    if let Some(percent) = line.find('%') {
                        let start = end + 2;
                        let end = percent;
                        let f = &line[start..end].parse::<f32>();
                        if let Ok(mut frequency) = f {
                            frequency /= 100.0;
                            let chars_raw: Vec<char> = title_slice
                                .chars()
                                .map(|x| x.to_ascii_lowercase())
                                .collect();
                            let chars: [char; 2] = chars_raw.as_slice().try_into().unwrap();
                            if frequency > 0.00001 {
                                digraphs.push(Digraph { chars, frequency });
                            }
                        }
                    }
                }
            }
        }
    }

    // Write results to file
    let pretty = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true);
    let s = to_string_pretty(&digraphs, pretty.clone()).expect("Serialization failed");
    output_file
        .write(s.as_bytes())
        .expect("Failed to write to file");

    // for d in digraphs.iter() {
    //     let s = to_string_pretty(&d, pretty.clone()).expect("Serialization failed");
    // }
}
