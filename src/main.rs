mod point;


fn main(){
    let p1 = point::Point{
        x: 1,
        y: 2
    };

    let p2 = point::Point{
        x: 2,
        y:3
    };

    let p3= p1.add(p2);
    println!("{:?}", p3);

    let p4 = point::Point::from_sum(p1, p3);
    println!("{:?}", p4);

}