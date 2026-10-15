//! The words: place names in two registers (design/next-items.md §2.4), the radio net's Ukrainian, and the
//! font that can set them. On the net Ukrainian comes first and the English gloss sits under it; where the
//! player reads fast (the counters, the ledger, the briefing) Latin first: "Vuhledar (Вугледар)". The spellings
//! are the 2010 national romanisation; the Russian-derived ones appear nowhere.

/// A place in Ukrainian and in its romanisation.
pub struct Name {
    pub uk: &'static str,
    pub en: &'static str,
}

impl Name {
    /// For the player's bookkeeping: "Vuhledar (Вугледар)".
    pub fn hud(&self) -> String {
        format!("{} ({})", self.en, self.uk)
    }
}

pub const VUHLEDAR: Name = Name { uk: "Вугледар", en: "Vuhledar" };
pub const MALA_TOKMACHKA: Name = Name { uk: "Мала Токмачка", en: "Mala Tokmachka" };
pub const ROBOTYNE: Name = Name { uk: "Роботине", en: "Robotyne" };
pub const TOKMAK: Name = Name { uk: "Токмак", en: "Tokmak" };

/// The interface's face: Liberation Mono (SIL Open Font License, src/fonts/), which has the Ukrainian letters
/// Bevy's built-in subset lacks, and the signal dots.
pub const FONT: &str = "embedded://bavovniatko/fonts/LiberationMono-Regular.ttf";

