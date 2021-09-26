use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::storage;
use ic_cdk_macros::*;
use rand::Rand;
use std::collections::BTreeMap;

mod rand;
mod utils;

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub max_score: u32,
    pub games: Vec<u32>,
}

// impl Profile {
//     pub fn add_game_id(&mut self, game_id: u32) {
//         self.games.push(game_id);
//     }
// }

#[derive(Clone, Debug, CandidType, Deserialize)]
enum GameState {
    New,
    Asked,
    Answered,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::New
    }
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Game {
    pub id: u32,
    pub keywords: Vec<String>,
    pub points: Vec<u8>,
    pub found: Vec<bool>,
    pub score: u32,
    pub questions: Vec<String>,
    pub answers: Vec<String>,
    pub principal_str: String,
    pub state: GameState,
}

//Alternative to returning rust Result type (can't receive it in JS)
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Res {
    pub ok: bool,
    pub payload: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct GameCollection {
    pub total_games: u32,
    pub games: BTreeMap<u32, Game>,
    pub global_max_score: u32,
    pub global_max_score_id: u32,
}

impl GameCollection {
    pub fn clear(&mut self) {
        self.total_games = 0;
        self.global_max_score = 0;
        self.global_max_score_id = 0;
        self.games.clear();
    }

    pub fn new_game(&mut self, principal: Principal) -> u32 {
        self.total_games += 1;

        //create a new game and populate it with keywords
        let mut new_game = Game::default();
        let mut points: Vec<u8> = Vec::new();

        new_game.id = self.total_games;
        new_game.keywords = Rand::rand_words(13, self.total_games as u64 + ic_cdk::api::time());

        //calculate keyword points
        new_game
            .keywords
            .iter()
            .map(|s| points.push(utils::scrabble_points(s.to_string())))
            .count();

        new_game.points = points;

        new_game.principal_str = principal.to_string();

        self.games.insert(self.total_games, new_game);

        //return the newly created game id so the client can redirect
        self.total_games
    }

    pub fn list_games(&self) -> Vec<Game> {
        let mut output: Vec<Game> = Vec::new();

        for (_, g) in self.games.iter() {
            output.push(g.clone())
        }

        output
    }

    pub fn get_game(&self, game_id: u32) -> Game {
        self.games
            .get(&game_id)
            .cloned()
            .unwrap_or_else(|| Game::default())
    }

    pub fn get_asked(&self) -> Vec<u32> {
        let mut found: Vec<u32> = Vec::new();

        self.games.iter().for_each(|g| {
            if g.1.to_owned().answers.len() == 0 && g.1.to_owned().questions.len() > 0 {
                found.push(*g.0);
            }
        });

        found
    }

    pub fn add_questions(
        &mut self,
        game_id: u32,
        questions: Vec<String>,
        principal: Principal,
    ) -> Result<(), String> {
        let game = self.games.get_mut(&game_id).unwrap();

        //remove keywords from questions
        let mut clean_questions: Vec<String> = Vec::new();

        //TODO: refactor without map
        questions
            .iter()
            .map(|q| {
                let mut tmpq = q.clone();
                game.keywords
                    .iter()
                    .map(|k| tmpq = tmpq.replace(k, ""))
                    .count();
                clean_questions.push(tmpq);
            })
            .count();

        //check if the caller owns the game and if the game is in the New state
        match game.state {
            GameState::New => {
                if game.principal_str == principal.to_string() {
                    game.questions = clean_questions;
                } else {
                    return Err("Tried to add questions to game owned by someone else".to_string());
                }
            }
            _ => return Err("Tried to add questions in unsupported state".to_string()),
        }

        game.state = GameState::Asked;
        Ok(())
    }

    pub fn add_answers(&mut self, game_id: u32, answers: Vec<String>) -> Result<(), String> {
        let game = self.games.get_mut(&game_id).unwrap();

        //check if the caller owns the game and if the game is in the New state
        match game.state {
            GameState::Asked => {
                game.answers = answers.clone();
            }
            _ => return Err("Tried to add answers in unsupported state".to_string()),
        }

        let mut tmp_found: Vec<bool> = Vec::new();
        for (_pos, _v) in game.keywords.iter().enumerate() {
            tmp_found.push(false);
        }

        for (pos, kword) in game.keywords.iter().enumerate() {
            answers.iter().for_each(|a| {
                if a.contains(kword) {
                    tmp_found[pos] = true;
                }
            })
        }

        game.found = tmp_found;

        let mut tmp_score: u32 = 0;

        for (pos, v) in game.found.iter().enumerate() {
            if *v == true {
                tmp_score += game.points[pos] as u32;
            }
        }

        game.score = tmp_score;

        game.state = GameState::Answered;
        Ok(())
    }
}

#[query(name = "getSelf")]
fn get_self() -> Profile {
    let id = ic_cdk::caller();
    let profile_store = storage::get::<ProfileStore>();

    profile_store
        .get(&id)
        .cloned()
        .unwrap_or_else(|| Profile::default())
}

#[query(name = "getGame")]
fn get_game(game_id: u32) -> Game {
    let game_store = storage::get::<GameCollection>();

    game_store.get_game(game_id)
}

#[query(name = "getAsked")]
fn get_asked() -> Vec<u32> {
    let game_store = storage::get::<GameCollection>();

    game_store.get_asked()
}

#[query(name = "listAllGames")]
fn list_games() -> Vec<Game> {
    let game_store = storage::get::<GameCollection>();

    let output = game_store.list_games();
    output
}

#[update(guard = "is_auth", name = "addQuestions")]
fn add_questions(game_id: u32, questions: Vec<String>) -> Res {
    let mut response = Res::default();
    let principal = ic_cdk::caller();

    let game_store = storage::get_mut::<GameCollection>();

    match game_store.add_questions(game_id, questions, principal) {
        Ok(()) => response.ok = true,
        Err(msg) => {
            response.ok = false;
            response.payload = msg.to_string();
        }
    }

    response
}

#[update(name = "addAnswers")]
fn add_answers(game_id: u32, answers: Vec<String>) -> Res {
    let mut response = Res::default();
    //let principal = ic_cdk::caller();
    //TODO: check principal of answers uploader?

    let game_store = storage::get_mut::<GameCollection>();

    match game_store.add_answers(game_id, answers) {
        Ok(()) => response.ok = true,
        Err(msg) => {
            response.ok = false;
            response.payload = msg.to_string();
        }
    }

    response
}

#[update(guard = "is_auth", name = "update")]
fn update(name: String) -> Res {
    let principal_id = ic_cdk::caller();
    let id_store = storage::get_mut::<IdStore>();
    let profile_store = storage::get_mut::<ProfileStore>();

    let mut response = Res::default();

    if id_store.contains_key(&name) {
        response.ok = false;
        response.payload = "Username already exists".to_string();

        return response;
    }

    let mut profile = Profile::default();
    profile.name = name.clone();

    id_store.insert(name.clone(), principal_id.clone());
    profile_store.insert(principal_id, profile);

    response.ok = true;
    response
}

#[update(guard = "is_auth", name = "startGame")]
fn start_game() -> Res {
    let id_str = ic_cdk::caller();
    let mut response = Res::default();

    let game_store = storage::get_mut::<GameCollection>();

    let game_id = game_store.new_game(id_str);

    response.ok = true;
    response.payload = game_id.to_string();

    //add the game_id to the user's profile
    let profile_store = storage::get_mut::<ProfileStore>();
    profile_store.get_mut(&id_str).unwrap().games.push(game_id);

    response
}

#[update(name = "clearAll")]
fn clear_all() {
    let id_store = storage::get_mut::<IdStore>();
    let profile_store = storage::get_mut::<ProfileStore>();
    let game_store = storage::get_mut::<GameCollection>();

    id_store.clear();
    profile_store.clear();
    game_store.clear();
}

//GUARDS

fn is_auth() -> Result<(), String> {
    let principal_id = ic_cdk::caller();

    if principal_id.to_text() == "2vxsx-fae" {
        return Err("User is anon".to_string());
    }

    Ok(())
}
