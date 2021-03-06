struct Table<'a> {
    table: Vec<Vec<&'a str>>,
    width: usize,
}

impl<'a> Table<'a> {
    fn add(&mut self, vec: Vec<&'a str>) {
        self.table.push(vec.clone());
        for field in vec {
            // println!("{}" , &field.len()  ) ;
            if field.len() > self.width {
                self.width = field.len();
                //println!("{}" , self.width );
            }
        }
    }

    fn render(&self) {
        let mut to_print: String = String::new();
        for rows in &self.table {
            for field in rows {
                let to_push = format!("| {} {} ", field, " ".repeat((self.width - field.len())));
                to_print.push_str(&to_push);
            }

            to_print.push_str("\n");
        }

        println!("{} ", to_print)
    }
}
//----------------------TEST------------------------------------------------
fn main() {
    println!("Hello, world!");
    let mut t: Table = Table {
        table: vec![vec!["ok", "lmk"]],
        width: 0,
    };
    t.add(vec!["foo", "baar"]);
    println!("{:?}, {} ", t.table, t.width);
    t.render();
}


