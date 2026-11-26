//! Everything the game says. The question generator (quiz.rs) draws from these
//! pools; the more there is here, the less two games feel alike.
//!
//! Rule for round titles: they are shown BEFORE anyone answers, so they must
//! never name or hint at a state, a nickname, a region or a country. Talk
//! about the trap, the vibe or the players, not the place. A test enforces it.

use crate::quiz::{Group, State};

macro_rules! states {
    ($($id:literal, $name:literal, $tier:literal, [$($r:literal),+ $(,)?];)*) => {
        pub static STATES: &[State] = &[$(State { id: $id, name: $name, tier: $tier, roasts: &[$($r),+] }),*];
    };
}

// Roasts are shown after the answer is revealed, so they can say anything.
states! {
    "US-AL", "Alabama", 3, [
        "Not Mississippi. Turn it around and it's Mississippi. Don't turn it around.",
        "Has a tiny little foot dipped in the Gulf, like it's testing the water.",
        "Basically a rectangle that got bored at the bottom.",
        "The left half of a matching pair, and it knows it.",
        "Sweet home, vague outline.",
    ];
    "US-AK", "Alaska", 1, [
        "So big they shrink it on every map out of politeness.",
        "Comes with a free spray of islands. Some assembly required.",
        "Twice the size of Texas. Mention that to a Texan, it's fun.",
        "The chunk of America that's mostly bears and daylight arguments.",
        "Looks like it's sneezing islands towards Russia.",
    ];
    "US-AZ", "Arizona", 3, [
        "A rectangle that lost a fight with a river.",
        "The Grand Canyon's landlord.",
        "Doesn't do daylight saving. Doesn't do corners either, apparently.",
        "Like New Mexico, but with a wobbly left edge and more cacti.",
        "So hot the outline is a heat mirage.",
    ];
    "US-AR", "Arkansas", 3, [
        "Missouri's shorter sibling with a messier right side.",
        "Pronounced like Kansas, absolutely not like Kansas.",
        "A square that was left too close to the river.",
        "Its whole right side is just the Mississippi doing whatever it wants.",
        "Looks like a stamp someone tore off carelessly.",
    ];
    "US-CA", "California", 1, [
        "A state shaped like it's leaning in for a hug from Nevada.",
        "Basically a country with a coastline problem.",
        "One big earthquake from becoming an island, according to every disaster movie.",
        "The elbow of America, permanently bent.",
        "Long enough to have three different weathers at once.",
    ];
    "US-CO", "Colorado", 3, [
        "Technically has hundreds of sides, all pretending to be four.",
        "If you said Wyoming, you're in great company and still wrong.",
        "A rectangle with mountains inside. You can't see the mountains. Sorry.",
        "Surveyors in the 1800s really committed to the straight line.",
        "The most rectangle-y rectangle. It's not a perfect one though. Nerds will tell you.",
    ];
    "US-CT", "Connecticut", 3, [
        "A rectangle with a little tail, like it's waving goodbye to New York.",
        "The state equivalent of beige paint.",
        "Has a weird little notch at the top, like a bite from Massachusetts.",
        "Small, tidy, and slightly smug about it.",
        "Spelled with a silent C to keep you humble.",
    ];
    "US-DE", "Delaware", 3, [
        "A state so small the outline is mostly coastline anxiety.",
        "That curved top border is a literal circle. Look it up.",
        "First state to sign the Constitution. Last state anyone recognises.",
        "Shaped like a sock someone stretched.",
        "You could drive across it during one song.",
    ];
    "US-FL", "Florida", 1, [
        "The only state everyone can draw, and it's still weird.",
        "America's thumbs-up. Or something else, depending on your angle.",
        "A panhandle with a giant peninsula attached. Or the other way round.",
        "Shaped like a gun, a pan, or a very confident thumb.",
        "Hangs off the bottom like it's trying to escape.",
    ];
    "US-GA", "Georgia", 3, [
        "Alabama's twin from a different mother.",
        "Peaches, and a border that gave up halfway down.",
        "Looks like a rectangle that got squished by the coast.",
        "Not the country. The other one.",
        "Its right side does a little jig on the way down.",
    ];
    "US-HI", "Hawaii", 1, [
        "A state you need a boat to visit all of.",
        "The only state that's mostly volcano by volume.",
        "Eight main islands and a firm no on the whole 'connected' thing.",
        "A dotted line in the Pacific, still growing.",
        "The world's most expensive scatter plot.",
    ];
    "US-ID", "Idaho", 1, [
        "A pistol, a boot, or a chair, depending on how much you've had to drink.",
        "The panhandle exists purely to annoy Montana.",
        "Famous for potatoes, shaped like a potato's worst nightmare.",
        "Looks like it's trying to reach Canada with one finger.",
        "Its left edge is a river having a moment.",
    ];
    "US-IL", "Illinois", 3, [
        "Indiana with a bit more wobble on the left.",
        "Chicago and a lot of corn, shaped like a dropped ice cream.",
        "Its bottom point is aiming straight at Kentucky.",
        "Like a person looking left with a big nose.",
        "Rivers on both sides, straight line on top, chaos at the bottom.",
    ];
    "US-IN", "Indiana", 3, [
        "Illinois, but someone used a ruler.",
        "Looks like a phone with a cracked bottom.",
        "Tall, tidy, and a bit frayed at the hem.",
        "Its only curvy bit is the river at the bottom. Living on the edge.",
        "Shaped like a boot that forgot to have a heel.",
    ];
    "US-IA", "Iowa", 3, [
        "A rectangle with river damage on both sides.",
        "So flat the outline is the most interesting thing about it.",
        "Looks like a slightly angry face looking right.",
        "Rivers on the left and right, rulers on the top and bottom.",
        "The most corn per square mile of any shape on this quiz.",
    ];
    "US-KS", "Kansas", 3, [
        "A rectangle that took a bite out of its top-right corner.",
        "Literally flatter than a pancake. That's been measured.",
        "Dorothy left, and honestly who could blame her.",
        "The bite in the corner is a river. Everything else is a ruler.",
        "One wobbly corner away from being perfectly boring.",
    ];
    "US-KY", "Kentucky", 2, [
        "Looks like a duck going backwards, or Victoria. Mostly Victoria.",
        "Has a tiny chunk stranded in a river loop. Truly committed.",
        "Its top edge is a river having the time of its life.",
        "Shaped like a stretched-out cat with its tail on the left.",
        "Horses, bourbon, and a border nobody could draw from memory.",
    ];
    "US-LA", "Louisiana", 1, [
        "A boot that's been left out in the rain too long.",
        "Half state, half swamp, all bayou fringe.",
        "The bottom is less coastline and more suggestion.",
        "An L for Louisiana. Helpful, honestly.",
        "So soggy at the edges the map has to guess.",
    ];
    "US-ME", "Maine", 2, [
        "A mitten made entirely of coastline.",
        "Like New Hampshire, if New Hampshire ate well.",
        "More jagged coast than it has any right to.",
        "The top right corner of America, sticking its elbow out.",
        "Shaped like a head wearing a very pointy hat.",
    ];
    "US-MD", "Maryland", 2, [
        "Looks like someone tried to cut out a state and sneezed.",
        "A shape so chaotic it should come with a warning label.",
        "Mostly made of a bay with some land holding it together.",
        "Its western arm is so thin you could step over it. Almost.",
        "Shaped like a bird having a really bad day.",
    ];
    "US-MA", "Massachusetts", 2, [
        "Flexing its little Cape Cod arm at the Atlantic.",
        "A rectangle that grew a hook.",
        "The arm at the end is doing a bicep curl.",
        "Spell it without looking. Go on. We'll wait.",
        "Comes with two bonus islands for people with boats and money.",
    ];
    "US-MI", "Michigan", 1, [
        "The mitten. Plus a second, bonus bit nobody asked about.",
        "The only state that gives directions with its hand.",
        "Two peninsulas and a bridge, like a state built from spare parts.",
        "Surrounded by lakes so big they have waves and shipwrecks.",
        "The upper part is technically connected to Wisconsin. It's complicated.",
    ];
    "US-MN", "Minnesota", 2, [
        "A square with an arm reaching for Lake Superior.",
        "Has a weird little chimney up top, thanks to a dodgy old map.",
        "Ten thousand lakes, one very confusing top border.",
        "Shaped like a person with one arm waving at Canada.",
        "Cold enough that the outline might be frostbite.",
    ];
    "US-MS", "Mississippi", 3, [
        "Alabama in a mirror. Only the left edge gives it away.",
        "Four S's, four I's, two P's, one identity crisis with Alabama.",
        "Its left side is a river that never learned to walk straight.",
        "A tall shape with a little shelf at the bottom.",
        "Spelling it is harder than recognising it. Barely.",
    ];
    "US-MO", "Missouri", 3, [
        "Has a bootheel. Not to be confused with the other boot, or Arkansas.",
        "A square that melted on the right.",
        "Its right side is the river, its bottom right is a toe.",
        "Looks like a slightly lumpy pillow with a tag.",
        "Show me state. Show me a straight edge. Nope.",
    ];
    "US-MT", "Montana", 2, [
        "A rectangle someone sat on from the left.",
        "Big Sky Country. Big, vague shape.",
        "Its left side is a jagged mountain border with Idaho. Mostly mountain.",
        "Looks like a face in profile if you really, really want it to.",
        "Huge, empty, and suspicious of visitors.",
    ];
    "US-NE", "Nebraska", 3, [
        "Like Kansas, but with a little chimney on the left.",
        "A panhandle state, for people who enjoy saying panhandle.",
        "Its right side is a river doing a wobbly dance.",
        "Shaped like a frying pan for very square food.",
        "Kansas wearing a top hat.",
    ];
    "US-NV", "Nevada", 1, [
        "California's other half, the one with all the casinos.",
        "A cheese wedge with a gambling problem.",
        "Looks like a slice of pizza someone took a bite from.",
        "Its bottom point is aiming straight for Vegas. Obviously.",
        "Mostly desert, slightly secret air base.",
    ];
    "US-NH", "New Hampshire", 3, [
        "Vermont, upside down. Seriously, check.",
        "Has about 13 miles of coastline and won't stop talking about it.",
        "Live free or die, and also be confused with Vermont forever.",
        "Shaped like a wedge of cheese someone stood up.",
        "The pointy end is up. Remember that. It won't help.",
    ];
    "US-NJ", "New Jersey", 2, [
        "A seahorse with attitude.",
        "Shaped like it's about to start an argument.",
        "Its shoreline is one long boardwalk in denial.",
        "Looks like a guy leaning back with his hands in his pockets.",
        "Small, loud, and shaped like a squiggle.",
    ];
    "US-NM", "New Mexico", 3, [
        "A square with a little step at the bottom, like it's about to go downstairs.",
        "Not new, and not Mexico.",
        "Arizona's quieter twin with a pocket at the bottom.",
        "A square that got one tooth knocked out.",
        "Green chile, red chile, extremely beige shape.",
    ];
    "US-NY", "New York", 2, [
        "A state with a long island poking out the bottom like a tongue.",
        "Mostly not the city, despite what the city thinks.",
        "Looks like a sleeping dog with a very long tail.",
        "The upstate part would like you to know it exists.",
        "A triangle that got into a fight with two lakes.",
    ];
    "US-NC", "North Carolina", 2, [
        "A wedge with a ragged coastal fringe.",
        "The Outer Banks are just the state's eyelashes.",
        "Long and pointy, like it's trying to reach Tennessee.",
        "First in flight, and first to fray at the edges.",
        "Its coast looks like it's been chewed.",
    ];
    "US-ND", "North Dakota", 3, [
        "A rectangle. Next.",
        "South Dakota's upstairs neighbour.",
        "The right edge is a river. Everything else is a straight face.",
        "Least visited state, least memorable shape.",
        "If rectangles had a capital, it'd be here.",
    ];
    "US-OH", "Ohio", 2, [
        "A heart, if you squint and really love Ohio.",
        "Round on the ends and hi in the middle.",
        "Its bottom edge is a river doing a lazy wave.",
        "Shaped like a slightly melted shield.",
        "The birthplace of aviation, and a very recognisable blob.",
    ];
    "US-OK", "Oklahoma", 1, [
        "A frying pan, handle and all.",
        "Texas's hat.",
        "The handle is called No Man's Land. Real name. Look it up.",
        "Its bottom border is a river doing parkour.",
        "Where the wind comes sweeping down the plain. And off the handle.",
    ];
    "US-OR", "Oregon", 2, [
        "Washington's quieter cousin, slightly more square.",
        "Its top border is a river and that's its personality.",
        "A rectangle that got bumped in the top-right.",
        "Rain on the left, desert on the right, a very square bottom.",
        "Where wagons ended up, then didn't know what to do.",
    ];
    "US-PA", "Pennsylvania", 2, [
        "A rectangle with a frilly right side and a tiny notch on top.",
        "Has a chimney poking up to Lake Erie.",
        "Spelled with one N and two N's, depending how tired you are.",
        "Looks like an envelope someone tore open on the right.",
        "Big, boxy, and a bit frayed on the Delaware side.",
    ];
    "US-RI", "Rhode Island", 3, [
        "Not an island. Barely a state. Love it.",
        "Mostly bay. Fit it in your pocket.",
        "Its full name used to be longer than its coastline.",
        "Shattered into islands at the bottom like a dropped plate.",
        "You could miss it if you blink on the highway.",
    ];
    "US-SC", "South Carolina", 2, [
        "An upside-down triangle with a frilly coast.",
        "North Carolina's snack-sized sibling.",
        "Looks like a slice of pie someone already started on.",
        "Shaped like a heart, if hearts were very angular.",
        "Palmetto trees and a very pointy top.",
    ];
    "US-SD", "South Dakota", 3, [
        "North Dakota, but the bottom right melted.",
        "A rectangle with a dented corner.",
        "Home of four giant faces carved into a mountain. None of them help.",
        "Its right edge does a little wiggle before giving up.",
        "Like North Dakota but with more presidents.",
    ];
    "US-TN", "Tennessee", 1, [
        "A parallelogram that tripped on the way in.",
        "Long, thin, and full of country music.",
        "Borders eight states, the most of anyone. Very social.",
        "Shaped like a guitar case left on its side.",
        "The left end is a river, the right end is a mountain mess.",
    ];
    "US-TX", "Texas", 1, [
        "Everything's bigger in Texas, including the ego.",
        "So recognisable it's on the belt buckles.",
        "Its own country for nine years and it will not let that go.",
        "The panhandle is the hat. The Rio Grande is the beard.",
        "Shaped like a bellowing cowboy, if you've had a few.",
    ];
    "US-UT", "Utah", 1, [
        "A square with a bite taken out by Wyoming.",
        "Colorado with a corner missing.",
        "Looks like someone tried to make a rectangle and ran out.",
        "Has a big salty lake and a very clean outline.",
        "The missing corner is the most famous missing thing in geography.",
    ];
    "US-VT", "Vermont", 3, [
        "New Hampshire flipped. They know. They're not happy.",
        "Shaped like a slice of pie someone fumbled.",
        "Maple syrup, cows, and a wedge that points down.",
        "Its right side is a river, so the shape is basically New Hampshire's fault.",
        "The only state capital without a McDonald's, and the shape is just as low-key.",
    ];
    "US-VA", "Virginia", 2, [
        "A triangle with a tail flicking into the bay.",
        "Looks a bit like Kentucky got lost in the Atlantic.",
        "Its top left border is a mountain ridge that couldn't decide.",
        "A wedge with a chunk of peninsula tucked across the water.",
        "Mother of presidents, father of confusing borders.",
    ];
    "US-WA", "Washington", 2, [
        "A square with a jigsaw nibble out of the top-left.",
        "Wet, green, and full of coffee.",
        "The Puget Sound bit looks like a bite out of a biscuit.",
        "Its bottom border is a river, its top is Canada's problem.",
        "Not the city. Nowhere near the city.",
    ];
    "US-WV", "West Virginia", 2, [
        "Shaped like a squashed octopus waving both arms.",
        "Has two panhandles. Greedy.",
        "Looks like a splat. A mountainous splat.",
        "Broke off from Virginia and kept all the wiggly bits.",
        "Country roads, and a border nobody could draw.",
    ];
    "US-WI", "Wisconsin", 2, [
        "A mitten for the other hand, held next to Michigan's.",
        "Has a thumb poking into Lake Michigan.",
        "Cheese, beer, and a peninsula giving a thumbs-up.",
        "Shaped like a slightly lumpy oven glove.",
        "Its left side is a river, its right side is a lake, its middle is dairy.",
    ];
    "US-WY", "Wyoming", 3, [
        "A perfect rectangle. Or Colorado. Nobody knows.",
        "Does Wyoming exist? The evidence is thin.",
        "Fewest people of any state, fewest features of any shape.",
        "Yellowstone is in the top-left corner. You can't see it, but it's there.",
        "So rectangular it's been accused of being a placeholder.",
    ];
    "AU-NSW", "New South Wales", 2, [
        "Australia's most populated slice, shaped like a bitten sandwich.",
        "Comes with the ACT sitting in it like a stone in a shoe.",
        "Straight lines on three sides, a long beach on the fourth.",
        "Looks like a ticket stub torn off the continent.",
        "Not new, not Wales, definitely south.",
    ];
    "AU-VIC", "Victoria", 2, [
        "Also known as 'basically Kentucky'.",
        "Small, wedge-shaped, and will tell you about the coffee.",
        "Its top border is the Murray River, wobbling the whole way.",
        "Looks like a wedge of cheese with a bite out of the bottom.",
        "Four seasons in one day, one very pointy east end.",
    ];
    "AU-QLD", "Queensland", 2, [
        "Shaped like a giant pointing finger. That's Cape York.",
        "Big, hot, and full of things that want to hurt you.",
        "Has a little step in its bottom left, like it tripped on South Australia.",
        "Looks like it's pointing at New Guinea. Rude.",
        "Beautiful one day, perfect the next, recognisable never.",
    ];
    "AU-SA", "South Australia", 3, [
        "A big box with two bites out of the bottom.",
        "The Bight is what happens when a state gets hangry.",
        "Mostly straight lines and a coastline that looks chewed.",
        "Two gulfs at the bottom like a pair of nostrils.",
        "Wine, desert, and a very confident rectangle.",
    ];
    "AU-WA", "Western Australia", 2, [
        "One third of a continent and still drawn as a big lump.",
        "Big enough to swallow Texas several times. Ask Texas.",
        "One straight edge on the right, a million beaches on the left.",
        "Shaped like a giant tooth.",
        "So big it has its own time zone and opinions.",
    ];
    "AU-TAS", "Tasmania", 1, [
        "The island everyone forgets until they need a map.",
        "Australia's little shield, hanging off the bottom.",
        "Shaped like a heart, if you're feeling generous.",
        "Regularly left off maps. It has noticed. It is hurt.",
        "Devils, apples, and a coastline that looks bitten.",
    ];
    "AU-NT", "Northern Territory", 3, [
        "A rectangle with a beard of mangroves.",
        "Not a state. Shaped like a state. Pretending.",
        "Ruler-straight on three sides, utter chaos on top.",
        "Crocodiles live along that jagged top edge. Avoid.",
        "Uluru is in there somewhere. It doesn't help.",
    ];
    "AU-ACT", "Australian Capital Territory", 3, [
        "A blob around Canberra. That's it. That's the territory.",
        "Small, oddly shaped, mostly public servants.",
        "Carved out of New South Wales like a cookie cutter gone wrong.",
        "Shaped like a sock puppet with ambitions.",
        "The world's most bureaucratic blob.",
    ];
}

