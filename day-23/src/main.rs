use scan_fmt::*;

fn main() {
    let nanobots = parse(include_str!("input"));
    let in_range = in_range(&nanobots);
    let center = center(&nanobots);
    println!("Answer p1: {}", in_range);
    println!("Answer p2: {:?}", center);
}

type Nanobot = (i64,i64,i64,i64);

fn in_range(nanobots: &Vec<Nanobot>) -> usize {
    let &(x0,y0,z0,r0) = nanobots.iter().max_by_key(|(_,_,_,r)| r).unwrap();
    nanobots
        .iter()
        .filter(|&&(x1,y1,z1,_)| {
            let dist = (x1 - x0).abs() + (y1 - y0).abs() + (z1 - z0).abs();
            if dist <= r0 { true } else { false }
        })
        .count()
}

use z3::*;

fn center(nanobots: &Vec<Nanobot>) -> i64 {

    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let cx = ctx.named_int_const("x");
    let cy = ctx.named_int_const("y");
    let cz = ctx.named_int_const("z");
    let mut num_in_range = ctx.from_i64(0);

    for &(x,y,z,r) in nanobots {

        let x = ctx.from_i64(x);
        let y = ctx.from_i64(y);
        let z = ctx.from_i64(z);
        let r = ctx.from_i64(r);

        let dx = x.sub(&[&cx]);
        let dy = y.sub(&[&cy]);
        let dz = z.sub(&[&cz]);

        let dx_abs = dx.ge(&ctx.from_i64(0)).ite(&dx, &dx.minus());
        let dy_abs = dy.ge(&ctx.from_i64(0)).ite(&dy, &dy.minus());
        let dz_abs = dz.ge(&ctx.from_i64(0)).ite(&dz, &dz.minus());

        let manhattan = dx_abs.add(&[&dy_abs,&dz_abs]);
        num_in_range = manhattan
            .le(&r)
            .ite(&ctx.from_i64(1), &ctx.from_i64(0))
            .add(&[&num_in_range]);
    }

    let optimize = Optimize::new(&ctx);
    optimize.maximize(&num_in_range);

    let dx_abs = cx.ge(&ctx.from_i64(0)).ite(&cx, &cx.minus());
    let dy_abs = cy.ge(&ctx.from_i64(0)).ite(&cy, &cy.minus());
    let dz_abs = cz.ge(&ctx.from_i64(0)).ite(&cz, &cz.minus());

    let manhattan = dx_abs.add(&[&dy_abs,&dz_abs]);
    optimize.minimize(&manhattan);

    optimize.check();

    let manhattan = optimize
        .get_model()
        .eval(&manhattan)
        .unwrap()
        .as_i64()
        .unwrap()
        .abs();
    manhattan
}

fn parse(input: &str) -> Vec<Nanobot> {
    input.lines().map(|line| {
        let (x,y,z,r) = scan_fmt!(line, "pos=<{},{},{}>, r={}", i64, i64, i64, i64);
        (x.unwrap(), y.unwrap(), z.unwrap(), r.unwrap())
    }).collect()
}


#[test]
fn test1() {
    let nanobots = parse(include_str!("example_input_1"));
    let in_range = in_range(&nanobots);
    assert_eq!(in_range, 7);
}

#[test]
fn test2() {
    let nanobots = parse(include_str!("example_input_2"));
    let center = center(&nanobots);
    assert_eq!(center, 36)
}
