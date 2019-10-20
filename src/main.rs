#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate rusty_scad;
use rusty_scad::*;

const MATTRESS_LENGTH         : f64 = 200.0;
const STORAGE_LENGTH          : f64 =  41.0;
const MATTRESS_WIDTH          : f64 = 140.0;

//                                    Rounded   400kg   500kg
const MOTOR_WIDTH             : f64 =  50.0; // 41.8; // 48.2
const MOTOR_HEIGHT            : f64 =  16.0; // 14.7; // 18.0
const MOTOR_DEPTH             : f64 =  25.0; // 18.8; // 27.0
const FRAME_HEIGHT            : f64 =  20.0;
const FRAME_THICKNESS         : f64 =   3.0;

const COVER_THICKNESS         : f64 =   0.5;
const COVER_GROOVE_DEPTH      : f64 =   1.0;
const CABLE_HOUSING_HEIGHT    : f64 =   3.0;

const BED_LENGTH              : f64 = MATTRESS_LENGTH+STORAGE_LENGTH+3.0*FRAME_THICKNESS;
const BED_WIDTH               : f64 = MATTRESS_WIDTH+2.0*FRAME_THICKNESS;


//{{{ Board Templates
//{{{
pub fn sideboard(name: &str, notches: i32) -> Object
{
	let mut board = cube(&(String::from("base board for ")+name), FRAME_THICKNESS, BED_LENGTH-2.0*FRAME_THICKNESS, FRAME_HEIGHT);

	let mut parts = vec![board];

	//{{{ Add the end dovetails

	let mut dovetail = cube(&(String::from("end dovetail for ")+name), FRAME_THICKNESS, FRAME_THICKNESS, FRAME_HEIGHT/(notches as f64));
	//dovetail.set_debug();

	dovetail.translate_z(FRAME_HEIGHT*(1.0-1.0/(notches as f64))/2.0);
	for i in (1..notches).step_by(2)
	{
		let mut dovetail1 = dovetail.clone();
		dovetail1.translate(0.0, BED_LENGTH/2.0-FRAME_THICKNESS/2.0, (-i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(dovetail1);

		let mut dovetail2 = dovetail.clone();
		dovetail2.translate(0.0, -BED_LENGTH/2.0+FRAME_THICKNESS/2.0, (-i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(dovetail2);
	}
	//}}}

	let mut board = union(name, parts);

	let mut parts = vec![board];

	//{{{ Cut out the notches for the bulkhead

	let mut cutout = cube(&(String::from("bulkhead cutout for ")+name), FRAME_THICKNESS, FRAME_THICKNESS, FRAME_HEIGHT/(notches as f64));
	cutout.translate(1.0, -BED_LENGTH/2.0+200.0+FRAME_THICKNESS*1.5, -FRAME_HEIGHT/2.0);
	//cutout.set_debug();

	for i in (1..notches).step_by(2)
	{
		let mut cutout = cutout.clone();
		cutout.translate_z((i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout);
	}
	//}}}


	let mut board = difference(name, parts);

	board.translate_y(BED_LENGTH/2.0-103.0);
	board
}
//}}}

//{{{
pub fn frontboard(name: &str, notches: i32) -> Object
{
	let mut board = cube(&(String::from("base board for ")+name), BED_WIDTH-2.0*FRAME_THICKNESS, FRAME_THICKNESS, FRAME_HEIGHT);

	let mut parts = vec![board];

	//{{{ Add the end dovetails

	let mut cutout = cube(&(String::from("end cutout for ")+name), FRAME_THICKNESS, FRAME_THICKNESS, FRAME_HEIGHT/(notches as f64));
	//cutout.set_debug();

	cutout.translate_z(-FRAME_HEIGHT*(1.0-1.0/(notches as f64))/2.0);
	for i in (1..notches).step_by(2)
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate( BED_WIDTH/2.0-FRAME_THICKNESS/2.0, 0.0, (i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(-BED_WIDTH/2.0+FRAME_THICKNESS/2.0, 0.0, (i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout2);
	}
	//}}}

	let mut board = union(name, parts);

	board
}
//}}}
//}}}

//{{{
pub fn bulkhead(name: &str, notches: i32) -> Object
{
	let mut board = cube(&(String::from("base board for ")+name), BED_WIDTH-2.0*FRAME_THICKNESS, FRAME_THICKNESS, FRAME_HEIGHT);

	let mut parts = vec![board];

	//{{{ Add the end dovetails

	let mut cutout = cube(&(String::from("end cutout for ")+name), FRAME_THICKNESS, FRAME_THICKNESS, FRAME_HEIGHT/(notches as f64));
	//cutout.set_debug();

	cutout.translate_z(-FRAME_HEIGHT*(1.0)/2.0);
	for i in (1..notches).step_by(2)
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate( BED_WIDTH/2.0-FRAME_THICKNESS/2.0-1.0, 0.0, (i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(-BED_WIDTH/2.0+FRAME_THICKNESS/2.0+1.0, 0.0, (i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout2);
	}
	//}}}

	let mut board = union(name, parts);

	board
}
//}}}

//{{{
pub fn headboard(name: &str, notches: i32) -> Object
{
	let mut board = frontboard(name, notches);

	let mut parts = vec![board];

	//{{{ Add the Grooves for the cover boards

	let mut bottom_groove = cube(&(String::from("end cutout for ")+name), MATTRESS_WIDTH+2.0*COVER_GROOVE_DEPTH, FRAME_THICKNESS, COVER_THICKNESS);
	bottom_groove.translate_z(-(FRAME_HEIGHT-COVER_THICKNESS)/2.0+0.5);
	bottom_groove.translate_y(-FRAME_THICKNESS+COVER_GROOVE_DEPTH);
	bottom_groove.set_debug();
	parts.push(bottom_groove);

	let mut upper_groove_l = cube(&(String::from("end cutout for ")+name), MATTRESS_WIDTH, FRAME_THICKNESS, COVER_THICKNESS);
	upper_groove_l.translate_z(-(FRAME_HEIGHT-COVER_THICKNESS)/2.0+0.5+CABLE_HOUSING_HEIGHT+COVER_THICKNESS);
	upper_groove_l.translate_y(-FRAME_THICKNESS+COVER_GROOVE_DEPTH);
	upper_groove_l.set_debug();
	parts.push(upper_groove_l);
	//}}}

	let mut board = difference(name, parts);

	board
}
//}}}

//{{{
pub fn footboard(name: &str, notches: i32) -> Object
{
	frontboard(name, notches)
}
//}}}

fn main()
{
	//{{{ Print all the constants

	eprintln!("MATTRESS_LENGTH = {}", MATTRESS_LENGTH);
	eprintln!("STORAGE_LENGTH  = {}", STORAGE_LENGTH );
	eprintln!("MATTRESw_WIDTH  = {}", MATTRESS_WIDTH );
	eprintln!("BED_LENGTH      = {}", BED_LENGTH     );
	eprintln!("BED_WIDTH       = {}", BED_WIDTH      );
	eprintln!("FRAME_HEIGHT    = {}", FRAME_HEIGHT   );
	eprintln!("FRAME_THICKNESS     = {}", FRAME_THICKNESS    );
	//}}}

	let csys = coordinate_system("tester");
	println!("{}", csys);

	let text = text("tester", "FooBar", "profont", 10, 1.0);
	println!("{}", text);








	let mut sideboard_l = sideboard("Sideboard", 6);
	sideboard_l.translate_x(-(BED_WIDTH-FRAME_THICKNESS)/2.0);
	sideboard_l.set_colour(colour_named("red"));
	println!("{}", sideboard_l);

	let mut sideboard_r = sideboard_l.clone();
	sideboard_r.scale_x(-1.0);
	println!("{}", sideboard_r);

	let mut headboard = headboard("Headboard", 6);
	headboard.translate_y(-(200.0+FRAME_THICKNESS)/2.0);
	headboard.set_colour(colour_named("green"));
	//headboard.set_root();
	println!("{}", headboard);

	let mut footboard = footboard("Footboard", 6);
	footboard.translate_y(BED_LENGTH-100.0-1.5*FRAME_THICKNESS);
	footboard.set_colour(colour_named("green"));
	println!("{}", footboard);

	let mut bulkhead = bulkhead("Bulkhead", 6);
	bulkhead.translate_y((MATTRESS_LENGTH+FRAME_THICKNESS)/2.0);
	bulkhead.set_colour(colour_named("yellow"));
	//bulkhead.set_root();
	println!("{}", bulkhead);

}
