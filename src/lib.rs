#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


#[allow(dead_code)]
pub mod structure {

    #[derive(Default)]
    struct Player<'a> {
        name : String,
        toons: Vec<&'a mut Toon<'a>>,
    }

    struct Toon<'a> {
        name  : String,
        player: &'a Player<'a>,
        level : i32,
        class : String,
        roles : Vec<&'a mut Role<'a>>,
    }

    struct Role<'a> {
        name    : String,
        toon    : &'a Toon<'a>,
        ear     : Option<Box<Equipment>>,
        implant1: Option<Box<Equipment>>,
        implant2: Option<Box<Equipment>>,
        relic1  : Option<Box<Equipment>>,
        relic2  : Option<Box<Equipment>>,
        mh      : Option<Box<Equipment>>,
        oh      : Option<Box<Equipment>>,
        head    : Option<Box<Equipment>>,
        chest   : Option<Box<Equipment>>,
        hand    : Option<Box<Equipment>>,
        legs    : Option<Box<Equipment>>,
        feet    : Option<Box<Equipment>>,
        bracer  : Option<Box<Equipment>>,
    }

    enum Equipment {
        Ear     {rating: i32, stat: String, augment: Augment},
        Implant {rating: i32, stat: String, augment: Augment},
        Relic   {rating: i32, augment: Augment},
        MH      {main_slot: MainSlot, mod_slot: ModSlot, enhance_slot: i32,
                 crystal: String, augment: Augment},
        OH      {main_slot: MainSlot, mod_slot: ModSlot, enhance_slot: i32,
                 crystal: String, augment: Augment},
        Head    {main_slot: MainSlot, mod_slot: ModSlot, enhance_slot: i32,
                 augment: Augment},
        Chest   {main_slot: MainSlot, mod_slot: ModSlot, enhance_slot: i32,
                 augment: Augment},
        Hand    {main_slot: MainSlot, mod_slot: ModSlot, enhance_slot: i32,
                 augment: Augment},
        Belt    {main_slot: MainSlot, mod_slot: ModSlot, augment: Augment},
        Legs    {main_slot: MainSlot, mod_slot: ModSlot, enhance_slot: i32,
                 augment: Augment},
        Feet    {main_slot: MainSlot, mod_slot: ModSlot, enhance_slot: i32,
                 augment: Augment},
        Bracer  {main_slot: MainSlot, mod_slot: ModSlot, augment: Augment},
    }

    struct Augment {
        rating: i32,
        stat  : String,
    }

    struct MainSlot {

    }

    struct ModSlot {

    }

    struct EnhanceSlot {

    }

}
