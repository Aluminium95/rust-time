#[link(name = "time", vers = "1.0")];
mod time {

	export getTime, time;

	type tm = {
		mutable tm_sec : ctypes::c_int,
		mutable tm_min : ctypes::c_int,
		mutable tm_hour : ctypes::c_int,
		mutable tm_mday : ctypes::c_int,
		mutable tm_mon : ctypes::c_int,
		mutable tm_yday : ctypes::c_int,
		mutable tm_wday : ctypes::c_int,
		mutable tm_year : ctypes::c_int,
		mutable tm_isdst : ctypes::c_int
	};

	type time = {
		sec : int,
		min : int,
			hour : int,
		day : int,
		mon : int,
		year : int
	};

	#[nolink]
	native mod libc {
 		fn time (timer : *()) -> ctypes::long;
		fn localtime (timer : *ctypes::long) -> *tm;
	}

	fn getTime () -> time unsafe {
		let t = libc::time (ptr::null());
		let tm = libc::localtime (ptr::addr_of (t));
		ret {	
			sec : (*tm).tm_sec as int,
			min : (*tm).tm_min as int,
			hour : (*tm).tm_hour as int,
			day : (*tm).tm_mday as int,
			mon : (*tm).tm_mon as int + 1,
			year : (*tm).tm_year as int +1956
		};
	}
}
