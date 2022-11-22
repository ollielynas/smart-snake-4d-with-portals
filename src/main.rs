use std::time::Instant;

use macroquad::prelude::*;
use ::rand::prelude::*;

#[derive(Debug, Clone)]
struct Node {
    is_food: bool,
    is_snake: bool,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    connections: Vec<usize>,
}

impl Node {
    fn new_blank(x: i32, y: i32, z: i32, w: i32, connections: Vec<usize>) -> Node {
        Node {
            is_food: false,
            is_snake: false,
            x,
            y,
            z,
            w,
            connections,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    nodes: Vec<Node>,
    size: i32,
    snake: Vec<usize>
}
impl Board {
    fn new_2d_no_portals(size: i32) -> Board {
        let mut new = Board {
            nodes: (0..size)
                .map(|m| {
                    (0..size)
                        .map(|n| Node::new_blank(n, m, 0, 0, vec![]))
                        .collect::<Vec<Node>>()
                })
                .into_iter()
                .flatten()
                .collect::<Vec<Node>>(),
            size,
            snake: vec![0,1]
        };

        for (n, i) in new.nodes.clone().iter().enumerate() {
            for j in &mut new.nodes {
                if j.x.abs_diff(i.x) + j.y.abs_diff(i.y) + j.z.abs_diff(i.z) + j.w.abs_diff(i.w)
                    == 1
                {
                    j.connections.push(n)
                }
            }
        }
        new
    }

    fn new_4d_no_portals(size: i32) -> Board {
        let mut new = Board {
            nodes: 
            (0..size)
                .map(|y| {
                    (0..size)
                        .map(|x| Board::new_2d_no_portals(size).nodes.into_iter().map(|z| Node::new_blank(z.x, z.y, x, y, vec![])).collect::<Vec<_>>())
                        .into_iter()
                        .flatten()
                        .collect::<Vec<Node>>()
                })
                .into_iter()
                .flatten()
                .collect::<Vec<Node>>(),
                size,
                snake: vec![0,1]
        };

        for (n, i) in new.nodes.clone().iter().enumerate() {
            for j in &mut new.nodes {
                if j.x.abs_diff(i.x) + j.y.abs_diff(i.y) + j.z.abs_diff(i.z) + j.w.abs_diff(i.w)
                    == 1
                {
                    j.connections.push(n)
                }
            }
        }
        new
    }

    fn get_index(&self, x:i32,y:i32,z:i32,w:i32) -> usize {
        (x + y*self.size + z*self.size.pow(2) + w*self.size.pow(3)) as usize
    }


}

enum Direction {
    X,
    Y,
    Z,
    W
}

#[macroquad::main("Snake")]
async fn main() {
    fn coord(node: &Node, size: i32) -> (f32, f32) {
        (
            15.0 * (node.x + (node.z) * size + (node.z + 1)) as f32,
            15.0 * (node.y + (node.w) * size + (node.w + 1)) as f32,
        )
    }
    fn index_coord(n: f32) -> (f32, f32) {
        (
            3.1415*n.powf(0.5)*n.powf(2.0).sin(),
            3.1415*n.powf(0.5)*n.powf(2.0).cos(),
        )
    }

    let size = 6;
    let dimensions = 4;
    let portals = 10;
    let food = 10;




    let mut b = match dimensions {
        2 => Board::new_2d_no_portals(size),
        4 => Board::new_4d_no_portals(size),
        _ => {
            warn!("Invalid number of dimensions, {dimensions}");
            Board{
                nodes: vec![],
                size: 0,
                snake: vec![]
            }
        }
    };

    let l = b.nodes.len();
    let mut rng = ::rand::thread_rng();
    for _ in 0..portals {
        b.nodes[rng.gen_range(0..l)].connections = vec![(rng.gen_range(0..l))];
    }

    for _ in 0..food {
        b.nodes[rng.gen_range(0..l)].is_food = true;
    }

    let mut direction = Direction::X;
    // can be 1 or -1
    let mut speed: i32 = 1; 
    
    let mut now = Instant::now();

    
    loop {
        clear_background(Color::from_rgba(10, 10, 10, 255));
        match get_char_pressed() {
            Some('w') => {
                direction = Direction::Y;
                speed = -1;
            },
            Some('a') => {
                direction = Direction::X;
                speed = -1;
            },
            Some('s') => {
                direction = Direction::Y;
                speed = 1;
            },
            Some('d') => {
                direction = Direction::X;
                speed = 1;
            },
            Some('g') => {
                direction = Direction::W;
                speed = 1;
            },
            Some('f') => {
                direction = Direction::Z;
                speed = -1;
            },
            Some('t') => {
                direction = Direction::W;
                speed = -1;
            },
            Some('h') => {
                direction = Direction::Z;
                speed = 1;
            }
            _ => {}
        }
        if now.elapsed().as_millis() > 500 {
            now = Instant::now();
            let head = &b.nodes[b.snake[b.snake.len()-1]];
            println!("head {:?}", head);
            let portaled = b.nodes[b.snake[b.snake.len()-1]].connections.len() == 1;
            if portaled {
                // portal_exit
                b.snake.push(
                    b.nodes[b.snake[b.snake.len()-1]].connections[0]
                );
            }
            if match direction {
                Direction::X => {
                    if !portaled && (0 ..b.size ).contains(&((head.x  + speed))) {
                        b.snake.push(b.get_index(head.x + speed , head.y, head.z, head.w));
                    }portaled || (0 ..b.size ).contains(&((head.x  + speed)))
                },
                Direction::Y => {
                    if !portaled && (0 ..b.size ).contains(&((head.y  + speed))) {
                    b.snake.push(b.get_index(head.x , head.y  + speed, head.z, head.w));
                    }portaled || (0 ..b.size ).contains(&((head.y  + speed)))
                },
                Direction::Z => {
                    if !portaled && (0 ..b.size ).contains(&((head.z  + speed))) {
                        b.snake.push(b.get_index(head.x, head.y, head.z + speed , head.w));
                    }portaled || (0 ..b.size ).contains(&((head.z  + speed)))
                },
                Direction::W => {
                    if !portaled && (0 ..b.size ).contains(&((head.w  + speed))) {
                        b.snake.push(b.get_index(head.x , head.y, head.z, head.w+ speed ));
                    }portaled || (0 ..b.size ).contains(&((head.w  + speed)))
                },
            } {
                // snake lived
                if !b.nodes[b.snake[b.snake.len()-1]].is_food {
                    b.nodes[b.snake[0]].is_snake = false;
                    b.snake.remove(0);
                }else {
                    println!("got food");
                    b.nodes[b.snake[b.snake.len()-1]].is_food = false;
                    b.nodes[rng.gen_range(0..l)].is_food = true;
                }
            }else {
            // snake is dead
            println!("died");
            direction = Direction::X;
            speed = 1;
            for i in &mut b.nodes {
                i.is_snake = false;
                i.is_food = false;
                b.snake = vec![0,1];
            }

            for _ in 0..food {
                b.nodes[rng.gen_range(0..l)].is_food = true;
            }
        }
        
        
    }
    
    for i in &b.snake {
        b.nodes[*i].is_snake = true;
    }
    
    for i in &b.nodes {
            for j in &i.connections {
                let a = (coord(i, size), coord(&b.nodes[*j], size));
                draw_line(a.0.0, a.0.1, a.1.0, a.1.1, 1.0, Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 })
            }
            let p = coord(i, size);
            draw_circle(
                p.0,
                p.1,
                match i.is_snake || i.is_food {
                    true => 4.0,
                    _ => 3.0,
                },
                match (i.is_food, i.is_snake) {
                    (false, false) => Color::from_rgba(255, 255, 255, 100),
                    (true, false) => GOLD,
                    (false, true) => WHITE,
                    (true, true) => GOLD,
                },
            );

            if i.connections.len() == 1 {
                draw_circle(
                p.0,
                p.1,
                match i.is_snake || i.is_food {
                    true => 4.0,
                    _ => 3.0,
                },
                Color { r: 0.1, g: 0.1, b: 1.0, a: 1.0 }
                );
                let p2 = coord(&b.nodes[i.connections[0]], size);
                if i.is_snake {
                    draw_circle(
                    p2.0,
                    p2.1,
                    5.0,
                    ORANGE
                    );
                    draw_line(p.0, p.1, p2.0, p2.1, 2.0, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 })
                }
            }

            
        }
        
        
        

        next_frame().await
    }
}
