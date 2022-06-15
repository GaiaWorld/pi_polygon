/**
 * 矩形切圆角矩形
 * * input:
 *      x, y, w, h: 矩形参数
 *      radius:     圆角半径
 *      z:          z 定位数据
 *      segment:    切分粒度
 * * output:
 *      points:     点坐标数据流
 *      indices     多边形点序号数据流
 */
// pub fn split_by_radius(x: f32, y: f32, w: f32, h: f32, radius: f32, z: f32, segment: Option<usize>) -> (Vec<f32>, Vec<u16>) {
pub fn split_by_radius(x: f32, y: f32, w: f32, h: f32, radius: f32, segment: Option<usize>) -> (Vec<f32>, Vec<u16>) {
    let mut points:  Vec<f32>;
    let mut indices: Vec<u16> = Vec::new();

    let point_len: usize = 2;

    match segment {
        Some(lv) => {
            // points = split_by_radius_with_level(x, y, w, h, radius, z, scale_level(lv as u16));
            points = split_by_radius_with_level(x, y, w, h, radius, scale_level(lv as u16));
        },
        None    => {
            // points = split_by_radius_0(x, y, w, h, radius, z);
            points = split_by_radius_0(x, y, w, h, radius);
        },
    }

    let mut index = 0;
    let count = points.len() / point_len;
    while index < count {
        indices.push(index as u16);
        index = index + 1;
    }

    (points, indices)
}
/**
 * 矩形切圆角矩形 - 带 边框 
 * * input:
 *      x, y, w, h: 矩形参数
 *      radius:     圆角半径
 *      border:     边框尺寸
 *      z:          z 定位数据
 *      segment:    切分粒度
 * * output:
 *      points:     点坐标数据流
 *      indices     多边形点序号数据流
 */
// pub fn split_by_radius_border(x: f32, y: f32, w: f32, h: f32, radius: f32, border: f32, z: f32, segment: Option<usize>) -> (Vec<f32>, Vec<u16>) {
pub fn split_by_radius_border(x: f32, y: f32, w: f32, h: f32, radius: f32, border: f32, segment: Option<usize>) -> (Vec<f32>, Vec<u16>) {
    match segment {
        Some(lv) => {
            // split_by_radius_border_with_level(x, y, w, h, radius, border, z, scale_level(lv as u16))
            split_by_radius_border_with_level(x, y, w, h, radius, border, scale_level(lv as u16))
        },
        None    => {
            // split_by_radius_border_0(x, y, w, h, radius, border, z)
            split_by_radius_border_0(x, y, w, h, radius, border)
        },
    }
}
/**
 * 根据指定切割线列表，切割出多个多边形
 * input:
 *      positions:  初始点列表
 *      indices:    初始多边形点序号列表
 *      lg_pos:     切割线在切割域内位置
 *      start:      切割域起点
 *      end:        切割域终点
 * output: 
 *      points:     结果点列表
 *      indices:    [结果多边形点序号列表]
 */
