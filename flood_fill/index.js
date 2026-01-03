/**
 * @param {number[][]} image
 * @param {number} sr
 * @param {number} sc
 * @param {number} color
 * @return {number[][]}
 */
var floodFill = function(image, sr, sc, color) {
  const originalColor = image[sr][sc];

  if (originalColor === color) return image;

  const rows = image.length;
  const cols = image[0].length;

  function dfs(r, c) {
    // odnt go out of bounds
    if (r >= rows || r < 0 || c >= cols || c < 0) {
      return;
    }

    // if the pixel is not the same color as target pixel do nothing
    if (image[r][c] !== originalColor) {
      return;
    }

    image[r][c] = color;

    dfs( r + 1, c); // up
    dfs(r - 1, c); // down
    dfs(r, c - 1); // left
    dfs(r, c + 1); //right
  }

  dfs(sr, sc);
  return image;
};

var floodFill = function(image, sr, sc, color) {
  const originalColor = image[sr][sc];

  const rows = image.length;
  const cols = image[0].length;

  function dfs(r, c) {
    if (r < 0 || r >= rows || c < 0 || c >= cols) {
      return;
    }

    if (image[r][c] !== originalColor) {
      return;
    }

    image[r][c] = color;

    dfs(r + 1, c);
    dfs(r - 1, c);
    dfs(r, c + 1);
    dfs(r, c - 1);
  }

  dfs(sr, sc);
  return image;
};

const p = [[1,1,1],[1,1,0],[1,0,1]];
console.log(floodFill(p, 1, 1, 2)); // should be [[2,2,2],[2,2,0],[2,0,1]]
