use rand::{ thread_rng, Rng };

pub fn generate_strong_passwords() -> String {
    let chars =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
  abcdefghijklmnopqrstuvwxyz\
  0123456789\
  !@#$%^&*()-_=+{}[]|:;,.<>/?";
    let mut rng = thread_rng();
    let password = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect();
    password
}
