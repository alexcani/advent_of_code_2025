use aoc::Context;
use aoc::Point;

fn solve1(red_tiles: &[Point]) -> i64 {
    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let box1 = &red_tiles[i];
            let box2 = &red_tiles[j];
            let width = (box1.x - box2.x).abs() + 1;
            let height = (box1.y - box2.y).abs() + 1;
            let area = width * height;
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

struct Edge {
    p1: Point,
    p2: Point,
}

enum EdgesRelation {
    Collinear,
    Intersect(bool),
}
fn edges_intersect(e1: &Edge, e2: &Edge) -> EdgesRelation {
    let den = (e1.p1.x - e1.p2.x) * (e2.p1.y - e2.p2.y) - (e1.p1.y - e1.p2.y) * (e2.p1.x - e2.p2.x);
    if den == 0 {
        return EdgesRelation::Collinear;
    }
    let t = ((e1.p1.x - e2.p1.x) * (e2.p1.y - e2.p2.y) - (e1.p1.y - e2.p1.y) * (e2.p1.x - e2.p2.x))
        as f64
        / den as f64;
    let u = -((e1.p1.x - e1.p2.x) * (e1.p1.y - e2.p1.y) - (e1.p1.y - e1.p2.y) * (e1.p1.x - e2.p1.x))
        as f64
        / den as f64;

    // Normally we would check 0 <= t <= 1 and 0 <= u <= 1, which means that touching at the endpoints counts as intersection,
    // but for this problem edges touching at the endpoints do not count as intersection.
    EdgesRelation::Intersect(t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0)
}

fn solve2(red_tiles: &[Point]) -> i64 {
    // Build edges
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..(red_tiles.len() - 1) {
        let p1 = red_tiles[i];
        let p2 = red_tiles[i + 1];
        edges.push(Edge { p1, p2 });
    }
    // Final edge wrapping around
    let p1 = red_tiles[red_tiles.len() - 1];
    let p2 = red_tiles[0];
    edges.push(Edge { p1, p2 });

    // Find max area achieved by any pair of points that is fully enclosed by edges
    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let box1 = &red_tiles[i];
            let box2 = &red_tiles[j];
            // If on the same x or y coordinate, skip
            if box1.x == box2.x || box1.y == box2.y {
                continue;
            }

            let width = (box1.x - box2.x).abs() + 1;
            let height = (box1.y - box2.y).abs() + 1;
            let area = width * height;

            // Check if the rectangle defined by box1 and box2 intersects any edges
            // Insert additional edges in the diagonals to check first
            let mut intersects = false;
            let rect_edges = vec![
                // Diagonal edges
                Edge {
                    p1: Point::new(box1.x, box1.y),
                    p2: Point::new(box2.x, box2.y),
                },
                Edge {
                    p1: Point::new(box1.x, box2.y),
                    p2: Point::new(box2.x, box1.y),
                },
                // Rectangle edges
                Edge {
                    p1: Point::new(box1.x, box1.y),
                    p2: Point::new(box2.x, box1.y),
                },
                Edge {
                    p1: Point::new(box2.x, box1.y),
                    p2: Point::new(box2.x, box2.y),
                },
                Edge {
                    p1: Point::new(box2.x, box2.y),
                    p2: Point::new(box1.x, box2.y),
                },
                Edge {
                    p1: Point::new(box1.x, box2.y),
                    p2: Point::new(box1.x, box1.y),
                },
            ];
            'outer: for re in &rect_edges {
                for e in &edges {
                    match edges_intersect(re, e) {
                        EdgesRelation::Collinear => continue,
                        EdgesRelation::Intersect(true) => {
                            intersects = true;
                            break 'outer;
                        }
                        EdgesRelation::Intersect(false) => continue,
                    }
                }
            }

            if !intersects && area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

pub fn solve(ctx: &mut Context) {
    let red_tiles: Vec<Point> = ctx
        .input()
        .iter()
        .map(|l| {
            let mut split = l.split(',');
            let x = split.next().unwrap().trim().parse::<i64>().unwrap();
            let y = split.next().unwrap().trim().parse::<i64>().unwrap();
            Point::new(x, y)
        })
        .collect();

    ctx.set_sol1(solve1(&red_tiles));
    ctx.set_sol2(solve2(&red_tiles));
}
