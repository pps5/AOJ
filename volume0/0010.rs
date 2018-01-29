use std::io::{stdin, BufRead, BufReader};

struct Point {
    x: f32,
    y: f32,
}

fn points(values: Vec<f32>) -> [Point; 3] {
    return [
        Point {
            x: values[0],
            y: values[1],
        },
        Point {
            x: values[2],
            y: values[3],
        },
        Point {
            x: values[4],
            y: values[5],
        },
    ];
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    return ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt();
}

fn angle(l1: f32, l2: f32, l3: f32) -> f32 {
    return ((l1.powf(2.0) + l2.powf(2.0) - l3.powf(2.0)) / (2.0 * l1 * l2)).acos();
}

fn main() {
    // どうあがいても通らなかったので諦めた。
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let v: Vec<f32> = line.unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<f32>().ok())
            .collect();
        if v.len() > 1 {
            let p = points(v);
            let a = distance(&p[1], &p[2]);
            let b = distance(&p[0], &p[2]);
            let c = distance(&p[0], &p[1]);
            let sin_2a = (angle(b, c, a) * 2.0).sin();
            let sin_2b = (angle(c, a, b) * 2.0).sin();
            let sin_2c = (angle(a, b, c) * 2.0).sin();
            let x = (sin_2a * p[0].x + sin_2b * p[1].x + sin_2c * p[2].x) /
                (sin_2a + sin_2b + sin_2c);
            let y = (sin_2a * p[0].y + sin_2b * p[1].y + sin_2c * p[2].y) /
                (sin_2a + sin_2b + sin_2c);
            let r = a / (angle(b, c, a)).sin() / 2.0;
            println!("{:.3} {:.3} {:.3}", x, y, r);
        }
    }
}
