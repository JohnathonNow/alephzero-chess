var zoom = 1024;
var xCord = 0;
var yCord = 0;
var cell_prefix = "square_";
var toMove = null;
var madePawnsBlack = new Set();
var madePawnsWhite = new Set();
var size = 12;

var pieces = [
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
    return x >= 0 && x < size && y >= 0 && y < size;
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

function addPawns() {
    for (var i = 0; i < size; i++) {
        var x = xCord + i;
        if (displayed(x, 1) && !madePawnsBlack.has(x)) {
            pieces.push(
    {'x': x, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true}
            );
            madePawnsBlack.add(x);
        }
        if (displayed(x, 6) && !madePawnsWhite.has(x)) {
            pieces.push(
    {'x': x, 'y': 6, 'type': 'white_pawn', 'color': 'black', 'alive': true}
            );
            madePawnsWhite.add(x);
        }
    }
}

function render() {
    xCord = parseInt(document.getElementById("xport").value);
    yCord = parseInt(document.getElementById("yport").value);
    addPawns();
    for (var i = 0; i < size*size; i++) {
        var n = document.getElementById(cell_prefix + i);
        n.className = "";
        n.classList.add((Math.floor(i/size) + i + xCord) % 2 == 0? "white_square" : "black_square");
    }
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (displayed(piece.x, piece.y) && piece.alive) {
            var d = piece.x - xCord + (piece.y - yCord) * size;
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
    for (var i = 0; i < size*size; i++) {
        var n = document.createElement("li");
        n['data-x'] = i % size;
        n['data-y'] = Math.floor(i/size);
        n.id = cell_prefix + i;
        n.onmousedown = moveBegin;
        n.onmouseup = moveEnd;
        palette.appendChild(n);
    }
    zoom = palette.offsetWidth;
    /*
    document.getElementById('zoomout').onclick = function(e) {
        chzoom(1/1.2);
    };
    document.getElementById('zoomin').onclick = function(e) {
        chzoom(1.2);
    };
    */
    document.getElementById('xport').oninput = render;
    document.getElementById('yport').oninput = render;
    var deltas = document.getElementsByClassName("delta");
    for (var i = 0; i < deltas.length; i++) {
        deltas[i].onclick = delta;
    } 
    render();
};

function moveBegin(e) {
    var grabbedPiece = getPiece(e.target['data-x'] + xCord, e.target['data-y'] + yCord);
    if (grabbedPiece) {
        toMove = grabbedPiece;
    }
    render();
}
function delta(e) {
    var ele = document.getElementById(e.target.getAttribute('data-target'));
    ele.value = parseInt(ele.value) + parseInt(e.target.getAttribute('data-value'));
    render();
}
function moveEnd(e) {
    var grabbedPiece = getPiece(e.target['data-x'] + xCord, e.target['data-y'] + yCord);
    if (toMove && (!grabbedPiece || grabbedPiece.color != toMove.color) && toMove != grabbedPiece) { 
        toMove.x = e.target['data-x'] + xCord;
        toMove.y = e.target['data-y'] + yCord;
        toMove = null;
        if (grabbedPiece) {
            grabbedPiece.alive = false;
        }
    }
    render();
}
