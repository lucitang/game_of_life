pub struct Board<'a, 'b, 'c, 'd>{
    pub size: i32,
    pub initial_grid: &'a mut Vec<&'b mut Vec<i32>>,
    pub sum_grid: &'c mut Vec<&'d mut Vec<i32>>,
}

impl<'a, 'b, 'c, 'd> Board<'a, 'b, 'c, 'd> {
    fn print_initial_stage(&mut self){
        for row in self.initial_grid.iter_mut(){
            for j in row.iter_mut(){
                print!("{}      ",  j);
            }
            print!("\n");
        }
    }
    
}

impl<'a, 'b, 'c, 'd> Board<'a, 'b, 'c, 'd>{
    fn print_stage(&mut self){
        for i in self.sum_grid.iter_mut() {
            for j in i.iter_mut(){
                print!("{}      ", j);
            }
            print!("\n");
        }
    }

}

impl<'a, 'b, 'c, 'd> Board<'a, 'b, 'c, 'd>{
    fn count_of_live_neighbors(&self, row:i32, col:i32) -> i32{
        let  mut live_neighbors = 0;
        for i in (row - 1)..(row + 2){
            for j in (col - 1)..(col + 2){
                let a:usize = i as usize;
                let b:usize = j as usize;
                
                if(i < 0 || j < 0) || (i == row && j == col) || (i >= self.size || j >= self.size) {
                    continue;
                }
                if self.initial_grid[a][b] == 1 {
                    live_neighbors = live_neighbors + 1;
                }
            }
        }
        live_neighbors
    }
}

impl<'a, 'b, 'c, 'd> Board<'a, 'b, 'c, 'd> {
    fn sum_grid_maker(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                let a = i as usize;
                let b = j as usize;
                if  self.initial_grid[a][b] == 0{
                     if self.count_of_live_neighbors(i, j) == 3 {
                        self.sum_grid[a][b] = 1;
                    }
                    else {
                        self.sum_grid[a][b] = 0;
                    }
                }
                if self.initial_grid[a][b] == 1{
                     if self.count_of_live_neighbors(i, j) ==2 || self.count_of_live_neighbors(i, j) == 3 {
                        self.sum_grid[a][b] = 1;
                     }
                     else {
                        self.sum_grid[a][b] = 0;
                     }
                }
            }
        }
    }
}

impl <'a, 'b, 'c, 'd> Board<'a, 'b, 'c, 'd>{
     pub fn start(&mut self) {
        self.print_initial_stage();
        print!("\x1b[3;A");
        std::thread::sleep(std::time::Duration::from_secs_f32(2.0));
        loop {
            self.sum_grid_maker();
            self.print_stage();
            print!("\x1b[3;A");
            for i in 0..self.size {
                for j in 0..self.size{
                    let a = i as usize ;
                    let b = j as usize;
                    self.initial_grid[a][b] = self.sum_grid[a][b];
                }
            }
            std::thread::sleep(std::time::Duration::from_secs_f32(2.0));
        }
    }
}