/// Look-alike groups. Overlapping on purpose: a state's decoys come from any
/// group it's in, so the same answer gets different traps game to game.
/// Titles are about the trap, never the place (see the rule at the top).
pub static GROUPS: &[Group] = &[
    Group { members: &["US-CO", "US-WY", "US-ND", "US-SD", "US-KS"], titles: &["🟥 The Rectangle Round", "Squares Are Hard, Actually", "Four Straight Lines and a Prayer", "Ruler Enthusiasts Only"] },
    Group { members: &["US-CO", "US-WY", "US-UT", "AU-NT"], titles: &["Boxes of the World", "The Rectangle Identity Crisis", "Someone Lost the Protractor", "Corners Are Overrated"] },
    Group { members: &["US-ND", "US-SD", "US-MT", "US-NE"], titles: &["Stacked Like Bricks", "Wide Load", "Very Horizontal Energy", "Flat Pack, No Instructions"] },
    Group { members: &["US-KS", "US-NE", "US-OK", "US-IA"], titles: &["Handle With Care", "Pans and Pancakes", "Nearly Rectangles", "So Close To Being Boxes"] },
    Group { members: &["US-VT", "US-NH", "US-ME"], titles: &["🪞 Mirror, Mirror", "Which Way Is Up?", "One Of These Is Upside Down", "Twins, But One Is Evil"] },
    Group { members: &["US-VT", "US-NH", "US-AL", "US-MS"], titles: &["🪞 Evil Twins", "Flip It and Reverse It", "Separated At Birth", "Mirror Images"] },
    Group { members: &["US-AL", "US-MS", "US-GA", "AU-NT"], titles: &["The Tall Skinny Ones", "Standing Up Straight", "Portrait Mode", "Long Boys"] },
    Group { members: &["US-AL", "US-MS", "US-IN", "US-IL"], titles: &["Tall and Indistinct", "Vertical Rectangles With Ambitions", "Who Wore It Better?", "The Lineup"] },
    Group { members: &["US-IL", "US-IN", "US-OH"], titles: &["Siblings Who Fight", "Spot the Difference", "They All Look the Same To Me", "Neighbours, Lookalikes, Rivals"] },
    Group { members: &["US-AR", "US-MO", "US-IA", "US-PA"], titles: &["Ragged Right Edges", "Frayed At the Edges", "Bitten On One Side", "Squares That Got Wet"] },
    Group { members: &["US-AZ", "US-NM", "US-NV", "US-UT", "US-CO"], titles: &["Boxes and Wedges", "Mostly Straight, Slightly Rude", "The Not-Quite-Squares", "Geometry Class Flashback"] },
    Group { members: &["US-AZ", "US-NM", "AU-SA"], titles: &["Sunburnt Squares", "Chunky Boys", "The Big Blocks", "Square-ish, Allegedly"] },
    Group { members: &["US-KY", "US-TN", "US-VA", "US-NC", "AU-VIC"], titles: &["Wedge Week", "The Sideways Slices", "Lying Down On the Job", "Doorstops"] },
    Group { members: &["US-KY", "US-VA", "US-WV", "AU-VIC"], titles: &["Pointy Bits", "Arrowheads", "Somebody Drew These Angry", "Ouch, Sharp"] },
    Group { members: &["US-WA", "US-OR", "US-MT", "US-ID"], titles: &["Top Shelf", "Upper Deck Confusion", "Jagged Neighbours", "The Bitten Squares"] },
    Group { members: &["US-CT", "US-RI", "US-DE", "AU-ACT"], titles: &["🔍 Zoomed Way In", "Microscope Required", "Small But Mighty", "Pocket-Sized Problems"] },
    Group { members: &["US-CT", "US-MA", "US-RI", "US-NJ"], titles: &["Commuter Chaos", "Squiggles and Hooks", "Small and Shouty", "Elbows Out"] },
    Group { members: &["AU-WA", "AU-SA", "AU-NT", "AU-QLD"], titles: &["Drawn With a Ruler", "Straight Lines, Wild Coasts", "Big, Blocky and Baking", "Someone Had a Set Square"] },
    Group { members: &["AU-NSW", "AU-VIC", "AU-QLD", "AU-SA"], titles: &["Which One, Though?", "Spot the Difference", "Family Resemblance", "Cousins"] },
    Group { members: &["AU-NSW", "US-SC", "US-NC", "AU-VIC"], titles: &["Wedges With Beaches", "Pointy and Sandy", "Shoreline Shenanigans", "Triangles With Tans"] },
    Group { members: &["AU-QLD", "US-ID", "US-NV"], titles: &["Pointy Tops", "The Finger Round", "Reaching For Something", "Stick 'Em Up"] },
    Group { members: &["AU-WA", "US-CA", "US-OR", "AU-SA"], titles: &["Sunsets Over the Ocean", "Long Beaches", "Chunky With a View", "One Straight Side"] },
    Group { members: &["AU-TAS", "US-HI", "US-RI"], titles: &["Not Attached To Anything", "Water, Water Everywhere", "Bring a Boat", "Floating Free"] },
    Group { members: &["US-MN", "US-WI", "US-MI"], titles: &["Hands Up", "Gloves Off", "Thumbs and Fingers", "Lumpy Neighbours"] },
    Group { members: &["US-MD", "US-WV", "US-VA", "US-DE"], titles: &["The Chaos Shapes", "Who Drew These?", "Absolute Scribbles", "Cartographer Had Hiccups"] },
    Group { members: &["US-NY", "US-PA", "US-NJ", "US-VT"], titles: &["Busy Borders", "Neighbourhood Watch", "Rough Around the Edges", "Squeezed Together"] },
    Group { members: &["US-SC", "US-NC", "US-GA", "US-AL"], titles: &["Close Relatives", "The Usual Suspects", "Family Reunion", "You Again?"] },
    Group { members: &["US-LA", "US-MS", "US-AR"], titles: &["Wet Edges", "Soggy Bottoms", "The Leaky Ones", "Frayed and Floppy"] },
    Group { members: &["US-OK", "US-TX", "US-NE", "US-ID"], titles: &["Panhandles Assemble", "Shapes With Handles", "Grab It By the Handle", "Pots and Pans"] },
];

