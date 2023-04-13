use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

const NEXT_LINE: char = '\n';

fn main() {
    let dir = env::current_dir().unwrap();
    let j_file_name = "war3map.j";
    let j_file_path = dir.join(j_file_name);
    let config_name = "config.txt";
    let config_file_path = dir.join(config_name);

    if !check_file(j_file_path.to_str().unwrap()) {
        println!("Read j file failed");
        return;
    }

    if !check_file(config_file_path.to_str().unwrap()) {
        println!("Read config file failed");
        return;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(j_file_path.clone())
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let config_lines = read_lines(config_file_path.to_str().unwrap().to_string());

    for line in config_lines {
        let data = line.unwrap();
        let data = data.split(",").collect::<Vec<&str>>();
        let index = contents.find(data[0]);
        if index.is_none() {
            continue;
        }
        let index = index.unwrap();
        let insert_start = find_next_char_multis(&contents, NEXT_LINE, index, 2).unwrap();
        let end = find_next_char_multis(&contents, NEXT_LINE, index, 3).unwrap();
        let temp_str = &contents[insert_start + 1..end + 1].to_string();

        let start = find_next_char(&temp_str, '\"', 0).unwrap();
        let end = find_next_char(&temp_str, '\"', start + 1).unwrap();

        let temp_str = temp_str.replace(&temp_str[start + 1..end], "@TEMP");

        for name in data.iter().skip(1) {
            let str = temp_str.replace("@TEMP", name);
            let insert_pos = insert_start + 1;
            contents.insert_str(insert_pos, str.as_str());
        }
    }

    let mut file = File::create(j_file_path.clone()).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn find_next_char_multis(
    contents: &String,
    char: char,
    start_index: usize,
    count: usize,
) -> Option<usize> {
    let mut index = start_index;
    for i in 0..count {
        if i != 0 {
            index += 1;
        }

        let result = find_next_char(contents, char, index);

        if result.is_none() {
            return None;
        }

        index = result.unwrap();
    }

    return Some(index);
}

fn find_next_char(contents: &String, char: char, start_index: usize) -> Option<usize> {
    let index = contents
        .get(start_index..)
        .and_then(|s| s.find(char).map(|i| i + start_index));
    return index;
}

fn check_file(file_path: &str) -> bool {
    let metadata = fs::metadata(file_path);

    if metadata.is_err() {
        println!("{} didn't exists", file_path);
        return false;
    }

    let permission = metadata.unwrap().permissions();

    if permission.readonly() {
        println!("{} is readonly", file_path);
        return false;
    }

    return true;
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}
