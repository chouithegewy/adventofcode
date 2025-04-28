use std::fs;

fn main() {
    let path = "input.txt";
    let data = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let mut safe_count = 0;
    let mut line_count = 0;

    for (li, l) in lines.iter().enumerate() {
        let report = l
            .split(r" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let r = Report::new(report);

        if r.strictly_increasing() || r.strictly_decreasing() {
            safe_count += 1;
            println!("{}: {}: {}", li, l, safe_count);
        } else {
            for i in 0..r.data.len() {
                print!("{i}");
                let (data_minus_one_level1, data_minus_one_level2) = r.data.split_at(i).clone();
                let mut data_minus_one_level = data_minus_one_level1.to_vec();
                let mut data_minus_one_level2 = data_minus_one_level2.to_vec();
                data_minus_one_level2.remove(0);
                data_minus_one_level.extend(data_minus_one_level2);
                print!("{data_minus_one_level:?}");
                let new_report = Report::new(data_minus_one_level);
                if new_report.strictly_increasing() || new_report.strictly_decreasing() {
                    safe_count += 1;
                    println!();
                    println!("{}: {}: {}", li, l, safe_count);
                    break;
                }
                println!();
            }
        }
    }
}

struct Report {
    data: Vec<i32>,
}

impl Report {
    fn new(_data: Vec<i32>) -> Self {
        Report { data: _data }
    }

    fn strictly_increasing(&self) -> bool {
        &self
            .data
            .iter()
            .enumerate()
            .filter(|(a, b)| {
                (*a < self.data.len() - 1)
                    && (b < &&self.data[*a + 1])
                    && (b.abs_diff(self.data[*a + 1]) <= 3)
                    && (b.abs_diff(self.data[*a + 1]) >= 1)
            })
            .collect::<Vec<_>>()
            .len()
            >= &(self.data.len() - 1)
    }

    fn strictly_decreasing(&self) -> bool {
        &self
            .data
            .iter()
            .enumerate()
            .filter(|(a, b)| {
                (*a < self.data.len() - 1)
                    && (b > &&self.data[*a + 1])
                    && (b.abs_diff(self.data[*a + 1]) <= 3)
                    && (b.abs_diff(self.data[*a + 1]) >= 1)
            })
            .collect::<Vec<_>>()
            .len()
            >= &(self.data.len() - 1)
    }
}

fn difference(level: i32, lastlevel: i32) -> i32 {
    level - lastlevel
}

fn is_diff_ok(diff: i32) -> bool {
    diff <= 3 || diff >= 1
}
