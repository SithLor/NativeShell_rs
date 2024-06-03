//simple for isloating function to spreate process

//api:SperateProcessOnCall(arg) -> arg semi_isolated whick use ipc to communicate with the main process to return, and fully_isolated which use file to store the result and read it from the main process



const HIGH: u8 = 1;
const LOW: u8 = 0;
//impl shit serial protocall with 

//pin
//pin_0:grond 
//pin_1:data+
//pin_2:data- 
struct PinData {
    pin_1:u8,
    pin_2:u8,
    pin_3:u8,
    pin_4:u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_update() {
        let mut player = Player::new(10);
        player.update_score(5);
        assert_eq!(player.current_score, 5);
        assert_eq!(player.previous_score, 0);
    }

    #[test]
    fn test_cheat_detection() {
        let mut player = Player::new(10);
        player.update_score(5);
        player.update_score(3); // Decreasing score
        assert!(player.is_cheating());

        let mut player = Player::new(10);
        player.update_score(5);
        player.update_score(20); // Increasing score more than cheat_threshold
        assert!(player.is_cheating());

        let mut player = Player::new(10);
        player.update_score(5);
        player.update_score(10); // Increasing score within cheat_threshold
        assert!(!player.is_cheating());
    }
}