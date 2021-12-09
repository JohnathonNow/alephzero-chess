var xCord = 0;
var yCord = 0;
var cell_prefix = "square_";
var toMove = null;
var madePawnsBlack = new Set();
var madePawnsWhite = new Set();
var size = 8;
var scrollFromX = null;
var scrollFromY = null;
var scrollFromXS = null;
var scrollFromYS = null;

var pieces = [
    {'x': 0, 'y': 1, 'type': 'black_rook', 'color': 'black', 'alive': true},
    {'x': 1, 'y': 0, 'type': 'black_bishop', 'color': 'black', 'alive': true},
    {'x': 4, 'y': 3, 'type': 'black_knight', 'color': 'black', 'alive': true},
    {'x': 3, 'y': 3, 'type': 'black_bishop', 'color': 'black', 'alive': true},

    {'x': 5, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 5, 'y': 2, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 6, 'y': 4, 'type': 'black_pawn', 'color': 'black', 'alive': true},

    {'x': 1, 'y': 7, 'type': 'white_knight', 'color': 'white', 'alive': true},
    {'x': 2, 'y': 7, 'type': 'white_bishop', 'color': 'white', 'alive': true},
    {'x': 6, 'y': 7, 'type': 'white_knight', 'color': 'white', 'alive': true},

    {'x': 1, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 5, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},
    {'x': 7, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true},

    {'x': 0, 'y': 0, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 3, 'y': 0, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 4, 'y': 0, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 5, 'y': 0, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 6, 'y': 0, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 7, 'y': 0, 'type': 'checker', 'color': 'black', 'alive': false},

    {'x': 2, 'y': 1, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 5, 'y': 1, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 6, 'y': 1, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 7, 'y': 1, 'type': 'checker', 'color': 'black', 'alive': false},

    {'x': 0, 'y': 2, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 4, 'y': 2, 'type': 'checker', 'color': 'black', 'alive': false},

    {'x': 2, 'y': 3, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 3, 'y': 3, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 4, 'y': 3, 'type': 'checker', 'color': 'black', 'alive': false},

    {'x': 5, 'y': 4, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 6, 'y': 4, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 7, 'y': 4, 'type': 'checker', 'color': 'black', 'alive': false},

    {'x': 2, 'y': 5, 'type': 'checker', 'color': 'black', 'alive': false},
    {'x': 6, 'y': 5, 'type': 'checker', 'color': 'black', 'alive': false},
];

function displayed(x, y) {
    x -= xCord;
    y -= yCord;
    return x >= 0 && x < size && y >= 0 && y < size;
}

function getPieceWanted(x, y) {
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (piece.x == x && piece.y == y && !piece.alive) {
            return piece;
        }
    }
    return null;
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
    {'x': x, 'y': 6, 'type': 'white_pawn', 'color': 'white', 'alive': true}
            );
            madePawnsWhite.add(x);
        }
    }
}

function addSquares() {
    var palette = document.getElementById("zoomable");
	while (palette.firstChild) {
		palette.removeChild(palette.firstChild);
	}
    for (var i = 0; i < size*size; i++) {
        var n = document.createElement("li");
        n['data-x'] = i % size;
        n['data-y'] = Math.floor(i/size);
        n.id = cell_prefix + i;
        n.onmousedown = moveBegin;
        n.onmouseup = moveEnd;
        n.ontouchstart = startTouch;
        n.ontouchmove = moveTouch;
		n.style.width = 100/size + '%';
		n.style.height = n.style.width;
        palette.appendChild(n);
    }
}

