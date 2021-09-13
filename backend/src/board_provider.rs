pub trait BoardProvider {
    fn load(&self, game_id: String) -> Board;
    fn save(&self, board: Board, game_id: String);
}
