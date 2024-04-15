use slug::slugify;

fn main() {
    let str = "Hello world";
    let slug = slugify(str);
    println!("{str} -> {slug}");
}
