var zoom = 1024;
var xCord = 0;
var yCord = 0;
var cell_prefix = "square_";
var pieces = [
    {'x': 0, 'y': 1, 'type': 'black_pawn'},
    {'x': 1, 'y': 1, 'type': 'black_pawn'},
    {'x': 2, 'y': 1, 'type': 'black_pawn'},
    {'x': 3, 'y': 1, 'type': 'black_pawn'},
    {'x': 4, 'y': 1, 'type': 'black_pawn'},
    {'x': 5, 'y': 1, 'type': 'black_pawn'},
    {'x': 6, 'y': 1, 'type': 'black_pawn'},
    {'x': 7, 'y': 1, 'type': 'black_pawn'},
    {'x': 0, 'y': 6, 'type': 'white_pawn'},
    {'x': 1, 'y': 6, 'type': 'white_pawn'},
    {'x': 2, 'y': 6, 'type': 'white_pawn'},
    {'x': 3, 'y': 6, 'type': 'white_pawn'},
    {'x': 4, 'y': 6, 'type': 'white_pawn'},
    {'x': 5, 'y': 6, 'type': 'white_pawn'},
    {'x': 6, 'y': 6, 'type': 'white_pawn'},
    {'x': 7, 'y': 6, 'type': 'white_pawn'},

    {'x': 0, 'y': 0, 'type': 'black_rook'},
    {'x': 1, 'y': 0, 'type': 'black_knight'},
    {'x': 2, 'y': 0, 'type': 'black_bishop'},
    {'x': 3, 'y': 0, 'type': 'black_queen'},
    {'x': 4, 'y': 0, 'type': 'black_king'},
    {'x': 5, 'y': 0, 'type': 'black_bishop'},
    {'x': 6, 'y': 0, 'type': 'black_knight'},
    {'x': 7, 'y': 0, 'type': 'black_rook'},
    {'x': 0, 'y': 7, 'type': 'white_rook'},
    {'x': 1, 'y': 7, 'type': 'white_knight'},
    {'x': 2, 'y': 7, 'type': 'white_bishop'},
    {'x': 3, 'y': 7, 'type': 'white_queen'},
    {'x': 4, 'y': 7, 'type': 'white_king'},
    {'x': 5, 'y': 7, 'type': 'white_bishop'},
    {'x': 6, 'y': 7, 'type': 'white_knight'},
    {'x': 7, 'y': 7, 'type': 'white_rook'},
];

function displayed(x, y) {
    x -= xCord;
    y -= yCord;
    return x >= 0 && x < 8 && y >= 0 && y < 8;
}

function render() {
    for (var i = 0; i < 8*8; i++) {
        var n = document.getElementById(cell_prefix + i);
        n.classList.length = 0;
        n.classList.add((Math.floor(i/8) + i) % 2 == 0? "white_square" : "black_square");
    }
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (displayed(piece.x, piece.y)) {
            console.log(piece);
            var d = piece.x - xCord + (piece.y - yCord) * 8;
            var n = document.getElementById(cell_prefix + d);
            n.classList.add(piece.type);
        }
    }
}

function chzoom(w) {
    zoom *= w;

	document.getElementById("zoomable").style.width = zoom+"px";
	document.getElementById("zoomable").style.height = zoom+"px";
}
 
window.onload = function() {
    var palette = document.getElementById("zoomable");
    for (var i = 0; i < 8*8; i++) {
        var n = document.createElement("li");
        n.classList.add((Math.floor(i/8) + i) % 2 == 0? "white_square" : "black_square");
        n.id = cell_prefix + i;
        palette.appendChild(n);
    }
    zoom = palette.offsetWidth;
    document.getElementById('zoomout').onclick = function(e) {
        chzoom(1/1.2);
    };
    document.getElementById('zoomin').onclick = function(e) {
        chzoom(1.2);
    };
    render();
};
