#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate rusty_scad;
use rusty_scad::*;

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

const BOTTOM_COVER_BOTTOM     : f64 = -0.5*FRAME_HEIGHT+0.5;
const MIDDLE_COVER_BOTTOM     : f64 = -0.5*FRAME_HEIGHT+0.5+COVER_THICKNESS+CABLE_HOUSING_HEIGHT;

const BED_LENGTH              : f64 = MATTRESS_LENGTH+STORAGE_LENGTH+3.0*FRAME_THICKNESS;
const BED_WIDTH               : f64 = MATTRESS_WIDTH+2.0*FRAME_THICKNESS;

// The mattress will be centered
const FOOT_END                : f64 = -MATTRESS_LENGTH/2.0;
const HEAD_END                : f64 =  MATTRESS_LENGTH/2.0+(BED_LENGTH-MATTRESS_LENGTH)-2.0*FRAME_THICKNESS;


//{{{
pub fn dovetails(name: &str, bottom_stage: f64) -> Vec<Object>
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
pub fn cover_cutouts_front(name: &str, y: f64) -> Vec<Object>
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
pub fn cover_cutouts_side(name: &str, x: f64) -> Vec<Object>
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
pub fn sideboard(name: &str) -> Object
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
	let board = difference(name, parts);

	board
}
//}}}

//{{{
pub fn frontboard(name: &str) -> Object
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
pub fn headboard(name: &str) -> Object
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
	let mut board = difference(name, parts);

	board
}
//}}}

//{{{
pub fn bulkhead(name: &str) -> Object
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
pub fn bulkhead_spacer(name: &str) -> Object
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
pub fn footboard(name: &str) -> Object
{
	frontboard(name)
}
//}}}

//{{{
pub fn frame_slat(name: &str) -> Object
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

}
