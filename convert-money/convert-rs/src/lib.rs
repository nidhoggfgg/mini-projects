use std::collections::HashMap;

pub fn convert(num: f64) -> Option<String> {
    let splited = match split(num) {
        Some(s) => s,
        None => return None,
    };

    // 整数部分
    let mut int = splited.0;

    // 超过 9999_9999_9999_9999_9999
    if int.len() > 16 {
        return None;
    }

    // 结果
    let mut result = String::with_capacity(16);
    // 整数部分为零
    let full_zero = *int.last().unwrap() == '0' && int.len() == 1;
    // 量词后缀
    let suffix = ["仟", "佰", "拾", ""];
    // 数字到中文
    let transer = HashMap::from([
        ('0', "零"),
        ('1', "壹"),
        ('2', "贰"),
        ('3', "叁"),
        ('4', "肆"),
        ('5', "伍"),
        ('6', "陆"),
        ('7', "柒"),
        ('8', "捌"),
        ('9', "玖"),
    ]);
    // 主要的量词后缀
    let primarys = ["万亿", "亿", "万", ""];

    // 分割整数部分到 4 个为一组的单元，不足位补零
    let mut chunks = Vec::new();
    while !int.is_empty() {
        chunks.push(split_chunk(&mut int));
    }

    // 整数部分
    let mut is_zero = false;
    for (i, chunk) in chunks.iter().rev().enumerate() {
        let index = 4 - chunks.len() + i;
        let (tmp, (start_zero, end_zero)) =
            convert_chunk(chunk, &suffix, &transer, primarys[index]);

        // 下次循环再处理，因为不知道后面的数据
        if tmp.is_empty() {
            is_zero = true;
            continue;
        }

        if i > 0 && (is_zero || start_zero) {
            result.push('零');
        }
        result.push_str(&tmp);

        is_zero = end_zero;
    }

    if !full_zero {
        result.push('圆');
    }

    // 小数部分
    let float = splited.1;

    if !full_zero && float[0] == '0' && float[1] == '0' {
        result.push('整');
        return Some(result);
    }

    if float[0] != '0' {
        result.push_str(transer.get(&float[0]).unwrap());
        result.push('角');
    }

    if float[1] != '0' {
        result.push_str(transer.get(&float[1]).unwrap());
        result.push('分');
    }

    Some(result)
}

fn split(num: f64) -> Option<(Vec<char>, [char; 2])> {
    if num.is_sign_negative() || (!num.is_normal() && num != 0.0 ){
        return None;
    }

    let num = format!("{}", num);
    let num: Vec<&str> = num.split('.').collect();

    let mut fracs = ['0'; 2];
    if num.len() == 2 {
        let mut cs = num[1].chars();
        fracs[0] = cs.next().unwrap_or('0');
        fracs[1] = cs.next().unwrap_or('0');
    }

    let int: Vec<char> = num[0].chars().collect();
    Some((int, fracs))
}

fn split_chunk(arr: &mut Vec<char>) -> [char; 4] {
    let mut i = 4;
    let mut result = ['0'; 4];
    while !arr.is_empty() && i > 0 {
        i -= 1;
        result[i] = arr.pop().unwrap();
    }

    result
}