function compute(x, y) {
    var piece = getPiece(x, y);
    switch (piece.type) {
        case "white_pawn":
            let l = getPieceWanted(x - 1, y - 1);
            let r = getPieceWanted(x + 1, y - 1);
            return (l && l.type == "checker" ? 1:0) + (r && r.type == "checker" ? 1:0);
        case "white_bishop":
            var total = 0;
            for (var i = 1; i < 8; i += 1) {
                let a = getPieceWanted(x + i, y + i);
                if (a && a.type == "checker") {
                    total += 1;
                }
                let p = getPiece(x + i, y + i);
                if (p && p != piece) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                let a = getPieceWanted(x - i, y + i);
                if (a && a.type == "checker") {
                    total += 1;
                }
                let p = getPiece(x - i, y + i);
                if (p && p != piece) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                let a = getPieceWanted(x - i, y - i);
                if (a && a.type == "checker") {
                    total += 1;
                }
                let p = getPiece(x - i, y - i);
                if (p && p != piece) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                let a = getPieceWanted(x + i, y - i);
                if (a && a.type == "checker") {
                    total += 1;
                }
                let p = getPiece(x + i, y - i);
                if (p && p != piece) { break; }
            }
            return total;
        case "white_knight":
            let a = getPieceWanted(x - 2, y - 1);
            let b = getPieceWanted(x - 1, y - 2);
            let c = getPieceWanted(x + 1, y - 2);
            let d = getPieceWanted(x + 2, y - 1);
            let e = getPieceWanted(x + 2, y + 1);
            let f = getPieceWanted(x + 1, y + 2);
            let g = getPieceWanted(x - 1, y + 2);
            let h = getPieceWanted(x - 2, y + 1);
            return (a && a.type == "checker" ? 1:0) + 
                (b && b.type == "checker" ? 1:0) +
                (c && c.type == "checker" ? 1:0) +
                (d && d.type == "checker" ? 1:0) +
                (e && e.type == "checker" ? 1:0) +
                (f && f.type == "checker" ? 1:0) +
                (g && g.type == "checker" ? 1:0) +
                (h && h.type == "checker" ? 1:0);
    }
    return 0;
}

function render() {
    xCord = 0; //parseInt(document.getElementById("xport").value);
    yCord = 0; //parseInt(document.getElementById("yport").value);
    var newSize = 8; //parseInt(document.getElementById("zoom").value);
    if (size != newSize) {
        size = newSize;
        addSquares();
    }
    for (var i = 0; i < size*size; i++) {
        var n = document.getElementById(cell_prefix + i);
        n.className = "";
        var f = size % 2 == 0? Math.floor(i/size) : 0;
        n.classList.add((f + i + xCord) % 2 == 0? "white_square" : "black_square");
        n.innerHTML = "";
    }
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (displayed(piece.x, piece.y) && piece.alive) {
            var d = piece.x - xCord + (piece.y - yCord) * size;
            var n = document.getElementById(cell_prefix + d);
            n.classList.add(piece.type);
            if (piece.color == "white") {
                n.innerHTML = "<b>" + compute(piece.x, piece.y) + "</b>";
            }
        }
    }
}

window.onload = function() {
    addSquares();
    //document.getElementById('xport').oninput = render;
    //document.getElementById('yport').oninput = render;
    var deltas = document.getElementsByClassName("delta");
    for (var i = 0; i < deltas.length; i++) {
        deltas[i].onclick = delta;
    } 
    render();
};

function delta(e) {
    var ele = document.getElementById(e.target.getAttribute('data-target'));
    ele.value = parseInt(ele.value) + parseInt(e.target.getAttribute('data-value'));
    render();
}

function moveBegin(e) {
    var grabbedPiece = getPiece(parseInt(e.target['data-x']) + xCord, parseInt(e.target['data-y']) + yCord);
    if (grabbedPiece && !toMove) {
        toMove = grabbedPiece;
    }
    render();
}

function moveEnd(e) {
    var x = parseInt(e.target['data-x']) + xCord;
    var y = parseInt(e.target['data-y']) + yCord;
    var grabbedPiece = getPiece(x, y);
    if (toMove && (!grabbedPiece || grabbedPiece.color != toMove.color) && toMove != grabbedPiece) { 
        toMove.x = x;
        toMove.y = y;
        toMove = null;
        if (grabbedPiece) {
            grabbedPiece.alive = false;
        }
    }
    render();
}

function moveTouch(e) {
    e.preventDefault();
    var x = e.targetTouches[0].clientX;
    var y = e.targetTouches[0].clientY;
    var dx = Math.floor((scrollFromX - x) / e.target.clientWidth) * 2;
    var dy = Math.floor((scrollFromY - y) / e.target.clientHeight) * 2;
    document.getElementById("xport").value = scrollFromXS + dx;
    document.getElementById("yport").value = scrollFromYS + dy;
    render();
}
function startTouch(e) {
    scrollFromX = e.targetTouches[0].clientX;
    scrollFromY = e.targetTouches[0].clientY;
    scrollFromXS = parseInt(document.getElementById("xport").value);
    scrollFromYS = parseInt(document.getElementById("yport").value);
}
