fn main(){
	
let p = 210_000.00;
let r = 5.00;
let t = 3.00;
/*
p= principal
r=rate
t=time
*/
// formula for depriciation is A = P[1 - (R/100)]^t
let a:f64 = r / 100.00;
let b:f64 = 1.00-a;
let c:f64= b.powf(t);
let d = c * p;
println!("the total depriciation of the tv is {}",d);
// this would print the answer

}