pub fn split_by_lg(positions: Vec<f32>, indices: Vec<u16>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> (Vec<f32>, Vec<Vec<u16>>) {
    split_by_lg_0(positions, &indices, lg_pos, start, end)
}

/**
 * 根据指定切割线列表，切割出多个多边形
 * input:
 *      positions:  [初始点列表]
 *      indices:    [初始多边形点序号列表]
 *      lg_pos:     切割线在切割域内位置
 *      start:      切割域起点
 *      end:        切割域终点
 * output: 
 *      points:     结果点列表
 *      indices:    [结果多边形点序号列表]
 */
pub fn split_mult_by_lg(mut positions: Vec<f32>, indices: Vec<Vec<u16>>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> (Vec<f32>, Vec<Vec<u16>>){
    let mut res_indices: Vec<Vec<u16>> = Vec::new();
    for cfg in indices {
        let (_positions, ins) = split_by_lg(positions, cfg, lg_pos, start, end);
        positions = _positions;
        res_indices.extend_from_slice(&ins);
    }

    (positions, res_indices)
}
/**
 * 沿指定方向对指定属性列表做指定点的插值
 * input:
 *      positions   点列表
 *      indices
 *      attrs
 *      lg_attrs
 *      lg_pos
 *      start
 *      end
 * output:
 *      attrs
 */
pub fn interp_by_lg(positions: &[f32], indices: &[u16], attrs: Vec<Vec<f32>>, lg_attrs: &Vec<LgCfg>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> Vec<Vec<f32>> {
    interp_by_lg_0(positions, indices, attrs, lg_attrs, lg_pos, start, end)
}

pub fn interp_mult_by_lg(positions: &[f32], indices: &Vec<Vec<u16>>, mut attrs: Vec<Vec<f32>>, lg_attrs: Vec<LgCfg>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> Vec<Vec<f32>>{
    for cfg in indices {
        attrs = interp_by_lg(positions, cfg, attrs, &lg_attrs, lg_pos, start, end);
    }

    attrs
}

pub fn to_triangle(indices: &[u16], mut out_indices: Vec<u16>) -> Vec<u16> {
    out_indices.extend_from_slice(&to_triangle_0(indices));

    out_indices
}

pub fn mult_to_triangle(indices: &Vec<Vec<u16>>, mut out_indices: Vec<u16>) -> Vec<u16>{
    for cfg in indices {
        out_indices = to_triangle(cfg, out_indices);
    }

    out_indices
}

/**
 * 作角度为angle(单位： 角度)的直线， 计算其与一个凸多边形的交点， 如果只有一个交点， 返回两个相同的点
 * polygon 2 维点
 */
pub fn find_lg_endp(polygon: &[f32], angle: f32) -> ((f32, f32), (f32, f32)) {

    let direct_vec2 = get_direction_vector(angle);
    let mut point_dot: Vec<f32> = Vec::new();

    let point_len: usize = 2;
    let count = polygon.len() / point_len;

    let mut index: usize = 0;
    let mut min_dot = std::f32::MAX;
    let mut max_dot = std::f32::MIN;
    let mut min_index = 0;
    while index < count {
        let x = polygon[index * point_len];
        let y = polygon[index * point_len + 1];
        let dot = get_dot(x, y, direct_vec2[0], direct_vec2[1]);

        if dot < min_dot {
            min_dot     = dot;
            min_index   = index;
        }
        
        if max_dot < dot {
            max_dot = dot;
        }

        point_dot.push(dot);

        index = index + 1;
    }

    let total_dot   = max_dot - min_dot;
    // let ex_size_x   = direct_vec2[0] * 0.0001;
    // let ex_size_y   = direct_vec2[1] * 0.0001;
    // let start_point = (polygon[min_index * 2] - ex_size_x, polygon[min_index * 2 + 1]- ex_size_y);
    // let end_point   = (start_point.0 + total_dot * direct_vec2[0] + ex_size_x, start_point.1 + total_dot * direct_vec2[1] + ex_size_y);
    
    let start_point = (float_clip(polygon[min_index * 2]) , float_clip(polygon[min_index * 2 + 1]));
    let end_point   = (float_clip(start_point.0 + total_dot * direct_vec2[0]) , float_clip(start_point.1 + total_dot * direct_vec2[1]));

    (start_point, end_point)
}

/**
 * 处理带border 的圆角矩形
 * input:
 *      x, y: 左上
 *      w, h：矩形大小
 *      radius： 圆角半径
 *      z:
 *      level: 细分级别 4 / 8 / 16
 * output:
 *      (三维点数据, 多边形点索引)
 */
// pub fn split_by_radius_border_with_level(x: f32, y: f32, w: f32, h: f32, radius: f32, border: f32, z: f32, level: u16) -> (Vec<f32>, Vec<u16>) {
pub fn split_by_radius_border_with_level(x: f32, y: f32, w: f32, h: f32, radius: f32, border: f32, level: u16) -> (Vec<f32>, Vec<u16>) {
    let mut result: Vec<f32> = Vec::new();
    let mut result2: Vec<f32> = Vec::new();
    let mut result_indices: Vec<u16> = Vec::new();
    let mut check_list: Vec<(f32,f32,u8)> = Vec::new();
    check_list.push((x + radius,        y - radius,     2));
    check_list.push((x + radius,        y - h + radius, 3));
    check_list.push((x + w - radius,    y - h + radius, 4));
    check_list.push((x + w - radius,    y - radius,     1));

    // let point_len: usize = 3;
    let point_len: usize = 2;
    for data in check_list {

        let (_x,_y,a) = data;
        // let res     = get_one_quarter_arc_with_level(_x, _y, radius, a, z, level);
        let res     = get_one_quarter_arc_with_level(_x, _y, radius, a, level);
        let mut index = 0;
        for v in res {
            if index % point_len == 1 {
                result.push(y + y - v);
            } else {
                result.push(v);
            }

            index = index + 1;
        }
        
        // let res     = get_one_quarter_arc_with_level(_x, _y, radius - border, a, z, level);
        let res     = get_one_quarter_arc_with_level(_x, _y, radius - border, a, level);
        index = 0;
        for v in res {
            if index % point_len == 1 {
                result2.push(y + y - v);
            } else {
                result2.push(v);
            }
            
            index = index + 1;
        }
    }

    let count = result.len() / point_len;
    
    let mut temp_index: usize = 0;
    while temp_index < count - 1 {
        let indices: Vec<u16> = to_triangle_0(&[
            (temp_index) as u16,
            (temp_index + 1) as u16,
            (count + temp_index + 1) as u16,
            (count + temp_index) as u16
        ]);

        result_indices.extend_from_slice(&indices);

        temp_index = temp_index + 1;
    }
    
    let indices: Vec<u16> = to_triangle_0(&[
        (count - 1) as u16,
        (0) as u16,
        (count + 0) as u16,
        (count + count - 1) as u16
    ]);
    result_indices.extend_from_slice(&indices);

    result.extend_from_slice(&result2);
    (result, result_indices)
}

/**
 * 处理带border 的圆角矩形
 * input:
 *      x, y: 左上
 *      w, h：矩形大小
 *      radius： 圆角半径
 *      z:
 * output:
 *      (三维点数据, 多边形点索引)
 */
// pub fn split_by_radius_border_0(x: f32, y: f32, w: f32, h: f32, radius: f32, border: f32, z: f32) -> (Vec<f32>, Vec<u16>) {
pub fn split_by_radius_border_0(x: f32, y: f32, w: f32, h: f32, radius: f32, border: f32) -> (Vec<f32>, Vec<u16>) {
    let mut result: Vec<f32> = Vec::new();
    let mut result2: Vec<f32> = Vec::new();
    let mut result_indices: Vec<u16> = Vec::new();
    let mut check_list: Vec<(f32,f32,u8)> = Vec::new();
    check_list.push((x + radius,        y - radius,     2));
    check_list.push((x + radius,        y - h + radius, 3));
    check_list.push((x + w - radius,    y - h + radius, 4));
    check_list.push((x + w - radius,    y - radius,     1));

    // let point_len: usize = 3;
    let point_len: usize = 2;
    for data in check_list {

        let (_x,_y,a) = data;
        // let res     = get_one_quarter_arc(_x, _y, radius, a, z);
        let res     = get_one_quarter_arc(_x, _y, radius, a);
        let level   = res.len() / point_len - 1;
        let mut index = 0;
        for v in res {
            if index % point_len == 1 {
                result.push(y + y - v);
            } else {
                result.push(v);
            }

            index = index + 1;
        }
        
        // let res = get_one_quarter_arc_with_level(_x, _y, radius - border, a, z, level as u16);
        let res = get_one_quarter_arc_with_level(_x, _y, radius - border, a, level as u16);
        index = 0;
        for v in res {
            if index % point_len == 1 {
                result2.push(y + y - v);
            } else {
                result2.push(v);
            }
            
            index = index + 1;
        }
    }

    let count = result.len() / point_len;
    
    let mut temp_index: usize = 0;
    while temp_index < count - 1 {
        let indices: Vec<u16> = to_triangle_0(&[
            (temp_index) as u16,
            (temp_index + 1) as u16,
            (count + temp_index + 1) as u16,
            (count + temp_index) as u16
        ]);

        result_indices.extend_from_slice(&indices);

        temp_index = temp_index + 1;
    }
    
    let indices: Vec<u16> = to_triangle_0(&[
        (count - 1) as u16,
        (0) as u16,
        (count + 0) as u16,
        (count + count - 1) as u16
    ]);
    result_indices.extend_from_slice(&indices);

    result.extend_from_slice(&result2);
    (result, result_indices)
}

/**
 * 将一个矩形转化为圆角矩形， 返回多边形的顶点流， 顶点为三维顶点
 * 多边形方向：逆时针
 */
// pub fn split_by_radius_0(x: f32, y: f32, w: f32, h: f32, radius: f32, z: f32) -> Vec<f32> {
pub fn split_by_radius_0(x: f32, y: f32, w: f32, h: f32, radius: f32) -> Point2D_Vec {
    // get_rounded_rect(x, y, w, h, radius, z)
    get_rounded_rect(x, y, w, h, radius)
}

/**
 * 将一个矩形转化为圆角矩形， 返回多边形的顶点流， 顶点为三维顶点
 * 多边形方向：逆时针
 */
// pub fn split_by_radius_with_level(x: f32, y: f32, w: f32, h: f32, radius: f32, z: f32, level: u16) -> Vec<f32> {
pub fn split_by_radius_with_level(x: f32, y: f32, w: f32, h: f32, radius: f32, level: u16) -> Point2D_Vec {
    // get_rounded_rect_with_level(x, y, w, h, radius, z, level)
    get_rounded_rect_with_level(x, y, w, h, radius, level)
}
/**
 * (点数据流, [多边形顶点...])
 */
pub type PolygonCfg = (Vec<f32>, Vec<Vec<u16>>);
/**
 * (x, y, z)
 */
pub type Point3D = (f32, f32, f32);
/**
 * (x, y) 
 */
pub type Point2D = (f32, f32);

/**
 * 将一个多边形按照指示方向的多个空间线切分， 返回新的多边形
 * input:
 *      points: 初始点数据集 - 三维数据
 *      polygon_indices: 初始多边形点索引列表
 *      lg_pos: 分割百分比列表
 *      start： 分割方向的起点
 *      end： 分割方向的终点
 * output:
 *      (点数据流, [多边形顶点...])
 *      点数据流：在 points 后续添加点数据，
 *      [多边形顶点...]: 分割出来的多各多边形的索引列表， 索引列表为 points 中点索引
 */
pub fn split_by_lg_0(mut points: Vec<f32>, polygon_indices: &Vec<u16>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> PolygonCfg {
    let _end    =  (end.0, -end.1);
    let _start  =  (start.0, -start.1);
    let dist_x  = _end.0 - _start.0;
    let dist_y  = _end.1 - _start.1;
    let dist    = (dist_x.powi(2) + dist_y.powi(2)).sqrt();
    // let z: f32  = points[2];
    let lg_lines: Vec<LineCfg> = line_segment_vertical_lines(&_start, &_end, lg_pos);

    // let pointe_len: usize       = 3;
    let pointe_len: usize       = 2;

    let src_points_count        = points.len() / pointe_len;
    let src_polygon_point_count = polygon_indices.len();
    let src_lg_count            = lg_pos.len();
    let mut new_indices_list: Vec<u16>  = Vec::new();
    let mut new_dot_list: Vec<f32>      = Vec::new();

    // 方向上单位向量
    let direct_vec2 = [(_end.0 - _start.0) / dist , (_end.1 - _start.1) / dist];

    let mut lg_dot: Vec<f32> = Vec::new();
    for percent in lg_pos {
        lg_dot.push(get_dot(direct_vec2[0], direct_vec2[1], direct_vec2[0] * dist * percent, direct_vec2[1] * dist * percent));
        // lg_dot.push(dist * percent);
    }

    // 多边形各点在方向上的点积
    // let mut point0: Point3D;
    // let mut point1: Point3D;
    let mut point0: Point2D;
    let mut point1: Point2D;
    let mut line0: LineCfg;
    // println!("lg_lines: {:?}", lg_lines);

    // point0 = read_point_3d_f(&points, polygon_indices, src_polygon_point_count - 1);
    point0 = read_point_2d_f(&points, polygon_indices, src_polygon_point_count - 1);
    // println!("==================\n {:?}", points);

    let temp_dot = get_dot(direct_vec2[0], direct_vec2[1], point0.0 - _start.0, point0.1 - _start.1);
    new_indices_list.push(polygon_indices[src_polygon_point_count - 1]);
    new_dot_list.push(temp_dot);

    let mut result_count: u16 = src_points_count as u16;


    let mut index = 0;
    while index < src_polygon_point_count {

        // point1  = read_point_3d_f(&points, polygon_indices, index);
        point1  = read_point_2d_f(&points, polygon_indices, index);

        // println!("===========================边与界限求交点");

        if !(point0.0 == point1.0 && point0.1 == point1.1) {
            line0   = get_line_with_two_point(point0.0, point0.1, point1.0, point1.1);

            // println!("===========================获得边");
            // println!("{:?}", point1);

            if get_dot(direct_vec2[0], direct_vec2[1], point1.0 - point0.0, point1.1 - point0.1) > 0.0 {

                // println!("===========================获得边 - dot > 0.0");

                let l_count = lg_lines.len();
                let mut l_index: usize = 0;

                while l_index < l_count {
                    
                    // println!("===========================获得边 与 界限交点");

                    let (is_get, _x, _y) = get_two_lines_intersection(line0, lg_lines[l_index]);
                    let mut has_intersect = false;
                    
                    // println!("交点： x: {:?} --- y: {:?}", _x, _y);
                    if is_get {
                        if point0.0 == point1.0 {
                            // println!("满足条件01");
                            if is_between(point0.1, point1.1, _y) == true {
                                    has_intersect = true;
                            }
                        } else if point0.1 == point1.1 {
                            // println!("满足条件02");
                            if is_between(point0.0, point1.0, _x) == true {
                                has_intersect = true;
                            }
                        }
                        else if is_between(point0.0, point1.0, _x) == true {
                            // println!("满足条件03");
                            has_intersect = true;
                        }
                    }
                    if has_intersect {
                        // println!("满足条件04");
                        if !((point0.0 == _x && point0.1 == _y) || (point1.0 == _x && point1.1 == _y))  {
                            
                            // println!("满足条件05");

                            result_count = result_count + 1;
                            // points.extend_from_slice(&[_x, -_y, z]);
                            points.extend_from_slice(&[_x, -_y]);

                            new_indices_list.push((result_count - 1) as u16);
                            new_dot_list.push(lg_dot[l_index]);

                            // println!("新增： x: {:?} ---- y: {:?}", _x, -_y);
                        }
                    }
                    
                    l_index = l_index + 1;
                }
            } else {

                // println!("===========================获得边 - dot <= 0.0");

                let l_count = lg_lines.len();
                let mut l_index: usize = 0;

                while l_index < l_count {
                    // println!("===========================获得边 与 界限交点");

                    let (is_get, _x, _y) = get_two_lines_intersection(line0, lg_lines[l_count - l_index - 1]);

                    let mut has_intersect = false;
                    
                    // println!("交点： x: {:?} --- y: {:?}", _x, _y);

                    if is_get {
                        if point0.0 == point1.0 {
                            // println!("满足条件11");
                            if is_between(point1.1, point0.1, _y) == true {
                                    has_intersect = true;
                            }
                        } else if point0.1 == point1.1 {
                            // println!("满足条件12");
                            if is_between(point1.0, point0.0, _x) == true {
                                has_intersect = true;
                            }
                        }
                        else if is_between(point1.0, point0.0, _x) == true {
                            // println!("满足条件13");
                            has_intersect = true;
                        }
                    }

                    if has_intersect {
                        // println!("满足条件14");
                        if !((point0.0 == _x && point0.1 == _y) || (point1.0 == _x && point1.1 == _y)) {
                            // println!("满足条件15");
                            result_count = result_count + 1;
                            // points.extend_from_slice(&[_x, -_y, z]);
                            points.extend_from_slice(&[_x, -_y]);
                            
                            new_indices_list.push((result_count - 1) as u16);
                            new_dot_list.push(lg_dot[l_count - l_index - 1]);

                            // println!("新增： x: {:?} ---- y: {:?}", _x, -_y);
                        }
                    }

                    l_index = l_index + 1;
                }
            }
        }


        if index < src_polygon_point_count - 1 {
            let temp_dot = get_dot(direct_vec2[0], direct_vec2[1], point1.0 - _start.0, point1.1 - _start.1);
            new_indices_list.push(polygon_indices[index]);
            new_dot_list.push(temp_dot);
        }

        point0 = point1;

        index = index + 1;
    }

    let mut new_polygon_indices_list: Vec<Vec<u16>> = Vec::new();
    let mut min_dot: f32;
    let mut max_dot: f32;

    let new_point_count = new_indices_list.len();
    
    index   = 0;
    min_dot = lg_dot[index];

    let mut i_index = 0;
    let mut new_polygon: Vec<u16> = Vec::new();
    while i_index < new_point_count {
        if new_dot_list[i_index] <= min_dot {
            new_polygon.push(new_indices_list[i_index]);
        }

        i_index = i_index + 1;
    }
    if new_polygon.len() > 2 {
        new_polygon_indices_list.push(new_polygon);
    }

    index = 0;
    while index < src_lg_count - 1 {
        min_dot = lg_dot[index];
        max_dot = lg_dot[index + 1];

        i_index = 0;
        let mut new_polygon: Vec<u16> = Vec::new();
        while i_index < new_point_count {
            if min_dot <= new_dot_list[i_index] && new_dot_list[i_index] <= max_dot {
                new_polygon.push(new_indices_list[i_index]);
            }

            i_index = i_index + 1;
        }
        if new_polygon.len() > 2 {
            new_polygon_indices_list.push(new_polygon);
        }

        index = index + 1;
    }
    
    index   = src_lg_count - 1;
    max_dot = lg_dot[index];

    let mut new_polygon: Vec<u16> = Vec::new();
    i_index = 0;
    while i_index < new_point_count {
        if max_dot <= new_dot_list[i_index] {
            new_polygon.push(new_indices_list[i_index]);
        }

        i_index = i_index + 1;
    }
    if new_polygon.len() > 2 {
        new_polygon_indices_list.push(new_polygon);
    }

    // println!("split_by_lg_0: {:?}\n", points);
    // println!("split_by_lg_0: {:?}\n", new_polygon_indices_list);

    (points, new_polygon_indices_list)
}

fn read_point_3d_f(points: &[f32], indices: &[u16], indices_index: usize) -> Point3D {
    let ix = (indices[indices_index] * 3) as usize;
    (points[ix + 0], -points[ix + 1], points[ix + 2])
}

fn read_point_2d_f(points: &[f32], indices: &[u16], indices_index: usize) -> Point2D {
    let ix = (indices[indices_index] * 2) as usize;
    (points[ix + 0], -points[ix + 1])
}

fn read_point_3d(points: &[f32], indices: &[u16], indices_index: usize) -> Point3D {
    let ix = (indices[indices_index] * 3) as usize;
    (points[ix + 0], points[ix + 1], points[ix + 2])
}

fn read_point_2d(points: &[f32], indices: &[u16], indices_index: usize) -> Point2D {
    let ix = (indices[indices_index] * 2) as usize;
    (points[ix + 0], points[ix + 1])
}

#[derive(Debug, Clone)]
pub struct LgCfg{
    /**
     * 该属性长度
     */
    pub unit: usize,
    pub data: Vec<f32>,
}
// 将属性流按照线性渐变插值,  返回插值后的属性流
/**
 * in：
 *      points   三维点数据
 *      attrs   多个属性的 各点属性数据集 
 *      polygon_indices   多边形顶点数据
 *      lg_attrs 多种属性值列表
 *      lg_pos   插值区间列表
 *      start    插值计算起点
 *      end    插值计算终点
 * out:
 *      points 中点的属性数据流
 */
pub fn interp_by_lg_0(points: &[f32], polygon_indices: &[u16], mut attrs: Vec<Vec<f32>>, lg_attrs: &Vec<LgCfg>, lg_pos: &[f32], start: (f32, f32), end: (f32, f32)) -> Vec<Vec<f32>> {
    let dist_x  = end.0 - start.0;
    let dist_y  = end.1 - start.1;
    let dist    = (dist_x.powi(2) + dist_y.powi(2)).sqrt();
    let point_count = polygon_indices.len();


    let lg_count    = lg_pos.len();

    let attr_count  = lg_attrs.len();

    if attrs.len() == 0 {
        let mut index = 0;
        while index < attr_count {
            attrs.push(Vec::new());
            index = index + 1;
        }
    }

    // 有效属性，在插值方向上的起始
    let start_lg    = lg_pos[0];
    let end_lg      = lg_pos[lg_count - 1];
    // 方向上单位向量
    let direct_vec2 = [(end.0 - start.0) / dist, (end.1 - start.1) / dist];

    // let mut point: Point3D;
    let mut point: Point2D;
    
    let mut index = 0;
    while index < point_count {
        // point   = read_point_3d(points, polygon_indices, index);
        point   = read_point_2d(points, polygon_indices, index);
        let dot     = get_dot(point.0 - start.0, point.1 - start.1, direct_vec2[0], direct_vec2[1]);

        let mut attr_index: usize = 0;
        let lg = dot / dist;
        while attr_index < attr_count {
            let src_attr = &lg_attrs[attr_index];
            let pre: usize;
            let nxt: usize;
            let percent: f32;
            if lg <= start_lg {
                percent = 0.0;
                pre = 0;
                nxt = 1;
            } else if lg >= end_lg {
                percent = 1.0;
                pre = lg_count - 2;
                nxt = lg_count - 1;
            } else {
                let (_pre, _nxt) = find_pre_next_grad_direct(lg_pos, lg);
                pre = _pre;
                nxt = _nxt;
                if pre != nxt {
                    percent = (lg - lg_pos[pre]) / (lg_pos[nxt] - lg_pos[pre]);
                } else {
                    percent = 0.0;
                }
            }


            let mut pre_attr: f32;
            let mut nxt_attr: f32;
            let attr_size = src_attr.unit;
            let point_attr_index: u16 = (attr_size as u16) * polygon_indices[index];

            // 填充属性数据直到到当前点位置
            // let mut _l = attrs[attr_index as usize];
            // _l = fill_vec(_l, point_attr_index + (attr_size as u16), 0.0);
            // attrs[attr_index] = _l;
            let mut _cur_len    = attrs[attr_index as usize].len() as u16;
            let _targ_len       = point_attr_index + (attr_size as u16);
            while _cur_len < _targ_len {
                attrs[attr_index as usize].push(0.0);
                _cur_len = _cur_len + 1;
            }

            let mut a_index = 0;
            while a_index < attr_size {
                pre_attr = src_attr.data[pre * attr_size + a_index];
                nxt_attr = src_attr.data[nxt * attr_size + a_index];

                attrs[attr_index][point_attr_index as usize + a_index] = float_clip(pre_attr + (nxt_attr - pre_attr) * percent);

                a_index = a_index + 1;
            }

            attr_index = attr_index + 1;
        }

        index = index + 1;
    }

    attrs
}

fn insert_vec(mut data_list: Vec<f32>, data: &[f32], size: u16, index: u16) -> Vec<f32> {
    let trag_len = size * (index + 1);
    let mut curr_len = data_list.len() as u16;
    while curr_len < trag_len {
        data_list.push(0.0);

        curr_len = curr_len + 1;
    }

    let i_index = size * (index as u16);
    let mut i: u16 = 0;
    while i < size {
        data_list[(i_index + i) as usize] = data[i as usize];

        i = i + 1;
    }

    data_list
}

fn fill_vec(mut data_list: Vec<f32>, size: u16, value: f32) -> Vec<f32> {
    let trag_len = size;
    let mut i_index = data_list.len() as u16;

    while i_index < trag_len {
        data_list.push(value);
        
        i_index = i_index + 1;
    }

    data_list
}

//将多边形转换为三角形
pub fn to_triangle_0(indices: &[u16]) -> Vec<u16> {
    let mut res_indices: Vec<u16> = Vec::new();
    let mut index: usize = 1;
    let count = indices.len();
    while index <= count - 2 {
        res_indices.push(indices[0]);
        res_indices.push(indices[index]);
        res_indices.push(indices[index + 1]);

        index = index + 1;
    }

    res_indices
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
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
fn scale_level(mut lv: u16) -> u16 {
    if lv <= 4 {
        lv = 4;
    } else if lv <= 8 {
        lv = 8;
    } else {
        lv = 16;
    }

    lv
}
/**
 * 正方形切 4分之一圆弧
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
// pub fn get_one_quarter_arc(center_x: f32, center_y: f32, radius: f32, area_id: u8, z: f32) -> Vec<f32> {
pub fn get_one_quarter_arc(center_x: f32, center_y: f32, radius: f32, area_id: u8) -> Point2D_Vec {
    let mut segments: Vec<u16>;

    if radius < RADIUS_4_8 {
        segments    = copy_level4();
    } else if radius <= RADIUS_8_16 {
        segments    = copy_level8();
    } else {
        segments    = copy_level16();
    }

    // analy_one_quarter_arc(center_x, center_y, radius, area_id, z, &segments)
    analy_one_quarter_arc(center_x, center_y, radius, area_id, &segments)
}

// pub fn get_one_quarter_arc_with_level(center_x: f32, center_y: f32, radius: f32, area_id: u8, z: f32, level: u16) -> Vec<f32> {
pub fn get_one_quarter_arc_with_level(center_x: f32, center_y: f32, radius: f32, area_id: u8, level: u16) -> Point2D_Vec {
    let mut segments: Vec<u16>;

    if level == 4 {
        segments    = copy_level4();
    } else if level == 8 {
        segments    = copy_level8();
    } else {
        segments    = copy_level16();
    }

    // analy_one_quarter_arc(center_x, center_y, radius, area_id, z, &segments)
    analy_one_quarter_arc(center_x, center_y, radius, area_id, &segments)
}
/**
 * @return Vec<f32> : [x0, y0, x1 ,y1 ... ]
 */
// fn analy_one_quarter_arc(center_x: f32, center_y: f32, radius: f32, area_id: u8, z: f32, segments: &Vec<u16>) -> Vec<f32> {
fn analy_one_quarter_arc(center_x: f32, center_y: f32, radius: f32, area_id: u8, segments: &Vec<u16>) -> Point2D_Vec {
    
    let mut result_points: Vec<f32> = Vec::new();

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
        // result_points.push(z);

        index = index + 1;
    }

    result_points
}


/**
 * 矩形切 圆角
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
// pub fn get_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, z: f32) -> Vec<f32> {
pub fn get_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32) -> Point2D_Vec {
    let point_len: usize = 2;
    let mut result: Vec<f32> = Vec::new();
    let mut check_list: Vec<(f32,f32,u8)> = Vec::new();
    check_list.push((x + radius,        y - radius,     2));
    check_list.push((x + radius,        y - h + radius, 3));
    check_list.push((x + w - radius,    y - h + radius, 4));
    check_list.push((x + w - radius,    y - radius,     1));

    for data in check_list {
        let (_x,_y,a) = data;
        let res = get_one_quarter_arc(_x, _y, radius, a);
        let mut index = 0;
        for v in res {
            if index % point_len == 1 {
                result.push(y + y - v);
            } else {
                result.push(v);
            }

            index = index + 1;
        }
    }

    result
}

// pub fn get_rounded_rect_with_level(x: f32, y: f32, w: f32, h: f32, radius: f32, z: f32, level: u16) -> Vec<f32> {
pub fn get_rounded_rect_with_level(x: f32, y: f32, w: f32, h: f32, radius: f32, level: u16) -> Vec<f32> {
    let point_len: usize = 2;
    let mut result: Vec<f32> = Vec::new();
    let mut check_list: Vec<(f32,f32,u8)> = Vec::new();
    check_list.push((x + radius,        y - radius,     2));
    check_list.push((x + radius,        y - h + radius, 3));
    check_list.push((x + w - radius,    y - h + radius, 4));
    check_list.push((x + w - radius,    y - radius,     1));

    for data in check_list {
        let (_x,_y,a) = data;
        // let res = get_one_quarter_arc_with_level(_x, _y, radius, a, z, level);
        let res = get_one_quarter_arc_with_level(_x, _y, radius, a, level);
        // println!("{:?}", res);
        // println!("-------------------------------------");
        let mut index = 0;
        for v in res {
            if index % point_len == 1 {
                result.push(y + y - v);
            } else {
                result.push(v);
            }

            index = index + 1;
        }
    }

    result
}

///////////////////////////////////////////////////////////////////////////////////////
/**
 * input
 *      起点
 *          [x, y]
 *      终点
 *          [x, y]
 *      划分数据
 *          [percent...]
 * output
 *      lines
 *          [
 *              a, b, c
 *          ]
 */
pub type LineCfg = (f32, f32, f32);

fn line_segment_vertical_lines(start_point: &(f32, f32), end_point: &(f32, f32), percents: &[f32]) -> Vec<LineCfg> {
    let dist_x  = end_point.0 - start_point.0;
    let dist_y  = end_point.1 - start_point.1;
    let len     = (dist_x.powi(2) + dist_y.powi(2)).sqrt();
    let direct_vec2 = [ dist_x / len, dist_y / len];
    let v_vec2      = get_vertical_vec2(direct_vec2);
    
    let mut lines: Vec<LineCfg> = Vec::new();
    for percent in percents {
        let percent_len = len * percent;
        let point       = [start_point.0 + direct_vec2[0] * percent_len, start_point.1 + direct_vec2[1] * percent_len];
        let line_cfg    = get_line_with_direct(v_vec2, point[0], point[1]);
        lines.push(line_cfg);
    }

    lines
}

fn get_vertical_vec2(direct_vec2: [f32;2]) -> [f32;2] {
    [direct_vec2[1],-direct_vec2[0]]
}

fn get_line_with_direct(direct_vec2: [f32;2], x: f32, y: f32) -> (f32, f32, f32) {
    let new_x = direct_vec2[0] + x;
    let new_y = direct_vec2[1] + y;
    get_line_with_two_point(x, y, new_x, new_y)
}

fn get_line_with_two_point(x0: f32, y0: f32, x1: f32, y1: f32) -> (f32, f32, f32) {
    if x0 == x1 && y0 == y1 {
        (1.0, -1.0, 0.0)
    } else {
        (y0 - y1, x1 - x0, x0 * y1 - y0 * x1)
    }
}

fn get_line_with_slop(slop: f32, x: f32, y: f32) -> (f32, f32, f32) {
    (slop, -1.0, y - slop * x)
}

/**
 * 求两直线交点
 * @param line1 目标直线
 * @param line2 目标直线
 */
fn get_two_lines_intersection(line1: LineCfg, line2: LineCfg) -> (bool, f32, f32) {
    let (a0, b0, c0) = line1;
    let (a1, b1, c1) = line2;

    let d = a0 * b1 - a1 * b0;
    let mut x = 0.0;
    let mut y = 0.0;
    let is_get;

    // 线平行
    if d == 0.0 {
        if a0 * b1 == 0.0 && a0 != a1 && b0 != b1 {
            if a0 == 0.0 {
                is_get = true;
                x = c0 / b0;
                y = c1 / a1;
            } else {
                is_get = true;
                x = c0 / a0;
                y = c1 / b1;
            }
        } else {
            is_get = false;
        }
    } else {
        is_get = true;
        x = (b0 * c1 - b1 * c0) / d;
        y = (c0 * a1 - c1 * a0) / d;
    }

    // 目标点与端点差距 极小时，可能因为 目标点所在直线斜率 问题导致判断失误
    // 忽略计算机离散数学误差后参与比较
    if x.is_nan() {
        x = 0.0;
    }
    x = float_clip(x);
    y = float_clip(y);

    (is_get, x, y)
}

fn get_dot(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    float_clip(x0 * x1 + y0 * y1)
}

/**
 * 包含起点，不包含终点
 * @param a 起点
 * @param b 终点
 * @param v 目标
 */
fn is_between(a: f32, b: f32, in_v: f32) -> bool {
    // 目标点与端点差距 极小时，可能因为 目标点所在直线斜率 问题导致判断失误
    let v = in_v;

    if b < a {
        return b < v && v <= (a + 0.0);
    } else if a < b {
        return (a - 0.0) <= v && v < b;
    } else {
        return a == v;
    }
}

fn find_pre_next_grad_direct(data_list: &[f32], value: f32) -> (usize, usize) {
    let count = data_list.len();
    let mut pre_index: usize = 0;
    let mut nxt_index: usize = 0;

    let mut index: usize = 0;
    while index <= (count as usize) - 1 {
        if data_list[(index) as usize] <= value {
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

/**
 * 获得指定方向的 单位方向向量
 */
fn get_direction_vector(angle: f32) -> [f32; 2] {

    let _angle   = angle % 360.0;

    let radius  = ((_angle as f32) / 180.0) * std::f32::consts::PI;

    [radius.cos(), radius.sin()]
}

fn float_clip(v: f32) -> f32 {
    (v * 10000.0).round() / 10000.0
}

type Point2D_Vec = Vec<f32>;

#[test]
fn test() {
    // let pos_list = vec![
    //     0.0, 0.0,
    //     0.5, 0.0,
    //     0.5, 0.5,
    //     0.0, 0.5
    // ];
    // let uv_list = vec![];
    // let grad_list = vec![
    //     grad_analy::GradCfg {
    //         percent: 0.0, r: 1.0, g: 0.0,   b: 0.0,   a: 1.0
    //     },
    //     grad_analy::GradCfg {
    //         percent: 0.25, r: 1.0, g: 1.0,   b: 0.0,   a: 1.0
    //     },
    //     grad_analy::GradCfg {
    //         percent: 0.5, r: 1.0, g: 0.0,   b: 1.0,   a: 1.0
    //     },
    //     grad_analy::GradCfg {
    //         percent: 0.75, r: 1.0, g: 1.0,   b: 1.0,   a: 1.0
    //     },
    //     grad_analy::GradCfg {
    //         percent: 1.0, r: 1.0, g: 0.0,   b: 0.0,   a: 1.0
    //     }
    // ];
    // let rect = vec![0.0, 0.0, 0.0, 100.0, 100.0, 100.0, 100.0, 0.0];
    
    // let (points, indices) = split_by_radius(0.0, 0.0, 30.0, 30.0, 15.0, 0.1, Some(8));
    // println!("{:?}", points);
    // println!("=======================");
    // println!("{:?}", indices);
    // println!("=======================");

    // let points = vec![0.0, 0.0, 1.0, 0.0, 100.0, 1.0,  100.0, 100.0, 1.0, 100.0, 0.0, 1.0];
    // let (p1, p2) = find_lg_endp(&points, 270.0);
    // println!("{:?}", p1);
    // println!("=======================1");
    // println!("{:?}", p2);
    // println!("=======================2");

    // grad_analy::grad_analy(pos_list, uv_list, grad_list, 50);
    // grad_analy::polygon_grad_analy(pos_list, uv_list, grad_list, 45.0, 1.0);

    // let _sdl = sdl2::init().unwrap();

    // let res = find_lg_endp(&[0.0, 0.0, 0.0, 700.0, 1000.0, 700.0, 1000.0, 0.0], 30.0);
    // println!("{:?}", res);

    // let res = split_by_lg(
    //     vec![0.0, 0.0, -0.1, 0.0, 100.0, -0.1, 100.0, 100.0, -0.1, 100.0, 0.0, -0.1], 
    //     vec![0,1,2,3],
    //     &vec![0.5], 
    //     (0.0,0.0), 
    //     (100.0, 100.0)
    // );
    // println!("{:?}", res);

    // let p1 = (-0.8648649, 3.4594595);
    // let p2 = (33.547604, 23.327509);

    // let points = vec![-0.8648649, 3.4594595, 1.1, -0.8648649, 32.0, 1.1, 28.54054, 32.0, 1.1, 28.54054, 3.4594595, 1.1];
    // let indices = vec![0,1,2,3];
    
    // let (points0, indices0) = split_by_lg(
    //     points, 
    //     indices,
    //     &vec![0.0, 1.0], 
    //     p1, 
    //     p2
    // );
    // println!("{:?}", points0);
    // println!("=======================000");
    // println!("{:?}", indices0);
    // println!("=======================00");
    
    // let attrs = vec![];
    // let res = interp_mult_by_lg(
    //     &points0,
    //     &indices0,
    //     attrs, 
    //     vec![ LgCfg { unit: 1, data: vec![0.0, 0.5, 1.0] }], 
    //     &vec![0.0, 0.5, 1.0],
    //     (0.00, 0.00), 
    //     (100.0, 100.0)
    // );
    // println!("{:?}", res);
    // println!("=======================0");

    // let attrs = res;
    // let res = interp_by_lg(
    //     &points0,
    //     &indices0[1],
    //     attrs, 
    //     &vec![ LgCfg { unit: 1, data: vec![0.0, 0.5, 1.0] }], 
    //     &vec![0.0, 0.5, 1.0],
    //     (0.00, 0.00), 
    //     (100.0, 100.0)
    // );
    // println!("{:?}", res);

    // println!("{:?}", res);
    //     let res = interp_by_lg(
    //     &points0,
    //     &indices0[0],
    //     vec![vec![0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0],vec![100.0, 100.0, 136.60254, 136.60254, 200.0, 200.0, 163.39746, 163.39746]], 
    //     &vec![ LgCfg { unit: 2, data: vec![0.0, 1.0, 0.0, 1.0] }, LgCfg { unit: 2, data: vec![100.0, 100.0, 200.0, 200.0] }], 
    //     &vec![0.0, 1.0],
    //     (0.00, 0.00), 
    //     (118.30127, 68.30127)
    // );
    // println!("{:?}", res);

    let res = split_by_radius(0.0,0.0,50.0,50.0,5.0,Some(3));
    println!("{:?}", res);

    // let res = tool::polygon_tool::straight_line_cut_polygon();
    // println!("{:?}", res);
}
