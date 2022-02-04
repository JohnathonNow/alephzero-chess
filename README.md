# alephzero-chess  
An online infinite chess game, implemented with rust, wasm, and vanilla JS.

You can play it locally [here](https://alephzerochess.com/)!

## The Game  
The game is played with a standard chess board, except it extends infinitely in both directions.
Additionally, the knight's movement is slightly reinterpretted to help keep it on par with the bishop
(which would be able to move infinitely farther than the knight otherwise). The pieces are placed
where they would be normally relative to each other.

## On Your Turn  
Simply click the piece you want to move, and then click where you want to move it to. Options will
be displayed in orange. On desktop, you may click and drag a piece where you want it to go. On mobile,
however, clicking and dragging will pan the "camera" of the game. On both desktop and mobile,
you can also either edit the camera position text boxes, or use the + and - buttons, to move the camera.

## Dev Stuff  
The project's backend is written in rust. The front end is written in standard
vanilla JS+CSS+HTML, but utilizes wasm to avoid having to re-write the move validator
in JS. 

### Build Instructions  
To build the front end, run `wasm-pack build --target web` and then copy the resulting `pkg` directory into the static directory.

To build the game server, run `cargo build --bin server --features="server"`.

### Runtime Instructions  
The game currently takes no arguments and only listens on port 8080. Simply run the executable, perhaps by running `./target/debug/server`.
You must run the server from the `backend` directory of the project, as it uses resources from the `static` directory. Sorry about that.

