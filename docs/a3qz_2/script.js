var xCord = 0;
var yCord = 0;
var cell_prefix = "square_";
var toMove = null;
var toPromote = null;
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
    {'x': 5, 'y': 3, 'type': 'black_knight', 'color': 'black', 'alive': true},
    {'x': 3, 'y': 2, 'type': 'black_bishop', 'color': 'black', 'alive': true},

    {'x': 4, 'y': 1, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 5, 'y': 2, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 6, 'y': 3, 'type': 'black_pawn', 'color': 'black', 'alive': true},
    {'x': 0, 'y': 4, 'type': 'black_pawn', 'color': 'black', 'alive': true},

    {'x': 1, 'y': 7, 'type': 'white_knight', 'color': 'white', 'alive': true},
    {'x': 2, 'y': 7, 'type': 'white_bishop', 'color': 'white', 'alive': true},
    {'x': 7, 'y': 7, 'type': 'white_rook', 'color': 'white', 'alive': true},
    {'x': 4, 'y': 7, 'type': 'white_king', 'color': 'white', 'alive': true},
    {'x': 3, 'y': 7, 'type': 'white_queen', 'color': 'white', 'alive': true},

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

function movable(piece, xx, yy) {
    var x = piece.x;
    var y = piece.y;
    console.log(piece, xx, yy);
    switch (piece.type) {
        case "white_pawn":
            console.log(x, xx, y, yy);
            if (x == xx && y - 1 == yy) {
                return true;
            }
            if (x == xx && y - 2 == yy && y == 6) {
                return true;
            }
            break;
        case "white_queen":
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y + i == yy) { return true; }
                if (getPiece(x + i, y + i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y - i == yy) { return true; }
                if (getPiece(x + i, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y - i == yy) { return true; }
                if (getPiece(x - i, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y + i == yy) { return true; }
                if (getPiece(x - i, y + i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y == yy) { return true; }
                if (getPiece(x + i, y)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x == xx && y - i == yy) { return true; }
                if (getPiece(x, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y == yy) { return true; }
                if (getPiece(x - i, y)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x == xx && y + i == yy) { return true; }
                if (getPiece(x, y + i)) { break; }
            }
            break;
        case "white_bishop":
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y + i == yy) { return true; }
                if (getPiece(x + i, y + i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y - i == yy) { return true; }
                if (getPiece(x + i, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y - i == yy) { return true; }
                if (getPiece(x - i, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y + i == yy) { return true; }
                if (getPiece(x - i, y + i)) { break; }
            }
            break;
        case "white_rook":
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y == yy) { return true; }
                if (getPiece(x + i, y)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x == xx && y - i == yy) { return true; }
                if (getPiece(x, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y == yy) { return true; }
                if (getPiece(x - i, y)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x == xx && y + i == yy) { return true; }
                if (getPiece(x, y + i)) { break; }
            }
            break;
        case "white_knight":
            if (x - 1 == xx && y + 2 == yy) {
                return true;
            }
            if (x - 1 == xx && y - 2 == yy) {
                return true;
            }
            if (x + 1 == xx && y - 2 == yy) {
                return true;
            }
            if (x + 1 == xx && y + 2 == yy) {
                return true;
            }
            if (x - 2 == xx && y - 1 == yy) {
                return true;
            }
            if (x - 2 == xx && y + 1 == yy) {
                return true;
            }
            if (x + 2 == xx && y - 1 == yy) {
                return true;
            }
            if (x + 2 == xx && y + 1 == yy) {
                return true;
            }
            break;
    }
    return false;
}

function attackable(xx, yy) {
    for (var x = 0; x < 8; x++) {
    for (var y = 0; y < 8; y++) {
    var piece = getPiece(x, y);
    if (!piece) continue;
    switch (piece.type) {
        case "black_pawn":
            if (x + 1 == xx && y + 1 == yy) {
                return true;
            }
            if (x - 1 == xx && y + 1 == yy) {
                return true;
            }
            break;
        case "black_bishop":
            var total = 0;
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y + i == yy) { return true; }
                if (getPiece(x + i, y + i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y - i == yy) { return true; }
                if (getPiece(x + i, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y - i == yy) { return true; }
                if (getPiece(x - i, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y + i == yy) { return true; }
                if (getPiece(x - i, y + i)) { break; }
            }
            break;
        case "black_rook":
            var total = 0;
            for (var i = 1; i < 8; i += 1) {
                if (x + i == xx && y == yy) { return true; }
                if (getPiece(x + i, y)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x == xx && y - i == yy) { return true; }
                if (getPiece(x, y - i)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x - i == xx && y == yy) { return true; }
                if (getPiece(x - i, y)) { break; }
            }
            for (var i = 1; i < 8; i += 1) {
                if (x == xx && y + i == yy) { return true; }
                if (getPiece(x, y + i)) { break; }
            }
            break;
        case "black_knight":
            if (x - 1 == xx && y + 2 == yy) {
                return true;
            }
            if (x - 1 == xx && y - 2 == yy) {
                return true;
            }
            if (x + 1 == xx && y - 2 == yy) {
                return true;
            }
            if (x + 1 == xx && y + 2 == yy) {
                return true;
            }
            if (x - 2 == xx && y - 1 == yy) {
                return true;
            }
            if (x - 2 == xx && y + 1 == yy) {
                return true;
            }
            if (x + 2 == xx && y - 1 == yy) {
                return true;
            }
            if (x + 2 == xx && y + 1 == yy) {
                return true;
            }
            break;
    }
    }}
    return false;
}

function compute_rook(x, y, piece) {
    var total = 0;
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x + i, y);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x + i, y);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x - i, y);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x - i, y);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x, y - i);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x, y - i);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x, y + i);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x, y + i);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    return total;
}

