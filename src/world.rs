#[derive(Clone, Copy, PartialEq)]
pub enum Particle {
    Air,
    Sand,
    Water,
}

#[derive(Default)]
pub struct World {
    grid_width: usize,
    grid_height: usize,
    curr_grid: Vec<Particle>,
    next_grid: Vec<Particle>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid_width: width,
            grid_height: height,
            curr_grid: vec![Particle::Air; width * height],
            next_grid: vec![Particle::Air; width * height],
        }
    }

    pub fn update(&mut self) -> () {
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let idx = y * self.grid_width + x;
                match self.curr_grid[idx] {
                    Particle::Sand => self.update_sand(x, y, idx),
                    _ => {}
                }
            }
            std::mem::swap(&mut self.curr_grid, &mut self.next_grid);
            self.next_grid.fill(Particle::Air);
        }
    }

    pub fn particles(&self) -> &[Particle] {
        &self.curr_grid
    }

    fn update_sand(&mut self, x: usize, y: usize, idx: usize) -> () {
        if y >= self.grid_height - 1 {
            return;
        }

        let down = ((y + 1) * self.grid_width) + x;
        let down_left = down.saturating_sub(1);
        let down_right = down + 1;

        if self.curr_grid[down] == Particle::Air {
            self.next_grid[down] = Particle::Sand;
        } else if x > 0 && self.curr_grid[down_left] == Particle::Air {
            self.next_grid[down_left] = Particle::Sand;
        } else if x < self.grid_width - 1 && self.curr_grid[down_right] == Particle::Air {
            self.next_grid[down_right] = Particle::Sand;
        } else {
            self.next_grid[idx] = Particle::Sand;
        }
    }
}
