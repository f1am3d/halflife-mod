use cc;

extern {
    fn lib_test() -> String;
}

pub fn call() {
    unsafe {
        let string = lib_test();

        println!("{}", string);
    }
}

fn main () {

}