function compute_bishop(x, y, piece) {
    var total = 0;
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x + i, y + i);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x + i, y + i);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x - i, y + i);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x - i, y + i);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x - i, y - i);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x - i, y - i);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    for (var i = 1; i < 8; i += 1) {
        let p = getPiece(x + i, y - i);
        if (p && p != piece) { break; }
        let a = getPieceWanted(x + i, y - i);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    return total;
}


function compute(x, y) {
    var piece = getPiece(x, y);
    switch (piece.type) {
        case "white_pawn":
            var totalpawn = 0;
            let l = getPieceWanted(x - 1, y - 1);
            let pl = getPiece(x - 1, y - 1);
            if (!pl || pl == piece) {
                totalpawn += (l && l.type == "checker" ? 1:0);
            }
            let r = getPieceWanted(x + 1, y - 1);
            let pr = getPiece(x - 1, y - 1);
            if (!pr || pr == piece) {
                totalpawn += (r && r.type == "checker" ? 1:0);
            }
            return totalpawn;
        case "white_rook":
            return compute_rook(x, y, piece);
        case "white_bishop":
            return compute_bishop(x, y, piece);
        case "white_queen":
            return compute_bishop(x, y, piece) + compute_rook(x, y, piece);
        case "white_knight":
            return compute_knight(x, y, piece);
    }
    return 0;
}

function compute_knight(x, y, piece) {
    var total = 0;
    xs = [-2, -1, 1, 2, 2, 1, -1, -2];
    ys = [-1, -2, -2, -1, 1, 2, 2, 1];
    for (var i = 0; i < 8; i += 1) {
        let p = getPiece(x + xs[i], y + ys[i]);
        if (p && p != piece) { continue; }
        let a = getPieceWanted(x + xs[i], y + ys[i]);
        if (a && a.type == "checker") {
            total += 1;
        }
    }
    return total;
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
        if (toMove) {
            var x = parseInt(n['data-x']) + xCord;
            var y = parseInt(n['data-y']) + yCord;
            if (movable(toMove, x, y)) {
                if (attackable(x, y)) {
                    n.classList.add("attackable");
                } else {
                    n.classList.add('movable');
                }
            }
        }
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
    if (grabbedPiece && (toMove != grabbedPiece) && grabbedPiece.color == "white") {
        toMove = grabbedPiece;
    }
    render();
}

function moveEnd(e) {
    var x = parseInt(e.target['data-x']) + xCord;
    var y = parseInt(e.target['data-y']) + yCord;
    var grabbedPiece = getPiece(x, y);
    if (toMove && (!grabbedPiece) && toMove != grabbedPiece) { 
        if (!attackable(x, y) && movable(toMove, x, y)) {
            toMove.x = x;
            toMove.y = y;
            if (toMove.type == "white_pawn" && y == 0) {
                document.getElementById("overlay").style.display = "block";
                toPromote = toMove;
            }
        }
        toMove = null;
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

function promote(n) {
    document.getElementById("overlay").style.display = "none";
    switch (n) {
        case 0:
            toPromote.type = "white_knight";
            break;
        case 1:
            toPromote.type = "white_bishop";
            break;
        case 2:
            toPromote.type = "white_rook";
            break;
        case 3:
            toPromote.type = "white_queen";
            break;
    }
    render();
    toPromote = null;
}
