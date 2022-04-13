import init, { WasmBoard } from "/pkg/backend.js";


var xCord = 0;
var yCord = 0;
var cell_prefix = "square_";
var toMove = null;
var size = 12;
var scrollFromX = null;
var scrollFromY = null;
var scrollFromXS = null;
var scrollFromYS = null;
var gTurn = 0;
var toPromote = null;

var OFFLINE = true;
var AI = false;

var flipped = false;

var movable = [];
var board = null;

var cycle = [-1, -1];

function flip() {
    flipped = !flipped;
    render();
}

function centerOnPiece(piece) {
    document.getElementById("xport").value = piece.x - Math.floor(size / 2);
    document.getElementById("yport").value = piece.y - Math.floor(size / 2);
}

function cycleColor(color) {
    var pieces = JSON.parse(board.get_pieces());
    var i = color == "white" ? 0 : 1;
    do {
        cycle[i] = (cycle[i] + 1) % pieces.length;
    } while (pieces[cycle[i]].color != color || !pieces[cycle[i]].alive || (pieces[cycle[i]].piece == "pawn" && !pieces[cycle[i]].has_moved));
    centerOnPiece(pieces[cycle[i]]);
    render();
}

function promote(n) {
    var pt = ["knight", "bishop", "rook", "queen"][n];
    var toPromoteInfo = getPieceInfo(toPromote);
    if (OFFLINE) {
        document.getElementById("overlay").style.display = "none";
        board.promote("" + toPromoteInfo.y, "" + toPromoteInfo.x, pt);
        render();
        toPromote = null;
        return;
    }
    fetch("/promote/" + toPromoteInfo.y + "/" + toPromoteInfo.x + "/" + pt)
        .then(response => {
            if (!response.ok) {
                throw new Error(`Request failed with status ${response.status}`)
            }
            return response.text()
        })
        .then(data => {
            document.getElementById("overlay").style.display = "none";
            board.promote("" + toPromoteInfo.y, "" + toPromoteInfo.x, pt);
            render();
            toPromote = null;
        })
        .catch(error => console.log(error))

}
function undo() {
    console.log(board.undo_move());
    render();
}

window.promote = promote;
window.cycleColor = cycleColor;
window.flip = flip;
window.undo = undo;


function displayed(x, y) {
    x -= xCord;
    if (flipped) {
        y = -y;
    }
    y -= yCord;
    return x >= 0 && x < size && y >= 0 && y < size;
}
function getSpace(x, y) {
    x -= xCord;
    if (flipped) {
        y = -y;
    }
    y -= yCord;
    var d = x + (y) * size;
    return d;
}
function getPiece(x, y) {
    return board.get_piece_at("" + y, "" + x);
}

function getPieceInfo(i) {
    return JSON.parse(board.get_piece_info(i));
}

function addSquares() {
    var palette = document.getElementById("zoomable");
    while (palette.firstChild) {
        palette.removeChild(palette.firstChild);
    }
    for (var i = 0; i < size * size; i++) {
        var n = document.createElement("li");
        n['data-x'] = i % size;
        n['data-y'] = Math.floor(i / size);
        if (flipped) {
            n['data-y'] = Math.floor(i / size);
        }
        n.id = cell_prefix + i;
        n.onmousedown = moveBegin;
        n.onmouseup = moveEnd;
        n.ontouchstart = startTouch;
        n.ontouchmove = moveTouch;
        n.style.width = 100 / size + '%';
        n.style.height = n.style.width;
        palette.appendChild(n);
    }
}

function render() {
    var newSize = parseInt(document.getElementById("zoom").value);
    if (size != newSize) {
        size = newSize;
        addSquares();
    }
    xCord = parseInt(document.getElementById("xport").value);
    yCord = parseInt(document.getElementById("yport").value);
    if (flipped) {
        yCord = -yCord - size + 1;
    }
    var toMoveInfo = getPieceInfo(toMove);
    for (var i = 0; i < size * size; i++) {
        var n = document.getElementById(cell_prefix + i);
        n.className = "";
        var f = size % 2 == 0 ? Math.floor(i / size) : 0;
        n.classList.add((f + i + xCord) % 2 == 0 ? "white_square" : "black_square");
    }
    board.add_pawns("" + xCord, "" + size);
    var pieces = JSON.parse(board.get_pieces());
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (displayed(piece.x, piece.y) && piece.alive) {
            var d = getSpace(piece.x, piece.y);
            var n = document.getElementById(cell_prefix + d);
            n.classList.add(piece.type);
            if (piece.x == toMoveInfo.x && piece.y == toMoveInfo.y && toMove) {
                n.classList.add("selected");
            }
        }
    }
    if (toMove != null) {
        movable.length = 0;
        getMoves();
        console.log(movable);
        for (var i = 0; i < movable.length; i++) {
            var space = movable[i];
            var d = getSpace(space[1], space[0]);
            var n = document.getElementById(cell_prefix + d);
            if (n) {
                n.classList.add("movable");
            }
        }
    }
}

