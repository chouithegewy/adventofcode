use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "input.txt";
    let f = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = f.lines().collect();
    let mut l1 = vec![];
    let mut l2 = vec![];
    for l in lines {
        l1.push(l.split(r#"   "#).collect::<Vec<_>>()[0]);
        l2.push(l.split(r#"   "#).collect::<Vec<_>>()[1]);
    }
    l1.sort();
    l2.sort();
    let mut similarities = HashMap::new();
    for l in l2 {
        similarities.entry(l).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut sum: i32 = 0;
    for (i, e) in l1.iter().enumerate() {
        let num1 = l1[i].parse::<i32>().unwrap();
        let num2 = similarities.get(*e).unwrap_or(&0);
        let mut _sum = num1 * num2;
        if _sum < 0 {
            _sum = _sum * -1;
        } else {
            _sum = _sum * 1;
        }
        sum += _sum;
    }
    println!("{}", sum);
}
