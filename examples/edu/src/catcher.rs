use rocket::{catch, Request};

#[catch(404)]
pub fn not_found(req: &Request) {
    println!("{:#?}：{}", req.method(), req.uri().path());
    for header in req.headers().iter() {
        println!("\t{}", header);
    }
}