pub static EASY_TITLES: &[&str] = &[
    "Warm-up Lap", "A Freebie", "Everyone Knows This One", "Gimme", "Confidence Builder",
    "Don't Overthink It", "Free Points (Probably)", "Training Wheels", "Baby Steps",
    "Stretch Those Fingers", "Easy Peasy", "One For the Kids", "Get Your Eye In",
    "Just Vibes", "Ease On In", "Practice Swing", "Low Stakes", "Look, No Hands",
    "Nice and Gentle", "First Date Energy", "Kindergarten Rules", "Spot the Obvious",
    "If You Miss This, Leave", "We Believe In You", "Gentle Start",
];
pub static MEDIUM_TITLES: &[&str] = &[
    "Getting Warmer", "Now We're Talking", "Hmm.", "Bit Trickier", "Squint a Little",
    "Medium Rare", "Trust Your Gut", "Not As Easy As It Looks", "Think About It",
    "Plot Twist Incoming", "Concentrate", "Brain Cells, Assemble", "Steady On",
    "Getting Spicy", "Pay Attention", "Mild Panic", "Lean In", "Ooh, Tricky",
    "Look Closer", "Wait, What?", "Second Thoughts", "Half Sure", "No Pressure",
    "Middle of the Road", "Phone a Friend",
];
pub static HARD_TITLES: &[&str] = &[
    "Good Luck", "Absolutely Brutal", "This One's Personal", "Geography Teachers Only",
    "We're Sorry In Advance", "No Hints", "Pure Vibes", "Big Brain Time",
    "Abandon Hope", "Basically a Coin Flip", "Sweaty Palms", "Nightmare Fuel",
    "Even We Had To Check", "Brace Yourselves", "Expert Mode", "Don't Panic",
    "Pain", "This Is Fine 🔥", "Prove It", "Educated Guess Time", "Deep Breaths",
    "The Gloves Are Off", "Choose Wisely", "Here Be Dragons", "No Takebacks",
];

