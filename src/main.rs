#[derive(Debug)]
struct Foo<'a> {
    num: u32,
    name: &'a str,
    children: Vec<Foo<'a>>,
}

struct Name<'a> {
    name: &'a str,
    num: u32,
}

impl <'a> Name<'a> {
    fn to_addr<'b>(&'b self) -> usize {
        let p: *const Self = self as *const Self;
        let addr: usize = p as usize;
        addr
    }
    unsafe fn from_addr<'b>(address: usize) -> &'b Name<'a> {
        &*(address as *const Self)
    }
}

fn main() {
    // Let's take a mutable piece of data, a 4-byte integer in this case
    let mut some_data: Foo = Foo{ num: 14, name: "Hello",
            children: Vec::new()};
    println!("orig some_data = {}", some_data.num);

    // Create a mutable raw pointer pointing to the data above
    let data_ptr: *mut Foo = &mut some_data as *mut Foo;

    // Note: creating a raw pointer is totally safe but dereferencing a raw pointer requires an
    // unsafe block
    unsafe {
        (*data_ptr).num += 5;
        println!("Dereferenced data: {:?}", some_data);
        (*data_ptr).name = "what you up to?"
    }
    println!("final some_data = {:?}", some_data);


    let name = Name { name: "Theo", num: 778, };

    let addr = name.to_addr();

    let name2: &Name;
    unsafe {
        name2 = Name::from_addr(addr);
    }

    println!("name: name = {}, num = {}", name.name, name.num);
    println!("name2: name = {}, num = {}", name2.name, name2.num);

}
