
use project::overlapping_rect;


fn main() {
    let rects = vec! [
        overlapping_rect::Rect::new(1,1,1,4),
        overlapping_rect::Rect::new(0,0,0,489),
        overlapping_rect::Rect::new(1,0,2,6),
        overlapping_rect::Rect::new(3,5,6,7),
        overlapping_rect::Rect::new(0,0,1212,3),
        overlapping_rect::Rect::new(0,0,0,2),
    ];

    let result = overlapping_rect::total_rect_area(rects);
    println!("{}", result);

    std::fs::write("output", result.to_string()).expect("Unable to create result file");

    // assert_eq!(true, result);
}
