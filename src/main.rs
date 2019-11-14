#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//extern crate rusty_scad;
use rusty_scad::*;
use std::f64::consts::PI;

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

const FRAME_SLAT_COUNT        : i32 =    20;
const FRAME_SLAT_WIDTH        : f64 =   5.0;
const FRAME_SLAT_THICKNESS    : f64 =   3.0;
const FRAME_SLAT_SPACING      : f64 = (MATTRESS_LENGTH-(FRAME_SLAT_COUNT as f64)*FRAME_SLAT_WIDTH)/((FRAME_SLAT_COUNT as f64)-0.5);
const FRAME_SLAT_START        : f64 = 0.5*(MATTRESS_LENGTH-FRAME_SLAT_WIDTH);

const FOOT_CABLE_HEIGHT       : f64 = -0.5*FRAME_HEIGHT + 0.5*FRAME_SLAT_THICKNESS;
const HEAD_CABLE_HEIGHT       : f64 = FOOT_CABLE_HEIGHT + 1.0;

const BOTTOM_COVER_BOTTOM     : f64 = -0.5*FRAME_HEIGHT;
const MIDDLE_COVER_BOTTOM     : f64 = BOTTOM_COVER_BOTTOM+COVER_THICKNESS+CABLE_HOUSING_HEIGHT;

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
	let mut board = cube_coords(&(String::from("base board for ")+name),
		-FRAME_THICKNESS/2.0,
		FOOT_END,
		-FRAME_HEIGHT/2.0,
		FRAME_THICKNESS/2.0,
		HEAD_END,
		FRAME_HEIGHT/2.0);
	//}}}

	//board.set_debug();

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
	//let mut board = union(name, parts);
	let mut board = union("", parts);
	//}}}

	//{{{ Add some anchors TODO

	{
		let mut a = board.create_anchor("Foot pulley vertical");
		a.translate(0.0, FOOT_END-0.5*FRAME_THICKNESS, FOOT_CABLE_HEIGHT); // TODO
		a.rel_rotate(90.0, 315.0, 0.0);
	}
	{
		let mut a = board.create_anchor("Head pulley vertical");
		a.translate(0.0, HEAD_END+0.5*FRAME_THICKNESS, HEAD_CABLE_HEIGHT); // TODO
		a.rel_rotate(90.0, 225.0, 0.0);
	}
	{
		let mut a = board.create_anchor("Foot pulley horizontal");
		a.translate(0.0, FOOT_END-0.5*FRAME_THICKNESS, FOOT_CABLE_HEIGHT); // TODO
		a.rel_rotate(90.0, 315.0, -90.0);
		a.rel_translate_z(-5.0);
		a.rel_rotate_x(225.0);








		//a.translate(2.0, FOOT_END+0.5*FRAME_THICKNESS, FOOT_CABLE_HEIGHT); // TODO
		//a.rel_rotate(90.0, 315.0, 0.0);
	}
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
	//{{{ Add the cutouts for the primary rolls
	{
		let mut block = sprenger_block_3511100355_cutout("Foot pulley");
		//block.set_debug();
		block.anchor("Upper contact").snap_to(&mut parts[0].anchor("Foot pulley vertical"));
		parts.push(block);
	}
	{
		let mut block = sprenger_block_3511100355_cutout("Head pulley");
		//block.set_debug();
		block.anchor("Upper contact").snap_to(&mut parts[0].anchor("Head pulley vertical"));
		parts.push(block);
	}
	//}}}
	//{{{ Add the cutouts for the secondary rolls
	{
		let mut block = sprenger_block_3511100355_cutout("Secondary foot pulley");
		//block.set_debug();
		block.anchor("Upper contact").snap_to(&mut parts[0].anchor("Foot pulley horizontal"));
		parts.push(block);
	}
	//{
	//	let mut block = sprenger_block_3511100355_cutout("Head pulley");
	//	//block.set_debug();
	//	block.anchor("Upper contact").snap_to(&mut parts[0].anchor("Head pulley vertical"));
	//	parts.push(block);
	//}
	//}}}
	let mut board = difference(name, parts);

	//board.set_debug();

	board.set_show_anchors();
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
pub fn arc(mut start_point: Point2D, mut end_point: Point2D, pivot: Point2D, steps: i32) -> Vec<Point2D>
{
	let start_vector       = pivot - &start_point;
	let   end_vector       = pivot -   &end_point;

	let start_radius : f64 = start_vector.l2_norm();
	let   end_radius : f64 =   end_vector.l2_norm();
	let  step_radius : f64 = (end_radius-start_radius)/(steps.abs() as f64);

	let start_angle  : f64 = start_vector[0].atan2(start_vector[1]);
	let end_angle    : f64 =   end_vector[0].atan2(  end_vector[1]);

	let mut angle_range  : f64 = end_angle - start_angle;
	if steps.signum() <= 0
	{
		eprintln!("arc: steps is negative");
		if angle_range < 0.0
		{
			eprintln!("arc: angle too small");
			angle_range += 2.0*PI;
		};
		if angle_range > PI
		{
			eprintln!("arc: angle too big");
			angle_range -= 2.0*PI;
		};
	}
	let step_angle   : f64 = angle_range/(steps.abs() as f64);


	let steps = steps.abs();

	eprintln!("arc:\n\tstart_vector = {}\n\tend_vector = {}\n\tstart_angle = {}\n\tend_angle = {}\n\tangle_range = {}\n\tstart_radius = {}\n\tend_radius = {}\n\tsteps = {}", start_vector, end_vector, start_angle.to_degrees(), end_angle.to_degrees(), angle_range.to_degrees(), start_radius, end_radius, steps);

	let mut points = vec![start_point];

	for i in 0..steps.abs()
	{
		let phi = start_angle  + (i as f64)*step_angle;
		let r   = start_radius + (i as f64)*step_radius;
		let v   = vector2D(phi.sin(), phi.cos());

		eprintln!("		step = {}, phi = {}, r = {}, v = {}", i, phi.to_degrees(), r, v);


		points.push(pivot + &(v*r));
		//points.push(pivot + &(vector2D(phi.cos(), phi.sin())*r));
	}

	points
}
//}}}


