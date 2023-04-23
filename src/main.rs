mod pga2d;

fn main() {
    println!("e0*e0         : {}", pga2d::e0 * pga2d::e0);
    println!("pss           : {}", pga2d::e012);
    println!("pss*pss       : {}", pga2d::e012 * pga2d::e012);
}
