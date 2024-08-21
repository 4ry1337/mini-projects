use std::{thread::sleep, time::Duration, usize};

pub struct Cube {
    width: i32,
    height: i32,
    a: f32,
    b: f32,
    c: f32,
    cube_width: i32,
    f_cube_width: f32,
    z_buffer: Vec<f32>,
    buffer: Vec<char>,
    distance_from_cam: f32,
    horizontal_offset_value: f32,
    horizontal_offset: f32,
    increment_speed: f32,
    k1: f32,
    tick: usize,
}

impl Cube {
    pub fn new(
        height: i32,
        width: i32,
        cube_width: i32,
        horizontal_offset_value: f32,
        distance_from_cam: f32,
        increment_speed: f32,
        k1: f32,
    ) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cube_width,
            a: 0.0,
            b: 0.0,
            c: 0.0,
            f_cube_width: cube_width as f32,
            z_buffer: vec![0.0; size.try_into().unwrap()],
            buffer: vec![' '; size.try_into().unwrap()],
            distance_from_cam,
            horizontal_offset: 0.0,
            horizontal_offset_value,
            increment_speed,
            k1,
            tick: 0,
        }
    }

    pub fn run(&mut self) {
        print!("\x1b[2J");
        loop {
            self.update();
            self.render();
            sleep(Duration::from_millis(33));
        }
    }

    fn update(&mut self) {
        self.a += self.increment_speed;
        self.b += self.increment_speed;
        self.c += 0.01;
        self.buffer.fill(' ');
        self.z_buffer.fill(0.0);
        self.horizontal_offset = self.horizontal_offset_value * self.cube_width as f32;
        let mut cube_x = -self.f_cube_width;
        while cube_x < self.cube_width as f32 {
            let mut cube_y = -self.cube_width as f32;
            while cube_y < self.cube_width as f32 {
                self.calculate_for_surface(cube_x, cube_y, -self.f_cube_width, '@');
                self.calculate_for_surface(self.f_cube_width, cube_y, cube_x, '$');
                self.calculate_for_surface(-self.f_cube_width, cube_y, -cube_x, '~');
                self.calculate_for_surface(-cube_x, cube_y, self.f_cube_width, '#');
                self.calculate_for_surface(cube_x, -self.f_cube_width, -cube_y, ';');
                self.calculate_for_surface(cube_x, self.f_cube_width, cube_y, '+');
                cube_y += self.increment_speed;
            }
            cube_x += self.increment_speed;
        }
    }

    fn render(&mut self) {
        println!("\x1b[Htick: {}", self.tick);
        for _ in 0..self.width as usize + 1 {
            print!("\x1B[42m \x1B[0m");
        }
        for i in 0..self.height as usize {
            print!("\x1B[42m \x1B[0m");
            for j in 0..self.width as usize {
                print!("{}", self.buffer[i * self.width as usize + j]);
            }
            println!("\x1B[42m \x1B[0m");
        }
        for _ in 0..self.width as usize + 2 {
            print!("\x1B[42m \x1B[0m");
        }
        self.tick += 1;
    }

    fn calculate_for_surface(&mut self, cube_x: f32, cube_y: f32, cube_z: f32, ch: char) {
        let x = self.calculate_x(cube_x, cube_y, cube_z);
        let y = self.calculate_y(cube_x, cube_y, cube_z);
        let mut z = self.calculate_z(cube_x, cube_y, cube_z) + self.distance_from_cam;

        if z == 0.0 {
            z = 1e-6; // Avoid division by zero
        }

        let ooz = 1.0 / z;

        let xp =
            (self.width as f32 / 2.0 + self.horizontal_offset + self.k1 * ooz * x * 2.0) as i32;
        let yp = (self.height as f32 / 2.0 + self.k1 * ooz * y) as i32;

        let idx = xp + yp * self.width;
        if idx >= 0 && idx < self.width * self.height {
            if ooz > self.z_buffer[idx as usize] {
                self.z_buffer[idx as usize] = ooz;
                self.buffer[idx as usize] = ch;
            }
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        return j * f32::sin(self.a) * f32::sin(self.b) * f32::cos(self.c)
            - k * f32::cos(self.a) * f32::sin(self.b) * f32::cos(self.c)
            + j * f32::cos(self.a) * f32::sin(self.c)
            + k * f32::sin(self.a) * f32::sin(self.c)
            + i * f32::cos(self.b) * f32::cos(self.c);
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        return j * f32::cos(self.a) * f32::cos(self.c) + k * f32::sin(self.a) * f32::cos(self.c)
            - j * f32::sin(self.a) * f32::sin(self.b) * f32::sin(self.c)
            + k * f32::cos(self.a) * f32::sin(self.b) * f32::sin(self.c)
            - i * f32::cos(self.b) * f32::sin(self.c);
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        return k * f32::cos(self.a) * f32::cos(self.b) - j * f32::sin(self.a) * f32::cos(self.b)
            + i * f32::sin(self.b);
    }
}
