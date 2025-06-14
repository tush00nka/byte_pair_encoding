use std::collections::HashMap;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pair {
    l: Option<usize>,
    r: Option<usize>,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(l) = self.l {
            if let Some(r) = self.r {
                return write!(f, "({}, {})", l, r);
            }
            return write!(f, "({}, _)", l);
        }
        return write!(f, "(_, _)");
    }
}

impl Pair {
    fn new(l: Option<usize>, r: Option<usize>) -> Self {
        Self { l, r }
    }
}

fn main() {
    let mut frequency = HashMap::new();
    let mut pairs = vec![];
    let mut tokens_in = vec![];
    let mut tokens_out = vec![];

    for i in 0..256 {
        pairs.push(Pair::new(Some(i), None));
    }

    let text =
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus sit amet mi pretium felis gravida facilisis vitae vitae nunc. Nullam scelerisque nulla sed rhoncus facilisis. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse in ex vitae urna porttitor ultrices quis quis ante. Fusce blandit fermentum sapien in congue. Morbi id lorem consectetur, egestas tellus eget, finibus ipsum. Suspendisse a neque magna. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum tempus nisl eget est tempor, quis tempor velit commodo. Maecenas magna justo, placerat sit amet diam volutpat, tristique mollis elit. Morbi quis augue mauris. ".as_bytes();

    for i in 0..text.len() {
        let token = text[i] as usize;
        tokens_in.push(token);
    }

    loop {
        frequency.clear();
        tokens_out.clear();

        for i in 0..(tokens_in.len() - 1) {
            let pair = Pair::new(Some(tokens_in[i]), Some(tokens_in[i + 1]));

            if let Some(amount) = frequency.get_mut(&pair) {
                *amount += 1;
                continue;
            }

            frequency.insert(pair, 1);
        }

        let frequency = frequency
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<(Pair, i32)>>();

        let max_index = {
            let mut current_max = 0;
            for i in 1..frequency.len() {
                if frequency[i].1 > frequency[current_max].1 {
                    current_max = i;
                }
            }
            current_max
        };

        if frequency[max_index].1 <= 1 {
            break;
        }

        // println!("{} => {}", frequency[max_index].0, frequency[max_index].1);
        pairs.push(frequency[max_index].0);

        let mut i = 0;
        while i < tokens_in.len() {
            if i + 1 >= tokens_in.len() {
                tokens_out.push(tokens_in[i]);
                i += 1;
            } else {
                let pair = Pair::new(Some(tokens_in[i]), Some(tokens_in[i+1]));
                if pair == frequency[max_index].0 {
                    tokens_out.push(pairs.len() - 1);
                    i += 2;
                } else {
                    tokens_out.push(tokens_in[i]);
                    i += 1;
                }
            }
        }

        tokens_in = tokens_out.clone();
    }

    // render_tokens(&pairs, &tokens_in);

    // text.iter().map(|b| *b as char).for_each(|c| print!("{c}"));
    // print!("\n");
    // print!("\n");
    // render_tokens(&pairs, &tokens_in);

    generate_dot(&pairs);

    // frequency.sort_by(|a, b| b.1.cmp(&a.1));
    // for (pair, amount) in frequency.iter() {
    //     if *amount < 10 {
    //         continue;
    //     }
    //     println!("{} => {}", pair, amount);
    // }
}

#[allow(unused)]
fn render_tokens(pairs: &Vec<Pair>, tokens: &Vec<usize>) {
    for token in tokens.iter() {
        if pairs[*token].l == Some(*token) {
            print!("{}", *token as u8 as char);
        } else {
            print!("[{}]", *token);
            // print!("X");
        }
    }
    print!("\n");
}

fn generate_dot(pairs: &Vec<Pair>) {
    println!("digraph Pairs {{");
    for token in 0..pairs.len() {
        if pairs[token].l.is_none() || pairs[token].r.is_none() {
            continue;
        }

        if token != pairs[token].l.unwrap() {
            println!("\t{} -> {}", token, pairs[token].l.unwrap());
            println!("\t{} -> {}", token, pairs[token].r.unwrap());
        }
    }
    println!("}}");
}