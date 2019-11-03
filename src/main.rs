#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//extern crate rusty_scad;
use rusty_scad::*;

//{{{ Design Constants

const MATTRESS_LENGTH         : f64 = 200.0;
const STORAGE_LENGTH          : f64 =  41.0;
const MATTRESS_WIDTH          : f64 = 140.0;

const NOTCHES                 : i32 =     6;

//                                    Rounded   400kg   500kg
const MOTOR_WIDTH             : f64 =  50.0; // 41.8; // 48.2
const MOTOR_HEIGHT            : f64 =  16.0; // 14.7; // 18.0
const MOTOR_DEPTH             : f64 =  25.0; // 18.8; // 27.0
const FRAME_HEIGHT            : f64 =  20.0;
const FRAME_THICKNESS         : f64 =   3.0;

const COVER_THICKNESS         : f64 =   0.5;
const COVER_GROOVE_DEPTH      : f64 =   0.5;
const CABLE_HOUSING_HEIGHT    : f64 =   3.0;

const FRAME_SLAT_COUNT        : i32 =    10;
const FRAME_SLAT_WIDTH        : f64 =  10.0;
const FRAME_SLAT_THICKNESS    : f64 =   4.0;
const FRAME_SLAT_SPACING      : f64 = (MATTRESS_LENGTH-(FRAME_SLAT_COUNT as f64)*FRAME_SLAT_WIDTH)/((FRAME_SLAT_COUNT as f64)-0.5);

const BOTTOM_COVER_BOTTOM     : f64 = -0.5*FRAME_HEIGHT;
const MIDDLE_COVER_BOTTOM     : f64 = -0.5*FRAME_HEIGHT+COVER_THICKNESS+CABLE_HOUSING_HEIGHT;

const BED_LENGTH              : f64 = MATTRESS_LENGTH+STORAGE_LENGTH+3.0*FRAME_THICKNESS;
const BED_WIDTH               : f64 = MATTRESS_WIDTH+2.0*FRAME_THICKNESS;

// The mattress will be centered
const FOOT_END                : f64 = -MATTRESS_LENGTH/2.0;
const HEAD_END                : f64 =  MATTRESS_LENGTH/2.0+(BED_LENGTH-MATTRESS_LENGTH)-2.0*FRAME_THICKNESS;

const ROLL_WIDTH              : f64 =  0.7;
const ROLL_DIAMETER           : f64 =  2.5;

const DRILL_INSET             : f64 =  1.0;
const DRILL_DEPTH             : f64 = 18.0;
const DRILL_BORE_MINOR        : f64 =  1.0;
const DRILL_BORE_MAJOR        : f64 =  1.0;
const DRILL_MID_MINOR         : f64 =   DRILL_INSET + 0.5*DRILL_BORE_MINOR;
const DRILL_MID_MAJOR         : f64 =   DRILL_INSET + 0.5*DRILL_BORE_MAJOR;
const DRILL_BOTTOM            : f64 = -(DRILL_DEPTH - 0.5*FRAME_HEIGHT);
//}}}





//{{{
pub fn dovetails(name: &str, bottom_stage: f64) -> Vec<Object3D>
{
	let mut parts = vec![];

	//{{{ Add the end dovetails

	let dove_height = FRAME_HEIGHT/(NOTCHES as f64);
	let dove_base_z = -FRAME_HEIGHT/2.0+bottom_stage*dove_height;

	for i in (0..NOTCHES).step_by(2)
	{
		parts.push(cube_coords(&(String::from("end dovetail for ")+name),
			-FRAME_THICKNESS/2.0,
			-FRAME_THICKNESS/2.0,
			dove_base_z+(i as f64)*dove_height,

			 FRAME_THICKNESS/2.0,
			 FRAME_THICKNESS/2.0,
			dove_base_z+((i+1) as f64)*dove_height));
	}
	//}}}

	parts
}
//}}}

