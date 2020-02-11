#[macro_use]
extern crate neon;
extern crate capra;
use capra::common::gas::Gas;
use capra::zhl16::ZHL16;
use capra::zhl16::util;
use capra::common::dive_segment::{DiveSegment, SegmentType};
use capra::common::deco_algorithm::DecoAlgorithm;

use neon::prelude::*;
use neon::register_module;

fn ndl_demo(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let air = Gas::new(0.79, 0.21, 0.0).unwrap();
    let gf_low = 100;
    let gf_high = 100;
    let ascent_rate = -10;
    let descent_rate = 20;

    let mut zhl16 = ZHL16::new(&air,
                               util::ZHL16B_N2_A,
                               util::ZHL16B_N2_B,
                               util::ZHL16B_N2_HALFLIFE,
                               util::ZHL16B_HE_A,
                               util::ZHL16B_HE_B,
                               util::ZHL16B_HE_HALFLIFE,
                               gf_low, gf_high);

    let depth = cx.argument::<JsNumber>(0)?.value() as usize;
    let time = cx.argument::<JsNumber>(1)?.value() as usize;
    let dive = DiveSegment::new(SegmentType::DiveSegment,
                                depth, depth, time, ascent_rate,
                                descent_rate).unwrap();

    zhl16.add_bottom_time(&dive, &air);

    let result = zhl16.get_stops(ascent_rate, descent_rate, &air);
    let ndl = result.get(0).unwrap().get_time() as f64;

    Ok(cx.number(ndl))
}

register_module!(mut cx, {
    cx.export_function("ndlDemo", ndl_demo)
});
