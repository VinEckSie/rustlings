fn main() {
    // TODO: Fix the code to print "Hello world!".
    println!("Hello world!");

    let services: (&'static str ,&'static str ,&'static str ,&'static str ) = ("invest","invest consulting","blockchain consulting","ai consulting");
    let (serv1,_serv2,serv3,_serv4) = services;

    println!("{} {} {} {}", serv1, services.1, serv3, services.3);

}
