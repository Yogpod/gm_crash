#![feature(c_unwind)]
#[macro_use]
extern crate gmod;
use std::process;

#[lua_function]
unsafe fn kill_server(lua: gmod::lua::State) -> i32 {
	lua.push_globals();

    lua.get_global(lua_string!("Msg"));
    lua.push_string("[gm_crash] Killing server...\n");
    lua.call(1, 0);
    lua.pop();

    lua.get_global(lua_string!("hook"));
    lua.get_field(-1, lua_string!("Run"));
    lua.push_string("ShutDown");
    lua.call(1, 0);
    lua.pop();

    lua.pop();

	unsafe { std::ptr::null_mut::<i32>().write(42) };

	0
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	lua.push_globals();
	lua.get_field(-1, lua_string!("game"));
	lua.push_string("KillServer");
	lua.push_function(kill_server);
	lua.set_table(-3);
	lua.pop();

	lua.pop();
	0
}

#[gmod13_close]
unsafe fn gmod13_close(lua: gmod::lua::State) -> i32 {
	lua.push_globals();
	lua.get_field(-1, lua_string!("game"));
	lua.push_string("KillServer");
	lua.push_nil();
	lua.set_table(-3);
	lua.pop();

	lua.pop();
	0
}
