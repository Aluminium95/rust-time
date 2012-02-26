use std;
use timelib;

fn main () {
	let tm = timelib::time::gmtime ();
	std::io::println (
		#fmt (
			"%d:%d:%d [%d/%d/%d]",
			tm.hour, tm.min, tm.sec, 
			tm.day, tm.mon, tm.year
		));
}
