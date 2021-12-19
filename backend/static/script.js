import init, {WasmBoard} from "./pkg/backend.js";


var xCord = 0;
var yCord = 0;
var cell_prefix = "square_";
var toMove = null;
var madePawnsBlack = new Set();
var madePawnsWhite = new Set();
var size = 12;
var scrollFromX = null;
var scrollFromY = null;
var scrollFromXS = null;
var scrollFromYS = null;
var gTurn = 0;

var pieces = [];
var movable = [];
var board = null;

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

function render() {
    xCord = parseInt(document.getElementById("xport").value);
    yCord = parseInt(document.getElementById("yport").value);
    var newSize = parseInt(document.getElementById("zoom").value);
    if (size != newSize) {
        size = newSize;
        addSquares();
    }
    addPawns();
    for (var i = 0; i < size*size; i++) {
        var n = document.getElementById(cell_prefix + i);
        n.className = "";
        var f = size % 2 == 0? Math.floor(i/size) : 0;
        n.classList.add((f + i + xCord) % 2 == 0? "white_square" : "black_square");
    }
    for (var i = 0; i < pieces.length; i++) {
        var piece = pieces[i];
        if (displayed(piece.x, piece.y) && piece.alive) {
            var d = piece.x - xCord + (piece.y - yCord) * size;
            var n = document.getElementById(cell_prefix + d);
            n.classList.add(piece.type);
            if (piece == toMove) {
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

window.onload = function() {
    getBoard();
    addSquares();
    document.getElementById('xport').oninput = render;
    document.getElementById('yport').oninput = render;
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
        movable = [];
        getMoves();
    }
    render();
}

function moveEnd(e) {
    var x = parseInt(e.target['data-x']) + xCord;
    var y = parseInt(e.target['data-y']) + yCord;
    var grabbedPiece = getPiece(x, y);
    if (toMove && toMove != grabbedPiece) { 
        if (ismovable(e)  && (!grabbedPiece || grabbedPiece.color != toMove.color)) {
            make_move(x, y);
            toMove.x = x;
            toMove.y = y;
            if (grabbedPiece) {
                grabbedPiece.alive = false;
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
    fetch("/board/"+gTurn)
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed with status ${response.status}`)
      }
      return response.json()
    })
    .then(data => {
      console.log(data);
      pieces = data.pieces;
      for (var i = 0; i < pieces.length; i++) {
          board.place_piece(pieces[i].piece, pieces[i].color == "white", pieces[i].y, pieces[i].x);
      }
      madePawnsBlack.clear();
      madePawnsWhite.clear();
      for (var i = 0; i < data.black_pawns.length; i++) {
          madePawnsBlack.add(data.black_pawns[i]);
      }
      for (var i = 0; i < data.white_pawns.length; i++) {
        madePawnsWhite.add(data.white_pawns[i]);
    }
      gTurn = data["turn"] + 1;
      render();
      getBoard();
    })
    .catch(error => {console.log(error);
        gTurn = 0;
        getBoard();
    })
}

function getMoves() {
    ///legal/{px}/{py}/{wx}/{wy}/{zoom}
    /*
    fetch("/legal/"+toMove.y+"/"+toMove.x+"/"+yCord+"/"+xCord+"/"+size)
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed with status ${response.status}`)
      }
      return response.json()
    })
    .then(data => {
      console.log(data);
      movable = data;
      render();
    })
    .catch(error => console.log(error))*/
    movable = JSON.parse(board.get_legal_moves(""+toMove.y, ""+toMove.x, ""+yCord, ""+xCord, ""+size));
}

function make_move(x, y) {
    var tomx = toMove.x;
    var tomy = toMove.y;
    fetch("/move/"+toMove.y+"/"+toMove.x+"/"+y+"/"+x)
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed with status ${response.status}`)
      }
      return response.text()
    })
    .then(data => {
      board.do_move(""+tomy, ""+tomx, ""+y, ""+x);
      console.log(data);
      render();
    })
    .catch(error => console.log(error))
}

init()
  .then(() => {
    board = new WasmBoard();
    console.log(board);
    console.log(JSON.parse(board.get_legal_moves("1", "4", "0", "0", "8")));

  });