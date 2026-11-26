//! The quiz itself: every state, how hard it is, which ones look alike, the
//! roasts, and the question generator. All the variety lives here: each game
//! draws a fresh plan of difficulties, Aussie rounds and modifiers, decoys come
//! from overlapping look-alike groups, and titles, roasts and quips are pooled.

use rand::seq::{IndexedRandom, SliceRandom};
use rand::RngExt;
use std::collections::HashSet;

pub struct State {
    pub id: &'static str,
    pub name: &'static str,
    /// 1 easy (everyone gets it), 2 medium, 3 hard (rectangles and friends)
    pub tier: u8,
    pub roasts: &'static [&'static str],
}

impl State {
    pub fn au(&self) -> bool {
        self.id.starts_with("AU-")
    }
}

macro_rules! states {
    ($($id:literal, $name:literal, $tier:literal, [$($r:literal),+ $(,)?];)*) => {
        pub static STATES: &[State] = &[$(State { id: $id, name: $name, tier: $tier, roasts: &[$($r),+] }),*];
    };
}

states! {
    "US-AL", "Alabama", 3, ["Not Mississippi. Turn it around and it's Mississippi. Don't turn it around.", "Has a tiny little foot dipped in the Gulf, like it's testing the water."];
    "US-AK", "Alaska", 1, ["So big they shrink it on every map out of politeness.", "Comes with a free spray of islands. Some assembly required."];
    "US-AZ", "Arizona", 3, ["A rectangle that lost a fight with a river.", "The Grand Canyon's landlord."];
    "US-AR", "Arkansas", 3, ["Missouri's shorter sibling with a messier right side.", "Pronounced like Kansas, absolutely not like Kansas."];
    "US-CA", "California", 1, ["A state shaped like it's leaning in for a hug from Nevada.", "Basically a country with a coastline problem."];
    "US-CO", "Colorado", 3, ["Technically a hexacontagon: 697 sides, all pretending to be four.", "If you said Wyoming, you're in great company and still wrong."];
    "US-CT", "Connecticut", 3, ["A rectangle with a little tail, like it's waving goodbye to New York.", "The state equivalent of beige paint."];
    "US-DE", "Delaware", 3, ["A state so small the outline is mostly coastline anxiety.", "That curved top border is a literal circle. Look it up."];
    "US-FL", "Florida", 1, ["The only state everyone can draw, and it's still weird.", "America's thumbs-up. Or something else, depending on your angle."];
    "US-GA", "Georgia", 3, ["Alabama's twin from a different mother.", "Peaches, and a border that gave up halfway down."];
    "US-HI", "Hawaii", 1, ["A state you need a boat to visit all of.", "The only state that's mostly volcano by volume."];
    "US-ID", "Idaho", 1, ["A pistol, a boot, or a chair, depending on how much you've had to drink.", "The panhandle exists purely to annoy Montana."];
    "US-IL", "Illinois", 3, ["Indiana with a bit more wobble on the left.", "Chicago and a lot of corn, shaped like a dropped ice cream."];
    "US-IN", "Indiana", 3, ["Illinois, but someone used a ruler.", "Looks like a phone with a cracked bottom."];
    "US-IA", "Iowa", 3, ["A rectangle with river damage on both sides.", "So flat the outline is the most interesting thing about it."];
    "US-KS", "Kansas", 3, ["A rectangle that took a bite out of its top-right corner.", "Literally flatter than a pancake. That's been measured."];
    "US-KY", "Kentucky", 2, ["Looks like a duck going backwards, or Victoria. Mostly Victoria.", "Has a tiny chunk stranded in a river loop. Truly committed."];
    "US-LA", "Louisiana", 1, ["A boot that's been left out in the rain too long.", "Half state, half swamp, all bayou fringe."];
    "US-ME", "Maine", 2, ["A mitten made entirely of coastline.", "Like New Hampshire, if New Hampshire ate well."];
    "US-MD", "Maryland", 2, ["Looks like someone tried to cut out a state and sneezed.", "A shape so chaotic it should come with a warning label."];
    "US-MA", "Massachusetts", 2, ["Flexing its little Cape Cod arm at the Atlantic.", "A rectangle that grew a hook."];
    "US-MI", "Michigan", 1, ["The mitten. Plus a second, bonus bit nobody asked about.", "The only state that gives directions with its hand."];
    "US-MN", "Minnesota", 2, ["A square with an arm reaching for Lake Superior.", "Contains a weird little chimney up top that technically belongs to Canada's problem."];
    "US-MS", "Mississippi", 3, ["Alabama in a mirror. Only the left edge gives it away.", "Four S's, four I's, two P's, one identity crisis with Alabama."];
    "US-MO", "Missouri", 3, ["Has a bootheel. Not to be confused with the other boot, or Arkansas.", "A square that melted on the right."];
    "US-MT", "Montana", 2, ["A rectangle someone sat on from the left.", "Big Sky Country. Big, vague shape."];
    "US-NE", "Nebraska", 3, ["Like Kansas, but with a little chimney on the left.", "A panhandle state, for people who enjoy saying panhandle."];
    "US-NV", "Nevada", 1, ["California's other half, the one with all the casinos.", "A cheese wedge with a gambling problem."];
    "US-NH", "New Hampshire", 3, ["Vermont, upside down. Seriously, check.", "Has about 13 miles of coastline and won't stop talking about it."];
    "US-NJ", "New Jersey", 2, ["A seahorse with attitude.", "Shaped like it's about to start an argument."];
    "US-NM", "New Mexico", 3, ["A square with a little step at the bottom, like it's about to go downstairs.", "Not new, and not Mexico."];
    "US-NY", "New York", 2, ["A state with a long island poking out the bottom like a tongue.", "Mostly not the city, despite what the city thinks."];
    "US-NC", "North Carolina", 2, ["A wedge with a ragged coastal fringe.", "The Outer Banks are just the state's eyelashes."];
    "US-ND", "North Dakota", 3, ["A rectangle. Next.", "South Dakota's upstairs neighbour."];
    "US-OH", "Ohio", 2, ["A heart, if you squint and really love Ohio.", "Round on the ends and hi in the middle."];
    "US-OK", "Oklahoma", 1, ["A frying pan, handle and all.", "Texas's hat."];
    "US-OR", "Oregon", 2, ["Washington's quieter cousin, slightly more square.", "Its top border is a river and that's its personality."];
    "US-PA", "Pennsylvania", 2, ["A rectangle with a frilly right side and a tiny notch on top.", "Has a chimney poking up to Lake Erie."];
    "US-RI", "Rhode Island", 3, ["Not an island. Barely a state. Love it.", "Mostly bay. Fit it in your pocket."];
    "US-SC", "South Carolina", 2, ["An upside-down triangle with a frilly coast.", "North Carolina's snack-sized sibling."];
    "US-SD", "South Dakota", 3, ["North Dakota, but the bottom right melted.", "A rectangle with a dented corner."];
    "US-TN", "Tennessee", 1, ["A parallelogram that tripped on the way in.", "Long, thin, and full of country music."];
    "US-TX", "Texas", 1, ["Everything's bigger in Texas, including the ego.", "So recognisable it's on the belt buckles."];
    "US-UT", "Utah", 1, ["A square with a bite taken out by Wyoming.", "Colorado with a corner missing."];
    "US-VT", "Vermont", 3, ["New Hampshire flipped. They know. They're not happy.", "Shaped like a slice of pie someone fumbled."];
    "US-VA", "Virginia", 2, ["A triangle with a tail flicking into the bay.", "Looks a bit like Kentucky got lost in the Atlantic."];
    "US-WA", "Washington", 2, ["A square with a jigsaw nibble out of the top-left.", "Wet, green, and full of coffee."];
    "US-WV", "West Virginia", 2, ["Shaped like a squashed octopus waving both arms.", "Has two panhandles. Greedy."];
    "US-WI", "Wisconsin", 2, ["A mitten for the other hand, held next to Michigan's.", "Has a thumb poking into Lake Michigan."];
    "US-WY", "Wyoming", 3, ["A perfect rectangle. Or Colorado. Nobody knows.", "Does Wyoming exist? The evidence is thin."];
    "AU-NSW", "New South Wales", 2, ["Australia's most populated slice, shaped like a bitten sandwich.", "Comes with the ACT sitting in it like a stone in a shoe."];
    "AU-VIC", "Victoria", 2, ["Also known as 'basically Kentucky'.", "Small, wedge-shaped, and will tell you about the coffee."];
    "AU-QLD", "Queensland", 2, ["Shaped like a giant pointing finger. That's Cape York.", "Big, hot, and full of things that want to hurt you."];
    "AU-SA", "South Australia", 3, ["A big box with two bites out of the bottom.", "The Bight is what happens when a state gets hangry."];
    "AU-WA", "Western Australia", 2, ["One third of a continent and still drawn as a big lump.", "Big enough to swallow Texas several times. Ask Texas."];
    "AU-TAS", "Tasmania", 1, ["The island everyone forgets until they need a map.", "Australia's little shield, hanging off the bottom."];
    "AU-NT", "Northern Territory", 3, ["A rectangle with a beard of mangroves.", "Not a state. Shaped like a state. Pretending."];
    "AU-ACT", "Australian Capital Territory", 3, ["A blob around Canberra. That's it. That's the territory.", "Small, oddly shaped, mostly public servants."];
}

