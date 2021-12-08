var zoom = 1024;
var xCord = 0;
var yCord = 0;
var cell_prefix = "square_";
var toMove = null;

var pieces = [
    {'x': 0, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 1, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 2, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 3, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 4, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 5, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 6, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 7, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 0, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 1, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 2, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 3, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 4, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 5, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 6, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 7, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},

    {'x': 0, 'y': 0, 'type': 'black_rook', 'color': 'black', 'alive': true},
    {'x': 1, 'y': 0, 'type': 'black_knight', 'color': 'black', 'alive': true},
    {'x': 2, 'y': 0, 'type': 'black_bishop', 'color': 'black', 'alive': true},
    {'x': 3, 'y': 0, 'type': 'black_queen', 'color': 'black', 'alive': true},
    {'x': 4, 'y': 0, 'type': 'black_king', 'color': 'black', 'alive': true},
    {'x': 5, 'y': 0, 'type': 'black_bishop', 'color': 'black', 'alive': true},
    {'x': 6, 'y': 0, 'type': 'black_knight', 'color': 'black', 'alive': true},
    {'x': 7, 'y': 0, 'type': 'black_rook', 'color': 'black', 'alive': true},
    {'x': 0, 'y': 7, 'type': 'white_rook', 'color': 'white', 'alive': true},
    {'x': 1, 'y': 7, 'type': 'white_knight', 'color': 'white', 'alive': true},
    {'x': 2, 'y': 7, 'type': 'white_bishop', 'color': 'white', 'alive': true},
    {'x': 3, 'y': 7, 'type': 'white_queen', 'color': 'white', 'alive': true},
    {'x': 4, 'y': 7, 'type': 'white_king', 'color': 'white', 'alive': true},
    {'x': 5, 'y': 7, 'type': 'white_bishop', 'color': 'white', 'alive': true},
    {'x': 6, 'y': 7, 'type': 'white_knight', 'color': 'white', 'alive': true},
    {'x': 7, 'y': 7, 'type': 'white_rook', 'color': 'white', 'alive': true},
];

function displayed(x, y) {
    x -= xCord;
    y -= yCord;
    return x >= 0 && x < 8 && y >= 0 && y < 8;
}

function getPiece(x, y) {
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (piece.x == x && piece.y == y && piece.alive) {
            return piece;
        }
    }
    return null;
}

function render() {
    for (var i = 0; i < 8*8; i++) {
        var n = document.getElementById(cell_prefix + i);
        n.className = "";
        n.classList.add((Math.floor(i/8) + i) % 2 == 0? "white_square" : "black_square");
    }
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (displayed(piece.x, piece.y) && piece.alive) {
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
        n['data-x'] = i % 8;
        n['data-y'] = Math.floor(i/8);
        n.classList.add((Math.floor(i/8) + i) % 2 == 0? "white_square" : "black_square");
        n.id = cell_prefix + i;
        n.onclick = move;
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

function move(e) {
    grabbedPiece = getPiece(e.target['data-x'], e.target['data-y']);
    if (toMove && (!grabbedPiece || grabbedPiece.color != toMove.color)) { 
        toMove.x = e.target['data-x'];
        toMove.y = e.target['data-y'];
        toMove = null;
        if (grabbedPiece) {
            grabbedPiece.alive = false;
        }
    } else {
        toMove = grabbedPiece;
    }
    render();
}