//{{{ The Grooves for the cover boards
// TODO: Maybe remove these.
// Rationale: It should be easy to remove the covers. So they should simply be screwed in place
//{{{
pub fn cover_cutouts_front(name: &str, y: f64) -> Vec<Object3D>
{
	let mut parts = vec![];

	//{{{
	parts.push(cube_coords(&(String::from("bottom cover groove for ")+name),
		-MATTRESS_WIDTH/2.0-0.5,
		y-COVER_GROOVE_DEPTH,
		BOTTOM_COVER_BOTTOM,

		 MATTRESS_WIDTH/2.0+0.5,
		y+COVER_GROOVE_DEPTH,
		BOTTOM_COVER_BOTTOM+COVER_THICKNESS,
	));
	//}}}
	//{{{
	parts.push(cube_coords(&(String::from("left side middle cover groove for ")+name),
		-MATTRESS_WIDTH/2.0-0.5,
		y-COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM,

		-MOTOR_WIDTH/2.0-FRAME_THICKNESS+COVER_GROOVE_DEPTH,
		y+COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
	));
	//}}}
	//{{{
	parts.push(cube_coords(&(String::from("right side middle cover groove for ")+name),
		 MATTRESS_WIDTH/2.0+0.5,
		y-COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM,

		 MOTOR_WIDTH/2.0+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
		y+COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
	));
	//}}}
	//{{{
	parts.push(cube_coords(&(String::from("middle side middle cover groove for ")+name),
		-MOTOR_WIDTH/2.0-COVER_GROOVE_DEPTH,
		y-COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM,

		 MOTOR_WIDTH/2.0+COVER_GROOVE_DEPTH,
		y+COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
	));
	//}}}

	parts
}
//}}}

//{{{
pub fn cover_cutouts_side(name: &str, x: f64) -> Vec<Object3D>
{
	let mut parts = vec![];

	//{{{
	parts.push(cube_coords(&(String::from("middle cover groove for ")+name),
		x-COVER_GROOVE_DEPTH,
		HEAD_END+COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM,

		x+COVER_GROOVE_DEPTH,
		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
	));
	//}}}
	//{{{
	parts.push(cube_coords(&(String::from("bottom cover groove for ")+name),
		x-COVER_GROOVE_DEPTH,
		HEAD_END+COVER_GROOVE_DEPTH,
		BOTTOM_COVER_BOTTOM,

		x+COVER_GROOVE_DEPTH,
		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
		BOTTOM_COVER_BOTTOM+COVER_THICKNESS,
	));
	//}}}

	parts
}
//}}}
//}}}

//{{{
pub fn drill_minor(name: &str) -> Vec<Object3D>
{
	let mut parts = vec![];

	let mut drill = cylinder(&(String::from("minor vertical drill for ")+name), DRILL_DEPTH+1.0, 0.5*DRILL_BORE_MINOR, 0.5*DRILL_BORE_MINOR);
	drill.translate_z(DRILL_BOTTOM);
	drill.set_fn(20);
	//drill.set_debug();
	parts.push(drill);

	parts
}
//}}}


