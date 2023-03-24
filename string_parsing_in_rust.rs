use std::{collections::HashMap, ops::Index};
mod r#try;
mod train;
// use std::collections::HashMap;
fn getkv(data: &str) -> HashMap<String,String> {
    let mut key_value_pairs = HashMap::new();
    let mut line_no = 1;
    for line in data.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            key_value_pairs.insert(line_no.to_string(), parts[1].to_string());
        }
        line_no += 1;
    }
    key_value_pairs
}
fn findkey(lwc:&String)->String{
    let mut ql=0;
    let mut startat=0;
    let mut endat=0;
    let mut lw=0;
    let mut string_to_work_with="".to_string();
    // let mut p=0;
    // println!("interim1----->{}",lwc);
    
    for (l,i) in lwc.chars().enumerate(){
        
        if i.eq(&'"')
        {
            // println!("interimk----->{}",lwc.chars().skip(l).into_iter().collect::<String>());
            ql+=1;
            if(ql==2){
                startat=l;
                string_to_work_with=lwc.chars().skip(l).skip(1).into_iter().collect::<String>();
                break;
                // lwc[p..].to_string();
            }         
         // break;
         }
     }
    //  println!("interim2----->{}",string_to_work_with);
     let mut res=vec![];
     for (p,g) in string_to_work_with.chars().enumerate(){
        // println!("interimu----->{}",string_to_work_with.chars().skip(p).into_iter().collect::<String>());
        if(!g.eq(&'"')){
            res.push(g);
            // print!("{g}")
        }
        else{
            break;
        }
     }

     let result =res.into_iter().collect::<String>();
     println!("{}", result);
     result
}
fn checkindex(lwc:&String,l:&usize)->usize{
    if(l+4)<lwc.len(){
        (l+4)
    }
    else{
        0
    }
}
fn dostuffwithline(mut lwc:&String)->String{
    // println!("{lwc}");
    let key_string="text".to_string();
    let key_string_length=key_string.len();
    let lwc=&lwc.chars().filter(|c| !c.is_whitespace()).into_iter().collect::<String>();
    // let text_vec = lwc.chars().collect::<Vec<_>>();
    let mut result="".to_string();
    // let mut p=0;
    for (l,i) in lwc.chars().enumerate(){
       if i.eq(&'t')
       {
        let tms=lwc.chars().skip(l).take(key_string_length).into_iter().collect::<String>();
        // println!("{}",tms);
        // if (l+4)<lwc.len()
        {

        if((tms)==key_string)
            {
                // p=;
                result = findkey(&lwc.chars().skip(l).into_iter().collect::<String>());
                break;
            }
        // break;
        }
    }
        
    }
result
    // println!("{}",lwc[p..].to_string())
}
#[test]
fn tryit(){
    dostuffwithline(&r#"whtaever is "text": "this" whgasdasdsa"#.to_string());
    // dostuffwithline(&r#"whtaever is "tIT“""#.to_string());
}
fn parse_data(data: &String) -> HashMap<String, String> {
    let mut key_value_pairs = HashMap::new();
    let mut line_no = 1;
    let mut all_line_with_context = String::new();
    for line in data.lines() {
        if line.contains("title")
        {
            let line_with_context = data.lines().skip(line_no - 3).take(5).filter(|line| contains_non_english(line)).collect::<Vec<&str>>().join("\n");
            // let value = printallnonenglish(&line_with_context).trim().to_string();
            let value = dostuffwithline(&line_with_context).trim().to_string();
            all_line_with_context.push_str(&line_with_context);
            key_value_pairs.insert(line_no.clone().to_string(), value);
        }
        line_no += 1;
    }
    // println!("{}", all_line_with_context);
    key_value_pairs
    // getkv(&all_line_with_context)
}

fn printallnonenglish(input_string:&String)->String{
    let english_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
// let input_string = "sda \"text\":\"whatever\"\nsdasd  \"text\":\"sdfsdf\"\n asda\"text\":\"dsfsd\"\n\"text\":\"dsfs\" asdas";
let non_english_chars = input_string.chars().filter(|c| !english_chars.contains(*c)).collect::<String>();
println!("{}", non_english_chars);
non_english_chars
}

use regex::Regex;

// fn get_values(data: &str) -> HashMap<String, String> {
//     let mut key_value_pairs = HashMap::new();
//     let re = Regex::new(r"(?P<key>[a-zA-Z0-9_]+)=(?P<value>[^=\n]+)").unwrap();
//     let mut line_no = 1;
//     for capture in re.captures_iter(data) {
//         let value = capture.name("value").unwrap().as_str();
//         if !value.contains("{") && !value.contains("[") {
//             key_value_pairs.insert(line_no.to_string(), value.to_string());
//         }
//         line_no += 1;
//     }
//     key_value_pairs
// }
fn contains_non_english(text: &str) -> bool {
    let re = Regex::new(r"[^\p{ASCII}]").unwrap();
    re.is_match(text)
}
fn main(){
    print!("test")
}

#[cfg(test)]
mod tests {
    use std::fs::{File, self};

    use super::*;

    #[test]
    fn test_parse_data() {
        let data=fs::read_to_string(("./input.txt")).unwrap();
        // let data = "key1=value1\nkey2=value2\nkey3=value3\n";
        // let expected = vec![
        //     ("key1", "value1"),
        //     ("key2", "value2"),
        //     ("key3", "value3"),
        // ]
        // .into_iter()
        // .collect::<HashMap<&str, &str>>();
        // assert_eq!(parse_data(data), expected);


        // for g in parse_data(data){
        //     print!("{}---{}",g.0,g.1)
        // }
        let mut store = String::new();
    for (key, value) in parse_data(&data) {
        store.push_str(&format!("{}={}\n", key, value));
    }
    std::fs::write("./try.txt", store).unwrap();
    }
}
