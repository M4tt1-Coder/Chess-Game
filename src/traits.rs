pub trait FigureTrait {
    fn get_thrown_out(&mut self){
        println!("Oohh I lost my figure ...");
    }

    //These functions are called when the pawn of a player reaches the end 
    // -> develops the other figure type on the board
    fn change_to_rock(&mut self);

    fn change_to_knight(&mut self);

    fn change_to_bishop(&mut self);

    fn change_to_queen(&mut self);
}