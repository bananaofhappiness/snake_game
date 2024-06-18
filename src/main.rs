use std::{io::{self, Read}, sync::mpsc, thread::{self, sleep}, time::Duration};
use rand::{self, Rng};

const WIDTH: usize = 32;
const HEIGHT: usize = 16;
struct Snake {
    length: usize,
    head: Coordinates,
}

impl Snake {
    fn new(length: usize, head: Coordinates) -> Self {
        Self{length, head}
    }
}

struct Coordinates{
    x: usize, 
    y: usize,
}

enum Moving {
    Up,
    Left,
    Right,
    Down,
}

fn main() {
    let mut world: [[usize;WIDTH];HEIGHT] = [[0;WIDTH];HEIGHT];
    let mut snake = spawn_snake(&mut world);
    let mut moving = Moving::Down;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let mut buffer = [0; 1];
            io::stdin().read_exact(&mut buffer).unwrap();
            match buffer[0] {
                27 => {
                    let mut seq = [0; 2];
                    io::stdin().read_exact(&mut seq).unwrap();
    
                    if seq == [91, 65] {
                        tx.send(Moving::Up).unwrap();
                    } else if seq == [91, 66] {
                        tx.send(Moving::Down).unwrap()
                    } else if seq == [91, 67] {
                        tx.send(Moving::Right).unwrap()
                    } else if seq == [91, 68] {
                        tx.send(Moving::Left).unwrap()
                    }
                }
                _ => ()
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    loop {
        moving = rx.try_recv().unwrap_or(moving);
        display(&world);
        next(&mut world, &mut snake, &moving);
        check_fruit(&mut world);
        sleep(Duration::from_millis(150));
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn display(world: &[[usize;WIDTH];HEIGHT]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            match world[i][j] {
                0 => print!("█"),
                1 => print!("\x1b[92m█\x1b[0m"),
                2 => print!("\x1b[91m█\x1b[0m"),
                _ => print!("\x1b[92m█\x1b[0m"),
            }
        }
        println!();
    }
}

fn spawn_snake(world: &mut [[usize;WIDTH];HEIGHT]) -> Snake {
    let x: usize = 16;
    let y: usize = 8;
    let snake = Snake::new(5, Coordinates{x,y});
    world[y][x] = 1;
    snake
}

fn next(world: &mut [[usize;WIDTH];HEIGHT], snake: &mut Snake, moving: &Moving) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            match world[i][j] {
                0 => (),
                1 => {
                    match moving {
                        Moving::Right => {
                            world[i][j] = snake.length;
                            snake.head.x = module(snake.head.x as i8+1,WIDTH);
                            match world[snake.head.y][snake.head.x] {
                                2 => snake.length+=1,
                                0 => (),
                                1 => (),
                                _ => panic!(),
                            }
                        },
                        Moving::Left => {
                            world[i][j] = snake.length;
                            snake.head.x = module(snake.head.x as i8-1,WIDTH);
                            match world[snake.head.y][snake.head.x] {
                                2 => snake.length+=1,
                                0 => (),
                                1 => (),
                                _ => panic!(),
                            }
                        },
                        Moving::Down => {
                            world[i][j] = snake.length;
                            snake.head.y = module(snake.head.y as i8 +1,HEIGHT);
                            match world[snake.head.y][snake.head.x] {
                                2 => snake.length+=1,
                                0 => (),
                                1 => (),
                                _ => panic!(),
                            }
                        }
                        Moving::Up => {
                            world[i][j] = snake.length;
                            snake.head.y = module(snake.head.y as i8-1,HEIGHT);
                            match world[snake.head.y][snake.head.x] {
                                2 => snake.length+=1,
                                0 => (),
                                1 => (),
                                _ => panic!(),
                            }
                        }
                    }
                },
                2 => (),
                body => {
                    if body == 3 { 
                        world[i][j] = 0;
                    };
                    if body > 3 {
                        world[i][j] -= 1;
                    }
                },
            }
        }
    }
    // *world = back_world;
    world[snake.head.y][snake.head.x] = 1;
}

fn check_fruit(world: &mut [[usize;WIDTH];HEIGHT]){
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if world[i][j] == 2 {
                let pos = Coordinates{x: j, y: i};
                world[pos.y][pos.x] = 2;
                return
            }
        }
    }
    let mut rng = rand::thread_rng();
    let pos = Coordinates{x: rng.gen_range(0..WIDTH), y: rng.gen_range(0..HEIGHT)};
    world[pos.y][pos.x] = 2;
}

fn module(a: i8, b:usize) -> usize {
    ((a%b as i8+b as i8)%(b as i8)) as usize
}