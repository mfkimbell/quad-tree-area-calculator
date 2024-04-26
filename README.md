# quad-tree-area-calculator

  <b>Rust</b> project that places rectanges into a plane and
                      then recursively calculates the area by using a
                      <b>QuadTree</b> data structure. The recursion splits each
                      quadrant into 4 quadrants. It will mark one quadrant as
                      completely filled when inputting rectangles. If the
                      rectangles overlaps with multiple quadrants, it will only
                      insert the intersections. Areas cannot be double counted
                      because if a quadrant is already filled it does not add
                      the area.
                    
               
              
