pub fn parse_float(exp: &str) -> (f32, &str) {
    let mut i = 0;
    let mut c = exp.chars().nth(i).unwrap();
    let mut keep: bool = true;
    while keep && c.is_digit(10) || c == '.' || c == '-' {
        i = i + 1;
        let x = exp.chars().nth(i);
        if x.is_none() {
            keep = false;
        } else {
            c = x.unwrap();
        }
    }
    let num_len = i;
    let num_sclice = &exp[..num_len];
    let num: f32 = num_sclice.parse().unwrap();
    let rest_slice = &exp[num_len..];
    return (num, rest_slice);
}

fn parse_number(exp: &str) -> (f32, &str) {
    let mut _exp = &exp[..];
    while _exp.chars().nth(0).unwrap() == ' ' {
        _exp = &_exp[1..];
    }
    if _exp.chars().nth(0).unwrap() == '(' {
        _exp = &_exp[1..];
        let (nbr, xp) = parse_sum(&_exp);
        let mut _exp = xp;
        if _exp.chars().nth(0).unwrap() == ')' {
            _exp = &exp[1..];
        }
        return (nbr, _exp);
    }
    return parse_float(_exp);
}

fn parse_factors(exp: &str) -> (f32, &str) {
    let (a, _exp) = parse_number(exp);
    let mut _exp = _exp;
    let mut a = a;
    while _exp.is_empty() == false {
        while _exp.chars().nth(0).unwrap() == ' ' {
            _exp = &_exp[1..];
        }
        let op = _exp.chars().nth(0).unwrap();
        if op != '*' && op != '/' && op != '%' {
            return (a, _exp);
        }
        _exp = &_exp[1..];
        let (b, new_exp) = parse_number(_exp);
        _exp = new_exp;
        if op == '%' {
            a = a % b;
        } else if op == '/'{
            a = a / b;
        } else {
            a = a * b;
        }
    }
    return (a, _exp);
}

fn parse_sum(exp: &str) -> (f32, &str) {
    let (a, _exp) = parse_factors(exp);
    let mut a = a;
    let mut _exp = _exp;
    while _exp.len() > 0 {
        while _exp.chars().nth(0).unwrap() == ' ' {
            _exp = &_exp[1..];
        }
        let op = _exp.chars().nth(0).unwrap();
        if op != '+' && op != '-' {
            return (a, _exp);
        }
        _exp = &_exp[1..];
        let (b, new_exp) = parse_factors(_exp);
        _exp = new_exp;
        if op == '+' {
            a = a + b;
        } else {
            a = a - b;
        }
    }
    return (a, _exp);
}

pub fn evaluate(exp: &str) -> f32 {
    let (res, _exp) = parse_sum(exp);
    return res;
}
