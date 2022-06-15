
const VEC_ARR: [f32;34] = [
    1.0, 0.0,
    0.9951847266721969, 0.0980171403295606,
    0.9807852804032304, 0.1950903220161282,
    0.9569403357322088, 0.2902846772544623,
    0.9238795325112867, 0.3826834323650898,
    0.881921264348355,  0.4713967368259976,
    0.8314696123025452, 0.5555702330196022,
    0.773010453362737,  0.6343932841636455,
    0.7071067811865476, 0.7071067811865475,
    0.6343932841636455, 0.7730104533627369,
    0.5555702330196023, 0.8314696123025452,
    0.4713967368259978, 0.8819212643483549,
    0.3826834323650898, 0.9238795325112867,
    0.2902846772544623, 0.9569403357322089,
    0.1950903220161283, 0.9807852804032304,
    0.0980171403295607, 0.9951847266721968,
    0.0, 1.0
];
const RADIUS_4_8: f32       = 32.0;
const RADIUS_8_16: f32      = 128.0;


fn copy_level4() -> Vec<u16> {
    vec![0,4,8,12,16]
}
fn copy_level8() -> Vec<u16> {
    vec![ 0,2,4,6,8,10,12,14,16 ]
}
fn copy_level16() -> Vec<u16> {
    vec![ 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 ]
}

/**
 * input 
 *      ⚪心
 *          [f32;2]
 *      弧方向向量
 *          [f32;2]
 *      半径
 *          f32
 *      角度
 *          f32
 * output
 *      点列表 * 逆时针
 *          Vec<[f32;2]>
 */
pub fn get_one_quarter_arc(center_x: f32, center_y: f32, radius: f32, area_id: u8, z: f32) -> Vec<f32> {
    let mut segments: Vec<u16>;
    let mut result_points: Vec<f32> = Vec::new();

    if radius < RADIUS_4_8 {
        segments    = copy_level4();
    } else if radius <= RADIUS_8_16 {
        segments    = copy_level8();
    } else {
        segments    = copy_level16();
    }

    let count = segments.len() as u16;
    let mut index: u16 = 0;
    let mut _index: u16 = 0;
    while index <= count - 1 {
        let mut x: f32;
        let mut y: f32;

        if area_id == 1 {
            _index = index;
            _index = segments[index as usize] * 2;

            x = radius * VEC_ARR[(_index) as usize];
            y = radius * VEC_ARR[(_index + 1) as usize];
            x = center_x + x;
            y = center_y + y;
        } else if area_id == 2 {
            _index = index;
            _index = segments[index as usize] * 2;

            x = - radius * VEC_ARR[(_index + 1) as usize];
            y = radius * VEC_ARR[(_index) as usize];
            x = center_x + x;
            y = center_y + y;
        } else if area_id == 3 {
            _index = index;
            _index = segments[index as usize] * 2;

            x = - radius * VEC_ARR[(_index) as usize];
            y = - radius * VEC_ARR[(_index + 1) as usize];
            x = center_x + x;
            y = center_y + y;
        } else {
            _index = index;
            _index = segments[index as usize] * 2;

            x = radius * VEC_ARR[(_index + 1) as usize];
            y = - radius * VEC_ARR[(_index) as usize];
            x = center_x + x;
            y = center_y + y;
        }

        result_points.push(x);
        result_points.push(y);
        result_points.push(z);

        index = index + 1;
    }

    result_points
}

/**
 * input 
 *      左上坐标
 *          
 *      宽高
 *          
 *      半径
 *          f32
 * output
 *      点列表 * 逆时针
 *          Vec<f32>
 */
pub fn get_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, z: f32) -> Vec<f32> {

    let mut result: Vec<f32> = Vec::new();
    let mut check_list: Vec<(f32,f32,u8)> = Vec::new();
    check_list.push((x + w - radius,    y - radius,     1));
    check_list.push((x + radius,        y - radius,     2));
    check_list.push((x + radius,        y - h + radius, 3));
    check_list.push((x + w - radius,    y - h + radius, 4));

    for data in check_list {
        let (x,y,a) = data;
        let res = get_one_quarter_arc(x,y,radius,a, z);
        for v in res {
            result.push(v);
        }
    }

    result
}