// Modifier titles. Only the two Aussie modifiers may mention Australia: the
// options already show it, so it gives nothing away.
pub static DOWN_UNDER_TITLES: &[&str] = &[
    "🙃 Upside Down Round", "🙃 Flipped. Deal with it.", "🙃 Head Stand Required",
    "🙃 Someone Dropped the Map", "🙃 Wrong Way Up", "🙃 Turn Your Phone Around (Don't)",
];
pub static TILT_TITLES: &[&str] = &[
    "📐 Tilted. Someone bumped the table.", "📐 Slightly Wonky", "📐 Head-Tilt Required",
    "📐 Crooked Picture Frame", "📐 Hung By an Amateur", "📐 A Bit Squiffy",
];
pub static SPIN_TITLES: &[&str] = &[
    "🌀 Spin Cycle", "🌀 Round and Round", "🌀 Washing Machine Round",
    "🌀 Dizzy Yet?", "🌀 You Spin Me Right Round",
];
pub static FOG_TITLES: &[&str] = &[
    "🌫️ Foggy Round: it clears up. Wait, or guess early?", "🌫️ Pea Souper (it gets clearer)",
    "🌫️ Forgot My Glasses (it sharpens)", "🌫️ Patience Pays, Speed Pays More",
    "🌫️ Misty Morning (wait for it)",
];
pub static OUTLINE_TITLES: &[&str] = &[
    "✏️ Outline Only", "✏️ Ran Out of Ink", "✏️ Hollow Victory",
    "✏️ Just the Edges", "✏️ Colouring Book Round",
];
pub static WOBBLE_TITLES: &[&str] = &[
    "🍮 Wobbly Round", "🍮 Jelly Mode", "🍮 Earthquake Drill",
    "🍮 Hold Still, Would You", "🍮 Too Much Coffee",
];
pub static DOUBLE_TITLES: &[&str] = &[
    "✨ Double Points", "✨ Twice the Points, Twice the Shame", "✨ Bonus Round",
    "✨ Jackpot", "✨ Make It Count", "✨ Points Go Brrr",
];
pub static FINAL_TITLES: &[&str] = &[
    "🏁 Final Round: double points", "🏁 Last Chance (Double Points)", "🏁 Everything On This One",
    "🏁 The Decider (Double Points)", "🏁 Last One. Make It Hurt.", "🏁 Winner Takes Most",
];
pub static INVASION_TITLES: &[&str] = &[
    "🦘 Aussie Invasion", "🦘 Mostly Kangaroos", "🦘 Three Roos and a Stranger",
    "🦘 Odd One Out?", "🦘 Spot the Tourist (Or Don't)",
];
pub static ALL_AUSSIE_TITLES: &[&str] = &[
    "🦘 All-Aussie Round", "🦘 Strewth, Mate", "🦘 G'day, Pick One",
    "🦘 Fair Dinkum", "🦘 Crikey", "🦘 No Yanks Allowed",
];