window.onload = function () {
    addSquares();
    document.getElementById('xport').oninput = render;
    document.getElementById('yport').oninput = render;
    var deltas = document.getElementsByClassName("delta");
    for (var i = 0; i < deltas.length; i++) {
        deltas[i].onclick = delta;
    }
};

function delta(e) {
    var ele = document.getElementById(e.target.getAttribute('data-target'));
    ele.value = parseInt(ele.value) + parseInt(e.target.getAttribute('data-value'));
    render();
}

function moveBegin(e) {
    var y = parseInt(e.target['data-y']);
    if (flipped) {
        y = -y - 2 * yCord;
    }
    var grabbedPiece = getPiece(parseInt(e.target['data-x']) + xCord, y + yCord);
    console.log(y, y + yCord, grabbedPiece);
    if (grabbedPiece != null && toMove == null) {
        toMove = grabbedPiece;
        movable = [];
        getMoves();
    }
    render();
}

function moveEnd(e) {
    var x = parseInt(e.target['data-x']) + xCord;
    var y = parseInt(e.target['data-y']);
    if (flipped) {
        y = -y - 2 * yCord;
    }
    y += yCord;
    var grabbedPiece = getPiece(x, y);
    var grabbedPieceInfo = getPieceInfo(grabbedPiece);
    var toMoveInfo = getPieceInfo(toMove);
    if (toMove != null && toMove != grabbedPiece) {
        if (ismovable(e) && (!grabbedPiece || grabbedPieceInfo.color != toMoveInfo.color)) {
            make_move(x, y);
            toMoveInfo.x = x;
            toMoveInfo.y = y;
            if ((toMoveInfo.type == "white_pawn" && y == 0) || (toMoveInfo.type == "black_pawn" && y == 7)) {
                document.getElementById("overlay").style.display = "block";
                toPromote = toMove;
            }
            render();
            if (AI) {
                if (board.ai()) {
                    alert("YOU WON");
                }
                render();
            }
        }
        toMove = null;
        movable = [];
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

function ismovable(e) {
    return e.target.classList.contains("movable");
}

function getBoard() {
    if (OFFLINE) {
        return;
    }
    fetch("/board/" + gTurn)
        .then(response => {
            if (!response.ok) {
                throw new Error(`Request failed with status ${response.status}`)
            }
            return response.text();
        }).then(text => {
            var data = JSON.parse(text);
            board.build(text);
            gTurn = parseInt(data["turn"]) + 1;
            render();
            getBoard();
        })
        .catch(error => {
            console.log(error);
            gTurn = 0;
            setTimeout(getBoard, 1000);
        })
}

function getMoves() {
    var toMoveInfo = getPieceInfo(toMove);
    console.log(toMove, toMoveInfo);
    var yyCord = yCord;
    if (flipped) {
        yyCord = -yyCord - size + 1;
    }
    movable = JSON.parse(board.get_legal_moves("" + toMoveInfo.y, "" + toMoveInfo.x, "" + yyCord, "" + xCord, "" + size));
}

function make_move(x, y) {
    var toMoveInfo = getPieceInfo(toMove);
    var tomx = toMoveInfo.x;
    var tomy = toMoveInfo.y;
    if (OFFLINE) {
        board.do_move("" + tomy, "" + tomx, "" + y, "" + x);
        render();
        return;
    }
    fetch("/move/" + tomy + "/" + tomx + "/" + y + "/" + x)
        .then(response => {
            if (!response.ok) {
                throw new Error(`Request failed with status ${response.status}`)
            }
            return response.text()
        })
        .then(data => {
            board.do_move("" + tomy, "" + tomx, "" + y, "" + x);
            render();
        })
        .catch(error => console.log(error))
}


init()
    .then(() => {
        board = new WasmBoard();
        if (OFFLINE) {
            place_pieces()
            return;
        }
        getBoard();
    });


function place_pieces() {
    board.place_piece(
        "rook",
        false,
        "0",
        "0",
    );
    board.place_piece(
        "rook",
        true,
        "7",
        "0",
    );
    board.place_piece(
        "knight",
        false,
        "0",
        "1",
    );
    board.place_piece(
        "knight",
        true,
        "7",
        "1",
    );
    board.place_piece(
        "bishop",
        false,
        "0",
        "2",
    );
    board.place_piece(
        "bishop",
        true,
        "7",
        "2",
    );
    board.place_piece(
        "queen",
        false,
        "0",
        "3",
    );
    board.place_piece(
        "queen",
        true,
        "7",
        "3",
    );
    board.place_piece(
        "king",
        false,
        "0",
        "4",
    );
    board.place_piece(
        "king",
        true,
        "7",
        "4",
    );
    board.place_piece(
        "bishop",
        false,
        "0",
        "5",
    );
    board.place_piece(
        "bishop",
        true,
        "7",
        "5",
    );
    board.place_piece(
        "knight",
        false,
        "0",
        "6",
    );
    board.place_piece(
        "knight",
        true,
        "7",
        "6",
    );
    board.place_piece(
        "rook",
        false,
        "0",
        "7",
    );
    board.place_piece(
        "rook",
        true,
        "7",
        "7",
    );
    render();
}