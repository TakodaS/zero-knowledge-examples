use log::info;
use std::{
    error::Error,
    fs::{read_to_string, write},
};

const TEMPLATE: &'static str = r#"
import "./customSha256" as customSha256

def main(private field a):
    u32[8] out = customSha256(a)
    {}
    return 
"#;

fn generate_assert(index: usize, a: usize) -> String {
    return format!("assert(out[{}] == {})", index, a);
}

fn generate_all_asserts(hash_arr: &[u32]) -> Result<String, Box<dyn Error>> {
    let mut fstring = String::new();
    for (index, &num) in hash_arr.into_iter().enumerate() {
        fstring.push_str(&(generate_assert(index, num as usize) + "\n    "));
    }
    Ok(fstring)
}

fn extract_witness_hash(string: &str) -> Result<(usize, u32), Box<dyn Error>> {
    let parts: Vec<&str> = string.split_whitespace().collect::<Vec<&str>>();
    let split_first: Vec<&str> = parts[0].split("_").collect::<Vec<&str>>();
    let index: usize = split_first[1].parse::<usize>()?;
    let num = parts[1].parse::<u32>()?;
    Ok((index, num))
}

fn extract_witness_hashes(path: &str) -> Result<[u32; 8], Box<dyn Error>> {
    let contents = read_to_string(path)?;
    let mut outarr: [u32; 8] = [0; 8];
    for line in contents.lines() {
        if line.contains("out") {
            let (index, num) = extract_witness_hash(line)?;
            outarr[index] = num;
        }
    }
    Ok(outarr)
}

fn create_new_template(fstring: &str) -> Result<String, Box<dyn Error>> {
    let out = TEMPLATE.replace(r"{}", fstring);
    Ok(out)
}

pub fn write_new_template(wit_file: &str, out_file: &str) -> Result<(), Box<dyn Error>> {
    let arr = extract_witness_hashes(wit_file)?;
    let fstring = generate_all_asserts(&arr)?;
    let outstr = create_new_template(&fstring)?;
    write(out_file, outstr)?;
    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_split() {
        let instr = "out_1 2183719";
        let res = extract_witness_hash(instr).unwrap();
        assert_eq!(1, res.0);
        assert_eq!(2183719, res.1);
    }

    #[test]
    fn test_generate_asserts() {
        let arr: [u32; 1] = [0];
        let st = generate_all_asserts(&arr).unwrap();
        assert!(st.contains(r"assert(out[0] == 0)"));
    }
    #[test]
    fn test_output() {
        let temp = create_new_template("tee").unwrap();
        println!("{}", temp)
    }
}
