use libactionkv::ActionKV;

const USAGE: &str = "
Usage:
akv_mem FILE get KEY
akv_mem FILE delete KEY
akv_mem FILE insert KEY VALUE
akv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE).as_ref();
    let key = args.get(3).expect(USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&filename);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{key:?} not found"),
            Some(value) => println!("{value:?}"),
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
        "update" => {
            let value = maybe_value.expect(USAGE).as_ref();
            store.update(key, value).unwrap()
        }
        _ => eprintln!("{USAGE}"),
    }
}
