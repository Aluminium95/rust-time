#[link(name = "timelib", vers = "1.0")];
mod time {

	export localtime, time, gmtime;

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
	
	type time_t = ctypes::long;

	#[nolink]
	native mod libc {
 		fn time (timer : *()) -> time_t;
		fn localtime (timer : *time_t) -> *tm;
		fn gmtime (timer : *time_t) -> *tm;
	}

	fn localtime () -> time unsafe {
		let t = libc::time (ptr::null());
		ret tm_to_time (libc::localtime (ptr::addr_of (t)));
	}
	
	fn gmtime () -> time unsafe {
		let t = libc::time (ptr::null ());
		ret tm_to_time (libc::gmtime (ptr::addr_of (t)));
	}
	
	fn tm_to_time (tm : *tm) -> time unsafe {
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
