use rand::seq::IteratorRandom;
use Role::*;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all="snake_case")]
pub enum Role {
    SuperHero,
    Impostor,
    Crook,
    Kamikaze,
    Romeo,
    TwoFace,
    Bot,
}

/// Generates a random composition for a game
/// 
/// The composition will contain:
/// - One SuperHero
/// - One Impostor and or one Crook
/// - Two to three other roles, to fill up to five roles
pub fn generate_composition() -> Vec<Role> {
    let mut roles = vec![SuperHero];

    match rand::random::<u8>() % 3 {
        0 => roles.push(Impostor),
        1 => roles.push(Crook),
        _ => {
            roles.push(Impostor);
            roles.push(Crook)
        }
    }

    let other_roles = vec![Kamikaze, Romeo, TwoFace, Bot];
    let slice = other_roles
        .iter()
        .choose_multiple(&mut rand::thread_rng(), 5 - roles.len());
    slice.iter().for_each(|r| roles.push(**r));

    roles
}
