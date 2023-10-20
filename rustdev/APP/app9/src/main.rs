use std::fmt::Display;
use std::io;

fn main() {
    let text = read_line("Введите текст: ");
    let num = read_line("Введите число: ");
    let num: u8 = num.trim().parse().expect("Неверное число максимально 255");
    let num_u8 = read_line("Введите число для get_value(): ");
    let num_u8: u8 = num_u8
        .trim()
        .parse()
        .expect("Неверное число максимально 255");
    let message = Message {
        author: "Trash".to_string(),
        text: "fdsf".to_string(),
    };
    let post = Post {
        author: "bob".to_string(),
        content: "sdf".to_string(),
        likes: 123,
    };

    let dataa = return_object();

    print_value(text);
    print_value(num);

    let arr: [u8; 18] = [1, 2, 3, 4, 5, 4, 5, 6, 3, 4, 7, 8, 9, 8, 7, 6, 5, 4];

    println!("\n{:?}", find_duplicate(&arr));

    let data = Data {
        d1: "Hello".to_string(),
        d2: 777,
    };

    println!("\nData is {} and {}", data.get_d1(), data.get_d2());
    println!("{}", num_u8.get_value());
    println!("\n{}", message.get_info());
    println!("\n{}", post.get_info());

    post.hide_info();

    let gg = GG {
        info: "info".to_string(),
    };

    gg.pr_gg();
    get_object(gg);
    dataa.pr_hello();
}

trait GetInfo {
    fn get_info(&self) -> String;
    fn hide_info(&self) {
        println!("\nYou cant get info");
    }
}

struct GG {
    info: String,
}

trait Prgg {
    fn pr_gg(&self) {
        println!("\nGG");
    }
}
trait PrHello {
    fn pr_hello(&self) {
        println!("\nHello");
    }
}

impl Prgg for GG {}
impl PrHello for GG {}

fn get_object<T>(obj: T)
where
    T: Prgg + PrHello,
{
    obj.pr_gg();
    obj.pr_hello();
}

struct Message<A> {
    author: A,
    text: A,
}

struct Post<B, C> {
    author: B,
    content: B,
    likes: C,
}

impl GetInfo for Message<String> {
    fn get_info(&self) -> String {
        format!("\nMessage from {}: {}", self.author, self.text)
    }
}

impl GetInfo for Post<String, u8> {
    fn get_info(&self) -> String {
        format!(
            "\nPost from {} likes {}: {}",
            self.author, self.likes, self.content
        )
    }
}

trait GetData {
    fn get_value(&self) -> String;
}

impl GetData for u8 {
    fn get_value(&self) -> String {
        format!("\nget_value is {}", *self)
    }
}

/*enum Data_perechesl<T> {
    Data(T),
    Empty,
}*/

struct Data<S, N> {
    d1: S,
    d2: N,
}

impl<S, N> Data<S, N> {
    fn get_d1(&self) -> &S {
        &self.d1
    }
    fn get_d2(&self) -> &N {
        &self.d2
    }
}

fn find_duplicate<T: Display>(list: &[T]) -> Vec<T>
where
    T: PartialEq + Copy,
{
    let mut duplicate = Vec::new();

    for i in 0..list.len() {
        for j in (i + 1)..list.len() {
            if list[i] == list[j] {
                if !duplicate.contains(&list[i]) {
                    duplicate.push(list[i]);
                };
            }
        }
    }
    duplicate
}

fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn print_value<T: Display>(value: T) {
    println!("\nprint_value is {}", value);
}

fn return_object() -> impl PrHello {
    GG {
        info: "sdgdfsg".to_string(),
    }
}