//{{{
pub fn sideboard(name: &str) -> Object3D
{
	//{{{
	let board = cube_coords(&(String::from("base board for ")+name),
		-FRAME_THICKNESS/2.0,
		FOOT_END,
		-FRAME_HEIGHT/2.0,
		FRAME_THICKNESS/2.0,
		HEAD_END,
		FRAME_HEIGHT/2.0);
	//}}}

	//{{{ Add the end dovetails

	let mut parts = vec![board];
	{
		let mut dovetails_head = dovetails(name, 0.0);
		for dovetail in &mut dovetails_head { dovetail.translate_y(HEAD_END+0.5*FRAME_THICKNESS); }
		parts.append(&mut dovetails_head);

		let mut dovetails_foot = dovetails(name, 0.0);
		for dovetail in &mut dovetails_foot { dovetail.translate_y(FOOT_END-0.5*FRAME_THICKNESS); }
		parts.append(&mut dovetails_foot);
	}
	let mut board = union(name, parts);
	//}}}

	let mut parts = vec![board];
	//{{{ Cut out the notches for the bulkhead
	{
		let mut bulkhead_cutouts = dovetails(name, 0.5);
		for dovetail in &mut bulkhead_cutouts { dovetail.translate(2.0, MATTRESS_LENGTH/2.0+0.5*FRAME_THICKNESS, 0.0); }
		parts.append(&mut bulkhead_cutouts);
	}
	//}}}
	//{{{ Add the cover grooves
	{
		parts.append(&mut cover_cutouts_side(name, FRAME_THICKNESS/2.0));
	}
	//}}}
	//{{{ Add the drills
	{
		let mut drill = drill_minor(name);
		drill[0].translate(-0.5*FRAME_THICKNESS+DRILL_MID_MINOR, HEAD_END+FRAME_THICKNESS-DRILL_MID_MINOR, 0.0);
		parts.append(&mut drill);
	}
	//}}}
	//{{{ Add the drills
	{
		let mut drill = drill_minor(name);
		drill[0].translate(-0.5*FRAME_THICKNESS+DRILL_MID_MINOR, FOOT_END-FRAME_THICKNESS+DRILL_MID_MINOR, 0.0);
		parts.append(&mut drill);
	}
	//}}}
	let mut board = difference(name, parts);

	//{{{ Add some anchors TODO

	let mut a = board.create_anchor("left");
	a.translate_z(11.0);
	a.rel_rotate_y(90.0);
	//}}}

	board
}
//}}}

//{{{
pub fn frontboard(name: &str) -> Object3D
{
	let board = cube(&(String::from("base board for ")+name), BED_WIDTH-2.0*FRAME_THICKNESS, FRAME_THICKNESS, FRAME_HEIGHT);

	//{{{ Add the end dovetails

	let mut parts = vec![board];
	{
		let mut dovetails_left = dovetails(name, 1.0);
		for dovetail in &mut dovetails_left { dovetail.translate_x(-0.5*(MATTRESS_WIDTH+FRAME_THICKNESS)); }
		parts.append(&mut dovetails_left);

		let mut dovetails_right = dovetails(name, 1.0);
		for dovetail in &mut dovetails_right { dovetail.translate_x(0.5*(MATTRESS_WIDTH+FRAME_THICKNESS)); }
		parts.append(&mut dovetails_right);
	}
	let mut board = union(name, parts);
	//}}}

	board
}
//}}}

//{{{
pub fn headboard(name: &str) -> Object3D
{
	let mut board = frontboard(name);
	let mut parts = vec![board];

	//{{{ Cut out the notches for the bulkhead spacer
	{
		let mut bulkhead_cutouts = dovetails(name, 0.5);
		for dovetail in &mut bulkhead_cutouts { dovetail.translate(-0.5*(MOTOR_WIDTH+FRAME_THICKNESS), -2.0, 0.0); }
		parts.append(&mut bulkhead_cutouts);
	}
	{
		let mut bulkhead_cutouts = dovetails(name, 0.5);
		for dovetail in &mut bulkhead_cutouts { dovetail.translate( 0.5*(MOTOR_WIDTH+FRAME_THICKNESS), -2.0, 0.0); }
		parts.append(&mut bulkhead_cutouts);
	}
	//}}}
	//{{{ Add the cover grooves

	parts.append(&mut cover_cutouts_front(name, -FRAME_THICKNESS/2.0));
	//}}}
	//{{{ Add the drills
	{
		let mut drill = drill_minor(name);
		drill[0].translate(-0.5*BED_WIDTH+DRILL_MID_MINOR, 0.5*FRAME_THICKNESS-DRILL_MID_MINOR, 0.0);
		parts.append(&mut drill);
	}
	//}}}
	//{{{ Add the drills
	{
		let mut drill = drill_minor(name);
		drill[0].translate( 0.5*BED_WIDTH-DRILL_MID_MINOR, 0.5*FRAME_THICKNESS-DRILL_MID_MINOR, 0.0);
		parts.append(&mut drill);
	}
	//}}}
	let mut board = difference(name, parts);

	board
}
//}}}

