use std::{env, fs, vec};
mod xmlformat;
use xmlformat::FileParsed;

fn main() {
  let path_binding = get_path();
  let path = path_binding.as_str();
  let entries = get_files(&path);

  for entry in entries {
    let file_path = entry.unwrap().path();

    if !file_path.is_file() {
      continue;
    }
    let filename = String::from(
      file_path
        .file_name()
        .expect("Couldn't Get File Name")
        .to_str()
        .expect("Couldn't Become &str"),
    );
    if !filename.ends_with(".cfg") {
      continue;
    }

    let content_string = fs::read_to_string(file_path.clone()).unwrap();
    let a = parse_content_string(&content_string, &filename);
    let content = match a {
      Ok(item) => item,
      Err(item_err) => {
        println!("{}", item_err);
        continue;
      }
    };

    match create_lss(&content, path) {
      Ok(_) => {}
      Err(err) => println!("Couldn't Create LSS File because: {}", err),
    };
  }
}

fn get_files(path: &&str) -> fs::ReadDir {
  let mut dir_path = env::current_dir().unwrap();
  dir_path.push(path);
  fs::read_dir(dir_path).unwrap()
}

fn parse_content_string<'a>(
  content_string: &'a str,
  filename: &'a str,
) -> Result<FileParsed<'a>, String> {
  let mut category_name = "".to_string();
  let mut splits: Vec<&str> = vec![];
  let file_name = filename.trim_end_matches(".cfg");

  for (line_num, line) in content_string.lines().enumerate() {
    if line_num == 0 {
      category_name = get_name(&line, &filename)?;
    }
    if line.starts_with("sar_speedrun_cc_rule ") {
      let rule_line = line.trim_start_matches("sar_speedrun_cc_rule ");
      let parts: Vec<&str> = rule_line.split('"').collect();
      splits.push(parts[1]);
    } else if line.starts_with("sar_speedrun_cc_finish") {
      break;
    }
  }

  splits.remove(0);

  Ok(FileParsed {
    category_name,
    splits,
    file_name,
  })
}

fn get_name(line: &str, file_name: &str) -> Result<String, String> {
  if line.starts_with("sar_speedrun_cc_start ") {
    let start_line = line.trim_start_matches("sar_speedrun_cc_start ");
    let parts: Vec<&str> = start_line.split('"').collect();
    let name = parts[1].to_string();
    Ok(name)
  } else {
    Err(format!("\"{file_name}\" Is An Invalid File"))
  }
}

fn create_lss(file: &FileParsed, path: &str) -> std::io::Result<()> {
  let file_name = if path != "" {
    format!("{}/{}.lss", path, file.file_name)
  } else {
    format!("{}.lss", file.file_name)
  };
  let file_content = xmlformat::create_xml(file);

  let mut file = fs::File::create(file_name)?;
  std::io::Write::write_all(&mut file, file_content.as_bytes())?;

  Ok(())
}

fn get_path() -> String {
  let mut args = env::args().skip(1); // skip first argument (program name)

  let file_dir_string = match args.next() {
    Some(val) => match val.as_str() {
      "--path" | "-P" => {
        if let Some(file_dir_arg) = args.next() {
          file_dir_arg
        } else {
          println!("Error: --path requires a value. Using Current Dir As Default");
          "".to_string()
        }
      }
      _ => "".to_string(),
    },
    None => "".to_string(),
  };

  file_dir_string
}