/// Look-alike groups. Overlapping on purpose: a state's decoys come from any
/// group it's in, so the same answer gets different traps game to game.
pub struct Group {
    pub members: &'static [&'static str],
    pub titles: &'static [&'static str],
}

pub static GROUPS: &[Group] = &[
    Group { members: &["US-CO", "US-WY", "US-ND", "US-SD", "US-KS"], titles: &["🟥 The Rectangle Round", "Four Corners, Zero Clues", "Squares Are Hard, Actually"] },
    Group { members: &["US-CO", "US-WY", "US-UT", "AU-NT"], titles: &["Boxes of the World", "The Great Rectangle Confusion"] },
    Group { members: &["US-ND", "US-SD", "US-MT", "US-NE"], titles: &["The Flat Lands", "Prairie Lineup"] },
    Group { members: &["US-KS", "US-NE", "US-OK", "US-IA"], titles: &["Panhandle Panic", "Corn Belt Confusion"] },
    Group { members: &["US-VT", "US-NH", "US-ME"], titles: &["🪞 Mirror, Mirror", "Upside Down New England", "Which Way Is Up?"] },
    Group { members: &["US-VT", "US-NH", "US-AL", "US-MS"], titles: &["🪞 Evil Twins", "Flip It and Reverse It"] },
    Group { members: &["US-AL", "US-MS", "US-GA", "AU-NT"], titles: &["The Tall Skinny Ones", "Deep South, Deep Confusion"] },
    Group { members: &["US-AL", "US-MS", "US-IN", "US-IL"], titles: &["Tall and Indistinct", "Vertical Rectangles With Ambitions"] },
    Group { members: &["US-IL", "US-IN", "US-OH"], titles: &["Midwest Mess", "Ohio, Indiana, Who Cares?"] },
    Group { members: &["US-AR", "US-MO", "US-IA", "US-PA"], titles: &["River-Bitten Rectangles", "Ragged Right Edges"] },
    Group { members: &["US-AZ", "US-NM", "US-NV", "US-UT", "US-CO"], titles: &["Desert Boxes", "Four Corners Showdown"] },
    Group { members: &["US-AZ", "US-NM", "AU-SA"], titles: &["Hot Rectangles", "Sunburnt Squares"] },
    Group { members: &["US-KY", "US-TN", "US-VA", "US-NC", "AU-VIC"], titles: &["🦘 Wedge Week", "Is It Kentucky Or Victoria?", "The Sideways Slices"] },
    Group { members: &["US-KY", "US-VA", "US-WV", "AU-VIC"], titles: &["Appalachian Arrowheads", "Pointy Bits"] },
    Group { members: &["US-WA", "US-OR", "US-MT", "US-ID"], titles: &["Pacific North-Confusing", "The Wet Corner"] },
    Group { members: &["US-CT", "US-RI", "US-DE", "AU-ACT"], titles: &["🔍 The Tiny Ones", "Microstates", "Zoomed Way In"] },
    Group { members: &["US-CT", "US-MA", "US-RI", "US-NJ"], titles: &["Northeast Corridor Chaos", "Commuter Belt"] },
    Group { members: &["AU-WA", "AU-SA", "AU-NT", "AU-QLD"], titles: &["🦘 Straight Lines Down Under", "Drawn With a Ruler, Mate", "Outback Geometry"] },
    Group { members: &["AU-NSW", "AU-VIC", "AU-QLD", "AU-SA"], titles: &["🦘 Eastern Seaboard Scramble", "Mate, Which One?"] },
    Group { members: &["AU-NSW", "US-SC", "US-NC", "AU-VIC"], titles: &["Coastal Wedges", "Pacific vs Atlantic"] },
    Group { members: &["AU-QLD", "US-ID", "US-NV"], titles: &["Pointy Tops", "The Finger Round"] },
    Group { members: &["AU-WA", "US-CA", "US-OR", "AU-SA"], titles: &["West Coast Best Coast", "Sunsets Over the Ocean"] },
    Group { members: &["AU-TAS", "US-HI", "US-RI"], titles: &["Island Life", "Not Attached To Anything"] },
    Group { members: &["US-MN", "US-WI", "US-MI"], titles: &["Great Lakes, Mediocre Guesses", "Mitten Season"] },
    Group { members: &["US-MD", "US-WV", "US-VA", "US-DE"], titles: &["The Chaos Shapes", "Who Drew These?"] },
    Group { members: &["US-NY", "US-PA", "US-NJ", "US-VT"], titles: &["Empire State of Confusion", "Mid-Atlantic Muddle"] },
    Group { members: &["US-SC", "US-NC", "US-GA", "US-AL"], titles: &["Southern Comfort", "Y'all Better Know This"] },
    Group { members: &["US-LA", "US-MS", "US-AR"], titles: &["Bayou Business", "Mississippi River Meets Mississippi State"] },
    Group { members: &["US-OK", "US-TX", "US-NE", "US-ID"], titles: &["Handle With Care", "Panhandles Assemble"] },
];

