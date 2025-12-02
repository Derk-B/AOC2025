use std::
    fs::read_to_string
;

struct Range {
    left: i64,
    right: i64,
}

impl Range {
    fn new(l: i64, r: i64) -> Range {
        return Range { left: l, right: r };
    }
    fn get_invalid_ids(&self) -> Vec<i64> {
        let mut ids = Vec::new();

        for i in self.left..(self.right + 1) {
            // If the length is uneven, then forget about it
            let i_as_str = i.to_string();
            let i_str_len = i_as_str.len();
            if i_str_len % 2 != 0 {
                continue;
            }

            let m = i_str_len / 2;
            let (l, r) = i_as_str.split_at(m);
            if l == r {
                ids.push(i);
            }
        }

        ids
    }

    fn get_invalid_ids2(&self) -> Vec<i64> {
        let mut ids = Vec::new();

        for i in self.left..(self.right + 1) {
            let i_as_str = i.to_string();
            let i_str_len = i_as_str.len();

            for j in 1..(i_str_len/2)+1 {
                if i_str_len % j != 0 {
                    continue;
                }


                let mut parts: Vec<&str> = Vec::new();
                for ji in 0..(i_str_len / j) {
                    parts.push(&i_as_str[ji*j..(ji+1)*j]);
                }

                let is_invalid_id = parts.iter().fold(true, |acc, i| acc && (*i == parts[0]));
                if is_invalid_id {
                    ids.push(i);
                    break;
                }
            }
        }

        ids
    }
}

fn main() {
    let lines = read_lines("input.txt");

    if let Some(line) = lines.iter().next() {
        let ranges_as_str = line.split(',').collect::<Vec<&str>>();
        let ranges: Vec<Range> = ranges_as_str
            .iter()
            .map(|r_str| {
                let ends = r_str.split('-').collect::<Vec<&str>>();
                if ends.len() != 2 {
                    panic!("Corrupt input range. {:?}", r_str);
                }

                let left = match ends[0].parse::<i64>() {
                    Ok(l) => l,
                    Err(e) => panic!("Corrupt input range. {:?}", e),
                };
                let right = match ends[1].parse::<i64>() {
                    Ok(r) => r,
                    Err(e) => panic!("Corrupt input range. {:?}", e),
                };
                return Range::new(left, right);
            })
            .collect();

        let invalid_ids = ranges.iter().map(|range| range.get_invalid_ids()).collect::<Vec<Vec<i64>>>().concat();
        
        let mut invalid_id_sum = 0;
        for id in invalid_ids {
            invalid_id_sum += id;
        }
        
        println!("Part 1: {}", invalid_id_sum);

        let invalid_ids2 = ranges.iter().map(|range| range.get_invalid_ids2()).collect::<Vec<Vec<i64>>>().concat();
        let mut invalid_id_sum2 = 0;
        for id in invalid_ids2 {
            invalid_id_sum2 += id;
        }
        
        println!("Part 2: {}", invalid_id_sum2);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