//{{{
pub fn bulkhead(name: &str) -> Object3D
{
	let board = cube(&(String::from("base board for ")+name), MATTRESS_WIDTH, FRAME_THICKNESS, FRAME_HEIGHT);

	//{{{ Add the end dovetails

	let mut parts = vec![board];
	{
		let mut dovetails_left = dovetails(name, 0.5);
		for dovetail in &mut dovetails_left { dovetail.translate_x(-(0.5*(MATTRESS_WIDTH+FRAME_THICKNESS)-2.0)); }
		parts.append(&mut dovetails_left);

		let mut dovetails_right = dovetails(name, 0.5);
		for dovetail in &mut dovetails_right { dovetail.translate_x(0.5*(MATTRESS_WIDTH+FRAME_THICKNESS)-2.0); }
		parts.append(&mut dovetails_right);
	}
	let mut board = union(name, parts);
	//}}}

	let mut parts = vec![board];
	//{{{ Cut out the notches for the bulkhead spacer
	{
		let mut bulkhead_cutouts = dovetails(name, 0.5);
		for dovetail in &mut bulkhead_cutouts { dovetail.translate(-0.5*(MOTOR_WIDTH+FRAME_THICKNESS), 2.0, 0.0); }
		parts.append(&mut bulkhead_cutouts);
	}
	{
		let mut bulkhead_cutouts = dovetails(name, 0.5);
		for dovetail in &mut bulkhead_cutouts { dovetail.translate( 0.5*(MOTOR_WIDTH+FRAME_THICKNESS), 2.0, 0.0); }
		parts.append(&mut bulkhead_cutouts);
	}
	//}}}
	//{{{ Add the cover grooves
	{
		let mut cover_cutouts = cover_cutouts_front(name, FRAME_THICKNESS/2.0);
		cover_cutouts.pop();
		parts.append(&mut cover_cutouts);
	}
	//}}}
	let mut board = difference(name, parts);

	board
}
//}}}

//{{{
pub fn bulkhead_spacer(name: &str) -> Object3D
{
	//{{{
	let board = cube_coords(&(String::from("base board for ")+name),
	-0.5*FRAME_THICKNESS,
	HEAD_END,
	BOTTOM_COVER_BOTTOM+COVER_THICKNESS,

	0.5*FRAME_THICKNESS,
	0.5*MATTRESS_LENGTH+FRAME_THICKNESS,
	0.5*FRAME_HEIGHT);
	//}}}


	//{{{ Add the end dovetails

	let mut parts = vec![board];
	{
		let mut dovetails_head = dovetails(name, 0.5);
		for dovetail in &mut dovetails_head { dovetail.translate_y(HEAD_END+0.5*FRAME_THICKNESS-2.0); }
		parts.append(&mut dovetails_head);

		let mut dovetails_foot = dovetails(name, 0.5);
		for dovetail in &mut dovetails_foot { dovetail.translate_y(0.5*MATTRESS_LENGTH+0.5*FRAME_THICKNESS+2.0); }
		parts.append(&mut dovetails_foot);
	}
	let mut board = union(name, parts);
	//}}}


	//{{{ Add the cover grooves
	let mut parts = vec![board];
	{
		let mut cover_cutouts = cover_cutouts_side(name, -FRAME_THICKNESS/2.0);
		cover_cutouts.pop();
		parts.append(&mut cover_cutouts);
	}
	{
		let mut cover_cutouts = cover_cutouts_side(name,  FRAME_THICKNESS/2.0);
		cover_cutouts.pop();
		parts.append(&mut cover_cutouts);
	}
	let mut board = difference(name, parts);
	//}}}

	board
}
//}}}

//{{{
pub fn footboard(name: &str) -> Object3D
{
	frontboard(name)
}
//}}}

//{{{
pub fn frame_slat(name: &str) -> Object3D
{
	//{{{
	let board = cube_coords(&(String::from("base board for ")+name),
		-0.5*MATTRESS_WIDTH,
		-0.5*FRAME_SLAT_WIDTH,
		-0.5*FRAME_SLAT_THICKNESS,

		 0.5*MATTRESS_WIDTH,
		 0.5*FRAME_SLAT_WIDTH,
		 0.5*FRAME_SLAT_THICKNESS);
	//}}}

	board
}
//}}}


