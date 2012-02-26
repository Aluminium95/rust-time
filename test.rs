use std;
use time;
use timelib;
import timelib::time::*;

fn main () {
	let t = get_time ();
	let time = localtime (t);
}
