use rand::seq::SliceRandom;

// A copy of the list of adjectives for the Docker container name generator.
const ADJECTIVES: [&str; 108] = [
    "admiring",
    "adoring",
    "affectionate",
    "agitated",
    "amazing",
    "angry",
    "awesome",
    "beautiful",
    "blissful",
    "bold",
    "boring",
    "brave",
    "busy",
    "charming",
    "clever",
    "compassionate",
    "competent",
    "condescending",
    "confident",
    "cool",
    "cranky",
    "crazy",
    "dazzling",
    "determined",
    "distracted",
    "dreamy",
    "eager",
    "ecstatic",
    "elastic",
    "elated",
    "elegant",
    "eloquent",
    "epic",
    "exciting",
    "fervent",
    "festive",
    "flamboyant",
    "focused",
    "friendly",
    "frosty",
    "funny",
    "gallant",
    "gifted",
    "goofy",
    "gracious",
    "great",
    "happy",
    "hardcore",
    "heuristic",
    "hopeful",
    "hungry",
    "infallible",
    "inspiring",
    "intelligent",
    "interesting",
    "jolly",
    "jovial",
    "keen",
    "kind",
    "laughing",
    "loving",
    "lucid",
    "magical",
    "modest",
    "musing",
    "mystifying",
    "naughty",
    "nervous",
    "nice",
    "nifty",
    "nostalgic",
    "objective",
    "optimistic",
    "peaceful",
    "pedantic",
    "pensive",
    "practical",
    "priceless",
    "quirky",
    "quizzical",
    "recursing",
    "relaxed",
    "reverent",
    "romantic",
    "sad",
    "serene",
    "sharp",
    "silly",
    "sleepy",
    "stoic",
    "strange",
    "stupefied",
    "suspicious",
    "sweet",
    "tender",
    "thirsty",
    "trusting",
    "unruffled",
    "upbeat",
    "vibrant",
    "vigilant",
    "vigorous",
    "wizardly",
    "wonderful",
    "xenodochial",
    "youthful",
    "zealous",
    "zen",
];

