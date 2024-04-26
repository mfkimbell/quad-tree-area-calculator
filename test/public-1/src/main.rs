
use project::overlapping_rect;


fn main() {
    let rects = vec! [
        overlapping_rect::Rect::new(1,1,1,4),
        overlapping_rect::Rect::new(0,0,0,6),
        overlapping_rect::Rect::new(6,6,2,6),
        overlapping_rect::Rect::new(1,1,1,1),
        overlapping_rect::Rect::new(2,2,2,2),
        overlapping_rect::Rect::new(999,9,999,9),
        overlapping_rect::Rect::new(0,0,0,9999999999),
    ];

    let result = overlapping_rect::total_rect_area(rects);
    println!("{}", result);

    std::fs::write("output", result.to_string()).expect("Unable to create result file");

    // assert_eq!(true, result);
}