pub static EASY_TITLES: &[&str] = &[
    "Warm-up Lap", "A Freebie", "Even Your Nan Knows This", "Gimme", "Confidence Builder",
    "Don't Overthink It", "Free Points (Probably)", "Training Wheels", "Baby Steps",
];
pub static MEDIUM_TITLES: &[&str] = &[
    "Getting Warmer", "Now We're Talking", "Hmm.", "Bit Trickier", "Squint a Little",
    "Medium Rare", "Trust Your Gut", "Not As Easy As It Looks",
];
pub static HARD_TITLES: &[&str] = &[
    "Good Luck", "Absolutely Brutal", "This One's Personal", "Geography Teachers Only",
    "We're Sorry In Advance", "No Hints", "Pure Vibes", "Big Brain Time",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Modifier {
    None,
    /// shown rotated 180°, as nature intended
    DownUnder,
    /// shown rotated by a random angle
    Tilt,
    /// double points
    Double,
    /// final round: double points, double shame
    Final,
    /// a US answer with three Aussie decoys (a free one, cheekily)
    AussieInvasion,
    /// an Aussie answer with only Aussie options
    AllAussie,
}

pub struct Question {
    pub answer: &'static State,
    pub options: Vec<&'static State>,
    pub title: String,
    pub modifier: Modifier,
    /// degrees to rotate the shape on screen
    pub rotate: i32,
    pub tier: u8,
    pub multiplier: u32,
}

pub fn by_id(id: &str) -> &'static State {
    STATES.iter().find(|s| s.id == id).unwrap_or_else(|| panic!("unknown state {id}"))
}

