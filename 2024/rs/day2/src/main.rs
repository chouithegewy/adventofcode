use std::fs;
fn main() {
    let path = "input.txt";
    let data = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let mut safe_count = 0;
    let mut line_count = 0;
    'reports: for (li, l) in lines.iter().enumerate() {
        let report = l
            .split(r" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut lastlevel = 0;
        let mut increasing: bool = false;
        'lvl: for (i, level) in report.iter().enumerate() {
            if i == 0 {
                lastlevel = *level;
            } else {
                let diff = *level - lastlevel;
                let positive = if diff > 0 { true } else { false };
                let diff_abs = if !positive { diff * -1 } else { diff };
                if diff_abs > 3 || (diff_abs < 1) {
                    continue 'reports;
                }
                let currently_increasing = if positive { true } else { false };
                if i == 1 {
                    increasing = currently_increasing;
                } else if increasing != currently_increasing {
                    continue 'reports;
                }
                lastlevel = *level;
            }
        }
        safe_count += 1;
        if line_count >= 0 {
            println!("{}: {}; safe_count = {}", li + 1, l, safe_count);
        }
        line_count += 1;
    }
    println!("{}", safe_count);
}
