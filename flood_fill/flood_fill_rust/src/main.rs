fn main() {
    let m = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    println!("{:?}", flood_fill(m, 1, 1, 2));
}

pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let original_color = image[sr as usize][sc as usize];
    let rows = image.len();
    let cols = image[0].len();

    if original_color == color {
        return image;
    }

    let mut image_clone = image.clone();

    fn dfs(
        r: i32,
        c: i32,
        rows: usize,
        cols: usize,
        original_color: i32,
        color: i32,
        image: &mut Vec<Vec<i32>>,
    ) {
        if r < 0 || r >= rows as i32 || c < 0 || c >= cols as i32 {
            return;
        }

        let (r_usize, c_usize) = (r as usize, c as usize);

        if image[r_usize][c_usize] != original_color {
            return;
        }

        image[r_usize][c_usize] = color;

        dfs(r + 1, c, rows, cols, original_color, color, image);
        dfs(r - 1, c, rows, cols, original_color, color, image);
        dfs(r, c + 1, rows, cols, original_color, color, image);
        dfs(r, c - 1, rows, cols, original_color, color, image);
    }

    dfs(sr, sc, rows, cols, original_color, color, &mut image_clone);
    image_clone
}
