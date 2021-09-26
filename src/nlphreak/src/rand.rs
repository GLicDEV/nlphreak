use ic_cdk::print;
use std::vec;

const KX: u64 = 123456789;
const KY: u64 = 362436069;
const KZ: u64 = 521288629;
const KW: u64 = 88675123;

pub struct Rand {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

impl Rand {
    pub fn new(seed: u64) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u64 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }

    pub fn rand_words(n: u8, seed: u64) -> Vec<String> {
        let words: Vec<String> = vec![
            "abandon", "abstract", "access", "account", "acid", "acoustic", "across", "actual",
            "addict", "address", "advance", "again", "airport", "alien", "all", "alley", "alpha",
            "always", "amateur", "answer", "antenna", "anxiety", "any", "appear", "apple", "area",
            "army", "arrest", "arrive", "artefact", "ask", "athlete", "attend", "attitude",
            "attract", "aunt", "auto", "average", "avoid", "awake", "bag", "balcony", "bargain",
            "barrel", "base", "basic", "before", "between", "beyond", "biology", "bird", "bitter",
            "blade", "blame", "blast", "board", "boost", "boring", "brand", "bread", "bridge",
            "broom", "buddy", "bulb", "bulk", "bullet", "burger", "burst", "butter", "cabbage",
            "cabin", "cake", "cancel", "cannon", "canvas", "capable", "capital", "captain", "card",
            "castle", "cat", "cause", "census", "chapter", "chat", "cherry", "choice", "chunk",
            "cinnamon", "civil", "claim", "clap", "client", "clinic", "clip", "clog", "cloth",
            "cloud", "club", "code", "color", "column", "comic", "congress", "consider", "coral",
            "core", "correct", "cotton", "country", "cover", "cram", "crane", "crouch", "crucial",
            "cruel", "cruise", "current", "custom", "cute", "dance", "daring", "day", "decade",
            "decline", "defense", "define", "defy", "demise", "depth", "deputy", "detect",
            "device", "diesel", "diet", "dirt", "dismiss", "display", "divert", "dizzy", "doctor",
            "document", "donor", "dove", "draw", "dress", "drill", "duck", "dune", "dutch", "east",
            "easy", "echo", "economy", "educate", "effort", "electric", "else", "empower", "enact",
            "end", "energy", "enforce", "engage", "engine", "enrich", "equip", "era", "erase",
            "error", "escape", "evidence", "evolve", "example", "exclude", "explain", "expose",
            "extra", "fabric", "faculty", "fade", "faith", "fall", "false", "fancy", "farm",
            "father", "fatigue", "fault", "february", "female", "fence", "fetch", "final", "find",
            "fine", "finger", "fire", "first", "fiscal", "fitness", "flash", "flee", "fluid",
            "flush", "fly", "foil", "fold", "forest", "fork", "frame", "frog", "fruit", "fury",
            "future", "gate", "gauge", "genre", "gesture", "ghost", "ginger", "give", "glad",
            "glance", "glass", "glimpse", "globe", "gold", "gossip", "gown", "grace", "grain",
            "grass", "great", "group", "guess", "gun", "hair", "hand", "harsh", "hawk", "height",
            "helmet", "help", "high", "hip", "history", "hold", "hollow", "horror", "hospital",
            "hub", "human", "humor", "hunt", "husband", "hybrid", "image", "immense", "immune",
            "increase", "indicate", "infant", "inform", "initial", "inner", "insane", "install",
            "invite", "isolate", "ivory", "jaguar", "jealous", "junior", "kangaroo", "ketchup",
            "kick", "kingdom", "kitten", "knife", "lab", "later", "laugh", "lava", "lawsuit",
            "leader", "lecture", "leg", "legend", "letter", "library", "lift", "light", "list",
            "loan", "lounge", "love", "lunar", "magnet", "major", "manage", "mango", "maple",
            "marble", "march", "margin", "marriage", "menu", "mesh", "midnight", "mimic", "mind",
            "minute", "moment", "monkey", "monster", "month", "motor", "move", "must", "name",
            "nature", "never", "north", "novel", "obey", "obscure", "observe", "obvious", "ocean",
            "october", "odor", "office", "okay", "old", "omit", "once", "online", "only", "open",
            "opinion", "option", "orange", "organ", "original", "orphan", "other", "output",
            "over", "panda", "party", "patient", "pear", "peasant", "pencil", "people", "pepper",
            "photo", "phrase", "piano", "picnic", "pig", "pigeon", "pipe", "pistol", "pluck",
            "polar", "popular", "position", "post", "pottery", "pretty", "prevent", "primary",
            "program", "promote", "protect", "provide", "puppy", "purchase", "purpose", "put",
            "pyramid", "quote", "race", "random", "rapid", "raven", "real", "record", "recycle",
            "refuse", "relief", "rely", "reopen", "repair", "repeat", "replace", "rescue",
            "resist", "result", "return", "reveal", "rich", "right", "ring", "risk", "road",
            "roast", "roof", "rookie", "room", "royal", "sad", "sadness", "sail", "salmon",
            "salon", "salute", "sauce", "scan", "scare", "scout", "screen", "sea", "season",
            "segment", "sell", "sense", "sentence", "series", "service", "session", "shell",
            "shoe", "shrimp", "shuffle", "sign", "siren", "slab", "sleep", "slight", "slush",
            "smart", "smooth", "snake", "soap", "soda", "solution", "soon", "sorry", "sort",
            "sound", "space", "special", "sphere", "spike", "sponsor", "spot", "squeeze", "stable",
            "stairs", "start", "sting", "stomach", "stone", "stove", "strategy", "strong", "style",
            "submit", "suit", "surge", "survey", "swamp", "swim", "talk", "target", "task",
            "teach", "tennis", "test", "thing", "this", "thought", "three", "throw", "thunder",
            "tide", "timber", "tired", "tissue", "token", "tomorrow", "tongue", "top", "topic",
            "tornado", "toss", "track", "trap", "travel", "tray", "tree", "trick", "truly",
            "tuition", "tunnel", "turn", "twice", "twist", "type", "uncle", "uncover", "under",
            "update", "uphold", "usage", "useless", "vacant", "valid", "vendor", "vessel", "video",
            "vital", "vocal", "vote", "walnut", "water", "wealth", "wear", "weasel", "weird",
            "wet", "where", "whisper", "wide", "width", "wife", "wing", "wisdom", "wolf", "world",
            "young",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let wlen = words.len();
        let mut index: usize = 0;
        let mut output: Vec<String> = Vec::new();

        for _i in 1..n {
            let rand = Rand::new(seed + index as u64).rand();
            index = rand as usize % wlen;
            output.push(words[index].to_owned());

            //ic_cdk::println!("rand: {:?} {:?} {:?}", rand, index, wlen);
        }

        output
    }
}
