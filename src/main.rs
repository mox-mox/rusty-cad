#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate rusty_scad;
use rusty_scad::*;

const MATTRESS_LENGTH   : f64 = 200.0;
const STORAGE_LENGTH    : f64 =  41.0;
const MATTRESS_WIDTH    : f64 = 140.0;

const FRAME_HEIGHT      : f64 =  20.0;
const FRAME_WIDTH       : f64 =   3.0;

const BED_LENGTH        : f64 = MATTRESS_LENGTH+STORAGE_LENGTH+3.0*FRAME_WIDTH;
const BED_WIDTH         : f64 = MATTRESS_WIDTH+2.0*FRAME_WIDTH;

//{{{
pub fn sideboard(name: &str, notches: i32) -> Object
{
	let mut board = cube(&(String::from("base board for ")+name), FRAME_WIDTH, BED_LENGTH, FRAME_HEIGHT);

	let mut parts = vec![board];

	//{{{ Cut out the end dovetails

	let mut cutout = cube(&(String::from("end cutout for ")+name), FRAME_WIDTH*2.0, FRAME_WIDTH*2.0, FRAME_HEIGHT/(notches as f64));
	//cutout.set_debug();

	//{{{ Make sure the cutout overlaps the board edge
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate(0.0, BED_LENGTH/2.0, FRAME_HEIGHT/2.0);
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(0.0, -BED_LENGTH/2.0, FRAME_HEIGHT/2.0);
		parts.push(cutout2);
	}
	//}}}

	cutout.translate_z(FRAME_HEIGHT*(1.0/(notches as f64)-1.0)/2.0);
	for i in (1..notches).step_by(2)
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate(0.0, BED_LENGTH/2.0, (i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(0.0, -BED_LENGTH/2.0, (i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout2);
	}
	//}}}


	//{{{ Cut out the notches for the bulkhead

	let mut cutout = cube(&(String::from("bulkhead cutout for ")+name), FRAME_WIDTH, FRAME_WIDTH, FRAME_HEIGHT/(notches as f64));
	cutout.translate(1.0, -BED_LENGTH/2.0+200.0+FRAME_WIDTH*1.5, -FRAME_HEIGHT/2.0);
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
	let mut board = cube(&(String::from("base board for ")+name), BED_WIDTH, FRAME_WIDTH, FRAME_HEIGHT);

	let mut parts = vec![board];

	//{{{ Cut out the end dovetails

	let mut cutout = cube(&(String::from("end cutout for ")+name), FRAME_WIDTH*2.0, FRAME_WIDTH*2.0, FRAME_HEIGHT/(notches as f64));
	//cutout.set_debug();

	//{{{ Make sure the cutout overlaps the board edge
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate( BED_WIDTH/2.0, 0.0, -FRAME_HEIGHT/2.0);
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(-BED_WIDTH/2.0, 0.0, -FRAME_HEIGHT/2.0);
		parts.push(cutout2);
	}
	//}}}

	cutout.translate_z(-FRAME_HEIGHT*(1.0/(notches as f64)-1.0)/2.0);
	for i in (1..notches).step_by(2)
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate( BED_WIDTH/2.0, 0.0, -(i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(-BED_WIDTH/2.0, 0.0, -(i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout2);
	}
	//}}}

	let mut board = difference(name, parts);

	board
}
//}}}

//{{{
pub fn bulkhead(name: &str, notches: i32) -> Object
{
	let mut board = cube(&(String::from("base board for ")+name), BED_WIDTH+2.0*FRAME_WIDTH, FRAME_WIDTH, FRAME_HEIGHT);

	let mut parts = vec![board];

	//{{{ Cut out the end dovetails

	let mut cutout = cube(&(String::from("end cutout for ")+name), FRAME_WIDTH*2.0, FRAME_WIDTH*2.0, FRAME_HEIGHT/(notches as f64));
	//cutout.set_debug();

	//{{{ Make sure the cutout overlaps the board edge
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate( BED_WIDTH/2.0+FRAME_WIDTH, 0.0, -FRAME_HEIGHT/2.0);
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(-BED_WIDTH/2.0-FRAME_WIDTH, 0.0, -FRAME_HEIGHT/2.0);
		parts.push(cutout2);
	}
	//}}}

	cutout.translate_z(-FRAME_HEIGHT*(1.0/(notches as f64)-1.0)/2.0);
	for i in (1..notches).step_by(2)
	{
		let mut cutout1 = cutout.clone();
		cutout1.translate( BED_WIDTH/2.0+FRAME_WIDTH, 0.0, -(i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout1);

		let mut cutout2 = cutout.clone();
		cutout2.translate(-BED_WIDTH/2.0-FRAME_WIDTH, 0.0, -(i as f64)*FRAME_HEIGHT/(notches as f64));
		parts.push(cutout2);
	}
	//}}}

	let mut board = difference(name, parts);

	board
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
	eprintln!("FRAME_WIDTH     = {}", FRAME_WIDTH    );
	//}}}

	let mut sideboard_l = sideboard("Sideboard", 6);
	sideboard_l.translate_x(-(BED_WIDTH-FRAME_WIDTH)/2.0);
	sideboard_l.set_colour(colour_named("red"));
	println!("{}", sideboard_l);

	let mut sideboard_r = sideboard_l.clone();
	sideboard_r.scale_x(-1.0);
	println!("{}", sideboard_r);

	let mut headboard = frontboard("Headboard 1", 6);
	headboard.translate_y(-(200.0+FRAME_WIDTH)/2.0);
	headboard.set_colour(colour_named("green"));
	//headboard.set_root();
	//headboard.set_debug();
	println!("{}", headboard);

	let mut footboard = frontboard("Footboard 1", 6);
	footboard.translate_y(BED_LENGTH-100.0-1.5*FRAME_WIDTH);
	footboard.set_colour(colour_named("green"));
	println!("{}", footboard);


}
