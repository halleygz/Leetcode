impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let av = ((ax2-ax1) * (ay2-ay1)).abs();
        let bv = ((bx2-bx1) * (by2-by1)).abs();

        if bx2 <= ax1 || ax2 <= bx1 || ay2 <= by1 || by2 <= ay1 {
        return av + bv
        }
        
        let mut xs = [ax1,ax2,bx1,bx2];
        xs.sort();
        let mut ys = [ay1,ay2,by1,by2];
        ys.sort();

        av + bv - ((xs[2] - xs[1]) * (ys[2] - ys[1])).abs()
    }
}