fn lookalikes(id: &str) -> Vec<&'static State> {
    let mut seen = HashSet::new();
    GROUPS
        .iter()
        .filter(|g| g.members.contains(&id))
        .flat_map(|g| g.members.iter())
        .filter(|m| **m != id && seen.insert(**m))
        .map(|m| by_id(m))
        .collect()
}

struct Slot {
    tier: u8,
    au: bool,
    modifier: Modifier,
}

/// A whole game's worth of questions, drawn fresh every time.
pub fn game(rounds: usize) -> Vec<Question> {
    let mut rng = rand::rng();
    let n = rounds.max(3);

    // difficulty ramp: ~30% easy, ~35% medium, the rest hard
    let easy = (n * 3).div_ceil(10);
    let medium = (n * 35) / 100;
    let mut slots: Vec<Slot> = (0..n)
        .map(|i| Slot {
            tier: if i < easy { 1 } else if i < easy + medium { 2 } else { 3 },
            au: false,
            modifier: Modifier::None,
        })
        .collect();

    // Aussie answers: a quarter of the game, give or take, never round 1,
    // and at least one in the hard stretch when there's room.
    let aus = (n / 4 + rng.random_range(0..=1)).clamp(2, 8);
    let mut idx: Vec<usize> = (1..n).collect();
    idx.shuffle(&mut rng);
    for &i in idx.iter().take(aus) {
        slots[i].au = true;
    }
    if !slots.iter().any(|s| s.tier == 3 && s.au) {
        if let Some(i) = (easy + medium..n).collect::<Vec<_>>().choose(&mut rng) {
            slots[*i].au = true;
        }
    }

    // modifiers: none in the first two rounds or back to back, the last round is the final
    let mut last_mod = true;
    for (i, s) in slots.iter_mut().enumerate() {
        if i == n - 1 {
            s.modifier = Modifier::Final;
            continue;
        }
        if i < 2 || last_mod {
            last_mod = false;
            continue;
        }
        let roll: f64 = rng.random();
        s.modifier = match (s.au, s.tier) {
            (_, t) if roll < 0.12 && t >= 2 => Modifier::DownUnder,
            (_, t) if roll < 0.20 && t >= 2 => Modifier::Tilt,
            _ if roll < 0.28 => Modifier::Double,
            (false, t) if roll < 0.36 && t <= 2 => Modifier::AussieInvasion,
            (true, _) if roll < 0.42 => Modifier::AllAussie,
            _ => Modifier::None,
        };
        last_mod = s.modifier != Modifier::None;
    }

    let mut used = HashSet::new();
    slots.iter().map(|s| question(s, &mut used, &mut rng)).collect()
}

