use cotli_helper::crusader::*;
use cotli_helper::dps::*;
use cotli_helper::formation::*;
use cotli_helper::user_data::*;

pub fn user_data() -> UserData {
    use self::CrusaderName::*;
    UserData::default()
        // Slot 1
        .add_crusader(TheBushWhacker, Default::default())
        // .add_crusader(RoboRabbit, Default::default())
        // .add_crusader(GrahamTheDriver, Default::default())
        // .add_crusader(WarwickTheWarlock, Default::default())

        // Slot 2
        .add_crusader(JimTheLumberjack, Default::default())
        // .add_crusader(PilotPam, Default::default())
        .add_crusader(VeronicaTheAndroidArcher, Default::default())
        .add_crusader(Arachnobuddy, Default::default())

        // Slot 3
        .add_crusader(EmoWerewolf, Default::default())
        .add_crusader(SallyTheSuccubus, Default::default())
        // .add_crusader(KarenTheCatTeenager, Default::default())

        // Slot 4
        .add_crusader(SashaTheFierceWarrior, Default::default())
        .add_crusader(GroklokTheOrc, Default::default())
        // .add_crusader(MindyTheMime, Default::default())

        // Slot 5
        .add_crusader(TheWashedUpHermit, Default::default())
        .add_crusader(KyleThePartyBro, Default::default())
        // .add_crusader(SerpentKingDraco, Default::default())
        // .add_crusader(HenryTheScaredyGhoul, Default::default())
        .add_crusader(Grandmora, Default::default())

        // Slot 6
        .add_crusader(DetectiveKaine, Default::default())
        // .add_crusader(MisterTheMonkey, Default::default())
        .add_crusader(LarryTheLeprechaun, Default::default())
        // .add_crusader(BernardTheBartender, Default::default())

        // Slot 7
        .add_crusader(ThePrincess, Default::default())
        // .add_crusader(RoboTurkey, Default::default())
        // .add_crusader(RangerRayna, Default::default())
        .add_crusader(BaenarallAngelOfHope, Default::default())

        // Slot 8
        .add_crusader(NatalieDragon, Default::default())
        // .add_crusader(JackOLantern, Default::default())
        .add_crusader(PresidentBillySmithsonian, Default::default())
        // .add_crusader(KarlTheKicker, Default::default())

        // Slot 9
        .add_crusader(JasonMasterOfShadows, Default::default())
        // .add_crusader(PeteTheCarney, Default::default())
        .add_crusader(Broot, Default::default())
        // .add_crusader(PaulThePilgrim, Default::default())

        // Slot 10
        .add_crusader(ArtaxesTheLion, Default::default())
        .add_crusader(DrizzleTheDarkElf, Default::default())
        // .add_crusader(BubbaTheSwimmingOrc, Default::default())
        .add_crusader(SisaronTheDragonSorceress, Default::default())

        // Slot 11
        // .add_crusader(KhouriTheWitchDoctor, Default::default())
        // .add_crusader(MommaKaine, Default::default())
        // .add_crusader(BrogonPrinceOfDragons, Default::default())
        // .add_crusader(TheHalfBloodElf, Default::default())
        // .add_crusader(Foresight, Default::default())

        // Slot 12
        // .add_crusader(DarkGryphon, Default::default())
        // .add_crusader(RockyTheRockstar, Default::default())
        // .add_crusader(MontanaJames, Default::default())
        // .add_crusader(TheDarkHelper, Default::default())

        // Slot 13
        // .add_crusader(SarahTheCollector, Default::default())
        // .add_crusader(TheMetalSoldierette, Default::default())
        // .add_crusader(SnicketteTheSneaky, Default::default())

        // Slot 14
        // .add_crusader(GoldPanda, Default::default())
        // .add_crusader(RoboSanta, Default::default())
        // .add_crusader(LeerionTheRoyalDwarf, Default::default())
        // .add_crusader(KatieTheCupid, Default::default())

        // Slot 15
        // .add_crusader(PrinceSalTheMerman, Default::default())
        // .add_crusader(WendyTheWitch, Default::default())
        // .add_crusader(RobbieRaccoon, Default::default())
        // .add_crusader(PrincessValTheMermaid, Default::default())

        // Slot 16
        // .add_crusader(FirePhoenix, Default::default())
        // .add_crusader(AlanTheArchAngel, Default::default())
        // FrightOTron4000,
        // .add_crusader(Spaceking, Default::default())

        // Slot 17
        // .add_crusader(KingReginaldIV, Default::default())
        // .add_crusader(QueenSiri, Default::default())
        // .add_crusader(MrBogginsTheSubstitute, Default::default())
        // .add_crusader(SquigglesTheClown, Default::default())

        // Slot 18
        // .add_crusader(ThaliaTheThunderKing, Default::default())
        // .add_crusader(FrostyTheSnowman, Default::default())
        // .add_crusader(Littlefoot, Default::default())
        // .add_crusader(CindyTheCheerOrc, Default::default())

        // Slot 19
        // .add_crusader(MerciTheMadWizard, Default::default())
        // .add_crusader(TheBatBillionaire, Default::default())
        // .add_crusader(PetraThePilgrim, Default::default())

        // Slot 20
        // .add_crusader(NateDragon, Default::default())
        // .add_crusader(KizlblypTheAlienTraitor, Default::default())
        // .add_crusader(RoboRudolph, Default::default())

        // Slot 21
        // .add_crusader(TheExterminator, Default::default())
        // .add_crusader(GloriaTheGoodWitch, Default::default())

        // Slot 22
        // .add_crusader(TheShadowQueen, Default::default())
        // .add_crusader(IlsaTheInsaneWizard, Default::default())
}

pub fn default_crusader(name: CrusaderName) -> Crusader {
    user_data()
        .unlocked_crusaders()
        .into_iter()
        .find(|c| c.name == name)
        .unwrap()
        .at_level(Level(200))
}

pub fn worlds_wake<'a>() -> Formation<'a> {
    Formation::empty(vec![
        Coordinate::new(0, 0),
        Coordinate::new(0, 1),
        Coordinate::new(0, 2),
        Coordinate::new(0, 3),
        Coordinate::new(1, 0),
        Coordinate::new(1, 1),
        Coordinate::new(1, 2),
        Coordinate::new(2, 1),
        Coordinate::new(2, 2),
        Coordinate::new(3, 1),
    ])
}