/// The radio net's Ukrainian, by its English gloss. The gloss is what the code says; this is what is said
/// (the idiom of §2.4: плюс, прийняв, вихід, прихід, мінус, триста, пташка, коробочка, посадка). A line
/// without a Ukrainian here is shown in English alone. The net's stations have callsigns, fictional as
/// design/next.md decided: Маестро (Maestro) is the player, Беркут (Berkut) the guns, Сокіл (Sokil) the
/// observation post, Граніт (Hranit) the brigade's command post. A call names whom it is for, then "я" and
/// whose it is ("Маестро, я Беркут"); an alarm is shouted without the formula, and the bird's crew, being
/// the player's own, needs none.
pub fn uk(en: &str) -> &'static str {
    match en {
        // The day at Vuhledar
        "Bird up. Visibility good, frost. Watch the dacha road and the Pavlivka road." => "Пташка в повітрі. Видимість добра, мороз. Дивимось дачну дорогу і дорогу на Павлівку.",
        "All, this is Hranit. They're coming again today. Leave the roads clear. Let them in." => "Всім, я Граніт. Сьогодні знову підуть. Дороги не чіпати. Пускаємо їх.",
        "Bird: column on the dacha road. Twelve units, tanks leading, a plow in front. Smoke." => "Пташка: колона на дачній дорозі. Дванадцять коробочок, танки попереду, перший з тралом. Димлять.",
        "This is Sokil. Second column. Same route. Exactly the same route." => "Я Сокіл. Друга колона. Той самий маршрут. Точно той самий.",
        "Bird: Pavlivka road now. Volunteers by the look of it. BMP-1s." => "Пташка: тепер дорога на Павлівку. Схоже, добровольці. БМП-1.",
        "This is Sokil. They're driving past their own wrecks from this morning. Same line." => "Я Сокіл. Їдуть повз свої ж ранкові підбиті. Тією ж лінією.",
        "This is Sokil. Groups of five. Walking across the open field. Towards the tree line." => "Я Сокіл. По п'ятеро. Йдуть відкритим полем. На посадку.",
        "TOS! Solntsepyok on the town! Get down!" => "ТОС! Сонцепьок по місту! Всі вниз!",
        "Bird: interference. Their EW is up." => "Пташка: перешкоди. Їхній РЕБ працює.",
        // The morning south of Mala Tokmachka
        "Bird up. Still dark. Our guns are working the crest." => "Пташка в повітрі. Ще темно. Наша арта працює по гребеню.",
        "All, this is Hranit. The column is late. The guns will be finished before it gets there." => "Всім, я Граніт. Колона запізнюється. Арта закінчить раніше, ніж вони дійдуть.",
        "This is Sokil. Guns are done. Nothing on the track yet." => "Я Сокіл. Арта відпрацювала. На дорозі поки нікого.",
        "This is Sokil. First group on the western track. Nine vehicles. The Leopard with the plow in front, the rollers behind it." => "Я Сокіл. Перша група на західній дорозі. Дев'ять машин. Попереду Леопард з тралом, за ним катки.",
        "This is Sokil. Second group moving. Same plan, the other track." => "Я Сокіл. Друга група пішла. Той самий план, інша дорога.",
        "Ka-52s! Two of them, south-east, low over the ridge. Too far for anything we have." => "Ка-52! Два, південний схід, низько над гребенем. Задалеко для всього, що маємо.",
        "All, this is Hranit. Four Bradleys going back in. Get the infantry onto them." => "Всім, я Граніт. Чотири Бредлі йдуть назад по людей. Саджайте піхоту.",
        "This is Sokil. Helicopters again. South-west this time." => "Я Сокіл. Знову вертольоти. Цього разу південний захід.",
        "This is Sokil. Quiet on the lanes. Only the smoke." => "Я Сокіл. На проходах тихо. Тільки дим.",
        "All, this is Hranit. That's enough. All callsigns back to the start line. Bring the infantry." => "Всім, я Граніт. Досить. Всі позивні — назад на вихідну. Піхоту забрати.",
        "Launch from the helicopter! Missile inbound!" => "Пуск з вертольота! Ракета йде!",
        // Orders
        "All teams, this is Maestro. Wait until the column is committed, then fire." => "Всім розрахункам, я Маестро. Чекати, поки колона втягнеться, тоді вогонь.",
        "All teams, this is Maestro. Hold fire. Let them come." => "Всім розрахункам, я Маестро. Вогонь не відкривати. Нехай ідуть.",
        "All teams, this is Maestro. Fire at will." => "Всім розрахункам, я Маестро. Вогонь по готовності.",
        "All callsigns, this is Maestro. Push on. Keep to the lane." => "Всім позивним, я Маестро. Вперед. Тримайтесь проходу.",
        "All callsigns, this is Maestro. Hold where you are. Do not leave the lane." => "Всім позивним, я Маестро. Стояти на місці. З проходу не виходити.",
        "All callsigns, this is Maestro. Pull back. Back to the start line. Pick up the infantry on the way." => "Всім позивним, я Маестро. Відхід. На вихідну. По дорозі забрати піхоту.",
        // The guns
        "Maestro, this is Hranit. No more HIMARS for you today." => "Маестро, я Граніт. Хаймарсів на сьогодні для вас більше нема.",
        "No more smoke." => "Диму більше немає.",
        "No more RAAM." => "РААМ більше немає.",
        "Maestro, this is Berkut. Out of shells. Nothing to send." => "Маестро, я Беркут. Снарядів нема. Нічим працювати.",
        "Maestro, this is Berkut. Still firing the last mission. Wait." => "Маестро, я Беркут. Ще відпрацьовуємо попередню. Чекай.",
        "Three hundred. Need evacuation from the tree line." => "Триста. Потрібна евакуація з посадки.",
        "This is Sokil. Their position on the crest has gone quiet." => "Я Сокіл. Їхня позиція на гребені замовкла.",
        "The OP on the roof is gone. TOS took the building. No eyes from the high-rise." => "СП на даху нема. ТОС накрив будинок. З висотки очей немає.",
        "The water tower is down. No eyes from the farm." => "Вежа впала. З ферми очей немає.",
        "ATGM from the crest! Launch, launch!" => "ПТРК з гребеня! Пуск, пуск!",
        "Incoming on the lane! They have it registered. Get out of there!" => "Прихід по проходу! У них він пристріляний. Забирайтесь звідти!",
        // The tree line
        "Nobody left here to work the launcher." => "Тут вже нема кому працювати з пускової.",
        "Out of missiles at this position." => "На цій позиції ракет немає.",
        "No lock. Hold the seeker on him." => "Захвату нема. Тримай його в прицілі.",
        "Javelin away. Get down, move." => "Джавелін пішов. Вниз, зміна позиції.",
        "Hit! He's burning." => "Є попадання! Горить.",
        "Miss. Into the ground." => "Промах. В землю.",
        // The kill zone
        "Bradley at the wrecks. Ramp down. Get on, get on!" => "Бредлі біля підбитих. Апарель вниз. Сідайте, сідайте!",
        "Double mine under the plow. Plow's gone, he's not moving. The lane stops here." => "Подвійна міна під тралом. Трала нема, він стоїть. Прохід закінчується тут.",
        "Mine under the plow. The lead tank's plow is gone, it keeps going." => "Міна під тралом. Трал у головного зірвало, він іде далі.",
        "Mine under the plow. Plow's gone. The rest of the lane is not cleared." => "Міна під тралом. Трала нема. Далі прохід не розмінований.",
        "Mine! He went off the road. That one's done." => "Міна! Він з'їхав з дороги. Цей все.",
        "Another one on the shoulder. Tracks off." => "Ще один на узбіччі. Гусениці зірвало.",
        "He drove right past the last wreck, same line. Mine." => "Проїхав повз попереднього підбитого, тією ж лінією. Міна.",
        "Boom. Mine. He's not moving." => "Бум. Міна. Він стоїть.",
        "Mine! Left track gone. He's stuck in the lane." => "Міна! Ліва гусениця зірвана. Застряг у проході.",
        "He went round the Leopard. Off the lane. Mine." => "Обійшов Леопард. З'їхав з проходу. Міна.",
        "Another one. They're everywhere off the plowed strip." => "Ще один. Поза протраленою смугою вони скрізь.",
        "Mine. Crew's alive, they're getting out." => "Міна. Екіпаж живий, виходять.",
        "Turret's off. Jack in the box." => "Башта відлетіла. БК здетонував.",
        // The day's captions
        "06:45  The 155th Naval Infantry attacks down the dacha road" => "06:45  155-та морська піхота йде дачною дорогою",
        "08:40  A second column. The same road." => "08:40  Друга колона. Та сама дорога.",
        "10:20  The Alga battalion, from Pavlivka" => "10:20  Батальйон «Алга», від Павлівки",
        "12:00  Again, the dacha road" => "12:00  Знову дачна дорога",
        "13:20  No more armour. Infantry, on foot." => "13:20  Броні більше нема. Піхота, пішки.",
        "14:30  More men on foot" => "14:30  Ще люди пішки",
        "TOS-1A on the town" => "ТОС-1А по місту",
        "TOS-1A again" => "ТОС-1А знову",
        "TOS-1A" => "ТОС-1А",
        "04:23  The first group moves off behind the plows. It is getting light." => "04:23  Перша група рушає за тралами. Світає.",
        "05:29  The second group, eleven vehicles, on the eastern track" => "05:29  Друга група, одинадцять машин, на східній дорозі",
        "07:02  Four Bradleys go back in for the men" => "07:02  Чотири Бредлі йдуть назад по людей",
        "08:50  Everyone back to the start line" => "08:50  Всі назад на вихідну",
        _ => "",
    }
}