fn question(slot: &Slot, used: &mut HashSet<&'static str>, rng: &mut impl rand::Rng) -> Question {
    let unused = |f: &dyn Fn(&State) -> bool| -> Vec<&'static State> {
        STATES.iter().filter(|s| !used.contains(s.id) && f(s)).collect()
    };
    // answer: right country and tier if possible, then loosen
    let pool = [
        unused(&|s| s.au() == slot.au && s.tier == slot.tier),
        unused(&|s| s.au() == slot.au && s.tier.abs_diff(slot.tier) == 1),
        unused(&|s| s.au() == slot.au),
        unused(&|_| true),
    ];
    let answer = *pool.iter().find(|p| !p.is_empty()).unwrap().choose(rng).unwrap();
    used.insert(answer.id);

    let looks = lookalikes(answer.id);
    let mut options: Vec<&'static State> = vec![answer];
    // add random picks from `from` until there are `upto` options
    let take = |options: &mut Vec<&'static State>, from: &[&'static State], upto: usize, rng: &mut dyn rand::Rng| {
        let mut from: Vec<_> = from.iter().filter(|s| !options.iter().any(|o| o.id == s.id)).copied().collect();
        from.shuffle(rng);
        let k = upto.saturating_sub(options.len());
        options.extend(from.into_iter().take(k));
    };
    let others: Vec<&'static State> = STATES
        .iter()
        .filter(|s| s.id != answer.id && !looks.iter().any(|l| l.id == s.id))
        .collect();
    let aussies: Vec<&'static State> = STATES.iter().filter(|s| s.au()).collect();

    match slot.modifier {
        Modifier::AussieInvasion if !answer.au() => take(&mut options, &aussies, 4, rng),
        Modifier::AllAussie if answer.au() => {
            let au_looks: Vec<_> = looks.iter().filter(|s| s.au()).copied().collect();
            take(&mut options, &au_looks, 3, rng);
            take(&mut options, &aussies, 4, rng);
        }
        _ => {
            let n_looks: usize = match answer.tier.max(slot.tier) {
                1 => 0,
                2 => rng.random_range(1..=2),
                _ => 3,
            };
            // hard rounds: lead with one group so the trap is coherent
            if n_looks == 3 {
                let gs: Vec<_> = GROUPS.iter().filter(|g| g.members.contains(&answer.id)).collect();
                if let Some(g) = gs.choose(rng) {
                    let members: Vec<_> = g.members.iter().map(|m| by_id(m)).collect();
                    take(&mut options, &members, 4, rng);
                }
            }
            take(&mut options, &looks, 1 + n_looks, rng);
            // easy rounds keep the decoys obviously different
            let filler = if slot.tier == 1 { &others } else { &STATES.iter().collect::<Vec<_>>() };
            take(&mut options, filler, 4, rng);
        }
    }

    // The Aussie rule: there is always an Aussie option, and an Aussie answer
    // always has Aussie company (at least two more).
    let want_au = if answer.au() { 3 } else { 1 };
    while options.iter().filter(|s| s.au()).count() < want_au {
        let pos = options.iter().rposition(|s| !s.au() && s.id != answer.id).unwrap();
        let free = |from: &[&'static State]| -> Vec<&'static State> {
            from.iter().filter(|s| s.au() && !options.iter().any(|o| o.id == s.id)).copied().collect()
        };
        // an Aussie look-alike makes the best trap; otherwise any Aussie will do
        let (traps, any) = (free(&looks), free(&aussies));
        let from = if !traps.is_empty() && rng.random_bool(0.7) { traps } else { any };
        options[pos] = from.choose(rng).unwrap();
    }
    options.shuffle(rng);

    let title = title(slot, answer, &options, rng);
    let (rotate, multiplier) = match slot.modifier {
        Modifier::DownUnder => (180, 1),
        Modifier::Tilt => (rng.random_range(25..=65) * if rng.random_bool(0.5) { 1 } else { -1 }, 1),
        Modifier::Double | Modifier::Final => (0, 2),
        _ => (0, 1),
    };
    Question { answer, options, title, modifier: slot.modifier, rotate, tier: slot.tier, multiplier }
}

