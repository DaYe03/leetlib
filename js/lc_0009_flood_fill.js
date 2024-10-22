var floodFill = function(image, sr, sc, color) {
    if (checkCoordinates(sr, sc , image)){
        floodFill_rec(image, sr, sc, color, image[sr][sc]);
    }
    return image;
};

function floodFill_rec(image, r, c, color, original){
    image[r][c] = color;
    if (checkCoordinates(r, c+1, image) && image[r][c+1] == original && image[r][c+1] !== color){
        floodFill_rec(image, r, c+1, color, original);
    } 
    if(checkCoordinates(r+1, c,image)&& image[r+1][c]== original && image[r+1][c] !==color){
        floodFill_rec(image, r+1, c, color, original);
    } 
    if(checkCoordinates(r-1,c, image)&& image[r-1][c]  == original && image[r-1][c]!== color){
        floodFill_rec(image, r-1, c, color, original);  
    } 
    if(checkCoordinates(r, c-1, image)&& image[r][c-1] == original&& image[r][c-1]!== color){
        floodFill_rec(image, r, c-1, color, original);  
    }
}

function checkCoordinates(r, c, image){
    if (r >= image.length || c >= image[0].length || r<0 || c<0){
        return false;
    }
    return true;
}

console.log(floodFill([[1,1,1],[1,1,0],[1,0,1]], 1,1,2));