// Also inspired by the Docker container name generator.
// Instead of list of names from notable scientists and hackers, list of Pokémon species introduced in Generation I (1996) is used.
const POKEMONS: [&str; 149] = [
    // Fushigidane (フシギダネ)
    // There is a plant seed on its back right from the day this Pokémon is born. The seed slowly grows larger.
    "bulbasaur",

    // Fushigisō (フシギソウ)
    // When the bulb on its back grows large, it appears to lose the ability to stand on its hind legs.
    "ivysaur",

    // Fushigibana (フシギバナ)
    // Its plant blooms when it is absorbing solar energy. It stays on the move to seek sunlight.
    "venusaur",

    // Hitokage (ヒトカゲ)
    // It has a preference for hot things. When it rains, steam is said to spout from the tip of its tail.
    "charmander",

    // Rizādo (リザード)
    // It has a barbaric nature. In battle, it whips its fiery tail around and slashes away with sharp claws.
    "charmeleon",

    // Rizādon (リザードン)
    // It spits fire that is hot enough to melt boulders. It may cause forest fires by blowing flames.
    "charizard",

    // Zenigame (ゼニガメ)
    // When it retracts its long neck into its shell, it squirts out water with vigorous force.
    "squirtle",

    // Kamēru (カメール)
    // It is recognized as a symbol of longevity. If its shell has algae on it, that Wartortle is very old.
    "wartortle",

    // Kamekkusu (カメックス)
    // It crushes its foe under its heavy body to cause fainting. In a pinch, it will withdraw inside its shell.
    "blastoise",

    // Kyatapī (キャタピー)
    // For protection, it releases a horrible stench from the antenna on its head to drive away enemies.
    "caterpie ",

    // Toranseru (トランセル)
    // It is waiting for the moment to evolve. At this stage, it can only harden, so it remains motionless to avoid attack.
    "metapod",

    // Batafurī (バタフリー)
    // In battle, it flaps its wings at great speed to release highly toxic dust into the air.
    "butterfree",

    // Bīdoru (ビードル)
    // Beware of the sharp stinger on its head. It hides in grass and bushes where it eats leaves.
    "weedle",

    // Kokūn (コクーン)
    // Able to move only slightly. When endangered, it may stick out its stinger and poison its enemy.
    "kakuna",

    // Supiā (スピアー)
    // It has three poisonous stingers on its forelegs and its tail. They are used to jab its enemy repeatedly.
    "beedrill",

    // Poppo (ポッポ)
    // Very docile. If attacked, it will often kick up sand to protect itself rather than fight back.
    "pidgey",

    // Pijon (ピジョン)
    // This Pokémon is full of vitality. It constantly flies around its large territory in search of prey.
    "pidgeotto",

    // Pijotto (ピジョット)
    // This Pokémon flies at Mach 2 speed, seeking prey. Its large talons are feared as wicked weapons.
    "pidgeot",

    // Koratta (コラッタ)
    // Will chew on anything with its fangs. If you see one, you can be certain that 40 more live in the area.
    "rattata",

    // Ratta (ラッタ)
    // Its hind feet are webbed. They act as flippers, so it can swim in rivers and hunt for prey.
    "raticate",

    // Onisuzume (オニスズメ)
    // Inept at flying high. However, it can fly around very fast to protect its territory.
    "spearow",

    // Onidoriru (オニドリル)
    // A Pokémon that dates back many years. If it senses danger, it flies high and away, instantly.
    "fearow",

    // Ābo (アーボ)
    // The older it gets, the longer it grows. At night, it wraps its long body around tree branches to rest.
    "ekans",

    // Ābokku (アーボック)
    // The frightening patterns on its belly have been studied. Six variations have been confirmed.
    "arbok",

    // Pikachū (ピカチュウ)
    // When it is angered, it immediately discharges the energy stored in the pouches in its cheeks.
    "pikachu",

    // Raichū (ライチュウ)
    // Its tail discharges electricity into the ground, protecting it from getting shocked.
    "raichu",

    // Sando (サンド)
    // It loves to bathe in the grit of dry, sandy areas. By sand bathing, the Pokémon rids itself of dirt and moisture clinging to its body.
    "sandshrew",

    // Sandopan (サンドパン)
    // The drier the area Sandslash lives in, the harder and smoother the Pokémon’s spikes will feel when touched.
    "sandslash",

    // Nidoran♀ (ニドラン♀)
    // Females are more sensitive to smells than males. While foraging, they’ll use their whiskers to check wind direction and stay downwind of predators.
    "Nidoran♀",

    // Nidorīna (ニドリーナ)
    // The horn on its head has atrophied. It’s thought that this happens so Nidorina’s children won’t get poked while their mother is feeding them.
    "Nidorina",

    // Nidokuin (ニドクイン)
    // Nidoqueen is better at defense than offense. With scales like armor, this Pokémon will shield its children from any kind of attack.
    "nidoqueen",

    // Nidoran♂ (ニドラン♂)
    // The horn on a male Nidoran’s forehead contains a powerful poison. This is a very cautious Pokémon, always straining its large ears.
    "nidoran♂",

    // Nidorīno (ニドリーノ)
    // With a horn that’s harder than diamond, this Pokémon goes around shattering boulders as it searches for a moon stone.
    "nidorino",

    // Nidokingu (ニドキング)
    // When it goes on a rampage, it’s impossible to control. But in the presence of a Nidoqueen it’s lived with for a long time, Nidoking calms down.
    "nidoking",

    // Pippi (ピッピ)
    // It is said that happiness will come to those who see a gathering of Clefairy dancing under a full moon.
    "clefairy",

    // Pikushī (ピクシー)
    // A timid fairy Pokémon that is rarely seen, it will run and hide the moment it senses people.
    "clefable",

    // Rokon (ロコン)
    // While young, it has six gorgeous tails. When it grows, several new tails are sprouted.
    "vulpix",

    // Kyūkon (キュウコン)
    // It is said to live 1,000 years, and each of its tails is loaded with supernatural powers.
    "ninetales",

    // Purin (プリン)
    // When its huge eyes waver, it sings a mysteriously soothing melody that lulls its enemies to sleep.
    "jigglypuff",

    // Pukurin (プクリン)
    // It has a very fine fur. Take care not to make it angry, or it may inflate steadily and hit with a body slam.
    "wigglytuff",

    // Zubatto (ズバット)
    // It emits ultrasonic waves from its mouth to check its surroundings. Even in tight caves, Zubat flies around with skill.
    "zubat",

    // Gorubatto (ゴルバット)
    // It loves to drink other creatures’ blood. It’s said that if it finds others of its kind going hungry, it sometimes shares the blood it’s gathered.
    "golbat",

    // Nazonokusa (ナゾノクサ)
    // If exposed to moonlight, it starts to move. It roams far and wide at night to scatter its seeds.
    "oddish",

    // Kusaihana (クサイハナ)
    // Its pistils exude an incredibly foul odor. The horrid stench can cause fainting at a distance of 1.25 miles.
    "gloom",

    // Rafureshia (ラフレシア)
    // It has the world’s largest petals. With every step, the petals shake out heavy clouds of toxic pollen.
    "vileplume",

    // Parasu (パラス)
    // Burrows under the ground to gnaw on tree roots. The mushrooms on its back absorb most of the nutrition.
    "paras",

    // Parasekuto (パラセクト)
    // The bug host is drained of energy by the mushroom on its back. The mushroom appears to do all the thinking.
    "parasect",

    // Konpan (コンパン)
    // Poison oozes from all over its body. It catches small bug Pokémon at night that are attracted by light.
    "venonat",

    // Morufon (モルフォン)
    // The wings are covered with dustlike scales. Every time it flaps its wings, it looses highly toxic dust.
    "venomoth",

    // Diguda (ディグダ)
    // It lives about one yard underground, where it feeds on plant roots. It sometimes appears aboveground.
    "diglett",

    // Dagutorio (ダグトリオ)
    // Its three heads bob separately up and down to loosen the soil nearby, making it easier for it to burrow.
    "dugtrio",

    // Nyāsu (ニャース)
    // All it does is sleep during the daytime. At night, it patrols its territory with its eyes aglow.
    "meowth",

    // Perushian (ペルシアン)
    // Although its fur has many admirers, it is tough to raise as a pet because of its fickle meanness.
    "persian",

    // Kodakku (コダック)
    // It is constantly wracked by a headache. When the headache turns intense, it begins using mysterious powers.
    "psyduck",

    // Gorudakku (ゴルダック)
    // When it swims at full speed using its long, webbed limbs, its forehead somehow begins to glow.
    "golduck",

    // Mankī (マンキー)
    // It lives in groups in the treetops. If it loses sight of its group, it becomes infuriated by its loneliness.
    "mankey",

    // Okorizaru (オコリザル)
    // It becomes wildly furious if it even senses someone looking at it. It chases anyone that meets its glare.
    "primeape",

    // Gādi (ガーディ)
    // It has a brave and trustworthy nature. It fearlessly stands up to bigger and stronger foes.
    "growlithe",

    // Uindi (ウインディ)
    // An ancient picture scroll shows that people were captivated by its movement as it ran through prairies.
    "arcanine",

    // Nyoromo (ニョロモ)
    // For Poliwag, swimming is easier than walking. The swirl pattern on its belly is actually part of the Pokémon’s innards showing through the skin.
    "poliwag",

    // Nyorozo (ニョロゾ)
    // Staring at the swirl on its belly causes drowsiness. This trait of Poliwhirl’s has been used in place of lullabies to get children to go to sleep.
    "poliwhirl",

    // Nyorobon (ニョロボン)
    // Its body is solid muscle. When swimming through cold seas, Poliwrath uses its impressive arms to smash through drift ice and plow forward.
    "poliwrath",

    // Kēshii (ケーシィ)
    // This Pokémon uses its psychic powers while it sleeps. The contents of Abra’s dreams affect the powers that the Pokémon wields.
    "abra",

    // Yungerā (ユンゲラー)
    // Using its psychic power, Kadabra levitates as it sleeps. It uses its springy tail as a pillow.
    "kadabra",

    // Fūdin (フーディン)
    // It has an incredibly high level of intelligence. Some say that Alakazam remembers everything that ever happens to it, from birth till death.
    "alakazam",

    // Wanrikī (ワンリキー)
    // Its whole body is composed of muscles. Even though it’s the size of a human child, it can hurl 100 grown-ups.
    "machop",

    // Gōrikī (ゴーリキー)
    // Its muscular body is so powerful, it must wear a power-save belt to be able to regulate its motions.
    "machoke",

    // Kairikī (カイリキー)
    // It quickly swings its four arms to rock its opponents with ceaseless punches and chops from all angles.
    "machamp",

    // Madatsubomi (マダツボミ)
    // Prefers hot and humid places. It ensnares tiny bugs with its vines and devours them.
    "bellsprout",

    // Utsudon (ウツドン)
    // When hungry, it swallows anything that moves. Its hapless prey is dissolved by strong acids.
    "weepinbell",

    // Utsubotto (ウツボット)
    // Lures prey with the sweet aroma of honey. Swallowed whole, the prey is dissolved in a day, bones and all.
    "victreebel",

    // Menokurage (メノクラゲ)
    // Tentacool is not a particularly strong swimmer. It drifts across the surface of shallow seas as it searches for prey.
    "tentacool",

    // Dokukurage (ドククラゲ)
    // When the red orbs on Tentacruel’s head glow brightly, watch out. The Pokémon is about to fire off a burst of ultrasonic waves.
    "tentacruel",

    // Ishitsubute (イシツブテ)
    // Commonly found near mountain trails and the like. If you step on one by accident, it gets angry.
    "geodude",

    // Gorōn (ゴローン)
    // Often seen rolling down mountain trails. Obstacles are just things to roll straight over, not avoid.
    "graveler",

    // Gorōnya (ゴローニャ)
    // Once it sheds its skin, its body turns tender and whitish. Its hide hardens when it’s exposed to air.
    "golem",

    // Ponīta (ポニータ)
    // It can’t run properly when it’s newly born. As it races around with others of its kind, its legs grow stronger.
    "ponyta",

    // Gyaroppu (ギャロップ)
    // This Pokémon can be seen galloping through fields at speeds of up to 150 mph, its fiery mane fluttering in the wind.
    "rapidash",

    // Yadon (ヤドン)
    // It is incredibly slow and dopey. It takes five seconds for it to feel pain when under attack.
    "slowpoke",

    // Yadoran (ヤドラン)
    // When a Slowpoke went hunting in the sea, its tail was bitten by a Shellder. That made it evolve into Slowbro.
    "slowbro",

    // Koiru (コイル)
    // The electromagnetic waves emitted by the units at the sides of its head expel antigravity, which allows it to float.
    "magnemite",

    // Reakoiru (レアコイル)
    // Three Magnemite are linked by a strong magnetic force. Earaches will occur if you get too close.
    "magneton",

    // Kamonegi (カモネギ)
    // The stalk this Pokémon carries in its wings serves as a sword to cut down opponents. In a dire situation, the stalk can also serve as food.
    "farfetch'd",

    // Dōdō (ドードー)
    // Its short wings make flying difficult. Instead, this Pokémon runs at high speed on developed legs.
    "doduo",

    // Dōdorio (ドードリオ)
    // One of Doduo’s two heads splits to form a unique species. It runs close to 40 mph in prairies.
    "dodrio",

    // Pauwau (パウワウ)
    // Loves freezing-cold conditions. Relishes swimming in a frigid climate of around 14 degrees Fahrenheit.
    "seel",

    // Jugon (ジュゴン)
    // Its entire body is a snowy white. Unharmed by even intense cold, it swims powerfully in icy waters.
    "Dewgong",

    // Betobetā (ベトベター)
    // Born from sludge, these Pokémon now gather in polluted places and increase the bacteria in their bodies.
    "grimer",

    // Betobeton (ベトベトン)
    // It’s thickly covered with a filthy, vile sludge. It is so toxic, even its footprints contain poison.
    "muk",

    // Sherudā (シェルダー)
    // It is encased in a shell that is harder than diamond. Inside, however, it is surprisingly tender.
    "shellder",

    // Parushen (パルシェン)
    // Cloyster that live in seas with harsh tidal currents grow large, sharp spikes on their shells.
    "cloyster",

    // Gōsu (ゴース)
    // It wraps its opponent in its gas-like body, slowly weakening its prey by poisoning it through the skin.
    "gastly",

    // Gōsuto (ゴースト)
    // It likes to lurk in the dark and tap shoulders with a gaseous hand. Its touch causes endless shuddering.
    "haunter",

    // Gengā (ゲンガー)
    // To steal the life of its target, it slips into the prey’s shadow and silently waits for an opportunity.
    "gengar",

    // Iwāku (イワーク)
    // As it digs through the ground, it absorbs many hard objects. This is what makes its body so solid.
    "onix",

    // Surīpu (スリープ)
    // It remembers every dream it eats. It rarely eats the dreams of adults because children’s are much tastier.
    "drowzee",

    // Surīpā (スリーパー)
    // When it locks eyes with an enemy, it will use a mix of psi moves, such as Hypnosis and Confusion.
    "hypno",

    // Kurabu (クラブ)
    // It can be found near the sea. The large pincers grow back if they are torn out of their sockets.
    "krabby",

    // Kingurā (キングラー)
    // Its large and hard pincer has 10,000-horsepower strength. However, being so big, it is unwieldy to move.
    "kingler",

    // Biriridama (ビリリダマ)
    // It rolls to move. If the ground is uneven, a sudden jolt from hitting a bump can cause it to explode.
    "voltorb",

    // Marumain (マルマイン)
    // The more energy it charges up, the faster it gets. But this also makes it more likely to explode.
    "electrode",

    // Tamatama (タマタマ)
    // Though it may look like it’s just a bunch of eggs, it’s a proper Pokémon. Exeggcute communicates with others of its kind via telepathy, apparently.
    "exeggcute",

    // Nasshī (ナッシー)
    // Each of Exeggutor’s three heads is thinking different thoughts. The three don’t seem to be very interested in one another.
    "exeggutor",

    // Karakara (カラカラ)
    // When the memory of its departed mother brings it to tears, its cries echo mournfully within the skull it wears on its head.
    "cubone",

    // Garagara (ガラガラ)
    // This Pokémon overcame its sorrow to evolve a sturdy new body. Marowak faces its opponents bravely, using a bone as a weapon.
    "marowak",

    // Sawamurā (サワムラー)
    // This amazing Pokémon has an awesome sense of balance. It can kick in succession from any position.
    "hitmonlee",

    // Ebiwarā (エビワラー)
    // Its punches slice the air. They are launched at such high speed, even a slight graze could cause a burn.
    "hitmonchan",

    // Beroringa (ベロリンガ)
    // If this Pokémon’s sticky saliva gets on you and you don’t clean it off, an intense itch will set in. The itch won’t go away, either.
    "lickitung",

    // Dogāsu (ドガース)
    // Its body is full of poisonous gas. It floats into garbage dumps, seeking out the fumes of raw, rotting trash.
    "koffing",

    // Matadogasu (マタドガス)
    // It mixes gases between its two bodies. It’s said that these Pokémon were seen all over the Galar region back in the day.
    "weezing",

    // Saihōn (サイホーン)
    // Strong, but not too bright, this Pokémon can shatter even a skyscraper with its charging tackles.
    "rhyhorn",

    // Saidon (サイドン)
    // It begins walking on its hind legs after evolution. It can punch holes through boulders with its horn.
    "rhydon",

    // Rakkī (ラッキー)
    // This kindly Pokémon lays highly nutritious eggs and shares them with injured Pokémon or people.
    "chansey",

    // Monjara (モンジャラ)
    // Hidden beneath a tangle of vines that grows nonstop even if the vines are torn off, this Pokémon’s true appearance remains a mystery.
    "tangela",

    // Garūra (ガルーラ)
    // Although it’s carrying its baby in a pouch on its belly, Kangaskhan is swift on its feet. It intimidates its opponents with quick jabs.
    "kangaskhan",

    // Tattsū (タッツー)
    // Horsea makes its home in oceans with gentle currents. If this Pokémon is under attack, it spits out pitch-black ink and escapes.
    "horsea",

    // Shīdora (シードラ)
    // It’s the males that raise the offspring. While Seadra are raising young, the spines on their backs secrete thicker and stronger poison.
    "seadra",

    // Tosakinto (トサキント)
    // Its dorsal, pectoral, and tail fins wave elegantly in water. That is why it is known as the Water Dancer.
    "goldeen",

    // Azumaō (アズマオウ)
    // In autumn, its body becomes more fatty in preparing to propose to a mate. It takes on beautiful colors.
    "seaking",

    // Hitodeman (ヒトデマン)
    // If you visit a beach at the end of summer, you’ll be able to see groups of Staryu lighting up in a steady rhythm.
    "staryu",

    // Sutāmī (スターミー)
    // This Pokémon has an organ known as its core. The organ glows in seven colors when Starmie is unleashing its potent psychic powers.
    "starmie",

    // Bariyādo (バリヤード)
    // The broadness of its hands may be no coincidence—many scientists believe its palms became enlarged specifically for pantomiming.
    "Mr. Mime",

    // Sutoraiku (ストライク)
    // It slashes through grass with its sharp scythes, moving too fast for the human eye to track.
    "scyther",

    // Rūjura (ルージュラ)
    // In certain parts of Galar, Jynx was once feared and worshiped as the Queen of Ice.
    "jynx",

    // Erebū (エレブー)
    // Many power plants keep Ground-type Pokémon around as a defense against Electabuzz that come seeking electricity.
    "electabuzz",

    // Būbā (ブーバー)
    // Magmar dispatches its prey with fire. But it regrets this habit once it realizes that it has burned its intended prey to a charred crisp.
    "magmar",

    // Kairosu (カイロス)
    // These Pokémon judge one another based on pincers. Thicker, more impressive pincers make for more popularity with the opposite gender.
    "pinsir",

    // Kentarosu (ケンタロス)
    // Once it takes aim at its prey, it makes a headlong charge. It is famous for its violent nature.
    "tauros",

    // Koikingu (コイキング)
    // An underpowered, pathetic Pokémon. It may jump high on rare occasions but never more than seven feet.
    "magikarp",

    // Gyaradosu (ギャラドス)
    // Once it appears, it goes on a rampage. It remains enraged until it demolishes everything around it.
    "gyarados",

    // Rapurasu (ラプラス)
    // A smart and kindhearted Pokémon, it glides across the surface of the sea while its beautiful song echoes around it.
    "lapras",

    // Metamon (メタモン)
    // Its transformation ability is perfect. However, if made to laugh, it can’t maintain its disguise.
    "ditto",

    // Ībui (イーブイ)
    // Its ability to evolve into many forms allows it to adapt smoothly and perfectly to any environment.
    "eevee",

    // Shawāzu (シャワーズ)
    // It lives close to water. Its long tail is ridged with a fin, which is often mistaken for a mermaid’s.
    "vaporeon",

    // Sandāsu (サンダース)
    // It concentrates the weak electric charges emitted by its cells and launches wicked lightning bolts.
    "jolteon",

    // Būsutā (ブースター)
    // Inhaled air is carried to its flame sac, heated, and exhaled as fire that reaches over 3,000 degrees Fahrenheit.
    "flareon",

    // Porigon (ポリゴン)
    // State-of-the-art technology was used to create Porygon. It was the first artificial Pokémon to be created via computer programming.
    "porygon",

    // Omunaito (オムナイト)
    // Because some Omanyte manage to escape after being restored or are released into the wild by people, this species is becoming a problem.
    "omanyte",

    // Omusutā (オムスター)
    // Weighed down by a large and heavy shell, Omastar couldn’t move very fast. Some say it went extinct because it was unable to catch food.
    "omastar",

    // Kabuto (カブト)
    // This species is almost entirely extinct. Kabuto molt every three days, making their shells harder and harder.
    "kabuto",

    // Kabutopusu (カブトプス)
    // Kabutops slices its prey apart and sucks out the fluids. The discarded body parts become food for other Pokémon.
    "kabutops",

    // Putera (プテラ)
    // his is a ferocious Pokémon from ancient times. Apparently even modern technology is incapable of producing a perfectly restored specimen.
    "aerodactyl",

    // Kabigon (カビゴン)
    // It is not satisfied unless it eats over 880 pounds of food every day. When it is done eating, it goes promptly to sleep.
    "snorlax",

    // Furīzā (フリーザー)
    // It’s said that this Pokémon’s beautiful blue wings are made of ice. Articuno flies over snowy mountains, its long tail fluttering along behind it.
    "articuno",

    // Sandā (サンダー)
    // This Pokémon has complete control over electricity. There are tales of Zapdos nesting in the dark depths of pitch-black thunderclouds.
    "zapdos",

    // Faiyā (ファイヤー)
    // It’s one of the legendary bird Pokémon. When Moltres flaps its flaming wings, they glimmer with a dazzling red glow.
    "moltres",

    // Miniryū (ミニリュウ)
    // It sheds many layers of skin as it grows larger. During this process, it is protected by a rapid waterfall.
    "dratini",

    // Hakuryū (ハクリュー)
    // They say that if it emits an aura from its whole body, the weather will begin to change instantly.
    "dragonair",

    // Kairyū (カイリュー)
    // It is said that somewhere in the ocean lies an island where these gather. Only they live there.
    "dragonite",

    // Myūtsū (ミュウツー)
    // Its DNA is almost the same as Mew’s. However, its size and disposition are vastly different.
    // "mewtwo",

    // Myū (ミュウ)
    // When viewed through a microscope, this Pokémon’s short, fine, delicate hair can be seen.
    // "mew",
];

pub fn get_random_name() -> String {
    let mut rng = rand::thread_rng();
    format!("{}_{}", ADJECTIVES.choose(&mut rng).unwrap(), POKEMONS.choose(&mut rng).unwrap())
}