fn title(slot: &Slot, answer: &State, options: &[&State], rng: &mut impl rand::Rng) -> String {
    let pick = |xs: &[&str], rng: &mut dyn rand::Rng| xs.choose(rng).unwrap().to_string();
    match slot.modifier {
        Modifier::DownUnder => return pick(&["🙃 Down Under Round: shown the right way up, for once", "🙃 Australian Orientation", "🙃 Flipped. Deal with it."], rng),
        Modifier::Tilt => return pick(&["📐 Tilted. Someone bumped the table.", "📐 Slightly Wonky", "📐 Head-Tilt Required"], rng),
        Modifier::Double => return pick(&["✨ Double Points", "✨ Twice the Points, Twice the Shame", "✨ Bonus Round"], rng),
        Modifier::Final => return pick(&["🏁 Final Round: double points", "🏁 Last Chance (Double Points)", "🏁 Everything On This One"], rng),
        Modifier::AussieInvasion => return pick(&["🦘 Aussie Invasion", "🦘 Spot the Yank", "🦘 Mostly Kangaroos"], rng),
        Modifier::AllAussie => return pick(&["🦘 All-Aussie Round", "🦘 Strewth, Mate", "🦘 G'day, Pick One"], rng),
        Modifier::None => {}
    }
    // a hard round whose options mostly share a group gets that group's name
    if slot.tier == 3 {
        let best = GROUPS
            .iter()
            .filter(|g| g.members.contains(&answer.id))
            .max_by_key(|g| options.iter().filter(|o| g.members.contains(&o.id)).count());
        if let Some(g) = best {
            if options.iter().filter(|o| g.members.contains(&o.id)).count() >= 3 {
                return pick(g.titles, rng);
            }
        }
    }
    pick(match slot.tier { 1 => EASY_TITLES, 2 => MEDIUM_TITLES, _ => HARD_TITLES }, rng)
}

/// Lines for the reveal: who fell for what, who was fast, streaks, lead changes.
pub struct RoundFacts<'a> {
    pub state: &'a State,
    pub picks: Vec<(String, Option<&'a State>, f64)>, // name, pick, seconds taken
    pub everyone_right: bool,
    pub nobody_right: bool,
    pub streaks: Vec<(String, u32)>,
    pub broken: Vec<(String, u32)>,
    pub new_leader: Option<String>,
}