// ---- reveal quips. {name} {pick} {state} {s} {n} get filled in. ----

pub static NOBODY: &[&str] = &[
    "Nobody. Not a single one of you.", "Zero correct. {state} is filing a complaint.",
    "A clean sweep of wrongness.", "{state} has never felt so invisible.",
    "Wow. {state} will remember this.", "Not one. Geography teachers everywhere just felt a chill.",
    "Collectively, magnificently wrong.", "{state} is going to need a minute.",
    "The trap worked on everyone. We're thrilled.", "0 for everyone. Impressive in its own way.",
];
pub static EVERYONE: &[&str] = &[
    "Everyone got it. Boring. Harder ones coming. 😈", "Unanimous. Suspicious, honestly.",
    "Too easy? Noted.", "The hive mind has spoken correctly, for once.",
    "Full marks all round. Enjoy it, it won't last.", "Everyone's right. Somebody's cheating.",
    "A perfect round. We'll fix that.", "All correct. Show-offs, the lot of you.",
];
pub static SOLO_RIGHT: &[&str] = &[
    "Only {name} got it. Everyone else, look at {name}. Learn.", "{name}, alone, against the world. And right.",
    "One correct answer, and it was {name}. Smug face activated.", "{name} carried the whole lobby on that one.",
    "Just {name}. The rest of you should be ashamed.",
];
pub static NO_ANSWER: &[&str] = &[
    "{name} didn't answer. Thinking hard, or sleeping?", "{name} timed out. Respect the indecision.",
    "{name} chose silence. Powerful.", "{name} went to make a cuppa, apparently.",
    "{name} left us on read.", "{name} is still thinking. Probably.",
];
pub static NO_ANSWER_MANY: &[&str] = &[
    "{name} didn't answer. Group nap?", "{name} all timed out. Solidarity.",
    "{name}: silence. Powerful stuff.", "{name} all went quiet. Suspicious.",
];
pub static AU_FOR_US: &[&str] = &[
    "{name} said {pick}. That's the other side of the planet.", "{name} went with {pick}. Mate.",
    "{name} thinks {state} is in Australia. Bold.", "{name} answered {pick}. Wrong hemisphere, champ.",
    "{name} picked {pick}. The kangaroos thank you for your support.", "{name} said {pick}. A mere 13,000 km off.",
    "{name} chose {pick}. Passport not required, apparently.", "{name} went {pick}. The emus are confused too.",
];
pub static AU_FOR_US_MANY: &[&str] = &[
    "{name} all said {pick}. That's the other side of the planet.", "{name} went with {pick}. Mates.",
    "{name} all think {state} is in Australia. Bold.", "{name}: {pick}. Wrong hemisphere, the lot of you.",
    "{name} all flew to {pick}. Long flight, wrong place.",
];
pub static US_FOR_AU: &[&str] = &[
    "{name} said {pick}. Not even the right continent.", "{name} picked {pick}. The koalas are disappointed.",
    "{name} reckons {state} is American. Strewth.", "{name} went with {pick}. Someone get them an atlas.",
    "{name} said {pick}. Close. If you're a satellite.", "{name} picked {pick}. The wombats are laughing.",
];
pub static US_FOR_AU_MANY: &[&str] = &[
    "{name} all said {pick}. Not even the right continent.", "{name} picked {pick}. The koalas are disappointed in all of you.",
    "{name} reckon {state} is American. Strewth.", "{name}: {pick}. The magpies are swooping as we speak.",
];
pub static TRAP: &[&str] = &[
    "{name} fell for {pick}. Classic trap.", "{name} picked {pick}. Honestly? Fair.",
    "{name} said {pick}. We built that trap just for you.", "{pick}, {name}? So close, and yet.",
    "{name} went {pick}. Understandable. Still wrong.", "{name} picked {pick}. The evil twin strikes again.",
    "{name} said {pick}. Easy mistake. Still a mistake.",
];
pub static TRAP_MANY: &[&str] = &[
    "{name} all fell for {pick}. Classic trap.", "{name} picked {pick}. Honestly? Fair.",
    "{name} walked into the {pick} trap together. Touching.", "{name} all went {pick}. The trap is working beautifully.",
];
pub static WILD: &[&str] = &[
    "{name} said {pick}. Interesting choice.", "{name} picked {pick}. Was that a guess? That was a guess.",
    "{name} went {pick}. Bold strategy.", "{name} chose {pick}. We have questions.",
    "{name} said {pick}. Not even close, and we love that for you.", "{name} picked {pick}. Did you close your eyes?",
];
pub static WILD_MANY: &[&str] = &[
    "{name} all said {pick}. Group project energy.", "{name} went {pick}. Confidently, collectively wrong.",
    "{name} all picked {pick}. Copying each other's homework?",
];
pub static FASTEST: &[&str] = &[
    "{name} got it in {s}s. Showoff.", "Fastest finger: {name} ({s}s).",
    "{name}: {s}s, like they'd been waiting all day.", "{name} clocked {s}s. Calm down.",
    "{s}s from {name}. Did you even look?", "{name} answered in {s}s. Speed demon.",
];
pub static BUZZER: &[&str] = &[
    "{name} answered with {s}s to spare. Living dangerously.", "{name} got there just before the buzzer.",
    "{name} took their sweet time. {s}s left.",
];
pub static STREAK: &[&str] = &[
    "{name} is on a {n}-streak 🔥", "{name}: {n} in a row. Someone stop them.",
    "{n} straight for {name}. Insufferable.", "{name} is cooking. {n} in a row.",
    "{name}: {n} correct and counting.",
];
pub static BROKEN: &[&str] = &[
    "{name}'s {n}-streak dies here. RIP. 🪦", "And {name}'s {n}-streak is over. Tragic.",
    "{name} was on {n} in a row. Was.", "{name}'s streak of {n}: gone, reduced to atoms.",
];
pub static LEADER: &[&str] = &[
    "{name} takes the lead! 👑", "New leader: {name}. Enjoy it while it lasts.",
    "{name} is top of the table. For now.", "{name} storms to the front.",
    "All eyes on {name}, the new leader.",
];
pub static WINNER: &[&str] = &[
    "{name} wins. They will not shut up about this.", "{name} is the State Champion. Of shapes.",
    "All hail {name}, ruler of outlines.", "{name} wins! Somewhere, a geography teacher sheds a tear of joy.",
    "{name} takes it. Frame this moment.", "{name} wins. Screenshot it before they do.",
    "Bow down to {name}, master of borders.", "{name} wins and immediately becomes unbearable.",
];
pub static LOSER: &[&str] = &[
    "{name} came last. A map on the fridge might help.", "{name}: last place, first in our hearts.",
    "{name} finishes dead last. Wyoming understands.", "{name} came last. Tasmania would have been a safe guess, just saying.",
    "{name} came last. Bold of you to show up.", "{name} brought up the rear. Someone has to.",
    "{name} finished last. A glorious failure.",
];
