
#[cfg(test)]
mod tests;

mod error;
pub use self::error::EvalExprError;

fn parse_float(exp: &str) -> Result<(f64, &str), EvalExprError> {
    let mut len = 0;
    let mut c;
    let mut keep: bool = true;

    match exp.chars().nth(len) {
        None => return Err(EvalExprError),
        Some(val) => c = val
    };
    while keep && c.is_digit(10) || c == '.' {
        len += 1;
        match exp.chars().nth(len) {
            None => keep = false,
            Some(val) => c = val
        };
    }
    let num_sclice = &exp[..len];
    let rest_slice = &exp[len..];
    match num_sclice.parse::<f64>() {
        Err(_) => {
            Err(EvalExprError)
        },
        Ok(num) => {
            Ok((num, rest_slice))
        }
    }
}

fn parse_number(exp: &str) -> Result<(f64, &str), EvalExprError> {
    let mut _exp = &exp[..];

    if _exp.chars().nth(0).is_none() {
        return Err(EvalExprError);
    } else if _exp.chars().nth(0).unwrap() == '(' {
        let nbr;
        _exp = &_exp[1..];
        match parse_sum(&_exp) {
            Err(e) => return Err(e),
            Ok((num, exp)) => {
                nbr = num;
                _exp = exp;
            }
        };
        return match _exp.chars().nth(0) {
            Some(')') => {
                _exp = &_exp[1..];
                Ok((nbr, _exp))
            },
            _ => Err(EvalExprError)
        };
    }
    match parse_float(_exp) {
        Err(e) => {
            Err(e)
        },
        Ok((num, exp)) => {
            Ok((num, exp))
        }
    }
}

fn parse_sign(exp:&str, sign: bool) -> Result<(f64, &str), EvalExprError> {
    let next_char;
    let mut new_sign = sign;

    match exp.chars().nth(0) {
        None => return Err(EvalExprError),
        Some(val) => next_char = val
    };

    if next_char == '-' {
        new_sign = !sign;
    } else if next_char == '+' {
    } else {
        return match parse_number(&exp) {
            Err(e) => Err(e),
            Ok((mut num, new_exp)) => {
                if !new_sign {
                    num = -num;
                }
                Ok((num, new_exp))
            }
        }
    }
    parse_sign(&exp[1..], new_sign)
}

fn parse_pow(exp: &str) -> Result<(f64, &str), EvalExprError> {
    let mut a;
    let mut _exp;
    match parse_sign(exp, true) {
        Err(e) => return Err(e),
        Ok((num, new_exp)) => {
            a = num;
            _exp = new_exp;
        }
    };

    while !_exp.is_empty() {
        let op = _exp.chars().nth(0).unwrap();
        if op != '^' {
            return Ok((a, _exp));
        }
        _exp = &_exp[1..];
        let b;
        match parse_sign(_exp, true) {
            Err(e) => return Err(e),
            Ok((num, new_exp)) => {
                b = num;
                _exp = new_exp;
            }
        };
        a = a.powf(b);
    }
    Ok((a, _exp))
}

fn parse_factors(exp: &str) -> Result<(f64, &str), EvalExprError> {
    let mut a;
    let mut _exp;
    match parse_pow(exp) {
        Err(e) => return Err(e),
        Ok((num, new_exp)) => {
            a = num;
            _exp = new_exp;
        }
    };

    while !_exp.is_empty() {
        let op = _exp.chars().nth(0).unwrap();
        if op != '*' && op != '/' && op != '%' {
            return Ok((a, _exp));
        }
        _exp = &_exp[1..];
        let b;
        match parse_pow(_exp) {
            Err(e) => return Err(e),
            Ok((num, new_exp)) => {
                b = num;
                _exp = new_exp;
            }
        };
        a = if op == '%' {
            a % b
        } else if op == '/' {
            if b == 0. {
                return Err(EvalExprError);
            }
            a / b
        } else {
            a * b
        }
    }
    Ok((a, _exp))
}

fn parse_sum(exp: &str) -> Result<(f64, &str), EvalExprError> {
    let mut a;
    let mut _exp;
    match parse_factors(exp) {
        Err(e) => return Err(e),
        Ok((num, new_exp)) => {
            a = num;
            _exp = new_exp;
        }
    };

    while _exp.len() > 0 {
        let b;
        let op = _exp.chars().nth(0).unwrap();
        if op != '+' && op != '-' {
            return Ok((a, _exp));
        }
        _exp = &_exp[1..];
        match parse_factors(_exp) {
            Err(e) => return Err(e),
            Ok((num, new_exp)) => {
                b = num;
                _exp = new_exp;
            }
        };
        a = if op == '+' {
            a + b
        } else {
            a - b
        }
    }
    Ok((a, _exp))
}

pub fn evaluate(exp: &str) -> Result<f64, EvalExprError> {
    match parse_sum(&exp.replace(" ", "")[..]) {
        Err(e) => {
            Err(e)
        },
        Ok((res, _)) => {
            Ok((res * 100.).round() / 100.)
        }
    }
}