pub fn quips(f: &RoundFacts) -> Vec<String> {
    let mut rng = rand::rng();
    let st = f.state.name;
    let fill = |t: &str, name: &str, pick: &str| t.replace("{state}", st).replace("{name}", name).replace("{pick}", pick);
    let mut lines: Vec<(u8, String)> = vec![]; // (priority, line)

    if f.nobody_right && !f.picks.is_empty() {
        let t = [
            "Nobody. Not a single one of you.", "Zero correct. {state} is filing a complaint.",
            "A clean sweep of wrongness.", "{state} has never felt so invisible.",
            "Wow. {state} will remember this.", "Not one. Geography teachers everywhere just felt a chill.",
        ];
        lines.push((0, fill(t.choose(&mut rng).unwrap(), "", "")));
    }
    if f.everyone_right && f.picks.len() > 1 {
        let t = [
            "Everyone got it. Boring. Harder ones coming. 😈", "Unanimous. Suspicious, honestly.",
            "Too easy? Noted.", "The hive mind has spoken correctly, for once.", "Full marks all round. Enjoy it, it won't last.",
        ];
        lines.push((0, t.choose(&mut rng).unwrap().to_string()));
    }
    // one line per wrong answer, naming everyone who gave it
    let mut groups: Vec<(Option<&State>, Vec<&str>)> = vec![];
    for (name, pick, _) in &f.picks {
        if pick.is_some_and(|p| p.id == f.state.id) {
            continue;
        }
        match groups.iter_mut().find(|(p, _)| p.map(|p| p.id) == pick.map(|p| p.id)) {
            Some((_, names)) => names.push(name),
            None => groups.push((*pick, vec![name])),
        }
    }
    let mut used: HashSet<&str> = HashSet::new();
    for (pick, names) in groups {
        let many = names.len() > 1;
        let who = join_names(&names);
        let (prio, t): (u8, &[&str]) = match pick {
            None if many => (3, &["{name} didn't answer. Group nap?", "{name} all timed out. Solidarity.", "{name}: silence. Powerful stuff."]),
            None => (3, &["{name} didn't answer. Thinking hard, or sleeping?", "{name} timed out. Respect the indecision.", "{name} chose silence. Powerful.", "{name} went to make a cuppa, apparently."]),
            Some(p) if p.au() && !f.state.au() && many => (1, &["{name} all said {pick}. That's the other side of the planet.", "{name} went with {pick}. Mates.", "{name} all think {state} is in Australia. Bold.", "{name}: {pick}. Wrong hemisphere, the lot of you."]),
            Some(p) if p.au() && !f.state.au() => (1, &["{name} said {pick}. That's the other side of the planet.", "{name} went with {pick}. Mate.", "{name} thinks {state} is in Australia. Bold.", "{name} answered {pick}. Wrong hemisphere, champ.", "{name} picked {pick}. The kangaroos thank you for your support."]),
            Some(p) if !p.au() && f.state.au() && many => (1, &["{name} all said {pick}. Not even the right continent.", "{name} picked {pick}. The koalas are disappointed in all of you.", "{name} reckon {state} is American. Strewth."]),
            Some(p) if !p.au() && f.state.au() => (1, &["{name} said {pick}. Not even the right continent.", "{name} picked {pick}. The koalas are disappointed.", "{name} reckons {state} is American. Strewth.", "{name} went with {pick}. Someone get them an atlas."]),
            Some(p) if lookalikes(f.state.id).iter().any(|l| l.id == p.id) && many => (2, &["{name} all fell for {pick}. Classic trap.", "{name} picked {pick}. Honestly? Fair.", "{name} walked into the {pick} trap together. Touching."]),
            Some(_) if many => (2, &["{name} all said {pick}. Group project energy.", "{name} went {pick}. Confidently, collectively wrong."]),
            Some(p) if lookalikes(f.state.id).iter().any(|l| l.id == p.id) => (2, &["{name} fell for {pick}. Classic trap.", "{name} picked {pick}. Honestly? Fair.", "{name} said {pick}. We built that trap just for you.", "{pick}, {name}? So close, and yet."]),
            Some(_) => (2, &["{name} said {pick}. Interesting choice.", "{name} picked {pick}. Was that a guess? That was a guess.", "{name} went {pick}. Bold strategy."]),
        };
        // don't tell the same joke twice in one reveal
        let fresh: Vec<&&str> = t.iter().filter(|x| !used.contains(*x)).collect();
        let line = **fresh.choose(&mut rng).unwrap_or(&&t[0]);
        used.insert(line);
        lines.push((prio, fill(line, &who, pick.map_or("", |p| p.name))));
    }
    if let Some((name, _, secs)) = f
        .picks
        .iter()
        .filter(|(_, p, _)| p.map(|p| p.id) == Some(f.state.id))
        .min_by(|a, b| a.2.total_cmp(&b.2))
    {
        if f.picks.len() > 1 {
            let t = ["{name} got it in {s}s. Showoff.", "Fastest finger: {name} ({s}s).", "{name}: {s}s, like they'd been waiting all day.", "{name} clocked {s}s. Calm down."];
            lines.push((2, fill(t.choose(&mut rng).unwrap(), name, "").replace("{s}", &format!("{secs:.1}"))));
        }
    }
    for (name, n) in &f.streaks {
        let t = ["{name} is on a {n}-streak 🔥", "{name}: {n} in a row. Someone stop them.", "{n} straight for {name}. Insufferable."];
        lines.push((1, fill(t.choose(&mut rng).unwrap(), name, "").replace("{n}", &n.to_string())));
    }
    for (name, n) in &f.broken {
        let t = ["{name}'s {n}-streak dies here. RIP. 🪦", "And {name}'s {n}-streak is over. Tragic.", "{name} was on {n} in a row. Was."];
        lines.push((1, fill(t.choose(&mut rng).unwrap(), name, "").replace("{n}", &n.to_string())));
    }
    if let Some(name) = &f.new_leader {
        let t = ["{name} takes the lead! 👑", "New leader: {name}. Enjoy it while it lasts.", "{name} is top of the table. For now."];
        lines.push((0, fill(t.choose(&mut rng).unwrap(), name, "")));
    }
    lines.shuffle(&mut rng);
    lines.sort_by_key(|l| l.0);
    lines.into_iter().take(4).map(|l| l.1).collect()
}

