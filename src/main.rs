fn main() {
    let u = 4294967295_u32;
    let i = u as i32;

    println!("{i:032b}={i}\n{u:032b}={u}");
}
