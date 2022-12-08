use crate::utils::error::BadAppleError;
use crate::utils::player::Player;

pub fn print_ascii_from_video(filename: &str) -> Result<(), BadAppleError> {
    let mut video_player = Player::new(filename)?;
    video_player.play_to_stdout()?;

    Ok(())
}