fn join_names(names: &[&str]) -> String {
    match names {
        [] => String::new(),
        [one] => one.to_string(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

pub fn winner_line(name: &str) -> String {
    let t = [
        "{name} wins. They will not shut up about this.", "{name} is the State Champion. Of shapes.",
        "All hail {name}, ruler of outlines.", "{name} wins! Somewhere, a geography teacher sheds a tear of joy.",
        "{name} takes it. Frame this moment.",
    ];
    t.choose(&mut rand::rng()).unwrap().replace("{name}", name)
}

pub fn loser_line(name: &str) -> String {
    let t = [
        "{name} came last. A map on the fridge might help.", "{name}: last place, first in our hearts.",
        "{name} finishes dead last. Wyoming understands.", "{name} came last. Tasmania would have been a safe guess, just saying.",
    ];
    t.choose(&mut rand::rng()).unwrap().replace("{name}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_group_member_exists() {
        for g in GROUPS {
            for m in g.members {
                by_id(m);
            }
        }
        assert_eq!(STATES.len(), 58);
    }

    #[test]
    fn games_follow_the_rules() {
        for rounds in [5, 10, 15, 20, 30] {
            for _ in 0..300 {
                let qs = game(rounds);
                assert_eq!(qs.len(), rounds);
                let mut seen = HashSet::new();
                for q in &qs {
                    assert!(seen.insert(q.answer.id), "repeat answer");
                    assert_eq!(q.options.len(), 4);
                    let ids: HashSet<_> = q.options.iter().map(|o| o.id).collect();
                    assert_eq!(ids.len(), 4, "duplicate option");
                    assert!(ids.contains(q.answer.id));
                    let au = q.options.iter().filter(|o| o.au()).count();
                    assert!(au >= 1, "no aussie option");
                    if q.answer.au() {
                        assert!(au >= 3, "aussie answer without aussie company");
                    }
                }
                assert!(qs.iter().filter(|q| q.answer.au()).count() >= 2);
                assert_eq!(qs.last().unwrap().modifier, Modifier::Final);
            }
        }
    }

    #[test]
    fn shapes_cover_every_state() {
        let shapes: serde_json::Value = serde_json::from_str(include_str!("../web/shapes.json")).unwrap();
        for s in STATES {
            assert_eq!(shapes[s.id]["name"], s.name, "{}", s.id);
        }
    }
}

#[cfg(test)]
mod sample {
    /// `cargo test sample -- --ignored --nocapture` prints a random game
    #[test]
    #[ignore]
    fn sample() {
        for (i, q) in super::game(15).iter().enumerate() {
            let opts: Vec<_> = q.options.iter().map(|o| if o.id == q.answer.id { format!("[{}]", o.id) } else { o.id.to_string() }).collect();
            println!("{:>2} t{} {:<15?} {:<45} {}", i + 1, q.tier, q.modifier, q.title, opts.join(" "));
        }
    }
}
