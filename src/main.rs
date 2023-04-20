mod ipv4_class;
mod ipv4_cidr;
fn fill_str(s: &str, amount: usize, fill: &str) -> String {
    let mut iter = s.chars().peekable();
    let mut res = String::from("");

    while iter.peek().is_some() {
        let chunk: String = iter.by_ref().take(amount).collect();
        res.push_str(&chunk);
        res.push_str(fill);
    }

    res.truncate(res.len()-1);

    return res
}

fn binary_ip(n: u32) -> String {
    let s = format!("{:0<32b}", n);

    fill_str(&s, 8, ".")
}

fn show_ip(ip: Ipv4Addr) -> String {
    let n = <Ipv4Addr as Into<u32>>::into(ip);
    let bin = binary_ip(n);

    format!("{:<15} {}", ip, bin)
}

