// /**
//  * 解析渐变角度 获得 渐进方向单位向量
//  */
// /**
//  * 复制初始点 位置数据
//  */
// /**
//  * 遍历初始点 获得渐变方向上 点积量
//  * 查找渐变方向上 最小点积 和 最大点积
//  * 计算各点 渐变百分比
//  * 获得渐变起点序号
//  * 从 渐变起点 构建多边形路径
//  * 从 渐变起点 重排UV数据
//  * 从多边形路径 获得多边形各边所在直线
//  */
// /**
//  * 复制初始点 UV数据
//  */
// /**
//  * 复制 渐变配置
//  * 依据 方向向量 和 渐变百分比 计算 渐变向量
//  * 渐变向量 获得 渐变线 斜率，获得渐变线所在直线
//  */
// /**
//  * 建立 结果点数组
//  * 遍历多边形各边直线 与 各渐变线求交点，
//  * 复制边起点到结果点数组
//  * 渐变线的遍历顺序 以 渐变向量与边向量点积 是否大于 0 ，决定按 渐变进度 升序求交点
//  * 只保留处于 各边上的 交点
//  * 保存 到结果点数组
//  * 计算 交点 颜色，uv
//  * 交点与初始点在同一数组
//  * 记录 渐变线分别记录 各自线上的点
//  */
// /**
//  * 交点与初始点在同一数组
//  * 按 渐变方向上的 进度排序
//  * 遍历 渐变数组 寻找两个渐变之间的 点
//  * 找到的点 按在结果点数组序号升序排列
//  * 对排序后的点 构建三角形序号数组
//  */

///////////////////////////////////////////
const POINT_POS_DATA_INDEX  :i8 = 0;
const POINT_POS_DATA_LEN    :i8 = 3;

const POINT_UV_DATA_INDEX   :i8 = 3;
const POINT_UV_DATA_LEN     :i8 = 2;

const POINT_COLOR_DATA_INDEX:i8 = 5;
const POINT_COLOR_DATA_LEN  :i8 = 4;

const POINT_DOT_INDEX       :i8 = 9;
const POINT_DOT_LEN         :i8 = 1;

const POINT_DIRECT_INDEX    :i8 = 10;
const POINT_DIRECT_LEN      :i8 = 1;

const POINT_VEC_INDEX       :i8 = 11;
const POINT_VEC_LEN         :i8 = 2;

const POINT_ISSRC_INDEX     :i8 = 13;

const POINT_DATA_SIZE       :i8 = 14;

type PointData      = [f32; POINT_DATA_SIZE as usize];
type PointDataList  = Vec<PointData>;

///////////////////////////////////////////
const GRAD_DIRECT_INDEX     :i8 = 0;
const GRAD_DIRECT_LEN       :i8 = 1;

const GRAD_COLOR_INDEX      :i8 = 1;
const GRAD_COLOR_LEN        :i8 = 4;

const GRAD_DIRECT_VEC_INDEX        :i8 = 5;
const GRAD_DIRECT_VEC_LEN          :i8 = 2;

const GRAD_ORIG_VEC_INDEX     :i8 = 7;
const GRAD_ORIG_VEC_LEN       :i8 = 2;

const GRAD_DATA_SIZE        :i8 = 9;
type GradData               = [f32; GRAD_DATA_SIZE as usize];
type GradDataList           = Vec<GradData>;
type GradPointIndexList     = Vec<u16>;

///////////////////////////////////////////
const GRAD_LINE_PARAM_INDEX      :i8 = 0;
const GRAD_LINE_PARAM_LEN        :i8 = 3;

const GRAD_LINE_SLOP_INDEX       :i8 = 3;
const GRAD_LINE_SLOP_LEN         :i8 = 1;

const GRAD_LINE_VEC_INDEX        :i8 = 4;
const GRAD_LINE_VEC_LEN          :i8 = 2;

const GRAD_LINE_DATA_SIZE        :i8 = 6;

type GradLineCfg            = [f32; GRAD_LINE_DATA_SIZE as usize];
type GradLineCfgList        = Vec<GradLineCfg>;

type GradResultPointCfg       = Vec<u16>;
type GradResultPointCfgList   = Vec<Vec<u16>>;

///////////////////////////////////////////
const LINE_PARAM_INDEX      :i8 = 0;
const LINE_PARAM_LEN        :i8 = 3;

const LINE_SLOP_INDEX       :i8 = 3;
const LINE_SLOP_LEN         :i8 = 1;

const LINE_VEC_INDEX        :i8 = 4;
const LINE_VEC_LEN          :i8 = 2;

const LINE_P0_INDEX         :i8 = 6;
const LINE_P1_INDEX         :i8 = 7;

const LINE_DATA_SIZE        :i8 = 8;

type LineCfg            = [f32; LINE_DATA_SIZE as usize];
type LineCfgList        = Vec<LineCfg>;
type LineIndexCfg       = [u16; 2];
type LineIndexCfgList   = Vec<LineIndexCfg>;