//{{{
pub fn small_roll(name: &str) -> Object3D
{
	let mut roll1 = cylinder(&(String::from("Lower half roll for ") + name), 0.5*ROLL_WIDTH, 0.4*ROLL_DIAMETER, 0.5*ROLL_DIAMETER);
	roll1.scale_z(-1.0);
	let roll2 = cylinder(&(String::from("Upper half roll for ") + name), 0.5*ROLL_WIDTH, 0.4*ROLL_DIAMETER, 0.5*ROLL_DIAMETER);
	let mut roll = union(name, [roll1, roll2]);
	roll.set_fn(20);

	//{{{ Add some anchors TODO

	{
		roll.create_anchor("Origin");
	}
	{
		let mut a = roll.create_anchor("Contact");
		a.translate(-0.5*ROLL_DIAMETER, -0.5*ROLL_DIAMETER, 0.0);
	}

	//}}}

	roll
}
//}}}

//{{{
pub fn sprenger_block_3511100355(name: &str) -> Object3D
{
	const ROLL_DIAMETER           : f64 =  2.5;
	const ROLL_WIDTH              : f64 =  0.7;
	const ROLL_HEIGHT             : f64 =  1.7;


	const BLOCK_WIDTH             : f64 = 1.02;
	const BLOCK_DIAMETER          : f64 =  2.5;
	const BLOCK_HEIGHT            : f64 =  3.38;
	const BLOCK_HEIGHT_SLANT      : f64 =  2.2;
	const BASE_WIDTH              : f64 =  3.5;

	let block = cube_coords(&(String::from("base board for ")+name),
		-0.5*BASE_WIDTH,
		-0.5*BLOCK_DIAMETER,
		0.0,
		 0.5*BASE_WIDTH,
		 0.5*BLOCK_DIAMETER,
		BLOCK_HEIGHT);

	let mut parts = vec![block];
	{
		let mut c1=cylinder(&(String::from("base board for ")+name), BLOCK_DIAMETER+2.0, 0.175, 0.175);
		c1.rotate_x(90.0);
		c1.translate(-0.51-0.175, 1.0+0.5*BLOCK_DIAMETER, 0.175+0.13);
		c1.set_debug();
		parts.push(c1);
	}
	{
		let mut c1=cylinder(&(String::from("base board for ")+name), BLOCK_DIAMETER+2.0, 0.175, 0.175);
		c1.rotate_x(90.0);
		c1.translate(0.51+0.175, 1.0+0.5*BLOCK_DIAMETER, 0.175+0.13);
		c1.set_debug();
		parts.push(c1);
	}
	{
		let mut c1=cylinder(&(String::from("base board for ")+name), BLOCK_DIAMETER+2.0, 0.175, 0.175);
		c1.rotate_x(90.0);
		c1.translate(0.51+0.175, 1.0+0.5*BLOCK_DIAMETER, 0.175+0.13);
		c1.set_debug();
		parts.push(c1);
	}




	let mut board = difference(name, parts);
	board.set_fn(20);

	board
}
//}}}

