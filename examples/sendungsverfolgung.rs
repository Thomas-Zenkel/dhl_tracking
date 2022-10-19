use dhl_tracking::SendungsverfolgungBuilder;

fn main() {
    let sv = SendungsverfolgungBuilder::new()
        .sandbox(true)
        .passwd_entwicklerportal("your login-password entwicklerportal".to_string())
        .entwickler_id("EntwicklerID from Konto".to_owned())
        .build()
        .unwrap();
    println!("{:?}", sv.get_piece_detail("00340434161094022115").unwrap());
}