pub struct GradCfg {
    pub percent: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct GradResult {
    pub point_list: Vec<f32>,
    pub point_color_list: Vec<f32>,
    pub attr_list: Vec<Vec<f32>>,
    pub triangle_indices: Vec<u16>,
}
/**
 * pos_list
 *      点坐标
 * attr_list
 *      属性列表
 */
pub fn polygon_grad_analy(pos_list: Vec<f32>, attr_list: Vec<Vec<f32>>, grad_list: &[GradCfg], angle: f32, z: f32) -> GradResult {

    let direction_vec2      = get_direction_vector(angle);

    let mut src_point_list  = copy_src_point(&pos_list, &attr_list);

    let mut min_dot: f32;
    let mut min_dot_index: i16;
    let mut max_dot: f32;
    let mut max_dot_index: i16;
    let mut total_dot: f32;
    let (min_dot_index, min_dot, max_dot_index, max_dot, total_dot) = compute_src_point_direct_dot(&mut src_point_list, direction_vec2);

    compute_src_point_direct(&mut src_point_list, min_dot, total_dot);

    let mut resort_src_point_list = resort_src_point(&mut src_point_list, min_dot_index);

    let orignal_vec2: [f32; 2] = [(resort_src_point_list[0])[(POINT_VEC_INDEX + 0) as usize], (resort_src_point_list[0])[(POINT_VEC_INDEX + 1) as usize]];

    let (resort_src_line_list, resort_src_line_index_list) = analy_resort_src_point_line(&resort_src_point_list);
    
    let (grad_cfg_list, grad_line_list) = copy_grad_list(grad_list, orignal_vec2, direction_vec2, total_dot);

    // resort_src_point_list = compute_src_point_color(resort_src_point_list, & grad_cfg_list);
    compute_src_point_color(&mut resort_src_point_list, & grad_cfg_list);

    let (result_point_list, grad_line_result_indexs) = analy_intersections(& grad_cfg_list, grad_line_list, resort_src_line_list, resort_src_line_index_list, resort_src_point_list, direction_vec2);

    let result_triangles = analy_triangles(&result_point_list, &grad_cfg_list, &grad_line_result_indexs);
    
    // 
    let mut result_datas: GradResult = GradResult {
        point_list: Vec::new(),
        point_color_list: Vec::new(),
        attr_list: Vec::new(),
        triangle_indices: result_triangles,
    };

    for data in result_point_list {
        result_datas.point_list.push(data[(POINT_POS_DATA_INDEX) as usize]);
        result_datas.point_list.push(data[(POINT_POS_DATA_INDEX + 1) as usize]);
        result_datas.point_list.push(z);
        
        // result_datas.point_list.push(data[(POINT_POS_DATA_INDEX) as usize]);
        // result_datas.point_list.push(data[(POINT_POS_DATA_INDEX + 1) as usize]);
        
        result_datas.point_color_list.push(data[(POINT_COLOR_DATA_INDEX) as usize]);
        result_datas.point_color_list.push(data[(POINT_COLOR_DATA_INDEX + 1) as usize]);
        result_datas.point_color_list.push(data[(POINT_COLOR_DATA_INDEX + 2) as usize]);
        result_datas.point_color_list.push(data[(POINT_COLOR_DATA_INDEX + 3) as usize]);
    }

    result_datas
}

#[allow(dead_code)]
fn get_dot(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    x0 * x1 + y0 * y1
}
fn new_point_data() -> PointData {
    [ std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN
    , std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN
    , std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN
    ]
}
fn clone_point_data(p: PointData) -> PointData {
    [ p[0], p[1], p[2], p[3], p[4]
    , p[5], p[6], p[7], p[8], p[9]
    , p[10], p[11], p[12], p[13]
    ]
}
fn new_line_data() -> LineCfg {
    [ std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN
    , std::f32::NAN, std::f32::NAN, std::f32::NAN
    ]
}
fn new_grad_data() -> GradData {
    [ std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN
    , std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN
    ]
}
fn new_grad_line_data() -> GradLineCfg {
    [ std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN, std::f32::NAN
    , std::f32::NAN
    ]
}

#[allow(dead_code)]
fn get_direction_vector(angle: f32) -> [f32; 2] {
    let mut radius: f32 = 0.0;

    let _angle   = angle % 360.0;

    radius  = ((_angle as f32) / 180.0) * std::f32::consts::PI;

    [radius.cos(), radius.sin()]
}

#[allow(dead_code)]
fn copy_src_point(pos_list: &Vec<f32>, uv_list: &Vec<Vec<f32>>) -> PointDataList {
    let mut list: PointDataList = Vec::new();
    let has_attr: bool = uv_list.len() > 1;

    let count = (pos_list.len() / 3) as i16;
    let mut index = 0;
    while index < count {
        
        let mut _point: PointData = new_point_data();

        _point[(POINT_POS_DATA_INDEX + 0) as usize] = pos_list[(index * 2 + 0) as usize];
        _point[(POINT_POS_DATA_INDEX + 1) as usize] = pos_list[(index * 2 + 1) as usize];
        // _point[(POINT_POS_DATA_INDEX + 2) as usize] = point[2];
        
        _point[(POINT_VEC_INDEX + 0) as usize]  = pos_list[(index * 2 + 0) as usize];
        _point[(POINT_VEC_INDEX + 1) as usize]  = pos_list[(index * 2 + 1) as usize];
        
        if has_attr {
            // let uv = &uv_list[(index) as usize];
            // _point[(POINT_UV_DATA_INDEX + 0) as usize]  = uv[0];
            // _point[(POINT_UV_DATA_INDEX + 1) as usize]  = uv[1];
        }

        _point[(POINT_ISSRC_INDEX) as usize]    = 1.0;

        list.push(_point);

        index = index + 1;
    }
    // for point in pos_list {
    //     let mut _point: PointData = new_point_data();

    //     _point[(POINT_POS_DATA_INDEX + 0) as usize] = point[0];
    //     _point[(POINT_POS_DATA_INDEX + 1) as usize] = point[1];
    //     _point[(POINT_POS_DATA_INDEX + 2) as usize] = point[2];
        
    //     _point[(POINT_VEC_INDEX + 0) as usize]  = point[0];
    //     _point[(POINT_VEC_INDEX + 1) as usize]  = point[1];
        
    //     if has_attr {
    //         let uv = &uv_list[(index) as usize];
    //         _point[(POINT_UV_DATA_INDEX + 0) as usize]  = uv[0];
    //         _point[(POINT_UV_DATA_INDEX + 1) as usize]  = uv[1];
    //     }

    //     _point[(POINT_ISSRC_INDEX) as usize]    = 1.0;

    //     list.push(_point);

    //     index = index + 1;
    // }
    
    list
}

#[allow(dead_code)]
fn copy_grad_list(grad_list: &[GradCfg], orignal_vec: [f32; 2], direct_vec: [f32; 2], total_dot: f32) -> (GradDataList, GradLineCfgList) {
    let mut list: GradDataList = Vec::new();
    let mut grad_line_list: GradLineCfgList = Vec::new();

    for grad in grad_list {
        let mut grad_data: GradData = new_grad_data();

        grad_data[(GRAD_COLOR_INDEX + 0) as usize]  = grad.r;
        grad_data[(GRAD_COLOR_INDEX + 1) as usize]  = grad.g;
        grad_data[(GRAD_COLOR_INDEX + 2) as usize]  = grad.b;
        grad_data[(GRAD_COLOR_INDEX + 3) as usize]  = grad.a;
        
        grad_data[(GRAD_DIRECT_INDEX + 0) as usize] = grad.percent;

        grad_data[(GRAD_DIRECT_VEC_INDEX + 0) as usize] = direct_vec[0] * grad.percent * total_dot;
        grad_data[(GRAD_DIRECT_VEC_INDEX + 1) as usize] = direct_vec[1] * grad.percent * total_dot;

        grad_data[(GRAD_ORIG_VEC_INDEX + 0) as usize] = orignal_vec[0] + grad_data[(GRAD_DIRECT_VEC_INDEX + 0) as usize];
        grad_data[(GRAD_ORIG_VEC_INDEX + 1) as usize] = orignal_vec[1] + grad_data[(GRAD_DIRECT_VEC_INDEX + 1) as usize];

        let mut grad_line: GradLineCfg = new_grad_line_data();

        if grad.percent != 0.0 && grad.percent != 1.0 {
            // 垂直渐变
            if direct_vec[1] == 0.0 {
                grad_line[(GRAD_LINE_PARAM_INDEX + 0) as usize] = 1.0;
                grad_line[(GRAD_LINE_PARAM_INDEX + 1) as usize] = 0.0;
                grad_line[(GRAD_LINE_PARAM_INDEX + 2) as usize] = -grad_data[(GRAD_ORIG_VEC_INDEX + 0) as usize];
            // 水平渐变
            } else if direct_vec[0] == 0.0 {
                grad_line[(GRAD_LINE_PARAM_INDEX + 0) as usize] = 0.0;
                grad_line[(GRAD_LINE_PARAM_INDEX + 1) as usize] = 1.0;
                grad_line[(GRAD_LINE_PARAM_INDEX + 2) as usize] = -grad_data[(GRAD_ORIG_VEC_INDEX + 1) as usize];
            // 普通斜向
            } else {
                let slop  = - (grad_data[(GRAD_DIRECT_VEC_INDEX + 0) as usize] / grad_data[(GRAD_DIRECT_VEC_INDEX + 1) as usize]);
                grad_line = get_line2(slop, grad_data[(GRAD_ORIG_VEC_INDEX + 0) as usize], grad_data[(GRAD_ORIG_VEC_INDEX + 1) as usize]);
            }
        }

        list.push(grad_data);
        grad_line_list.push(grad_line);
    }
    
    (list, grad_line_list)
}

#[allow(dead_code)]
fn compute_src_point_direct_dot(list: &mut PointDataList, direct_vec: [f32; 2]) -> (i16, f32, i16, f32, f32) {
    let mut index: i16 = 0;
    let mut min_dot: f32 = 0.0;
    let mut min_dot_index: i16 = 0;
    let mut max_dot: f32 = 0.0;
    let mut max_dot_index: i16 = 0;

    for point in list {
        let dot = get_dot(point[(POINT_VEC_INDEX + 0) as usize], point[(POINT_VEC_INDEX + 1) as usize], direct_vec[0], direct_vec[1]);

        if dot <= min_dot {
            min_dot = dot;
            min_dot_index = index;
        }
        if max_dot <= dot {
            max_dot = dot;
            max_dot_index = index;
        }

        point[(POINT_DOT_INDEX + 0) as usize] = dot;

        index = index + 1;
    }

    (min_dot_index, min_dot, max_dot_index, max_dot, max_dot - min_dot)
}

#[allow(dead_code)]
fn compute_src_point_direct(list: &mut PointDataList, min_dot: f32, total_dot: f32) {
    // let index: u16 = 0;
    for point in list {
        let direct = (point[(POINT_DOT_INDEX) as usize] - min_dot) / total_dot;
        point[POINT_DIRECT_INDEX as usize] = direct;

        point[(POINT_COLOR_DATA_INDEX) as usize] = direct;
    }
}

#[allow(dead_code)]
// fn compute_src_point_color(list: &Vec<PointData>, grad_list: &GradDataList) -> Vec<PointData> {
fn compute_src_point_color(list: &mut Vec<PointData>, grad_list: &GradDataList) {
    let count = list.len();

    let mut index: u16 = 0;
    let mut targ_direct: f32;
    let mut pre_grad: GradData;
    let mut nxt_grad: GradData;
    let mut percent: f32;
    while index <= (count as u16) - 1 {
        
        // let mut targ_point: PointData  = clone_point_data(list[(index) as usize]);
        let mut targ_point: PointData  = list[(index) as usize];

        targ_direct = targ_point[(POINT_DIRECT_INDEX) as usize];

        let (pre_index, nxt_index) = find_pre_next_grad_direct(&grad_list, targ_direct);

        pre_grad = grad_list[(pre_index) as usize];
        nxt_grad = grad_list[(nxt_index) as usize];
        percent  = (targ_direct - pre_grad[(GRAD_DIRECT_INDEX) as usize]) / (nxt_grad[(GRAD_DIRECT_INDEX) as usize] - pre_grad[(GRAD_DIRECT_INDEX) as usize]);

        (list[(index) as usize])[(POINT_COLOR_DATA_INDEX) as usize]       = (nxt_grad[(GRAD_COLOR_INDEX) as usize] - pre_grad[(GRAD_COLOR_INDEX) as usize]) * percent + pre_grad[(GRAD_COLOR_INDEX) as usize];
        (list[(index) as usize])[(POINT_COLOR_DATA_INDEX + 1) as usize]   = (nxt_grad[(GRAD_COLOR_INDEX + 1) as usize] - pre_grad[(GRAD_COLOR_INDEX + 1) as usize]) * percent + pre_grad[(GRAD_COLOR_INDEX + 1) as usize];
        (list[(index) as usize])[(POINT_COLOR_DATA_INDEX + 2) as usize]   = (nxt_grad[(GRAD_COLOR_INDEX + 2) as usize] - pre_grad[(GRAD_COLOR_INDEX + 2) as usize]) * percent + pre_grad[(GRAD_COLOR_INDEX + 2) as usize];
        (list[(index) as usize])[(POINT_COLOR_DATA_INDEX + 3) as usize]   = (nxt_grad[(GRAD_COLOR_INDEX + 3) as usize] - pre_grad[(GRAD_COLOR_INDEX + 3) as usize]) * percent + pre_grad[(GRAD_COLOR_INDEX + 3) as usize];
    
        index = index + 1;
    }

    // list
}


#[allow(dead_code)]
fn find_pre_next_grad_direct(grad_list: &GradDataList, targ_direct: f32) -> (u16, u16) {
    let count = grad_list.len();
    let mut pre_index: u16 = 0;
    let mut nxt_index: u16 = 0;

    let mut index: u16 = 0;
    while index <= (count as u16) - 1 {
        if grad_list[(index) as usize][(GRAD_DIRECT_INDEX) as usize] <= targ_direct{
            pre_index = index;
        } else {
            nxt_index = index;
            break;
        }

        nxt_index = index;

        index = index + 1;
    }

    if pre_index == nxt_index {
        pre_index = nxt_index - 1;
    }

    (pre_index, nxt_index)
}

#[allow(dead_code)]
fn resort_src_point(list: &mut PointDataList, orignal_index: i16) -> Vec<PointData> {
    let count = list.len();
    let mut index: i16 = orignal_index;
    let mut new_point_list: Vec<PointData> = Vec::new();

    while index <= (count - 1) as i16 {
        let sort_point = clone_point_data(list[index as usize]);
        new_point_list.push(sort_point);
        index = index + 1;
    }

    index = 0;
    while index <= orignal_index -1  {
        let sort_point = clone_point_data(list[index as usize]);
        new_point_list.push(sort_point);
        index = index + 1;
    }

    new_point_list
}

#[allow(dead_code)]
fn analy_resort_src_point_line(list: &Vec<PointData>) -> (Vec<LineCfg>, Vec<LineIndexCfg>) {
    let mut index: u16   = 0;
    let count: usize    = list.len();
    let mut line_list: Vec<LineCfg> = Vec::new();
    let mut line_inedx_list: Vec<LineIndexCfg> = Vec::new();

    while index < ((count - 1) as u16) {
        let point0 = list[index as usize];
        let point1 = list[(index + 1) as usize];

        let mut line_index_cfg: LineIndexCfg = [0,0];
        line_index_cfg[0] = index;
        line_index_cfg[1] = index + 1;

        let line_cfg: LineCfg = get_line(
            point0[(POINT_POS_DATA_INDEX + 0) as usize], 
            point0[(POINT_POS_DATA_INDEX + 1) as usize], 
            point1[(POINT_POS_DATA_INDEX + 0) as usize], 
            point1[(POINT_POS_DATA_INDEX + 1) as usize]
        );

        line_list.push(line_cfg);
        line_inedx_list.push(line_index_cfg);

        index = index + 1;
    }
    
    let point0 = list[(count - 1) as usize];
    let point1 = list[0];

    let mut line_index_cfg: LineIndexCfg = [0,0];
    line_index_cfg[0] = (count - 1) as u16;
    line_index_cfg[1] = 0;

    let line_cfg: LineCfg = get_line(
        point0[(POINT_POS_DATA_INDEX + 0) as usize], 
        point0[(POINT_POS_DATA_INDEX + 1) as usize], 
        point1[(POINT_POS_DATA_INDEX + 0) as usize], 
        point1[(POINT_POS_DATA_INDEX + 1) as usize]
    );

    line_list.push(line_cfg);
    line_inedx_list.push(line_index_cfg);

    (line_list, line_inedx_list)
}

#[allow(dead_code)]
fn get_line(x0: f32, y0: f32, x1: f32, y1: f32) -> LineCfg {
    
    let mut line_cfg: LineCfg = new_line_data();

    let a = y0 - y1;
    let b = x1 - x0;
    let c = x0 * y1 - y0 * x1;
    let slop: f32;
    if b == 0.0 {
        slop = 0.0;
    } else {
        slop = a / -b;
    } 

    line_cfg[(LINE_PARAM_INDEX + 0) as usize]   = a;
    line_cfg[(LINE_PARAM_INDEX + 1) as usize]   = b;
    line_cfg[(LINE_PARAM_INDEX + 2) as usize]   = c;
    line_cfg[(LINE_SLOP_INDEX + 0) as usize]    = slop;

    line_cfg[(LINE_VEC_INDEX + 0) as usize]     = b;
    line_cfg[(LINE_VEC_INDEX + 1) as usize]     = -a;

    line_cfg
}

#[allow(dead_code)]
fn get_line2(slop: f32, x: f32, y: f32) -> GradLineCfg {

    let a = slop;
    let b = -1.0;
    let c = y - a * x;
    
    let mut line: GradLineCfg = new_grad_line_data();

    line[(GRAD_LINE_PARAM_INDEX + 0) as usize]  = a;
    line[(GRAD_LINE_PARAM_INDEX + 1) as usize]  = b;
    line[(GRAD_LINE_PARAM_INDEX + 2) as usize]  = c;
    
    line[(GRAD_LINE_SLOP_INDEX + 0) as usize]   = slop;
    
    line[(GRAD_LINE_VEC_INDEX + 0) as usize]    = b;
    line[(GRAD_LINE_VEC_INDEX + 1) as usize]    = -a;

    line
}

#[allow(dead_code)]
fn analy_intersections(
    grad_cfg_list: & GradDataList, 
    grad_line_list: GradLineCfgList, 
    resort_line_list: LineCfgList, 
    resort_line_index_list: LineIndexCfgList, 
    resort_point_list: Vec<PointData>,
    direct_vec2: [f32; 2]
) -> (PointDataList, Vec<Vec<u16>>) {
    let mut result_point_list: PointDataList = Vec::new();
       
    let first_grad_index: u16    = 1;
    let last_grad_index: u16     = (grad_cfg_list.len() as u16) - 2;
    let grad_count: u16          = last_grad_index - first_grad_index + 1;

    let resort_line_count: u16   = resort_line_list.len() as u16;

    let mut resort_line_index: u16 = 0;
    let mut grad_index: u16 = 0;

    let mut grad_result_point_list: GradResultPointCfgList = Vec::new();
    grad_result_point_list.push(Vec::new());
    grad_index = 0;
    while grad_index < grad_count {
        let mut vec = Vec::new();
        grad_result_point_list.push(vec);

        grad_index = grad_index + 1;
    }
    grad_result_point_list.push(Vec::new());

    while resort_line_index < resort_line_count {
        let resort_line: LineCfg            = resort_line_list[(resort_line_index) as usize];
        let resort_line_index_cfg           = resort_line_index_list[(resort_line_index) as usize];
        let p0: PointData                   = resort_point_list[(resort_line_index_cfg[0]) as usize];
        let p1: PointData                   = resort_point_list[(resort_line_index_cfg[1]) as usize];
        let x0  = p0[(POINT_VEC_INDEX) as usize];
        let y0  = p0[(POINT_VEC_INDEX + 1) as usize];
        let x1  = p1[(POINT_VEC_INDEX) as usize];
        let y1  = p1[(POINT_VEC_INDEX + 1) as usize];

        let result_point: PointData = clone_point_data(p0);
        result_point_list.push(result_point);

        let is_same_direct = get_dot(
            resort_line[(LINE_VEC_INDEX + 0) as usize], 
            resort_line[(LINE_VEC_INDEX + 1) as usize], 
            direct_vec2[0], 
            direct_vec2[1]
        ) >= 0.0;

        grad_index = 0;
        while grad_index < grad_count {
            let mut has_intersect: bool = false;

            let mut grad_targ_index: u16 = 0;
            if is_same_direct {
                grad_targ_index = first_grad_index + grad_index;
            } else {
                grad_targ_index = first_grad_index + grad_count - grad_index - 1;
            }

            let grad_cfg    = grad_cfg_list[(grad_targ_index) as usize];
            let grad_line   = grad_line_list[(grad_targ_index) as usize];

            let (is_get, mut x, mut y) = get_intersection(
                (resort_line[(LINE_PARAM_INDEX) as usize],resort_line[(LINE_PARAM_INDEX +1 ) as usize],resort_line[(LINE_PARAM_INDEX + 2) as usize]), 
                (grad_line[(GRAD_LINE_PARAM_INDEX) as usize],grad_line[(GRAD_LINE_PARAM_INDEX +1 ) as usize],grad_line[(GRAD_LINE_PARAM_INDEX + 2) as usize])
            );
            
            // 目标点与端点差距 极小时，可能因为 目标点所在直线斜率 问题导致判断失误
            // 忽略计算机离散数学误差后参与比较
            x = (x * 100000.0).round() / 100000.0;
            y = (y * 100000.0).round() / 100000.0;

            if is_get == true {
                if x0 == x1 {
                    if is_between(y0, y1, y) == true {
                        has_intersect = true;
                    }
                } else if y0 == y1 {
                    if is_between(x0, x1, x) == true {
                        has_intersect = true;
                    }
                }
                else if is_between(x0, x1, x) == true {
                    has_intersect = true;
                }

                if has_intersect {
                    let mut result_point = new_point_data();
                    let grad_result_point: &mut GradResultPointCfg = &mut grad_result_point_list[(grad_targ_index) as usize];

                    result_point[(POINT_POS_DATA_INDEX) as usize] = x;
                    result_point[(POINT_POS_DATA_INDEX + 1) as usize] = y;

                    result_point[(POINT_VEC_INDEX) as usize] = x;
                    result_point[(POINT_VEC_INDEX + 1) as usize] = y;

                    result_point[(POINT_COLOR_DATA_INDEX) as usize]     = grad_cfg[(GRAD_COLOR_INDEX) as usize];
                    result_point[(POINT_COLOR_DATA_INDEX + 1) as usize] = grad_cfg[(GRAD_COLOR_INDEX + 1) as usize];
                    result_point[(POINT_COLOR_DATA_INDEX + 2) as usize] = grad_cfg[(GRAD_COLOR_INDEX + 2) as usize];
                    result_point[(POINT_COLOR_DATA_INDEX + 3) as usize] = grad_cfg[(GRAD_COLOR_INDEX + 3) as usize];
                    
                    result_point[(POINT_DIRECT_INDEX) as usize]         = grad_cfg[(GRAD_DIRECT_INDEX) as usize];

                    // analy_uvs(& p0, & p1, &mut result_point);

                    result_point_list.push(result_point);

                    grad_result_point.push((result_point_list.len() - 1) as u16);
                }
            }
            grad_index = grad_index + 1;
        }
        resort_line_index = resort_line_index + 1;
    }
    
    (result_point_list, grad_result_point_list)
}


/**
 * 分析目标点UV
 * @param prePoint 前一个点
 * @param nextPoint 下一个点
 * @param currPoint 目标点
 */
fn analy_uvs(pre_point: & PointData, nxt_point: & PointData, cur_point:  &mut PointData) {
    let pre_direct = pre_point[(POINT_DIRECT_INDEX) as usize];
    let nxt_direct = nxt_point[(POINT_DIRECT_INDEX) as usize];
    let cur_direct = cur_point[(POINT_DIRECT_INDEX) as usize];
    let percent = (cur_direct - pre_direct) / (nxt_direct - pre_direct);

    let pre_u = pre_point[(POINT_UV_DATA_INDEX) as usize];
    let pre_v = pre_point[(POINT_UV_DATA_INDEX + 1) as usize];

    let nxt_u = nxt_point[(POINT_UV_DATA_INDEX) as usize];
    let nxt_v = nxt_point[(POINT_UV_DATA_INDEX + 1) as usize];

    cur_point[(POINT_UV_DATA_INDEX) as usize]       = pre_u + percent * (nxt_u - pre_u);
    cur_point[(POINT_UV_DATA_INDEX + 1) as usize]   = pre_v + percent * (nxt_v - pre_v);
}

#[allow(dead_code)]
fn analy_triangles(result_point_list: &PointDataList, grad_cfg_list: &GradDataList, grad_result_points: &Vec<Vec<u16>>) -> Vec<u16> {
    let restore_result_index_list = resort_result_points(&result_point_list);
    let mut triangle_indexs: Vec<u16> = Vec::new();
    
    let mut min_grad = grad_cfg_list[1];
    let mut max_grad = grad_cfg_list[1];
    let mut min_grad_result_indexs = &grad_result_points[1];
    let mut max_grad_result_indexs = &grad_result_points[1];

    analy_triangles_lower_min(
        & min_grad,
        & min_grad_result_indexs,
        & result_point_list,
        & restore_result_index_list,
        &mut triangle_indexs
    );

    let count = grad_cfg_list.len() as u16;
    let mut index: u16 = 1;
    while index < count - 2 {
        min_grad = grad_cfg_list[(index) as usize];
        max_grad = grad_cfg_list[(index + 1) as usize];
        min_grad_result_indexs = &grad_result_points[(index) as usize];
        max_grad_result_indexs = &grad_result_points[(index + 1) as usize];

        analy_triangles_between_min_max(
            & min_grad,
            & min_grad_result_indexs,
            & max_grad,
            & max_grad_result_indexs,
            & result_point_list,
            & restore_result_index_list,
            &mut triangle_indexs
        );

        index = index + 1;
    }

    analy_triangles_big_max(
        & max_grad,
        & max_grad_result_indexs,
        & result_point_list,
        & restore_result_index_list,
        &mut triangle_indexs
    );

    triangle_indexs
}

#[allow(dead_code)]
fn resort_result_points(result_point_list: &PointDataList) -> Vec<u16> {
    let count = result_point_list.len();

    let mut resort_result_index_list: Vec<u16> = Vec::new();

    let mut index: u16 = 0;
    while index < count as u16 {
        let point_data = result_point_list[index as usize];

        let count_0 = resort_result_index_list.len();
        let mut index_0: u16 = 0;
        let mut result_index: i16 = -1;
        while index_0 < (count_0 as u16) {
            let comp_index = resort_result_index_list[index_0 as usize];
            let comp_direct = result_point_list[comp_index as usize][(POINT_DIRECT_INDEX) as usize];

            if point_data[(POINT_DIRECT_INDEX) as usize] <= comp_direct {
                result_index = index_0 as i16;
                break;
            }

            index_0 = index_0 + 1;
        }

        if result_index <= 0 {
            resort_result_index_list.push(index);
        } else {
            resort_result_index_list.insert((result_index) as usize, index);
        }

        index = index + 1;
    }

    resort_result_index_list
}

#[allow(dead_code)]
fn analy_triangles_lower_min(min_grad: &GradData, grad_result_point: &Vec<u16>, result_points: &PointDataList, resort_result_indexs: &Vec<u16>, triangle_indexs: &mut Vec<u16>) {
    let mut index_list: Vec<u16> = Vec::new();
    let count = resort_result_indexs.len() as u16;
    let mut index: u16 = 0;
    while index < count {
        let result_index    = resort_result_indexs[(index) as usize];
        let point_data      = result_points[(result_index) as usize];
        let point_direct    = point_data[(POINT_DIRECT_INDEX) as usize];
        let point_is_src    = point_data[(POINT_ISSRC_INDEX) as usize] == 1.0;

        if point_is_src && point_direct < min_grad[(GRAD_DIRECT_INDEX) as usize] {
            index_list.push(result_index);
        }

        index = index + 1;
    }

    let count_0 = grad_result_point.len() as u16;
    let mut index0 = 0;
    while index0 < count_0 {
        index_list.push(grad_result_point[index0 as usize]);
        index0 = index0 + 1;
    }

    index_list.sort();

    analy_triangles_points(&index_list, triangle_indexs);
}

#[allow(dead_code)]
fn analy_triangles_between_min_max(
    min_grad: &GradData, 
    min_grad_result_point: &Vec<u16>, 
    max_grad: &GradData, 
    max_grad_result_point: &Vec<u16>, 
    result_points: &PointDataList, 
    resort_result_indexs: &Vec<u16>, 
    triangle_indexs: &mut Vec<u16>
) {
    let mut index_list: Vec<u16> = Vec::new();
    let count = resort_result_indexs.len() as u16;
    let mut index: u16 = 0;
    while index < count {
        let result_index    = resort_result_indexs[(index) as usize];
        let point_data      = result_points[(result_index) as usize];
        let point_direct    = point_data[(POINT_DIRECT_INDEX) as usize];
        let point_is_src    = point_data[(POINT_ISSRC_INDEX) as usize] == 1.0;
        
        if     point_is_src 
            && min_grad[(GRAD_DIRECT_INDEX) as usize] <= point_direct
            && point_direct < max_grad[(GRAD_DIRECT_INDEX) as usize]
        {
            index_list.push(result_index);
        }

        index = index + 1;
    }

    let count_0 = min_grad_result_point.len() as u16;
    let mut index0 = 0;
    while index0 < count_0 {
        index_list.push(min_grad_result_point[index0 as usize]);
        index0 = index0 + 1;
    }
    
    let count_0 = max_grad_result_point.len() as u16;
    let mut index0 = 0;
    while index0 < count_0 {
        index_list.push(max_grad_result_point[index0 as usize]);
        index0 = index0 + 1;
    }

    index_list.sort();

    analy_triangles_points(&index_list, triangle_indexs);
}

#[allow(dead_code)]
fn analy_triangles_big_max(max_grad: &GradData, grad_result_point: &Vec<u16>, result_points: &PointDataList, resort_result_indexs: &Vec<u16>, triangle_indexs: &mut Vec<u16>) {
    let mut index_list: Vec<u16> = Vec::new();
    let count = resort_result_indexs.len() as u16;
    let mut index: u16 = 0;
    while index < count {
        let result_index    = resort_result_indexs[(index) as usize];
        let point_data      = result_points[(result_index) as usize];
        let point_direct    = point_data[(POINT_DIRECT_INDEX) as usize];
        let point_is_src    = point_data[(POINT_ISSRC_INDEX) as usize] == 1.0;

        if point_is_src && max_grad[(GRAD_DIRECT_INDEX) as usize] < point_direct {
            index_list.push(result_index);
        }

        index = index + 1;
    }

    let count_0 = grad_result_point.len() as u16;
    let mut index0 = 0;
    while index0 < count_0 {
        index_list.push(grad_result_point[index0 as usize]);
        index0 = index0 + 1;
    }

    index_list.sort();

    analy_triangles_points(&index_list, triangle_indexs);
}

#[allow(dead_code)]
fn analy_triangles_points(index_list: &Vec<u16>, triangle_indexs: &mut Vec<u16>) {
    let count = index_list.len() as u16;
    let mut index: u16 = 1;
    while index < count - 1 {
        triangle_indexs.push(index_list[0]);
        triangle_indexs.push(index_list[(index) as usize]);
        triangle_indexs.push(index_list[(index + 1) as usize]);

        index = index + 1;
    }
}

/**
 * 求两直线交点
 * @param line1 目标直线
 * @param line2 目标直线
 */
#[allow(dead_code)]
fn get_intersection(line1: (f32,f32,f32), line2: (f32,f32,f32)) -> (bool, f32, f32) {
    let (a0, b0, c0) = line1;
    let (a1, b1, c1) = line2;

    let d = a0 * b1 - a1 * b0;
    // 线平行
    if d == 0.0 {
        if a0 * b1 == 0.0 && a0 != a1 && b0 != b1 {
            if a0 == 0.0 {
                return (true, c0 / b0, c1 / a1);
            } else {
                return (true, c0 / a0, c1 / b1);
            }
        } else {
            return (false, 0.0, 0.0);
        }
    } else {
        return (true, (b0 * c1 - b1 * c0) / d, (c0 * a1 - c1 * a0) / d);
    }
}

/**
 * 包含起点，不包含终点
 * @param a 起点
 * @param b 终点
 * @param v 目标
 */

#[allow(dead_code)]
fn is_between(a: f32, b: f32, in_v: f32) -> bool {
    // 目标点与端点差距 极小时，可能因为 目标点所在直线斜率 问题导致判断失误
    // 忽略计算机离散数学误差后参与比较
    // let v = (in_v * 10000.0).round() / 10000.0;
    let v = in_v;

    if b < a {
        return b < v && v <= (a + 0.0);
    } else if a < b {
        return (a - 0.0) <= v && v < b;
    } else {
        return a == v;
    }
}