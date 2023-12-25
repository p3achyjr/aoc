use std::fs;

fn appx_eq(x: f64, y: f64, rel_tol: f64) -> bool {
    let diff = (x - y).abs();
    let max = x.abs().max(y.abs());

    diff <= rel_tol * max
}

fn simplify_t0_lhs(((c0, v0), (c1, v1)): ((f64, f64), (f64, f64))) -> (f64, f64) {
    let c1 = c1 - c0;
    let c1 = c1 / v0;
    let v1 = v1 / v0;

    (c1, v1)
}

fn solve_for_times(eqn0: ((f64, f64), (f64, f64)), eqn1: ((f64, f64), (f64, f64))) -> (f64, f64) {
    let (xc1, xv1) = simplify_t0_lhs(eqn0);
    let (yc1, yv1) = simplify_t0_lhs(eqn1);

    // the above expressions are:
    // `t0 = xc1 + xv1 * t1`
    // `t0 = yc1 + yv1 * t1`
    //
    // setting them equal, we get:
    // `xc1 + xv1 * t1 = yc1 + yv1 * t1`
    //
    // then solving, we get:
    // `xc1 - yc1 = (yv1 - xv1) * t1` =>
    // `(xc1 - yc1) / (yv1 - xv1) = t1`
    let c = xc1 - yc1;
    let v = yv1 - xv1;
    let t1 = c / v;

    // now that we have t1, we can plug into either equation.
    let t0 = xc1 + xv1 * t1;
    let t0_check = yc1 + yv1 * t1;
    if !appx_eq(t0_check, t0, 1e-6) {
        panic!("Mismatched t0: {:?}, {:?}", t0, t0_check);
    }

    (t0, t1)
}

fn is_parallel((xv0, yv0): (f64, f64), (xv1, yv1): (f64, f64)) -> bool {
    let coeff_x = xv0 / xv1;
    let coeff_y = yv0 / yv1;
    if !appx_eq(coeff_x, coeff_y, 1e-6) {
        false
    } else {
        true
    }
}

fn solve(
    (xc0, yc0): (i64, i64),
    (xv0, yv0): (i64, i64),
    (xc1, yc1): (i64, i64),
    (xv1, yv1): (i64, i64),
) -> Option<(f64, f64)> {
    let eqn0 @ ((xc0, xv0), (xc1, xv1)) = ((xc0 as f64, xv0 as f64), (xc1 as f64, xv1 as f64));
    let eqn1 @ ((yc0, yv0), (yc1, yv1)) = ((yc0 as f64, yv0 as f64), (yc1 as f64, yv1 as f64));

    if is_parallel((xv0, yv0), (xv1, yv1)) {
        return None;
    }

    // solve for t1.
    let (t0, t1) = solve_for_times(eqn0, eqn1);
    if t0 < 0.0 || t1 < 0.0 {
        return None;
    }

    let (x, y) = (xc0 + xv0 * t0, yc0 + yv0 * t0);
    let (x_check, y_check) = (xc1 + xv1 * t1, yc1 + yv1 * t1);

    if !appx_eq(x, x_check, 1e-6) || !appx_eq(y, y_check, 1e-6) {
        panic!(
            "Mismatched Coordinates: {:?}, {:?}",
            (x, y),
            (x_check, y_check)
        );
    }

    Some((x, y))
}

fn main() {
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let mut vectors: Vec<((i64, i64, i64), (i64, i64, i64))> = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("@").collect();
            let c: Vec<i64> = parts[0]
                .split(", ")
                .map(|d_str| d_str.trim().parse::<i64>().unwrap())
                .collect();
            let v: Vec<i64> = parts[1]
                .split(", ")
                .map(|d_str| d_str.trim().parse::<i64>().unwrap())
                .collect();

            ((c[0], c[1], c[2]), (v[0], v[1], v[2]))
        })
        .collect();
    vectors.sort();
    println!("Vectors: {:?}", vectors);

    let mut num_intersects = 0;
    for i in 0..vectors.len() {
        for j in (i + 1)..vectors.len() {
            let ((xc0, yc0, _), (xv0, yv0, _)) = vectors[i];
            let ((xc1, yc1, _), (xv1, yv1, _)) = vectors[j];
            match solve((xc0, yc0), (xv0, yv0), (xc1, yc1), (xv1, yv1)) {
                None => (),
                Some((x_int, y_int)) => {
                    // println!(
                    //     "{:?}, {:?}. Intersect: {:?}",
                    //     vectors[i],
                    //     vectors[j],
                    //     (x_int, y_int)
                    // );

                    if x_int >= MIN && x_int <= MAX && y_int >= MIN && y_int <= MAX {
                        num_intersects += 1;
                    }
                }
            }
        }
    }

    println!("Num Intersects: {}", num_intersects);
}