//{{{
pub fn sprenger_block_3511100355_cutout(name: &str) -> Object3D
{
	const ROLL_DIAMETER           : f64 =  2.5;
	const ROLL_DIAMETER_INNER     : f64 =  1.875;
	const SCREW_HEAD_DIAMETER     : f64 =  0.75;
	const SCREW_HEAD_HEIGHT       : f64 =  0.125;
	const WIRE_DIAMETER           : f64 =  0.3;
	const ROLL_WIDTH              : f64 =  0.7;
	const ROLL_HEIGHT             : f64 =  1.7;


	const BLOCK_WIDTH             : f64 = 1.02;
	const BLOCK_DIAMETER          : f64 =  2.5;
	const BLOCK_HEIGHT            : f64 =  3.38;
	const BASE_WIDTH              : f64 =  3.5;
	const SHEET_THICKNESS         : f64 =  0.13;
	const LOWER_BEND_RADIUS       : f64 =  0.17;

	const BLOCK_HEIGHT_SLANT      : f64 =  2.2;
	const SLANT_INSET             : f64 =  0.475;
	const SLANT_HEIGHT            : f64 = BLOCK_HEIGHT-BLOCK_HEIGHT_SLANT;
	#[allow(non_snake_case)]
	let   SLANT_ANGLE             : f64 = (SLANT_INSET/SLANT_HEIGHT).atan().to_degrees();

	//{{{
	let mut points : Vec<Point2D> = vec![
		point2D(-0.5*BASE_WIDTH, 0.0),
		point2D(-0.5*BASE_WIDTH, SHEET_THICKNESS),
	];


	points.append(&mut arc(point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, SHEET_THICKNESS),                    // Start
				           point2D(-0.5*BLOCK_WIDTH                  , SHEET_THICKNESS+LOWER_BEND_RADIUS),  // End
				           point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
				            20));                                                                           // Steps


	points.append(&mut arc(point2D(-0.5*BLOCK_WIDTH,                   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Start
				           point2D( 0.5*BLOCK_WIDTH,                   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // End
				           point2D(0.0,                                BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Pivot
				            40));                                                                           // Steps

	points.append(&mut arc(point2D( 0.5*BLOCK_WIDTH,                   SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Start
				           point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, SHEET_THICKNESS),                    // End
				           point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
				           -20));                                                                           // Steps

	points.push(point2D( 0.5*BASE_WIDTH, SHEET_THICKNESS));
	points.push(point2D( 0.5*BASE_WIDTH, 0.0));

	//points.append(&mut arc(point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, 0.0),                                // Start
	//			           point2D( 0.5*BLOCK_WIDTH-SHEET_THICKNESS,   SHEET_THICKNESS+LOWER_BEND_RADIUS),  // End
	//			           point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
	//			           -20));                                                                           // Steps

	//points.append(&mut arc(point2D( 0.5*BLOCK_WIDTH-SHEET_THICKNESS,   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Start
	//			           point2D(-0.5*BLOCK_WIDTH+SHEET_THICKNESS,   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // End
	//			           point2D(0.0,                                BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Pivot
	//			            40));                                                                           // Steps

	//points.append(&mut arc(
	//			           point2D(-0.5*BLOCK_WIDTH+SHEET_THICKNESS,   SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Start
	//                       point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, 0.0),                                // End
	//			           point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
	//			           -20));                                                                           // Steps
	//}}}

	let mut poly = polygon(name, points);

	poly.linear_extrude(BLOCK_DIAMETER);
	poly.rotate_x(90.0);
	poly.translate_y(0.5*BLOCK_DIAMETER);

	let mut slant1 = cube_coords("slant for sprenger_block_3511100355", -BASE_WIDTH, 0.0, -1.0, BASE_WIDTH, 1.0, 2.0);
	slant1.rotate_x(SLANT_ANGLE);
	slant1.translate(0.0, 0.5*BLOCK_DIAMETER, BLOCK_HEIGHT_SLANT);

	let mut slant2 = slant1.clone();
	slant2.scale_y(-1.0);


	let mut housing = difference(name, [poly, slant1, slant2]);

	//{{{ Add the screw heads

	let mut screw_head1 = cylinder(&(String::from("Lower half roll for ") + name), SCREW_HEAD_HEIGHT, 0.5*SCREW_HEAD_DIAMETER, 0.0);
	screw_head1.set_fn(40);
	screw_head1.rotate_y(90.0);
	screw_head1.translate(0.5*BLOCK_WIDTH, 0.0, ROLL_HEIGHT);
	let mut screw_head2 = cylinder(&(String::from("Lower half roll for ") + name), SCREW_HEAD_HEIGHT, 0.5*SCREW_HEAD_DIAMETER, 0.0);
	screw_head2.set_fn(40);
	screw_head2.rotate_y(-90.0);
	screw_head2.translate(-0.5*BLOCK_WIDTH, 0.0, ROLL_HEIGHT);
	//}}}

	let mut block = union(name, [housing, screw_head1, screw_head2]);

	//{{{ Anchors::Contact: y and z align to the cable
	{
		let mut a = block.create_anchor("Upper contact");
		a.translate(0.0, -0.5*ROLL_DIAMETER_INNER-0.5*WIRE_DIAMETER, ROLL_HEIGHT+0.5*ROLL_DIAMETER_INNER+0.5*WIRE_DIAMETER);
	}
	{
		let mut a = block.create_anchor("Lower contact");
		a.translate(0.0, -0.5*ROLL_DIAMETER_INNER-0.5*WIRE_DIAMETER, ROLL_HEIGHT-0.5*ROLL_DIAMETER_INNER-0.5*WIRE_DIAMETER);
	}
	//}}}

	block
}
//}}}

//{{{
pub fn sprenger_block_3511100355(name: &str) -> Object3D
{
	const ROLL_DIAMETER           : f64 =  2.5;
	const ROLL_DIAMETER_INNER     : f64 =  1.875;
	const SCREW_HEAD_DIAMETER     : f64 =  0.75;
	const SCREW_HEAD_HEIGHT       : f64 =  0.125;
	const WIRE_DIAMETER           : f64 =  0.3;
	const ROLL_WIDTH              : f64 =  0.7;
	const ROLL_HEIGHT             : f64 =  1.7;


	const BLOCK_WIDTH             : f64 = 1.02;
	const BLOCK_DIAMETER          : f64 =  2.5;
	const BLOCK_HEIGHT            : f64 =  3.38;
	const BASE_WIDTH              : f64 =  3.5;
	const SHEET_THICKNESS         : f64 =  0.13;
	const LOWER_BEND_RADIUS       : f64 =  0.17;

	const BLOCK_HEIGHT_SLANT      : f64 =  2.2;
	const SLANT_INSET             : f64 =  0.475;
	const SLANT_HEIGHT            : f64 = BLOCK_HEIGHT-BLOCK_HEIGHT_SLANT;
	#[allow(non_snake_case)]
	let   SLANT_ANGLE             : f64 = (SLANT_INSET/SLANT_HEIGHT).atan().to_degrees();

	//{{{
	let mut points : Vec<Point2D> = vec![
		point2D(-0.5*BASE_WIDTH, 0.0),
		point2D(-0.5*BASE_WIDTH, SHEET_THICKNESS),
	];


	points.append(&mut arc(point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, SHEET_THICKNESS),                    // Start
				           point2D(-0.5*BLOCK_WIDTH                  , SHEET_THICKNESS+LOWER_BEND_RADIUS),  // End
				           point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
				            20));                                                                           // Steps


	points.append(&mut arc(point2D(-0.5*BLOCK_WIDTH,                   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Start
				           point2D( 0.5*BLOCK_WIDTH,                   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // End
				           point2D(0.0,                                BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Pivot
				            40));                                                                           // Steps

	points.append(&mut arc(point2D( 0.5*BLOCK_WIDTH,                   SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Start
				           point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, SHEET_THICKNESS),                    // End
				           point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
				           -20));                                                                           // Steps

	points.push(point2D( 0.5*BASE_WIDTH, SHEET_THICKNESS));
	points.push(point2D( 0.5*BASE_WIDTH, 0.0));

	points.append(&mut arc(point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, 0.0),                                // Start
				           point2D( 0.5*BLOCK_WIDTH-SHEET_THICKNESS,   SHEET_THICKNESS+LOWER_BEND_RADIUS),  // End
				           point2D( 0.5*BLOCK_WIDTH+LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
				           -20));                                                                           // Steps

	points.append(&mut arc(point2D( 0.5*BLOCK_WIDTH-SHEET_THICKNESS,   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Start
				           point2D(-0.5*BLOCK_WIDTH+SHEET_THICKNESS,   BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // End
				           point2D(0.0,                                BLOCK_HEIGHT-0.5*BLOCK_WIDTH),       // Pivot
				            40));                                                                           // Steps

	points.append(&mut arc(
				           point2D(-0.5*BLOCK_WIDTH+SHEET_THICKNESS,   SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Start
	                       point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, 0.0),                                // End
				           point2D(-0.5*BLOCK_WIDTH-LOWER_BEND_RADIUS, SHEET_THICKNESS+LOWER_BEND_RADIUS),  // Pivot
				           -20));                                                                           // Steps
	//}}}

	let mut poly = polygon(name, points);

	poly.linear_extrude(BLOCK_DIAMETER);
	poly.rotate_x(90.0);
	poly.translate_y(0.5*BLOCK_DIAMETER);

	//{{{ Cut of the slanted sides

	let mut slant1 = cube_coords("slant for sprenger_block_3511100355", -BASE_WIDTH, 0.0, -1.0, BASE_WIDTH, 1.0, 2.0);
	slant1.rotate_x(SLANT_ANGLE);
	slant1.translate(0.0, 0.5*BLOCK_DIAMETER, BLOCK_HEIGHT_SLANT);

	let mut slant2 = slant1.clone();
	slant2.scale_y(-1.0);
	//}}}


	let housing = difference(name, [poly, slant1, slant2]);

	//{{{ Add the roll and screw heads

	let mut roll1 = cylinder(&(String::from("Lower half roll for ") + name), 0.5*ROLL_WIDTH, 0.5*ROLL_DIAMETER_INNER, 0.5*ROLL_DIAMETER);
	roll1.set_fn(40);
	roll1.rotate_y(90.0);
	roll1.translate_z(ROLL_HEIGHT);
	let mut roll2 = cylinder(&(String::from("Lower half roll for ") + name), 0.5*ROLL_WIDTH, 0.5*ROLL_DIAMETER_INNER, 0.5*ROLL_DIAMETER);
	roll2.set_fn(40);
	roll2.rotate_y(-90.0);
	roll2.translate_z(ROLL_HEIGHT);

	let mut screw_head1 = cylinder(&(String::from("Lower half roll for ") + name), SCREW_HEAD_HEIGHT, 0.5*SCREW_HEAD_DIAMETER, 0.0);
	screw_head1.set_fn(40);
	screw_head1.rotate_y(90.0);
	screw_head1.translate(0.5*BLOCK_WIDTH, 0.0, ROLL_HEIGHT);
	let mut screw_head2 = cylinder(&(String::from("Lower half roll for ") + name), SCREW_HEAD_HEIGHT, 0.5*SCREW_HEAD_DIAMETER, 0.0);
	screw_head2.set_fn(40);
	screw_head2.rotate_y(-90.0);
	screw_head2.translate(-0.5*BLOCK_WIDTH, 0.0, ROLL_HEIGHT);
	//}}}

	let mut block = union(name, [housing, roll1, roll2, screw_head1, screw_head2]);

	//{{{ Anchors::Contact: y and z align to the cable
	{
		let mut a = block.create_anchor("Upper contact");
		a.translate(0.0, -0.5*ROLL_DIAMETER_INNER-0.5*WIRE_DIAMETER, ROLL_HEIGHT+0.5*ROLL_DIAMETER_INNER+0.5*WIRE_DIAMETER);
	}
	{
		let mut a = block.create_anchor("Lower contact");
		a.translate(0.0, -0.5*ROLL_DIAMETER_INNER-0.5*WIRE_DIAMETER, ROLL_HEIGHT-0.5*ROLL_DIAMETER_INNER-0.5*WIRE_DIAMETER);
	}
	//}}}

	block
}
//}}}



fn main()
{

//	////{{{ Print all the constants
//
//	//eprintln!("MATTRESS_LENGTH = {}", MATTRESS_LENGTH);
//	//eprintln!("STORAGE_LENGTH  = {}", STORAGE_LENGTH );
//	//eprintln!("MATTRESw_WIDTH  = {}", MATTRESS_WIDTH );
//	//eprintln!("BED_LENGTH      = {}", BED_LENGTH     );
//	//eprintln!("BED_WIDTH       = {}", BED_WIDTH      );
//	//eprintln!("FRAME_HEIGHT    = {}", FRAME_HEIGHT   );
//	//eprintln!("FRAME_THICKNESS     = {}", FRAME_THICKNESS    );
//	////}}}




	//{{{
	let mut sideboard_l = sideboard("Sideboard_L");
	sideboard_l.translate_x(-(BED_WIDTH-FRAME_THICKNESS)/2.0);
	sideboard_l.set_colour(colour_named("red"));
	sideboard_l.set_show_anchors();
	//println!("{}", sideboard_l);
	//}}}

//
//	//{{{
//	let mut sideboard_r = sideboard_l.clone();
//	sideboard_r.name = String::from("Sideboard_R");
//	sideboard_r.scale_x(-1.0);
//	println!("{}", sideboard_r);
//	//}}}
//
//	//{{{
//	let mut headboard = headboard("Headboard");
//	headboard.translate_y(BED_LENGTH-100.0-1.5*FRAME_THICKNESS);
//	headboard.set_colour(colour_named("green"));
//	println!("{}", headboard);
//	//}}}
//
//	//{{{
//	let mut footboard = footboard("Footboard");
//	footboard.translate_y(-(200.0+FRAME_THICKNESS)/2.0);
//	footboard.set_colour(colour_named("green"));
//	println!("{}", footboard);
//	//}}}
//
//	//{{{
//	let mut bulkhead = bulkhead("Bulkhead");
//	bulkhead.translate_y((MATTRESS_LENGTH+FRAME_THICKNESS)/2.0);
//	bulkhead.set_colour(colour_named("yellow"));
//	println!("{}", bulkhead);
//	//}}}
//
//	//{{{
//	let mut bottom_cover = cube_coords("Bottom cover",
//		-(0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH),
//		HEAD_END+COVER_GROOVE_DEPTH,
//		BOTTOM_COVER_BOTTOM,
//
//		0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH,
//		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
//		BOTTOM_COVER_BOTTOM+COVER_THICKNESS,
//		);
//	bottom_cover.set_colour(colour_named("blue"));
//	println!("{}", bottom_cover);
//	//}}}
//
//	//{{{
//	let mut middle_cover_l = cube_coords("Bottom cover",
//		-(0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH),
//		HEAD_END+COVER_GROOVE_DEPTH,
//		MIDDLE_COVER_BOTTOM,
//
//		-0.5*MOTOR_WIDTH-FRAME_THICKNESS+COVER_GROOVE_DEPTH,
//		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
//		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
//		);
//	middle_cover_l.set_colour(colour_named("blue"));
//	println!("{}", middle_cover_l);
//	//}}}
//
//	//{{{
//	let mut middle_cover_r = cube_coords("Bottom cover",
//		0.5*MATTRESS_WIDTH+COVER_GROOVE_DEPTH,
//		HEAD_END+COVER_GROOVE_DEPTH,
//		MIDDLE_COVER_BOTTOM,
//
//		0.5*MOTOR_WIDTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
//		0.5*MATTRESS_LENGTH+FRAME_THICKNESS-COVER_GROOVE_DEPTH,
//		MIDDLE_COVER_BOTTOM+COVER_THICKNESS,
//		);
//	middle_cover_r.set_colour(colour_named("blue"));
//	println!("{}", middle_cover_r);
//	//}}}
//
//	//{{{
//	let mut bulkhead_spacer_l = bulkhead_spacer("Bulkhead spacer L");
//	bulkhead_spacer_l.translate_x(-0.5*(MOTOR_WIDTH+FRAME_THICKNESS));
//	println!("{}", bulkhead_spacer_l);
//	//}}}
//
//	//{{{
//	let mut bulkhead_spacer_r = bulkhead_spacer_l.clone();
//	bulkhead_spacer_r.name = String::from("Bulkhead spacer R");
//	bulkhead_spacer_r.scale_x(-1.0);
//	println!("{}", bulkhead_spacer_r);
//	//}}}
//





	//{{{ Primary Foot Roll Left

	{
		let mut block = sprenger_block_3511100355("Foot pulley");
		block.set_debug();
		block.anchor("Upper contact").snap_to(&mut sideboard_l.anchor("Foot pulley vertical"));
		println!("{}", block);
	}
	//}}}


	//{{{ Secondary Foot Roll Left

	{
		let mut block = sprenger_block_3511100355("Secondary foot pulley");
		block.set_debug();
		block.anchor("Upper contact").snap_to(&mut sideboard_l.anchor("Foot pulley horizontal"));
		println!("{}", block);
	}
	//}}}


	//{{{ Slat Frame

	for i in 0..FRAME_SLAT_COUNT
	{
		let i = i as f64;
		let mut slat = frame_slat(&format!("Frame slat {}", i));
		slat.translate(0.0, FRAME_SLAT_START - i*(FRAME_SLAT_SPACING+FRAME_SLAT_WIDTH), -0.5*(FRAME_HEIGHT-FRAME_SLAT_THICKNESS));
		slat.set_colour(colour_rgba(1.0, 1.0, 0.0, 0.2));
		println!("{}", slat);
	}




	//}}}






//
//	//{{{
//	let mut block = sprenger_block_3511100355_cutout("tester");
//	//let mut block = sprenger_block_3511100355("tester");
//	block.set_debug();
//	block.set_show_anchors();
//	println!("{}", block);
//	//}}}
//
//
//	//{{{
//	let mut poly = polygon("tester", [[0.0, 0.0], [0.0, 25.0], [25.0, 0.0], [5.0, 5.0], [15.0, 5.0], [5.0, 15.0]], [[0, 1, 2], [3, 4, 5]]);
//	poly.linear_extrude(14.0);
//	println!("{}", poly);
//	//}}}
//
//
//	//{{{
//	let mut square = square("tester", 5.0, 2.0);
//	let mut circle = circle("tester", 1.0);
//	circle.set_fn(20);
//	let mut test   = difference("diff", [square, circle]);
//	test.set_debug();
//	let mut cube   = cube_coords("cube", 0.0, 0.0, 0.0,  2.0, 2.0, 2.0);
//	let mut test2  = difference("erence", [cube, test]);
//	//roll.anchor("Contact").snap_to(&mut sideboard_l.anchor("left"));
//	println!("{}", test2);
//	//}}}
//


}