fn main()
{



	////{{{ Print all the constants

	//eprintln!("MATTRESS_LENGTH = {}", MATTRESS_LENGTH);
	//eprintln!("STORAGE_LENGTH  = {}", STORAGE_LENGTH );
	//eprintln!("MATTRESw_WIDTH  = {}", MATTRESS_WIDTH );
	//eprintln!("BED_LENGTH      = {}", BED_LENGTH     );
	//eprintln!("BED_WIDTH       = {}", BED_WIDTH      );
	//eprintln!("FRAME_HEIGHT    = {}", FRAME_HEIGHT   );
	//eprintln!("FRAME_THICKNESS     = {}", FRAME_THICKNESS    );
	////}}}

	//{{{
	let mut sideboard_l = sideboard("Sideboard_L");
	sideboard_l.translate_x(-(BED_WIDTH-FRAME_THICKNESS)/2.0);
	sideboard_l.set_colour(colour_named("red"));
	sideboard_l.set_show_anchors();
	println!("{}", sideboard_l);
	//}}}

	//{{{
	let mut sideboard_r = sideboard_l.clone();
	sideboard_r.name = String::from("Sideboard_R");
	sideboard_r.scale_x(-1.0);
	println!("{}", sideboard_r);
	//}}}

	//{{{
	let mut headboard = headboard("Headboard");
	headboard.translate_y(BED_LENGTH-100.0-1.5*FRAME_THICKNESS);
	headboard.set_colour(colour_named("green"));
	println!("{}", headboard);
	//}}}

	//{{{
	let mut footboard = footboard("Footboard");
	footboard.translate_y(-(200.0+FRAME_THICKNESS)/2.0);
	footboard.set_colour(colour_named("green"));
	println!("{}", footboard);
	//}}}

	//{{{
	let mut bulkhead = bulkhead("Bulkhead");
	bulkhead.translate_y((MATTRESS_LENGTH+FRAME_THICKNESS)/2.0);
	bulkhead.set_colour(colour_named("yellow"));
	println!("{}", bulkhead);
	//}}}

	//{{{
	let mut bottom_cover = cube_coords("Bottom cover",
		-(0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH),
		HEAD_END+COVER_GROOVE_DEPTH,
		BOTTOM_COVER_BOTTOM,

		0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH,
		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
		BOTTOM_COVER_BOTTOM+COVER_THICKNESS,
		);
	bottom_cover.set_colour(colour_named("blue"));
	println!("{}", bottom_cover);
	//}}}

	//{{{
	let mut middle_cover_l = cube_coords("Bottom cover",
		-(0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH),
		HEAD_END+COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM,

		-0.5*MOTOR_WIDTH-FRAME_THICKNESS+COVER_GROOVE_DEPTH,
		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
		);
	middle_cover_l.set_colour(colour_named("blue"));
	println!("{}", middle_cover_l);
	//}}}

	//{{{
	let mut middle_cover_r = cube_coords("Bottom cover",
		0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH,
		HEAD_END+COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM,

		0.5*MOTOR_WIDTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
		);
	middle_cover_r.set_colour(colour_named("blue"));
	println!("{}", middle_cover_r);
	//}}}

	//{{{
	let mut bulkhead_spacer_l = bulkhead_spacer("Bulkhead spacer L");
	bulkhead_spacer_l.translate_x(-0.5*(MOTOR_WIDTH+FRAME_THICKNESS));
	println!("{}", bulkhead_spacer_l);
	//}}}

	//{{{
	let mut bulkhead_spacer_r = bulkhead_spacer_l.clone();
	bulkhead_spacer_r.name = String::from("Bulkhead spacer R");
	bulkhead_spacer_r.scale_x(-1.0);
	println!("{}", bulkhead_spacer_r);
	//}}}

	//{{{ Slat Frame

	for i in 0..FRAME_SLAT_COUNT
	{
		let mut slat = frame_slat(&format!("Frame slat {}", i));
		slat.translate(0.0, -0.5*(MATTRESS_LENGTH-FRAME_SLAT_WIDTH)+((2*i) as f64)*FRAME_SLAT_SPACING, -0.5*(FRAME_HEIGHT-FRAME_SLAT_THICKNESS));
		println!("{}", slat);
	}




	//}}}

	//{{{
	let mut roll = small_roll("tester");
	roll.anchor("Contact").snap_to(&mut sideboard_l.anchor("left"));
	println!("{}", roll);
	//}}}


	//{{{
	let mut block = sprenger_block_3511100355("tester");
	//println!("{}", block);
	//}}}
	////{{{
	//let mut pipe = pipe("tester", 3.0, 2.0, 1.0);
	//pipe.set_fn(20);
	//println!("{}", pipe);
	////}}}
	////{{{
	//let mut wedge = wedge("tester", 3.0, 2.0, 45.0);
	//println!("{}", wedge);
	////}}}

	//{{{
	let mut pipe = pipe_cut("tester", 3.0, 2.0, 1.0, 185.0);
	pipe.set_fn(50);
	println!("{}", pipe);
	//}}}
}
