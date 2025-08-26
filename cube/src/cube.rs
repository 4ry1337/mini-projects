use std::{
    ops::{Add, AddAssign},
    thread::sleep,
    time::Duration,
};

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Angle(f32, f32, f32);

impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Angle {
    pub fn random(rand_min: f32, rand_max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(rand_min..=rand_max);
        let y = rng.gen_range(rand_min..=rand_max);
        let z = rng.gen_range(rand_min..=rand_max);
        Self(x, y, z)
    }
}

#[derive(Debug, Clone)]
pub struct Cube {
    width: i32,
    height: i32,
    angle: Angle,
    cube_width: i32,
    f_cube_width: f32,
    z_buffer: Vec<f32>,
    buffer: Vec<u8>,
    distance_from_cam: f32,
    horizontal_offset_value: f32,
    horizontal_offset: f32,
    random_speed: bool,
    random_speed_min: f32,
    random_speed_max: f32,
    rotation_speed: Angle,
    k1: f32,
    tick: usize,
}

const COLORS: [&str; 7] = [
    " ",
    "\x1B[38;5;46m*\x1B[0m",
    "\x1B[38;5;47m*\x1B[0m",
    "\x1B[38;5;48m*\x1B[0m",
    "\x1B[38;5;49m*\x1B[0m",
    "\x1B[38;5;50m*\x1B[0m",
    "\x1B[38;5;51m*\x1B[0m",
];

impl Cube {
    pub fn new(height: i32, width: i32, cube_width: i32) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cube_width,
            angle: Angle(0.0, 0.0, 0.0),
            f_cube_width: cube_width as f32,
            z_buffer: vec![0.0; size.try_into().unwrap()],
            buffer: vec![0; size.try_into().unwrap()],
            distance_from_cam: 100.0,
            horizontal_offset: 0.0,
            horizontal_offset_value: 0.0,
            random_speed: false,
            random_speed_min: 0.0,
            random_speed_max: 0.20,
            rotation_speed: Angle(0.07, 0.07, 0.1),
            k1: 40.0,
            tick: 0,
        }
    }

    pub fn random_rotation(&mut self) {
        self.random_speed = true;
    }

    pub fn set_initial_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.angle = Angle(x, y, z);
    }

    pub fn set_speed(&mut self, x: f32, y: f32, z: f32) {
        self.rotation_speed = Angle(x, y, z);
    }

    pub fn speed(&self) -> Angle {
        match self.random_speed {
            true => Angle::random(self.random_speed_min, self.random_speed_max),
            false => self.rotation_speed.clone(),
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
        self.angle += self.speed();
        self.buffer.fill(0);
        self.z_buffer.fill(0.0);
        self.horizontal_offset = self.horizontal_offset_value * self.cube_width as f32;
        let mut cube_x = -self.f_cube_width;
        while cube_x < self.cube_width as f32 {
            let mut cube_y = -self.cube_width as f32;
            while cube_y < self.cube_width as f32 {
                self.calculate_for_surface(cube_x, cube_y, -self.f_cube_width, 1);
                self.calculate_for_surface(self.f_cube_width, cube_y, cube_x, 2);
                self.calculate_for_surface(-self.f_cube_width, cube_y, -cube_x, 3);
                self.calculate_for_surface(-cube_x, cube_y, self.f_cube_width, 4);
                self.calculate_for_surface(cube_x, -self.f_cube_width, -cube_y, 5);
                self.calculate_for_surface(cube_x, self.f_cube_width, cube_y, 6);
                cube_y += self.speed().1;
            }
            cube_x += self.speed().0;
        }
    }

    fn render(&mut self) {
        println!("\x1b[Htick: {}", self.tick);
        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                print!(
                    "{}",
                    COLORS[self.buffer[i * self.width as usize + j] as usize]
                );
            }
            println!();
        }
        self.tick += 1;
    }

    fn calculate_for_surface(&mut self, cube_x: f32, cube_y: f32, cube_z: f32, value: u8) {
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
        if idx >= 0 && idx < self.width * self.height && ooz > self.z_buffer[idx as usize] {
            self.z_buffer[idx as usize] = ooz;
            self.buffer[idx as usize] = value;
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        j * f32::sin(self.angle.0) * f32::sin(self.angle.1) * f32::cos(self.angle.2)
            - k * f32::cos(self.angle.0) * f32::sin(self.angle.1) * f32::cos(self.angle.2)
            + j * f32::cos(self.angle.0) * f32::sin(self.angle.2)
            + k * f32::sin(self.angle.0) * f32::sin(self.angle.2)
            + i * f32::cos(self.angle.1) * f32::cos(self.angle.2)
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        j * f32::cos(self.angle.0) * f32::cos(self.angle.2)
            + k * f32::sin(self.angle.0) * f32::cos(self.angle.2)
            - j * f32::sin(self.angle.0) * f32::sin(self.angle.1) * f32::sin(self.angle.2)
            + k * f32::cos(self.angle.0) * f32::sin(self.angle.1) * f32::sin(self.angle.2)
            - i * f32::cos(self.angle.1) * f32::sin(self.angle.2)
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        k * f32::cos(self.angle.0) * f32::cos(self.angle.1)
            - j * f32::sin(self.angle.0) * f32::cos(self.angle.1)
            + i * f32::sin(self.angle.1)
    }
}
