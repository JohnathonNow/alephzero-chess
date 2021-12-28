import init, { WasmBoard } from "./pkg/backend.js";


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

var movable = [];
var board = null;


function promote(n) {
    var pt = ["knight", "bishop", "rook", "queen"][n];
    var toPromoteInfo = getPieceInfo(toPromote);
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

window.promote = promote;

function displayed(x, y) {
    x -= xCord;
    y -= yCord;
    return x >= 0 && x < size && y >= 0 && y < size;
}

function getPiece(x, y) {
    return board.get_piece_at(""+y, ""+x);
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
    xCord = parseInt(document.getElementById("xport").value);
    yCord = parseInt(document.getElementById("yport").value);
    var newSize = parseInt(document.getElementById("zoom").value);
    var toMoveInfo = getPieceInfo(toMove);
    if (size != newSize) {
        size = newSize;
        addSquares();
    }
    for (var i = 0; i < size * size; i++) {
        var n = document.getElementById(cell_prefix + i);
        n.className = "";
        var f = size % 2 == 0 ? Math.floor(i / size) : 0;
        n.classList.add((f + i + xCord) % 2 == 0 ? "white_square" : "black_square");
    }
    board.add_pawns(""+xCord, ""+size);
    var pieces = JSON.parse(board.get_pieces());
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (displayed(piece.x, piece.y) && piece.alive) {
            var d = piece.x - xCord + (piece.y - yCord) * size;
            var n = document.getElementById(cell_prefix + d);
            n.classList.add(piece.type);
            if (piece.x == toMoveInfo.x && piece.y == toMoveInfo.y) { //fix this
                n.classList.add("selected");
            }
        }
    }
    for (var i = 0; i < movable.length; i++) {
        var space = movable[i];
        var d = space[1] - xCord + (space[0] - yCord) * size;
        var n = document.getElementById(cell_prefix + d);
        n.classList.add("movable");
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
    var grabbedPiece = getPiece(parseInt(e.target['data-x']) + xCord, parseInt(e.target['data-y']) + yCord);
    if (grabbedPiece && !toMove) {
        toMove = grabbedPiece;
        movable = [];
        getMoves();
    }
    render();
}

function moveEnd(e) {
    var x = parseInt(e.target['data-x']) + xCord;
    var y = parseInt(e.target['data-y']) + yCord;
    var grabbedPiece = getPiece(x, y);
    var grabbedPieceInfo = getPieceInfo(grabbedPiece);
    var toMoveInfo = getPieceInfo(toMove);
    if (toMove && toMove != grabbedPiece) {
        if (ismovable(e) && (!grabbedPiece || grabbedPieceInfo.color != toMoveInfo.color)) {
            make_move(x, y);
            toMoveInfo.x = x;
            toMoveInfo.y = y;
            if ((toMoveInfo.type == "white_pawn" && y == 0) || (toMoveInfo.type == "black_pawn" && y == 7)) {
                document.getElementById("overlay").style.display = "block";
                toPromote = toMove;
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
    movable = JSON.parse(board.get_legal_moves("" + toMoveInfo.y, "" + toMoveInfo.x, "" + yCord, "" + xCord, "" + size));
}

function make_move(x, y) {
    var toMoveInfo = getPieceInfo(toMove);
    var tomx = toMoveInfo.x;
    var tomy = toMoveInfo.y;
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
        getBoard();
    });