fn convert_chunk(
    chunk: &[char; 4],
    prefix: &[&str],
    transer: &HashMap<char, &str>,
    primary: &str,
) -> (String, (bool, bool)) {
    let mut result = String::new();

    if chunk[0] == '0' && chunk[1] == '0' && chunk[2] == '0' && chunk[3] == '0' {
        return (result, (true, true));
    }

    // 可填入 0 标志
    let mut zero = false;

    let make_unit = |i: usize| format!("{}{}", transer.get(&chunk[i]).unwrap_or(&"?"), prefix[i]);

    if chunk[0] != '0' {
        result = make_unit(0);
        zero = true;
    }

    if chunk[1] != '0' {
        let tmp = make_unit(1);
        result.push_str(&tmp);
        zero = true;
    } else if zero && (chunk[2] != '0' || chunk[3] != '0') {
        result.push('零');
        zero = false;
    }

    if chunk[2] != '0' {
        let tmp = make_unit(2);
        result.push_str(&tmp);
    } else if zero && chunk[3] != '0' {
        result.push('零');
    }

    if chunk[3] != '0' {
        let tmp = make_unit(3);
        result.push_str(&tmp);
    }

    result.push_str(primary);
    (result, (chunk[0] == '0', chunk[3] == '0'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let num = 123456.12;
        assert_eq!(
            split(num),
            Some((vec!['1', '2', '3', '4', '5', '6'], ['1', '2'],))
        );
    }

    #[test]
    fn test_split_chunk() {
        let mut int = vec!['2', '3', '4', '5', '6'];
        assert_eq!(split_chunk(&mut int), ['3', '4', '5', '6']);
        assert_eq!(split_chunk(&mut int), ['0', '0', '0', '2']);
    }

    #[test]
    fn test_convert_chunk() {
        let prefix = ["仟", "佰", "拾", ""];
        let transer = HashMap::from([
            ('0', "零"),
            ('1', "壹"),
            ('2', "贰"),
            ('3', "叁"),
            ('4', "肆"),
            ('5', "伍"),
            ('6', "陆"),
            ('7', "柒"),
            ('8', "捌"),
            ('9', "玖"),
        ]);
        let primary = "#";
        let f = |chunk: &[char; 4]| convert_chunk(chunk, &prefix, &transer, primary);

        let chunk = ['1', '2', '3', '4'];
        assert_eq!(f(&chunk), (String::from("壹仟贰佰叁拾肆#"), (false, false)));

        let chunk = ['0', '0', '1', '2'];
        assert_eq!(f(&chunk), (String::from("壹拾贰#"), (true, false)));

        let chunk = ['1', '0', '3', '4'];
        assert_eq!(f(&chunk), (String::from("壹仟零叁拾肆#"), (false, false)));

        let chunk = ['1', '0', '0', '4'];
        assert_eq!(f(&chunk), (String::from("壹仟零肆#"), (false, false)));

        let chunk = ['1', '0', '0', '0'];
        assert_eq!(f(&chunk), (String::from("壹仟#"), (false, true)));
    }

    #[test]
    fn test_convert() {
        // 测试 '零' 是否正确
        let num = 1_2345.67;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹万贰仟叁佰肆拾伍圆陆角柒分")
        );

        let num = 100_2345.67;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹佰万零贰仟叁佰肆拾伍圆陆角柒分")
        );

        let num = 1_0000_2345.67;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹亿零贰仟叁佰肆拾伍圆陆角柒分")
        );

        let num = 10_1010_2345.67;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹拾亿零壹仟零壹拾万零贰仟叁佰肆拾伍圆陆角柒分")
        );

        let num = 101_0110_0345.67;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹佰零壹亿零壹佰壹拾万零叁佰肆拾伍圆陆角柒分")
        );

        // 小数测试
        let num = 1_2345.0;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹万贰仟叁佰肆拾伍圆整")
        );

        let num = 1_2345.07;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹万贰仟叁佰肆拾伍圆柒分")
        );

        let num = 1_2345.6;
        assert_eq!(
            convert(num).unwrap(),
            String::from("壹万贰仟叁佰肆拾伍圆陆角")
        );

        let num = 0.67;
        assert_eq!(convert(num).unwrap(), String::from("陆角柒分"));

        let num = 0.0;
        assert_eq!(convert(num).unwrap(), String::from(""));

        // 非正常数据
        let num = f64::INFINITY;
        assert_eq!(convert(num), None);

        let num = f64::NAN;
        assert_eq!(convert(num), None);

        let num = -1.0;
        assert_eq!(convert(num), None);
    }
}
