pub struct Passage {
    pub title: &'static str,
    pub text: &'static str,
    pub questions: [Question; 3],
}

pub struct Question {
    pub prompt: &'static str,
    pub choices: [&'static str; 3],
    pub correct: usize,
    pub explanation: &'static str,
}

macro_rules! question {
    ($prompt:expr, [$a:expr, $b:expr, $c:expr], $correct:expr, $why:expr) => {
        Question {
            prompt: $prompt,
            choices: [$a, $b, $c],
            correct: $correct,
            explanation: $why,
        }
    };
}

pub const PASSAGES: [Passage; 12] = [
    Passage {
        title: "A bench in the shade",
        text: "Mina went to the park with a sandwich and a book. The bench by the path was warm in the sun, so she walked to a bench under a tree. From there she could see the small lake. She ate her sandwich first, then opened her book. When a leaf fell onto the page, Mina used it to mark her place.",
        questions: [
            question!("Why did Mina move to another bench?", ["The first bench was wet", "She wanted some shade", "The path was closed"], 1, "The first bench was warm in the sun, and the other bench was under a tree."),
            question!("What did Mina do before opening the book?", ["She ate her sandwich", "She fed the ducks", "She went home"], 0, "She ate her sandwich first."),
            question!("What marked her place in the book?", ["A paper ticket", "A small stone", "A fallen leaf"], 2, "A leaf fell onto the page, and Mina used it as a marker."),
        ],
    },
    Passage {
        title: "The missing blue cup",
        text: "Leo put two cups on the kitchen table, a red one for himself and a blue one for his sister. The blue cup was gone when he came back with water. His sister was on the balcony. She had taken the cup outside to water a small plant. Leo brought her the bottle, and they filled the cup together.",
        questions: [
            question!("Which cup did Leo leave for his sister?", ["The green cup", "The red cup", "The blue cup"], 2, "Leo set out the blue cup for his sister."),
            question!("Where did Leo find his sister?", ["On the balcony", "In the kitchen", "In the garden"], 0, "His sister had gone onto the balcony."),
            question!("What did she want to water?", ["The kitchen floor", "A small plant", "A bowl of fruit"], 1, "She had taken the cup outside to water a plant."),
        ],
    },
    Passage {
        title: "A late bus",
        text: "Nhi reached the bus stop at eight. A note on the sign said the bus would arrive ten minutes late. Instead of walking home, she sat beside an older man who was waiting too. They watched a baker put fresh bread in a shop window across the street. When the bus arrived, Nhi stood back so the man could get on first.",
        questions: [
            question!("How late was the bus?", ["Five minutes", "Ten minutes", "One hour"], 1, "The note on the sign gave a ten-minute delay."),
            question!("What did the baker put in the window?", ["Fresh bread", "A new sign", "Small cakes"], 0, "They saw fresh bread being placed in the window."),
            question!("Who did Nhi let board first?", ["The baker", "The driver", "The older man"], 2, "Nhi stood back to let the older man get on first."),
        ],
    },
    Passage {
        title: "Bread for a neighbor",
        text: "Every Saturday, Jo made two small loaves of bread. She kept one and took the other to a neighbor who lived upstairs. One Saturday the neighbor was out. Jo left a note by the door and took the bread home. Later, when the neighbor knocked, Jo handed over a still-warm loaf wrapped in a clean cloth.",
        questions: [
            question!("How many loaves did Jo make on Saturdays?", ["One", "Two", "Three"], 1, "Jo made two small loaves every Saturday."),
            question!("What did Jo leave by the door?", ["The bread", "A basket", "A note"], 2, "She left a note because her neighbor was out."),
            question!("When did Jo give away the second loaf?", ["When the neighbor knocked", "Before she baked it", "When she went upstairs"], 0, "The neighbor came to Jo later, and she handed over the warm loaf."),
        ],
    },
    Passage {
        title: "The empty shelf",
        text: "Sam went to the library to borrow a book about the moon. The shelf label was easy to find, but the shelf was empty. A librarian looked up the title and told Sam the only copy was due back on Friday. Sam left his name on the waiting list. For that afternoon he chose a book about clouds.",
        questions: [
            question!("What book had Sam planned to borrow?", ["A book about rain", "A book about the moon", "A book about birds"], 1, "Sam was looking for a book about the moon."),
            question!("When was that book due back?", ["Friday", "Monday", "Sunday"], 0, "The librarian said the only copy was due back on Friday."),
            question!("What did Sam choose for the afternoon?", ["A map of the moon", "A puzzle", "A book about clouds"], 2, "While waiting for the moon book, he chose a book about clouds."),
        ],
    },
    Passage {
        title: "The quiet morning",
        text: "Before school, Tuan liked to write one line in a notebook. On Tuesday he could hear heavy rain against the roof, but he could not see the street. The window was covered with small drops. He wrote about the sound. At breakfast his sister read the line and said she could almost hear the rain too.",
        questions: [
            question!("When did Tuan usually write his line?", ["After school", "Before school", "At bedtime"], 1, "He wrote in his notebook before school."),
            question!("Why could he not see the street?", ["The window was covered in drops", "The street lights were broken", "The curtains were closed"], 0, "Drops covered the window."),
            question!("What did Tuan describe that morning?", ["The street", "His breakfast", "The sound of the rain"], 2, "He listened to the rain and wrote about its sound."),
        ],
    },
    Passage {
        title: "A garden on the roof",
        text: "The school garden used to be beside the gate. It had plenty of space, but a tall wall cast a shadow over it for most of the day. The garden club moved a few light pots to a sunny corner of the roof. The plants there grew more leaves, although the wind dried the soil faster. The club kept both gardens. They put plants that liked more sun on the roof and chose plants suited to shade for the gate.",
        questions: [
            question!("Why did the club first try the roof?", ["It had more sun", "It was less windy", "It had deeper soil"], 0, "The roof's sunny corner addressed the shade at the gate."),
            question!("What problem did the roof introduce?", ["Strong shade", "Less space by the gate", "Soil drying faster"], 2, "Wind made the soil in the roof pots dry faster."),
            question!("Why did the club keep both gardens?", ["The roof plants stopped growing", "Each place suited different plants", "The gate could not be opened"], 1, "The club matched plants that liked sun and shade to the two locations."),
        ],
    },
    Passage {
        title: "A bridge built twice",
        text: "A design class built a paper bridge between two desks. The first bridge used a flat sheet. It bent as soon as Asha put a coin in the middle. For the second test, the class folded the same sheet into ridges and kept the desks in the same place. The folded bridge held several coins. The class recorded the difference in shape so the next group could repeat the test.",
        questions: [
            question!("What did the class change for the second test?", ["The gap between the desks", "The coins", "The shape of the same sheet"], 2, "They folded the sheet into ridges and kept the desks in place."),
            question!("Why was keeping the desks in place useful?", ["It kept the distance the same", "It made the coins smaller", "It made the paper heavier"], 0, "Keeping the span the same helped isolate the change in shape."),
            question!("What did the class record for the next group?", ["The names of the students", "The difference in shape", "The height of the desks only"], 1, "They wrote down the change in shape so another group could repeat it."),
        ],
    },
    Passage {
        title: "The market map",
        text: "A street market printed a map with every stall marked the same way. Visitors who wanted vegetables still had to walk past each sign to find the right sellers. The organizers tried adding symbols for vegetables, bread, and crafts. They added a key to explain the symbols. Because some visitors printed the map in black and white, they chose shapes that stayed distinct without color.",
        questions: [
            question!("What was hard about the first map?", ["The streets had no names", "Every stall looked the same", "It showed the wrong market"], 1, "Every stall used the same mark, so the map did not show what each sold."),
            question!("What was the purpose of the key?", ["To unlock the market", "To list lost items", "To explain the symbols"], 2, "The key showed what kind of stall each symbol represented."),
            question!("Why did the organizers choose distinct shapes?", ["Some copies had no color", "The stalls changed names daily", "They wanted fewer streets"], 0, "Visitors with black-and-white copies still needed to tell the symbols apart."),
        ],
    },
    Passage {
        title: "Listening by the pond",
        text: "A science club counted frogs near a pond by listening for calls. They counted six calls just after sunset on a warm evening, only two the following morning, and seven again after dark two days later. They did not assume that frogs had left and returned. A call was evidence of activity, but a frog that stayed quiet would not be counted. The club scheduled future counts for the same time of day.",
        questions: [
            question!("What did the club count directly?", ["Frog calls", "All frogs in the pond", "Shadows on the water"], 0, "The measurements counted calls heard near the pond."),
            question!("Why might a low count miss some frogs?", ["The pond moved during the day", "A quiet frog would not be heard", "All frogs shared one call"], 1, "A frog could be present while making no audible call."),
            question!("How did the club change future counts?", ["They watched only in the morning", "They stopped making records", "They used the same time of day"], 2, "Using the same time made the new counts more comparable."),
        ],
    },
    Passage {
        title: "Borrowing a bike",
        text: "On Monday Pia borrowed a bike to visit a friend. The shortest route went down a hill that ended beside a busy crossing. On her return she used a longer path beside a stream. The stream path took a few extra minutes but had no busy crossing. When Pia described the routes to a beginner, she suggested the stream path and explained the trade-off.",
        questions: [
            question!("What was the advantage of the hill route?", ["It had no crossing", "It was the shortest route", "It followed a stream"], 1, "The hill route was the shortest."),
            question!("What did the stream path cost Pia?", ["A fare", "A broken bike", "A few extra minutes"], 2, "The longer stream path took a few extra minutes."),
            question!("Why might a beginner prefer the stream path?", ["It avoids the busy crossing", "It is the shortest route", "It always goes downhill"], 0, "Pia recommended the slower route because it avoided the busy crossing."),
        ],
    },
    Passage {
        title: "Two versions of a notice",
        text: "The town hall posted a notice that the reading room would close for a meeting. At first the notice gave only the date. Visitors guessed different closing times. The next notice gave the date, the closing hour, and the hour the room would open again. The change did not make the meeting shorter, but visitors could plan around it.",
        questions: [
            question!("What information was on the first notice?", ["The date only", "Both opening hours", "A list of visitors"], 0, "The original notice stated the date but left out the hours."),
            question!("Why did visitors guess different times?", ["The date had been erased", "The meeting used several rooms", "The closing hour was missing"], 2, "The first notice did not specify when the room would close."),
            question!("What did the second notice improve?", ["It shortened the meeting", "It helped visitors plan", "It changed the room's location"], 1, "Both hours made it easier to plan a visit around the meeting."),
        ],
    },
];
