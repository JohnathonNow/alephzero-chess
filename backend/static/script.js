var zoom = 1024;

function chzoom(w) {
    zoom *= w;

	document.getElementById("zoomable").style.width = zoom+"px";
	document.getElementById("zoomable").style.height = zoom+"px";
}
 
window.onload = function() {
    var palette = document.getElementById("zoomable");
    var cell_prefix = "square_";
    for (var i = 0; i < 8*8; i++) {
        var n = document.createElement("li");
        n.style["background-color"] = "#000000";
        if (i > 7 && i < 16) {
            n.style["background-image"] = "url(\"/pieces/white_pawn.svg\")";
        }
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
};
