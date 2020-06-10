fn main() {
    // let bla = Some(0u8);
    let bla = Some(3u8);
    match bla {
        Some(3) => println!("[?]={:?}","three"),
        _ => (),
    }

    if let Some(3) = bla {
        println!("[?]={:?}", "threeeeeee");
    }

    if let Some(5) = bla {
        println!("[?]={:?}", "fiiiiiive");
    } else {
        println!("oh well...");
    }
}
