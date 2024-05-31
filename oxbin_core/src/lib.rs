//simple for isloating function to spreate process

//api:SperateProcessOnCall(arg) -> arg semi_isolated whick use ipc to communicate with the main process to return, and fully_isolated which use file to store the result and read it from the main process


//detect cheateninge by checking the Score of the player scale value with older value 

struct Player {
    current_score: u32,
    previous_score: u32,
    cheat_threshold: u32,
}

impl Player {
    fn new(cheat_threshold: u32) -> Self {
        Player {
            current_score: 0,
            previous_score: 0,
            cheat_threshold,
        }
    }

    fn update_score(&mut self, new_score: u32) {
        self.previous_score = self.current_score;
        self.current_score = new_score;
    }

    fn is_cheating(&self) -> bool {
        self.current_score < self.previous_score || self.current_score > self.previous_score + self.cheat_threshold
    }
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