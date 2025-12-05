use std::fs;

fn read_file(path: &str) -> &'static [&'static str] {
    static CACHE: std::sync::OnceLock<Vec<&'static str>> = std::sync::OnceLock::new();

    &CACHE.get_or_init(|| {
        let content = fs::read_to_string(path).expect("Erro ao ler arquivo");

        let leaked = Box::leak(content.into_boxed_str());

        leaked.split_whitespace().collect::<Vec<&'static str>>()
    })
}

fn split_direction(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();

    chars.next().map(|c| (c, chars.as_str()))
}

fn get(slice: &[i32], idx: isize) -> i32 {
    let len = slice.len() as isize;
    let idx = ((idx % len) + len) % len; // garante que fica positivo

    slice[idx as usize]
}

pub fn day1() {
    let seq = read_file("./src/days/input.txt");
    let interval = (0..=99).collect::<Vec<i32>>();
    let mut counter: i32 = 50;
    let mut collisions: i32 = 0;

    for x in seq {
        if let Some((direction, value)) = split_direction(x) {
            if direction == 'L' {
                //iremos decrementar
                let value: i32 = value.parse().expect("deu erro, oxe?");
                let res = get(&interval, -(value - counter) as isize);
                counter = res;

                println!("resultado L {:?}", res);
                if res == 0 {
                    collisions += 1
                }
            } else {
                //iremos decrementar
                let value: i32 = value.parse().expect("deu erro, oxe?");
                let res = get(&interval, (value + counter) as isize);
                counter = res;

                println!("resultado R {:?}", res);
                if res == 0 {
                    collisions += 1
                }
            }
        }
    }
    println!("Colisoes {:?}", collisions);
}
