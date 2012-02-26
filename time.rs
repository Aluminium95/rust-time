#[link(name = "timelib", vers = "1.0")];
mod time {

	export time_t, time;
	export localtime, gmtime;
	export get_localtime, get_gmtime, get_time;
	
	/**
	 * The tm struct in C
	 */
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
	
	/**
	 * New time struct 
	 */
	type time = {
		sec : int,
		min : int,
		hour : int,
		day : int,
		mon : int,
		year : int,
	};
	
	/** 
	 * The libc time_t type
	 */
	type time_t = ctypes::c_int;
	
	/**
	 * Libc
	 */
	#[nolink]
	native mod libc {
		fn gmtime (timer : *time_t) -> *tm;
 		fn time (timer : *time_t) -> time_t;
		fn localtime (timer : *time_t) -> *tm;
	}

	/**
	 * Binding for libc::time 
	 */
	fn time (timer : *time_t) -> time_t unsafe {
		ret libc::time (timer);
	}

	/**
	 * Simplify time 
	 */
	fn get_time () -> time_t unsafe {
		ret libc::time (ptr::null ());
	}
	
	/**
	 * Binding for libc::gmtime 
	 */
	fn gmtime (timer : time_t) -> time unsafe {
		let tm = libc::gmtime (ptr::addr_of (timer));
		ret tm_to_time (tm);
	}

	/**
	 * Binding for libc::localtime ()
	 */
	fn localtime (timer : time_t) -> time unsafe {
		let tm = libc::localtime (ptr::addr_of (timer));
		ret tm_to_time (tm);
	}
	
	/**
	 * Simplify localtime 
	 */
	fn get_localtime () -> time unsafe {
		let t = time (ptr::null ());
		ret localtime (t);
	}

	fn get_gmtime () -> time unsafe {
		let t = time (ptr::null ());
		ret gmtime (t);
	}
	
	/** 
	 * Convert a libc::tm into time
	 */
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
