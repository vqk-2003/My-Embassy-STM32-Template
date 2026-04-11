let arch = variable::get("arch");

arch = switch arch {
	"cortex-M0, M0+, and M1" => "thumbv6m-none-eabi",
	"Cortex-M4 and M7" => "thumbv7em-none-eabi",
	"Cortex-M4F and M7F" => "thumbv7em-none-eabihf",
	"Cortex-M23" => "thumbv8m.base-none-eabi",
	"Cortex-M33 and M35P" => "thumbv8m.main-none-eabi",
	"Cortex-M33F and M35PF" => "thumbv8m.main-none-eabihf",
	_ => "thumbv7m-none-eabi",
};

variable::set("arch", arch);
