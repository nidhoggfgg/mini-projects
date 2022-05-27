use generator::gen_passwd;

fn main() {
    let auth = "ᑋᵉᑊᑊᵒ ᵕ̈ ᑋᵉᑊᑊᵒ";
    let target = "some";
    let symbols = ['.', '@', '_', '-', ':', '!'];
    let passwd = gen_passwd(auth, target, 20, true, true, &symbols);
    println!("passwd: {}", passwd);
}
