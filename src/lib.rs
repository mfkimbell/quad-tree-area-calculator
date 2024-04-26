pub mod overlapping_rect {
    #[derive(Debug, Clone)]
    pub enum Quad {
        Empty,
        Filled,
        Node {
            x: i64,
            y: i64,
            q1: Box<Quad>,
            q2: Box<Quad>,
            q3: Box<Quad>,
            q4: Box<Quad>,
        },
    }

    #[derive(Debug, Clone)]
    pub struct Rect {
        x0: i64,
        y0: i64,
        x1: i64,
        y1: i64,
    }

    impl Rect {
        pub fn new(x0: i64, y0: i64, x1: i64, y1: i64) -> Self {
            Rect { x0, y0, x1, y1 }
        }
    }

    impl Quad {
        // fn new_node(x: i64, y: i64) -> Self {
        //     println!("Creating new node at ({}, {})", x, y);
        //     Quad::Node {
        //         x,
        //         y,
        //         q1: Box::new(Quad::Empty),
        //         q2: Box::new(Quad::Empty),
        //         q3: Box::new(Quad::Empty),
        //         q4: Box::new(Quad::Empty),
        //     }
        // }
        // pub fn insert_helper(
        //     &mut self,
        //     q1: bool,
        //     q2: bool,
        //     q3: bool,
        //     q4: bool,
        //     x: i64,
        //     y: i64,
        //     rect: &Rect
        // ) -> &Rect {
        //     if q1 {
        //         let intersect_rect = ;
        //         intersect_rect
        //     }

        //     if q2 {
        //         let intersect_rect = &Rect::new(std::cmp::min(*x, rect.x0), std::cmp::max(y,rect.y0), std::cmp::min(*x, rect.x1), std::cmp::max(y, rect.y1));
        //         intersect_rect
        //     }

        //     if q3 {
        //         let intersect_rect = &Rect::new(std::cmp::min(*x, rect.x0), std::cmp::min(*y,rect.y0), std::cmp::min(*x, rect.x1), std::cmp::min(*y, rect.y1));
        //         intersect_rect
        //     }

        //     if q4 {
        //         let intersect_rect = &Rect::new(std::cmp::max(*x, rect.x0), std::cmp::min(*y,rect.y0), std::cmp::max(*x, rect.x1), std::cmp::min(*y, rect.y1));
        //         intersect_rect
        //     }

        //     let bad_rect = &Rect::new(0, 0, 0, 0);
        //     bad_rect
        //     // if intersect_rect.x1 > intersect_rect.x0 && intersect_rect.y1 > intersect_rect.y0 {
        //     // }
        //     // self.clone()
        // }
        // if rect.x1 <= rect.x0 || rect.y1 <= rect.y0 {
        //     println!("Rectangle has zero or negative area, skipping insertion.");
        //     return;
        // }
        // if let Quad::Node {
        //     x,
        //     y,
        //     q1,
        //     q2,
        //     q3,
        //     q4,
        // } = self
        // {
        //     println!("Pre-insertion quad statuses at Node ({}, {}):", x, y);
        //     println!("q1: {:?}", q1);
        //     println!("q2: {:?}", q2);
        //     println!("q3: {:?}", q3);
        //     println!("q4: {:?}", q4);
        // }

        pub fn insert(&self, rect: &Rect) -> Quad {
            println!("Inserting rect: {:?}", rect);

            if rect.x0 == rect.x1 || rect.y0 == rect.y1 {
                return self.clone();
            }
            match self {
                Quad::Filled => {
                    Quad::Filled
                }
                Quad::Empty => {
                    Quad::Node {
                        x: rect.x0,
                        y: rect.y0,
                        q1: Box::new(Quad::Node {
                            x: rect.x1,
                            y: rect.y1,
                            q1: Box::new(Quad::Empty),
                            q2: Box::new(Quad::Empty),
                            q3: Box::new(Quad::Filled),
                            q4: Box::new(Quad::Empty),
                        }),
                        q2: Box::new(Quad::Empty),
                        q3: Box::new(Quad::Empty),
                        q4: Box::new(Quad::Empty),
                    }
                }
                Quad::Node {
                    x,
                    y,
                    q1,
                    q2,
                    q3,
                    q4,
                } => {
                    Quad::Node {
                        x: *x,
                        y: *y,
                        q1: Box::new(q1.insert(&Rect::new(std::cmp::max(*x, rect.x0), std::cmp::max(*y,rect.y0), std::cmp::max(*x, rect.x1), std::cmp::max(*y, rect.y1)))),
                        
                        q2: Box::new(q2.insert(&Rect::new(std::cmp::min(*x, rect.x0), std::cmp::max(*y, rect.y0), std::cmp::min(*x, rect.x1), std::cmp::max(*y, rect.y1)))),
                        
                        q3: Box::new(q3.insert(&Rect::new(std::cmp::min(*x, rect.x0), std::cmp::min(*y, rect.y0), std::cmp::min(*x, rect.x1), std::cmp::min(*y, rect.y1)))),
                        
                        q4: Box::new(q4.insert(&Rect::new(std::cmp::max(*x, rect.x0), std::cmp::min(*y, rect.y0), std::cmp::max(*x, rect.x1), std::cmp::min(*y, rect.y1)))),
                    }
                }
            }
        }
        
        // if let Quad::Node {
        //     x,
        //     y,
        //     q1,
        //     q2,
        //     q3,
        //     q4,
        // } = self
        // {
        //     println!("Post-insertion quad statuses at Node ({}, {}):", x, y);
        //     println!("q1: {:?}", q1);
        //     println!("q2: {:?}", q2);
        //     println!("q3: {:?}", q3);
        //     println!("q4: {:?}", q4);
        // }
        

        pub fn area(&self, x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
            match self {
                Quad::Empty => 0,
                Quad::Filled => {
                    // println!("x1: {}, x0: {}, y1: {}, y0: {}", x1, x0, y1, y0);
                    let area = (x1 - x0) * (y1 - y0);
                    // println!("Area: {}", area);
                    area
                }
                Quad::Node {
                    x,
                    y,
                    q1,
                    q2,
                    q3,
                    q4,
                } => {

                    let area_q1 = q1.area(*x, *y, x1, y1);
                    let area_q2 = q2.area(x0, *y, *x, y1);
                    let area_q3 = q3.area(x0, y0, *x, *y);
                    let area_q4 = q4.area(*x, y0, x1, *y);

                    area_q1 + area_q2 + area_q3 + area_q4
                }
            }
        }
        pub fn print_tree(&self, depth: usize) {
            match self {
                Quad::Empty => println!("{:indent$}Empty", "", indent = depth * 4),
                Quad::Filled => println!("{:indent$}Filled", "", indent = depth * 4),
                Quad::Node {
                    x,
                    y,
                    q1,
                    q2,
                    q3,
                    q4,
                } => {
                    println!(
                        "{:indent$}Node - x: {}, y: {}",
                        "",
                        x,
                        y,
                        indent = depth * 4
                    );
                    println!("{:indent$}q1:", "", indent = depth * 4 + 2); // Slightly more indentation for children
                    q1.print_tree(depth + 1);
                    println!("{:indent$}q2:", "", indent = depth * 4 + 2);
                    q2.print_tree(depth + 1);
                    println!("{:indent$}q3:", "", indent = depth * 4 + 2);
                    q3.print_tree(depth + 1);
                    println!("{:indent$}q4:", "", indent = depth * 4 + 2);
                    q4.print_tree(depth + 1);
                }
            }
        }
    }

    pub fn total_rect_area(rects: Vec<Rect>) -> i64 {
        let mut root = Quad::Empty;
        for rect in rects {
            root = root.insert(&rect); 
        }

        // root.print_tree(0);

        root.area(i64::MIN, i64::MIN, i64::MAX, i64::MAX) // Calculate the total area
    }
}

