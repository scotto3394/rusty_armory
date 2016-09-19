#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


#[allow(dead_code)]
pub mod structure {
//===========================================================================================
// pub structs
//===========================================================================================
    #[derive(Default, Debug)]
    pub struct Player {
        name : String,
        pub toons: Vec<Box<Toon>>,
    }

    #[derive(Default, Debug)]
    pub struct Toon {
        name  : String,
        level : i32,
        class : Option<Class>,
        pub roles : Vec<Box<Role>>,
    }

    #[derive(Debug)]
    pub enum Class {
        Guardian,
        Sentinel,
        Sage,
        Shadow,
        Gunslinger,
        Scoundrel,
        Vanguard,
        Trooper,
        Juggernaut,
        Marauder,
        Sorceror,
        Assassin,
        Sniper,
        Operative,
        Powertech,
        Mercenary,
    }

    #[derive(Default, Debug)]
    pub struct Role {
        name    : String,
        ear     : Option<Box<Cybernetics>>,
        implant1: Option<Box<Cybernetics>>,
        implant2: Option<Box<Cybernetics>>,
        relic1  : Option<Box<Relic>>,
        relic2  : Option<Box<Relic>>,
        mh      : Option<Box<Weapon>>,
        oh      : Option<Box<Weapon>>,
        head    : Option<Box<Clothes>>,
        chest   : Option<Box<Clothes>>,
        hand    : Option<Box<Accessory>>,
        legs    : Option<Box<Clothes>>,
        feet    : Option<Box<Clothes>>,
        bracer  : Option<Box<Accessory>>,
    }

    #[derive(Default, Debug)]
    pub struct Cybernetics {rating: i32, stat: String, augment: Option<Box<Augment>>}
    #[derive(Default, Debug)]
    pub struct Relic       {rating: i32, augment: Option<Box<Augment>>}
    #[derive(Default, Debug)]
    pub struct Weapon {
        main_slot   : Option<Box<MainSlot>>,
        mod_slot    : Option<Box<ModSlot>>,
        enhance_slot: Option<Box<EnhanceSlot>>,
        crystal     : Option<Box<Crystal>>,
        augment     : Option<Box<Augment>>,
    }
    #[derive(Default, Debug)]
    pub struct Clothes {
        main_slot   : Option<Box<MainSlot>>,
        mod_slot    : Option<Box<ModSlot>>,
        enhance_slot: Option<Box<EnhanceSlot>>,
        augment     : Option<Box<Augment>>,
    }
    #[derive(Default, Debug)]
    pub struct Accessory {
        main_slot   : Option<Box<MainSlot>>,
        mod_slot    : Option<Box<ModSlot>>,
        augment     : Option<Box<Augment>>,
    }
    #[derive(Default, Debug)]
    pub struct Augment     {rating: i32, stat:String}
    #[derive(Default, Debug)]
    pub struct MainSlot    {rating: i32}
    #[derive(Default, Debug)]
    pub struct ModSlot     {rating: i32, letter: char}
    #[derive(Default, Debug)]
    pub struct EnhanceSlot {rating: i32, stat: String}
    #[derive(Default, Debug)]
    pub struct Crystal     {rating: i32, stat: String}

//===========================================================================================
// Traits
//===========================================================================================

    pub trait HasRating {
        fn get_rating(&self) -> i32;
    }

    impl HasRating for Cybernetics { fn get_rating(&self) -> i32 {self.rating}}
    impl HasRating for Relic       { fn get_rating(&self) -> i32 {self.rating}}
    impl HasRating for Weapon {
         fn get_rating(&self) -> i32 {
             match self.main_slot {
                 None     => 0i32,
                 Some(ref num)=> num.rating,
             }
         }
    }
    impl HasRating for Clothes {
        fn get_rating(&self) -> i32 {
            match self.main_slot {
                None     => 0i32,
                Some(ref num)=> num.rating,
            }
        }
    }
    impl HasRating for Accessory {
        fn get_rating(&self) -> i32 {
            match self.main_slot {
                None     => 0i32,
                Some(ref num)=> num.rating,
            }
        }
    }

//===========================================================================================
// Implementations
//===========================================================================================

    impl Player {
        pub fn new(name: String) -> Player {
            Player{ name: name, ..Default::default()}
        }

        pub fn add_toon(&mut self, toon: Toon) {
            let x: Box<Toon> = Box::new(toon);
            self.toons.push(x);
        }
    }

    impl Toon {
        pub fn new(name: String) -> Toon {
            Toon {name: name, ..Default::default()}
        }
    }

    impl Role {
        pub fn new(name: String) -> Role {
            Role {name: name, ..Default::default()}
        }
    }


}
mod query {

}
