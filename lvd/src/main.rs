fn main() {
    println!("Hello, world!");

    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();

    let a_trimmed = a.trim().to_string();
    let b_trimmed = b.trim().to_string();

    let l = LevenshteinDistance::default();
    let d = l.calculate(&a_trimmed, &b_trimmed);
    println!("{} vs {} => {}", &a_trimmed, &b_trimmed, &d);
}

pub struct LevenshteinDistance {
    delete_cost: i32,
    insert_cost: i32,
    update_cost: i32,
}

impl Default for LevenshteinDistance {
    fn default() -> Self {
        Self {
            delete_cost: 1,
            insert_cost: 1,
            update_cost: 1,
        }
    }
}

impl LevenshteinDistance {
    pub fn new_with_cost(delete_cost: i32, insert_cost: i32, update_cost: i32) -> Self {
        Self {
            delete_cost,
            insert_cost,
            update_cost,
        }
    }

    pub fn calculate(&self, a: &String, b: &String) -> u32 {
        // get unicode chars length
        let a_len = a.chars().count();
        let b_len = b.chars().count();

        // create table
        let mut table: Vec<Vec<char>> = vec![Vec::new(); a_len + 1];
        for i in 0..a_len + 1 {
            table[i] = vec!['\0'; b_len + 1];
        }

        // initialize table
        for i in 0..a_len {
            table[i][0] = i as u8 as char;
        }

        for j in 0..b_len {
            table[0][j] = j as u8 as char;
        }

        // calculate edit distance
        for i in 1..a_len + 1 {
            for j in 1..b_len + 1 {
                let cost = if a.chars().collect::<Vec<char>>()[i - 1]
                    == b.chars().collect::<Vec<char>>()[j - 1]
                {
                    0
                } else {
                    self.update_cost
                };

                let delete_d = table[i - 1][j] as i32 + self.delete_cost;
                let insert_d = table[i][j - 1] as i32 + self.insert_cost;
                let update_d = table[i - 1][j - 1] as i32 + cost;
                let d = std::cmp::min(delete_d, std::cmp::min(insert_d, update_d));

                table[i][j] = d as u8 as char;
            }
        }

        // print table contents
        // {
        //     table.iter().for_each(|x| {
        //         x.iter().for_each(|y| {
        //             print!("{} ", *y as u8);
        //         });
        //         println!("");
        //     });
        // }

        table[a_len][b_len] as u32
    }
}
