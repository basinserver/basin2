use cenum::Cenum;
use std::collections::HashMap;
use super::BlockStateContainerImpl;

pub trait BlockStateProperty<T: Send + Sync + 'static>: Sized {
    fn all(&self) -> Vec<T>;
    fn state_from(&self, value: &str) -> Option<T>;
}

pub trait BlockStateContainer: Sized {
    fn all(&self) -> Vec<Self>;
    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self>;
}

pub struct IntProperty {
    pub maximum: u8,
}

impl BlockStateProperty<u8> for IntProperty {
    fn all(&self) -> Vec<u8> {
        (0..=self.maximum).collect()
    }

    fn state_from(&self, value: &str) -> Option<u8> {
        value.parse().ok()
    }
}

impl BlockStateProperty<bool> for bool {
    fn all(&self) -> Vec<bool> {
        vec![false, true]
    }

    fn state_from(&self, value: &str) -> Option<bool> {
        value.parse().ok()
    }
}

#[derive(Default, Debug, Clone)]
pub struct BlockStateNone {
}

impl BlockStateContainerImpl for BlockStateNone {}

impl BlockStateContainer for BlockStateNone {
    fn all(&self) -> Vec<Self> {
        return vec![];
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateNone {})
    }
}

// generated
#[cenum]
#[derive(Copy)]
pub enum Axis {
    X,
    Y,
    Z
}

impl Default for Axis {
    fn default() -> Axis { Axis::X }
}

impl BlockStateProperty<Axis> for Axis {
    fn all(&self) -> Vec<Axis> {
        vec![Axis::X, Axis::Y, Axis::Z]
    }

    fn state_from(&self, value: &str) -> Option<Axis> {
        match value {
            "x" => Some(Axis::X),
            "y" => Some(Axis::Y),
            "z" => Some(Axis::Z),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down
}

impl Default for Facing {
    fn default() -> Facing { Facing::North }
}

impl BlockStateProperty<Facing> for Facing {
    fn all(&self) -> Vec<Facing> {
        vec![Facing::North, Facing::East, Facing::South, Facing::West, Facing::Up, Facing::Down]
    }

    fn state_from(&self, value: &str) -> Option<Facing> {
        match value {
            "north" => Some(Facing::North),
            "east" => Some(Facing::East),
            "south" => Some(Facing::South),
            "west" => Some(Facing::West),
            "up" => Some(Facing::Up),
            "down" => Some(Facing::Down),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Instrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling
}

impl Default for Instrument {
    fn default() -> Instrument { Instrument::Harp }
}

impl BlockStateProperty<Instrument> for Instrument {
    fn all(&self) -> Vec<Instrument> {
        vec![Instrument::Harp, Instrument::Basedrum, Instrument::Snare, Instrument::Hat, Instrument::Bass, Instrument::Flute, Instrument::Bell, Instrument::Guitar, Instrument::Chime, Instrument::Xylophone, Instrument::IronXylophone, Instrument::CowBell, Instrument::Didgeridoo, Instrument::Bit, Instrument::Banjo, Instrument::Pling]
    }

    fn state_from(&self, value: &str) -> Option<Instrument> {
        match value {
            "harp" => Some(Instrument::Harp),
            "basedrum" => Some(Instrument::Basedrum),
            "snare" => Some(Instrument::Snare),
            "hat" => Some(Instrument::Hat),
            "bass" => Some(Instrument::Bass),
            "flute" => Some(Instrument::Flute),
            "bell" => Some(Instrument::Bell),
            "guitar" => Some(Instrument::Guitar),
            "chime" => Some(Instrument::Chime),
            "xylophone" => Some(Instrument::Xylophone),
            "iron_xylophone" => Some(Instrument::IronXylophone),
            "cow_bell" => Some(Instrument::CowBell),
            "didgeridoo" => Some(Instrument::Didgeridoo),
            "bit" => Some(Instrument::Bit),
            "banjo" => Some(Instrument::Banjo),
            "pling" => Some(Instrument::Pling),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum FacingHorizontal {
    North,
    South,
    West,
    East
}

impl Default for FacingHorizontal {
    fn default() -> FacingHorizontal { FacingHorizontal::North }
}

impl BlockStateProperty<FacingHorizontal> for FacingHorizontal {
    fn all(&self) -> Vec<FacingHorizontal> {
        vec![FacingHorizontal::North, FacingHorizontal::South, FacingHorizontal::West, FacingHorizontal::East]
    }

    fn state_from(&self, value: &str) -> Option<FacingHorizontal> {
        match value {
            "north" => Some(FacingHorizontal::North),
            "south" => Some(FacingHorizontal::South),
            "west" => Some(FacingHorizontal::West),
            "east" => Some(FacingHorizontal::East),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Part {
    Head,
    Foot
}

impl Default for Part {
    fn default() -> Part { Part::Head }
}

impl BlockStateProperty<Part> for Part {
    fn all(&self) -> Vec<Part> {
        vec![Part::Head, Part::Foot]
    }

    fn state_from(&self, value: &str) -> Option<Part> {
        match value {
            "head" => Some(Part::Head),
            "foot" => Some(Part::Foot),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum StraightRailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth
}

impl Default for StraightRailShape {
    fn default() -> StraightRailShape { StraightRailShape::NorthSouth }
}

impl BlockStateProperty<StraightRailShape> for StraightRailShape {
    fn all(&self) -> Vec<StraightRailShape> {
        vec![StraightRailShape::NorthSouth, StraightRailShape::EastWest, StraightRailShape::AscendingEast, StraightRailShape::AscendingWest, StraightRailShape::AscendingNorth, StraightRailShape::AscendingSouth]
    }

    fn state_from(&self, value: &str) -> Option<StraightRailShape> {
        match value {
            "north_south" => Some(StraightRailShape::NorthSouth),
            "east_west" => Some(StraightRailShape::EastWest),
            "ascending_east" => Some(StraightRailShape::AscendingEast),
            "ascending_west" => Some(StraightRailShape::AscendingWest),
            "ascending_north" => Some(StraightRailShape::AscendingNorth),
            "ascending_south" => Some(StraightRailShape::AscendingSouth),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Flower {
    Upper,
    Lower
}

impl Default for Flower {
    fn default() -> Flower { Flower::Upper }
}

impl BlockStateProperty<Flower> for Flower {
    fn all(&self) -> Vec<Flower> {
        vec![Flower::Upper, Flower::Lower]
    }

    fn state_from(&self, value: &str) -> Option<Flower> {
        match value {
            "upper" => Some(Flower::Upper),
            "lower" => Some(Flower::Lower),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum PistonType {
    Normal,
    Sticky
}

impl Default for PistonType {
    fn default() -> PistonType { PistonType::Normal }
}

impl BlockStateProperty<PistonType> for PistonType {
    fn all(&self) -> Vec<PistonType> {
        vec![PistonType::Normal, PistonType::Sticky]
    }

    fn state_from(&self, value: &str) -> Option<PistonType> {
        match value {
            "normal" => Some(PistonType::Normal),
            "sticky" => Some(PistonType::Sticky),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Stairs {
    Top,
    Bottom
}

impl Default for Stairs {
    fn default() -> Stairs { Stairs::Top }
}

impl BlockStateProperty<Stairs> for Stairs {
    fn all(&self) -> Vec<Stairs> {
        vec![Stairs::Top, Stairs::Bottom]
    }

    fn state_from(&self, value: &str) -> Option<Stairs> {
        match value {
            "top" => Some(Stairs::Top),
            "bottom" => Some(Stairs::Bottom),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum StairShape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight
}

impl Default for StairShape {
    fn default() -> StairShape { StairShape::Straight }
}

impl BlockStateProperty<StairShape> for StairShape {
    fn all(&self) -> Vec<StairShape> {
        vec![StairShape::Straight, StairShape::InnerLeft, StairShape::InnerRight, StairShape::OuterLeft, StairShape::OuterRight]
    }

    fn state_from(&self, value: &str) -> Option<StairShape> {
        match value {
            "straight" => Some(StairShape::Straight),
            "inner_left" => Some(StairShape::InnerLeft),
            "inner_right" => Some(StairShape::InnerRight),
            "outer_left" => Some(StairShape::OuterLeft),
            "outer_right" => Some(StairShape::OuterRight),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum ChestType {
    Single,
    Left,
    Right
}

impl Default for ChestType {
    fn default() -> ChestType { ChestType::Single }
}

impl BlockStateProperty<ChestType> for ChestType {
    fn all(&self) -> Vec<ChestType> {
        vec![ChestType::Single, ChestType::Left, ChestType::Right]
    }

    fn state_from(&self, value: &str) -> Option<ChestType> {
        match value {
            "single" => Some(ChestType::Single),
            "left" => Some(ChestType::Left),
            "right" => Some(ChestType::Right),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum East {
    Up,
    Side,
    None
}

impl Default for East {
    fn default() -> East { East::Up }
}

impl BlockStateProperty<East> for East {
    fn all(&self) -> Vec<East> {
        vec![East::Up, East::Side, East::None]
    }

    fn state_from(&self, value: &str) -> Option<East> {
        match value {
            "up" => Some(East::Up),
            "side" => Some(East::Side),
            "none" => Some(East::None),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum North {
    Up,
    Side,
    None
}

impl Default for North {
    fn default() -> North { North::Up }
}

impl BlockStateProperty<North> for North {
    fn all(&self) -> Vec<North> {
        vec![North::Up, North::Side, North::None]
    }

    fn state_from(&self, value: &str) -> Option<North> {
        match value {
            "up" => Some(North::Up),
            "side" => Some(North::Side),
            "none" => Some(North::None),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum South {
    Up,
    Side,
    None
}

impl Default for South {
    fn default() -> South { South::Up }
}

impl BlockStateProperty<South> for South {
    fn all(&self) -> Vec<South> {
        vec![South::Up, South::Side, South::None]
    }

    fn state_from(&self, value: &str) -> Option<South> {
        match value {
            "up" => Some(South::Up),
            "side" => Some(South::Side),
            "none" => Some(South::None),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum West {
    Up,
    Side,
    None
}

impl Default for West {
    fn default() -> West { West::Up }
}

impl BlockStateProperty<West> for West {
    fn all(&self) -> Vec<West> {
        vec![West::Up, West::Side, West::None]
    }

    fn state_from(&self, value: &str) -> Option<West> {
        match value {
            "up" => Some(West::Up),
            "side" => Some(West::Side),
            "none" => Some(West::None),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Hinge {
    Left,
    Right
}

impl Default for Hinge {
    fn default() -> Hinge { Hinge::Left }
}

impl BlockStateProperty<Hinge> for Hinge {
    fn all(&self) -> Vec<Hinge> {
        vec![Hinge::Left, Hinge::Right]
    }

    fn state_from(&self, value: &str) -> Option<Hinge> {
        match value {
            "left" => Some(Hinge::Left),
            "right" => Some(Hinge::Right),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum RailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast
}

impl Default for RailShape {
    fn default() -> RailShape { RailShape::NorthSouth }
}

impl BlockStateProperty<RailShape> for RailShape {
    fn all(&self) -> Vec<RailShape> {
        vec![RailShape::NorthSouth, RailShape::EastWest, RailShape::AscendingEast, RailShape::AscendingWest, RailShape::AscendingNorth, RailShape::AscendingSouth, RailShape::SouthEast, RailShape::SouthWest, RailShape::NorthWest, RailShape::NorthEast]
    }

    fn state_from(&self, value: &str) -> Option<RailShape> {
        match value {
            "north_south" => Some(RailShape::NorthSouth),
            "east_west" => Some(RailShape::EastWest),
            "ascending_east" => Some(RailShape::AscendingEast),
            "ascending_west" => Some(RailShape::AscendingWest),
            "ascending_north" => Some(RailShape::AscendingNorth),
            "ascending_south" => Some(RailShape::AscendingSouth),
            "south_east" => Some(RailShape::SouthEast),
            "south_west" => Some(RailShape::SouthWest),
            "north_west" => Some(RailShape::NorthWest),
            "north_east" => Some(RailShape::NorthEast),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Face {
    Floor,
    Wall,
    Ceiling
}

impl Default for Face {
    fn default() -> Face { Face::Floor }
}

impl BlockStateProperty<Face> for Face {
    fn all(&self) -> Vec<Face> {
        vec![Face::Floor, Face::Wall, Face::Ceiling]
    }

    fn state_from(&self, value: &str) -> Option<Face> {
        match value {
            "floor" => Some(Face::Floor),
            "wall" => Some(Face::Wall),
            "ceiling" => Some(Face::Ceiling),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum HorizontalAxis {
    X,
    Z
}

impl Default for HorizontalAxis {
    fn default() -> HorizontalAxis { HorizontalAxis::X }
}

impl BlockStateProperty<HorizontalAxis> for HorizontalAxis {
    fn all(&self) -> Vec<HorizontalAxis> {
        vec![HorizontalAxis::X, HorizontalAxis::Z]
    }

    fn state_from(&self, value: &str) -> Option<HorizontalAxis> {
        match value {
            "x" => Some(HorizontalAxis::X),
            "z" => Some(HorizontalAxis::Z),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum ComparatorMode {
    Compare,
    Subtract
}

impl Default for ComparatorMode {
    fn default() -> ComparatorMode { ComparatorMode::Compare }
}

impl BlockStateProperty<ComparatorMode> for ComparatorMode {
    fn all(&self) -> Vec<ComparatorMode> {
        vec![ComparatorMode::Compare, ComparatorMode::Subtract]
    }

    fn state_from(&self, value: &str) -> Option<ComparatorMode> {
        match value {
            "compare" => Some(ComparatorMode::Compare),
            "subtract" => Some(ComparatorMode::Subtract),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum FacingDownable {
    Down,
    North,
    South,
    West,
    East
}

impl Default for FacingDownable {
    fn default() -> FacingDownable { FacingDownable::Down }
}

impl BlockStateProperty<FacingDownable> for FacingDownable {
    fn all(&self) -> Vec<FacingDownable> {
        vec![FacingDownable::Down, FacingDownable::North, FacingDownable::South, FacingDownable::West, FacingDownable::East]
    }

    fn state_from(&self, value: &str) -> Option<FacingDownable> {
        match value {
            "down" => Some(FacingDownable::Down),
            "north" => Some(FacingDownable::North),
            "south" => Some(FacingDownable::South),
            "west" => Some(FacingDownable::West),
            "east" => Some(FacingDownable::East),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum SlabType {
    Top,
    Bottom,
    Double
}

impl Default for SlabType {
    fn default() -> SlabType { SlabType::Top }
}

impl BlockStateProperty<SlabType> for SlabType {
    fn all(&self) -> Vec<SlabType> {
        vec![SlabType::Top, SlabType::Bottom, SlabType::Double]
    }

    fn state_from(&self, value: &str) -> Option<SlabType> {
        match value {
            "top" => Some(SlabType::Top),
            "bottom" => Some(SlabType::Bottom),
            "double" => Some(SlabType::Double),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Leaves {
    None,
    Small,
    Large
}

impl Default for Leaves {
    fn default() -> Leaves { Leaves::None }
}

impl BlockStateProperty<Leaves> for Leaves {
    fn all(&self) -> Vec<Leaves> {
        vec![Leaves::None, Leaves::Small, Leaves::Large]
    }

    fn state_from(&self, value: &str) -> Option<Leaves> {
        match value {
            "none" => Some(Leaves::None),
            "small" => Some(Leaves::Small),
            "large" => Some(Leaves::Large),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum Attachment {
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall
}

impl Default for Attachment {
    fn default() -> Attachment { Attachment::Floor }
}

impl BlockStateProperty<Attachment> for Attachment {
    fn all(&self) -> Vec<Attachment> {
        vec![Attachment::Floor, Attachment::Ceiling, Attachment::SingleWall, Attachment::DoubleWall]
    }

    fn state_from(&self, value: &str) -> Option<Attachment> {
        match value {
            "floor" => Some(Attachment::Floor),
            "ceiling" => Some(Attachment::Ceiling),
            "single_wall" => Some(Attachment::SingleWall),
            "double_wall" => Some(Attachment::DoubleWall),
            _ => None,
        }
    }
}

#[cenum]
#[derive(Copy)]
pub enum StructureMode {
    Save,
    Load,
    Corner,
    Data
}

impl Default for StructureMode {
    fn default() -> StructureMode { StructureMode::Save }
}

impl BlockStateProperty<StructureMode> for StructureMode {
    fn all(&self) -> Vec<StructureMode> {
        vec![StructureMode::Save, StructureMode::Load, StructureMode::Corner, StructureMode::Data]
    }

    fn state_from(&self, value: &str) -> Option<StructureMode> {
        match value {
            "save" => Some(StructureMode::Save),
            "load" => Some(StructureMode::Load),
            "corner" => Some(StructureMode::Corner),
            "data" => Some(StructureMode::Data),
            _ => None,
        }
    }
}


pub type BlockStateAir = BlockStateNone;

pub type BlockStateStone = BlockStateNone;

pub type BlockStateGranite = BlockStateNone;

pub type BlockStatePolishedGranite = BlockStateNone;

pub type BlockStateDiorite = BlockStateNone;

pub type BlockStatePolishedDiorite = BlockStateNone;

pub type BlockStateAndesite = BlockStateNone;

pub type BlockStatePolishedAndesite = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateGrassBlock {
    pub snowy: bool,
}

impl BlockStateContainerImpl for BlockStateGrassBlock {}

impl BlockStateContainer for BlockStateGrassBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for snowy_i in self.snowy.all() {
        out.push(BlockStateGrassBlock { snowy: snowy_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGrassBlock { snowy: self.snowy.state_from(value.get("snowy")?)? })
    }
}


pub type BlockStateDirt = BlockStateNone;

pub type BlockStateCoarseDirt = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStatePodzol {
    pub snowy: bool,
}

impl BlockStateContainerImpl for BlockStatePodzol {}

impl BlockStateContainer for BlockStatePodzol {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for snowy_i in self.snowy.all() {
        out.push(BlockStatePodzol { snowy: snowy_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePodzol { snowy: self.snowy.state_from(value.get("snowy")?)? })
    }
}


pub type BlockStateCobblestone = BlockStateNone;

pub type BlockStateOakPlanks = BlockStateNone;

pub type BlockStateSprucePlanks = BlockStateNone;

pub type BlockStateBirchPlanks = BlockStateNone;

pub type BlockStateJunglePlanks = BlockStateNone;

pub type BlockStateAcaciaPlanks = BlockStateNone;

pub type BlockStateDarkOakPlanks = BlockStateNone;

pub type BlockStateOakSapling = BlockStateNone;

pub type BlockStateSpruceSapling = BlockStateNone;

pub type BlockStateBirchSapling = BlockStateNone;

pub type BlockStateJungleSapling = BlockStateNone;

pub type BlockStateAcaciaSapling = BlockStateNone;

pub type BlockStateDarkOakSapling = BlockStateNone;

pub type BlockStateBedrock = BlockStateNone;

pub type BlockStateWater = BlockStateNone;

pub type BlockStateLava = BlockStateNone;

pub type BlockStateSand = BlockStateNone;

pub type BlockStateRedSand = BlockStateNone;

pub type BlockStateGravel = BlockStateNone;

pub type BlockStateGoldOre = BlockStateNone;

pub type BlockStateIronOre = BlockStateNone;

pub type BlockStateCoalOre = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateOakLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateOakLog {}

impl BlockStateContainer for BlockStateOakLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateOakLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateSpruceLog {}

impl BlockStateContainer for BlockStateSpruceLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateSpruceLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateBirchLog {}

impl BlockStateContainer for BlockStateBirchLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateBirchLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateJungleLog {}

impl BlockStateContainer for BlockStateJungleLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateJungleLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateAcaciaLog {}

impl BlockStateContainer for BlockStateAcaciaLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateAcaciaLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateDarkOakLog {}

impl BlockStateContainer for BlockStateDarkOakLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateDarkOakLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedSpruceLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedSpruceLog {}

impl BlockStateContainer for BlockStateStrippedSpruceLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedSpruceLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedSpruceLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedBirchLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedBirchLog {}

impl BlockStateContainer for BlockStateStrippedBirchLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedBirchLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedBirchLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedJungleLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedJungleLog {}

impl BlockStateContainer for BlockStateStrippedJungleLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedJungleLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedJungleLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedAcaciaLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedAcaciaLog {}

impl BlockStateContainer for BlockStateStrippedAcaciaLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedAcaciaLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedAcaciaLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedDarkOakLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedDarkOakLog {}

impl BlockStateContainer for BlockStateStrippedDarkOakLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedDarkOakLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedDarkOakLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedOakLog {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedOakLog {}

impl BlockStateContainer for BlockStateStrippedOakLog {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedOakLog { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedOakLog { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateOakWood {}

impl BlockStateContainer for BlockStateOakWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateOakWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateSpruceWood {}

impl BlockStateContainer for BlockStateSpruceWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateSpruceWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateBirchWood {}

impl BlockStateContainer for BlockStateBirchWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateBirchWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateJungleWood {}

impl BlockStateContainer for BlockStateJungleWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateJungleWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateAcaciaWood {}

impl BlockStateContainer for BlockStateAcaciaWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateAcaciaWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateDarkOakWood {}

impl BlockStateContainer for BlockStateDarkOakWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateDarkOakWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedOakWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedOakWood {}

impl BlockStateContainer for BlockStateStrippedOakWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedOakWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedOakWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedSpruceWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedSpruceWood {}

impl BlockStateContainer for BlockStateStrippedSpruceWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedSpruceWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedSpruceWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedBirchWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedBirchWood {}

impl BlockStateContainer for BlockStateStrippedBirchWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedBirchWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedBirchWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedJungleWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedJungleWood {}

impl BlockStateContainer for BlockStateStrippedJungleWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedJungleWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedJungleWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedAcaciaWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedAcaciaWood {}

impl BlockStateContainer for BlockStateStrippedAcaciaWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedAcaciaWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedAcaciaWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStrippedDarkOakWood {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateStrippedDarkOakWood {}

impl BlockStateContainer for BlockStateStrippedDarkOakWood {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateStrippedDarkOakWood { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStrippedDarkOakWood { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakLeaves {
    pub persistent: bool,
}

impl BlockStateContainerImpl for BlockStateOakLeaves {}

impl BlockStateContainer for BlockStateOakLeaves {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for persistent_i in self.persistent.all() {
        out.push(BlockStateOakLeaves { persistent: persistent_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakLeaves { persistent: self.persistent.state_from(value.get("persistent")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceLeaves {
    pub persistent: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceLeaves {}

impl BlockStateContainer for BlockStateSpruceLeaves {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for persistent_i in self.persistent.all() {
        out.push(BlockStateSpruceLeaves { persistent: persistent_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceLeaves { persistent: self.persistent.state_from(value.get("persistent")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchLeaves {
    pub persistent: bool,
}

impl BlockStateContainerImpl for BlockStateBirchLeaves {}

impl BlockStateContainer for BlockStateBirchLeaves {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for persistent_i in self.persistent.all() {
        out.push(BlockStateBirchLeaves { persistent: persistent_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchLeaves { persistent: self.persistent.state_from(value.get("persistent")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleLeaves {
    pub persistent: bool,
}

impl BlockStateContainerImpl for BlockStateJungleLeaves {}

impl BlockStateContainer for BlockStateJungleLeaves {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for persistent_i in self.persistent.all() {
        out.push(BlockStateJungleLeaves { persistent: persistent_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleLeaves { persistent: self.persistent.state_from(value.get("persistent")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaLeaves {
    pub persistent: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaLeaves {}

impl BlockStateContainer for BlockStateAcaciaLeaves {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for persistent_i in self.persistent.all() {
        out.push(BlockStateAcaciaLeaves { persistent: persistent_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaLeaves { persistent: self.persistent.state_from(value.get("persistent")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakLeaves {
    pub persistent: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakLeaves {}

impl BlockStateContainer for BlockStateDarkOakLeaves {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for persistent_i in self.persistent.all() {
        out.push(BlockStateDarkOakLeaves { persistent: persistent_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakLeaves { persistent: self.persistent.state_from(value.get("persistent")?)? })
    }
}


pub type BlockStateSponge = BlockStateNone;

pub type BlockStateWetSponge = BlockStateNone;

pub type BlockStateGlass = BlockStateNone;

pub type BlockStateLapisOre = BlockStateNone;

pub type BlockStateLapisBlock = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateDispenser {
    pub facing: Facing,
    pub triggered: bool,
}

impl BlockStateContainerImpl for BlockStateDispenser {}

impl BlockStateContainer for BlockStateDispenser {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for triggered_i in self.triggered.all() {
        out.push(BlockStateDispenser { facing: facing_i,triggered: triggered_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDispenser { facing: self.facing.state_from(value.get("facing")?)?,triggered: self.triggered.state_from(value.get("triggered")?)? })
    }
}


pub type BlockStateSandstone = BlockStateNone;

pub type BlockStateChiseledSandstone = BlockStateNone;

pub type BlockStateCutSandstone = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateNoteBlock {
    pub instrument: Instrument,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateNoteBlock {}

impl BlockStateContainer for BlockStateNoteBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for instrument_i in self.instrument.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateNoteBlock { instrument: instrument_i,powered: powered_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateNoteBlock { instrument: self.instrument.state_from(value.get("instrument")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateWhiteBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateWhiteBed {}

impl BlockStateContainer for BlockStateWhiteBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateWhiteBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateWhiteBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOrangeBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateOrangeBed {}

impl BlockStateContainer for BlockStateOrangeBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateOrangeBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOrangeBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMagentaBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateMagentaBed {}

impl BlockStateContainer for BlockStateMagentaBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateMagentaBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMagentaBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightBlueBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateLightBlueBed {}

impl BlockStateContainer for BlockStateLightBlueBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateLightBlueBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightBlueBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateYellowBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateYellowBed {}

impl BlockStateContainer for BlockStateYellowBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateYellowBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateYellowBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLimeBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateLimeBed {}

impl BlockStateContainer for BlockStateLimeBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateLimeBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLimeBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePinkBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStatePinkBed {}

impl BlockStateContainer for BlockStatePinkBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStatePinkBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePinkBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGrayBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateGrayBed {}

impl BlockStateContainer for BlockStateGrayBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateGrayBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGrayBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightGrayBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateLightGrayBed {}

impl BlockStateContainer for BlockStateLightGrayBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateLightGrayBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightGrayBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCyanBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateCyanBed {}

impl BlockStateContainer for BlockStateCyanBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateCyanBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCyanBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpleBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStatePurpleBed {}

impl BlockStateContainer for BlockStatePurpleBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStatePurpleBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpleBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlueBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateBlueBed {}

impl BlockStateContainer for BlockStateBlueBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateBlueBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlueBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrownBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateBrownBed {}

impl BlockStateContainer for BlockStateBrownBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateBrownBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrownBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGreenBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateGreenBed {}

impl BlockStateContainer for BlockStateGreenBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateGreenBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGreenBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateRedBed {}

impl BlockStateContainer for BlockStateRedBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateRedBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlackBed {
    pub facing: FacingHorizontal,
    pub occupied: bool,
    pub part: Part,
}

impl BlockStateContainerImpl for BlockStateBlackBed {}

impl BlockStateContainer for BlockStateBlackBed {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for occupied_i in self.occupied.all() {
        
    for part_i in self.part.all() {
        out.push(BlockStateBlackBed { facing: facing_i,occupied: occupied_i,part: part_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlackBed { facing: self.facing.state_from(value.get("facing")?)?,occupied: self.occupied.state_from(value.get("occupied")?)?,part: self.part.state_from(value.get("part")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePoweredRail {
    pub powered: bool,
    pub shape: StraightRailShape,
}

impl BlockStateContainerImpl for BlockStatePoweredRail {}

impl BlockStateContainer for BlockStatePoweredRail {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        
    for shape_i in self.shape.all() {
        out.push(BlockStatePoweredRail { powered: powered_i,shape: shape_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePoweredRail { powered: self.powered.state_from(value.get("powered")?)?,shape: self.shape.state_from(value.get("shape")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDetectorRail {
    pub powered: bool,
    pub shape: StraightRailShape,
}

impl BlockStateContainerImpl for BlockStateDetectorRail {}

impl BlockStateContainer for BlockStateDetectorRail {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        
    for shape_i in self.shape.all() {
        out.push(BlockStateDetectorRail { powered: powered_i,shape: shape_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDetectorRail { powered: self.powered.state_from(value.get("powered")?)?,shape: self.shape.state_from(value.get("shape")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStickyPiston {
    pub extended: bool,
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateStickyPiston {}

impl BlockStateContainer for BlockStateStickyPiston {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for extended_i in self.extended.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStateStickyPiston { extended: extended_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStickyPiston { extended: self.extended.state_from(value.get("extended")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateCobweb = BlockStateNone;

pub type BlockStateGrass = BlockStateNone;

pub type BlockStateFern = BlockStateNone;

pub type BlockStateDeadBush = BlockStateNone;

pub type BlockStateSeagrass = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateTallSeagrass {
    pub half: Flower,
}

impl BlockStateContainerImpl for BlockStateTallSeagrass {}

impl BlockStateContainer for BlockStateTallSeagrass {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for half_i in self.half.all() {
        out.push(BlockStateTallSeagrass { half: half_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTallSeagrass { half: self.half.state_from(value.get("half")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePiston {
    pub extended: bool,
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStatePiston {}

impl BlockStateContainer for BlockStatePiston {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for extended_i in self.extended.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePiston { extended: extended_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePiston { extended: self.extended.state_from(value.get("extended")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePistonHead {
    pub facing: Facing,
    pub short: bool,
    pub type_: PistonType,
}

impl BlockStateContainerImpl for BlockStatePistonHead {}

impl BlockStateContainer for BlockStatePistonHead {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for short_i in self.short.all() {
        
    for type_i in self.type_.all() {
        out.push(BlockStatePistonHead { facing: facing_i,short: short_i,type_: type_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePistonHead { facing: self.facing.state_from(value.get("facing")?)?,short: self.short.state_from(value.get("short")?)?,type_: self.type_.state_from(value.get("type")?)? })
    }
}


pub type BlockStateWhiteWool = BlockStateNone;

pub type BlockStateOrangeWool = BlockStateNone;

pub type BlockStateMagentaWool = BlockStateNone;

pub type BlockStateLightBlueWool = BlockStateNone;

pub type BlockStateYellowWool = BlockStateNone;

pub type BlockStateLimeWool = BlockStateNone;

pub type BlockStatePinkWool = BlockStateNone;

pub type BlockStateGrayWool = BlockStateNone;

pub type BlockStateLightGrayWool = BlockStateNone;

pub type BlockStateCyanWool = BlockStateNone;

pub type BlockStatePurpleWool = BlockStateNone;

pub type BlockStateBlueWool = BlockStateNone;

pub type BlockStateBrownWool = BlockStateNone;

pub type BlockStateGreenWool = BlockStateNone;

pub type BlockStateRedWool = BlockStateNone;

pub type BlockStateBlackWool = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateMovingPiston {
    pub facing: Facing,
    pub type_: PistonType,
}

impl BlockStateContainerImpl for BlockStateMovingPiston {}

impl BlockStateContainer for BlockStateMovingPiston {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for type_i in self.type_.all() {
        out.push(BlockStateMovingPiston { facing: facing_i,type_: type_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMovingPiston { facing: self.facing.state_from(value.get("facing")?)?,type_: self.type_.state_from(value.get("type")?)? })
    }
}


pub type BlockStateDandelion = BlockStateNone;

pub type BlockStatePoppy = BlockStateNone;

pub type BlockStateBlueOrchid = BlockStateNone;

pub type BlockStateAllium = BlockStateNone;

pub type BlockStateAzureBluet = BlockStateNone;

pub type BlockStateRedTulip = BlockStateNone;

pub type BlockStateOrangeTulip = BlockStateNone;

pub type BlockStateWhiteTulip = BlockStateNone;

pub type BlockStatePinkTulip = BlockStateNone;

pub type BlockStateOxeyeDaisy = BlockStateNone;

pub type BlockStateCornflower = BlockStateNone;

pub type BlockStateWitherRose = BlockStateNone;

pub type BlockStateLilyOfTheValley = BlockStateNone;

pub type BlockStateBrownMushroom = BlockStateNone;

pub type BlockStateRedMushroom = BlockStateNone;

pub type BlockStateGoldBlock = BlockStateNone;

pub type BlockStateIronBlock = BlockStateNone;

pub type BlockStateBricks = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateTnt {
    pub unstable: bool,
}

impl BlockStateContainerImpl for BlockStateTnt {}

impl BlockStateContainer for BlockStateTnt {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for unstable_i in self.unstable.all() {
        out.push(BlockStateTnt { unstable: unstable_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTnt { unstable: self.unstable.state_from(value.get("unstable")?)? })
    }
}


pub type BlockStateBookshelf = BlockStateNone;

pub type BlockStateMossyCobblestone = BlockStateNone;

pub type BlockStateObsidian = BlockStateNone;

pub type BlockStateTorch = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateWallTorch {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateWallTorch {}

impl BlockStateContainer for BlockStateWallTorch {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateWallTorch { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateWallTorch { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateFire {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateFire {}

impl BlockStateContainer for BlockStateFire {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateFire { east: east_i,north: north_i,south: south_i,up: up_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateFire { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


pub type BlockStateSpawner = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateOakStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateOakStairs {}

impl BlockStateContainer for BlockStateOakStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateOakStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateChest {
    pub facing: FacingHorizontal,
    pub type_: ChestType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateChest {}

impl BlockStateContainer for BlockStateChest {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateChest { facing: facing_i,type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateChest { facing: self.facing.state_from(value.get("facing")?)?,type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedstoneWire {
    pub east: East,
    pub north: North,
    pub south: South,
    pub west: West,
}

impl BlockStateContainerImpl for BlockStateRedstoneWire {}

impl BlockStateContainer for BlockStateRedstoneWire {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateRedstoneWire { east: east_i,north: north_i,south: south_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedstoneWire { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


pub type BlockStateDiamondOre = BlockStateNone;

pub type BlockStateDiamondBlock = BlockStateNone;

pub type BlockStateCraftingTable = BlockStateNone;

pub type BlockStateWheat = BlockStateNone;

pub type BlockStateFarmland = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateFurnace {
    pub facing: FacingHorizontal,
    pub lit: bool,
}

impl BlockStateContainerImpl for BlockStateFurnace {}

impl BlockStateContainer for BlockStateFurnace {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for lit_i in self.lit.all() {
        out.push(BlockStateFurnace { facing: facing_i,lit: lit_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateFurnace { facing: self.facing.state_from(value.get("facing")?)?,lit: self.lit.state_from(value.get("lit")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakSign {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateOakSign {}

impl BlockStateContainer for BlockStateOakSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateOakSign { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakSign { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceSign {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceSign {}

impl BlockStateContainer for BlockStateSpruceSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSpruceSign { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceSign { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchSign {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBirchSign {}

impl BlockStateContainer for BlockStateBirchSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBirchSign { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchSign { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaSign {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaSign {}

impl BlockStateContainer for BlockStateAcaciaSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateAcaciaSign { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaSign { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleSign {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateJungleSign {}

impl BlockStateContainer for BlockStateJungleSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateJungleSign { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleSign { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakSign {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakSign {}

impl BlockStateContainer for BlockStateDarkOakSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDarkOakSign { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakSign { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakDoor {
    pub facing: FacingHorizontal,
    pub half: Flower,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateOakDoor {}

impl BlockStateContainer for BlockStateOakDoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for hinge_i in self.hinge.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateOakDoor { facing: facing_i,half: half_i,hinge: hinge_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakDoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,hinge: self.hinge.state_from(value.get("hinge")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLadder {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateLadder {}

impl BlockStateContainer for BlockStateLadder {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateLadder { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLadder { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRail {
    pub shape: RailShape,
}

impl BlockStateContainerImpl for BlockStateRail {}

impl BlockStateContainer for BlockStateRail {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for shape_i in self.shape.all() {
        out.push(BlockStateRail { shape: shape_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRail { shape: self.shape.state_from(value.get("shape")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCobblestoneStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateCobblestoneStairs {}

impl BlockStateContainer for BlockStateCobblestoneStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateCobblestoneStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCobblestoneStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakWallSign {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateOakWallSign {}

impl BlockStateContainer for BlockStateOakWallSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateOakWallSign { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakWallSign { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceWallSign {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceWallSign {}

impl BlockStateContainer for BlockStateSpruceWallSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSpruceWallSign { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceWallSign { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchWallSign {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBirchWallSign {}

impl BlockStateContainer for BlockStateBirchWallSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBirchWallSign { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchWallSign { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaWallSign {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaWallSign {}

impl BlockStateContainer for BlockStateAcaciaWallSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateAcaciaWallSign { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaWallSign { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleWallSign {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateJungleWallSign {}

impl BlockStateContainer for BlockStateJungleWallSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateJungleWallSign { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleWallSign { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakWallSign {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakWallSign {}

impl BlockStateContainer for BlockStateDarkOakWallSign {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDarkOakWallSign { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakWallSign { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLever {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateLever {}

impl BlockStateContainer for BlockStateLever {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateLever { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLever { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStonePressurePlate {
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateStonePressurePlate {}

impl BlockStateContainer for BlockStateStonePressurePlate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        out.push(BlockStateStonePressurePlate { powered: powered_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStonePressurePlate { powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateIronDoor {
    pub facing: FacingHorizontal,
    pub half: Flower,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateIronDoor {}

impl BlockStateContainer for BlockStateIronDoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for hinge_i in self.hinge.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateIronDoor { facing: facing_i,half: half_i,hinge: hinge_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateIronDoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,hinge: self.hinge.state_from(value.get("hinge")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakPressurePlate {
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateOakPressurePlate {}

impl BlockStateContainer for BlockStateOakPressurePlate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        out.push(BlockStateOakPressurePlate { powered: powered_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakPressurePlate { powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSprucePressurePlate {
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateSprucePressurePlate {}

impl BlockStateContainer for BlockStateSprucePressurePlate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        out.push(BlockStateSprucePressurePlate { powered: powered_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSprucePressurePlate { powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchPressurePlate {
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateBirchPressurePlate {}

impl BlockStateContainer for BlockStateBirchPressurePlate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        out.push(BlockStateBirchPressurePlate { powered: powered_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchPressurePlate { powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJunglePressurePlate {
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateJunglePressurePlate {}

impl BlockStateContainer for BlockStateJunglePressurePlate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        out.push(BlockStateJunglePressurePlate { powered: powered_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJunglePressurePlate { powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaPressurePlate {
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaPressurePlate {}

impl BlockStateContainer for BlockStateAcaciaPressurePlate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        out.push(BlockStateAcaciaPressurePlate { powered: powered_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaPressurePlate { powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakPressurePlate {
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakPressurePlate {}

impl BlockStateContainer for BlockStateDarkOakPressurePlate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        out.push(BlockStateDarkOakPressurePlate { powered: powered_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakPressurePlate { powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedstoneOre {
    pub lit: bool,
}

impl BlockStateContainerImpl for BlockStateRedstoneOre {}

impl BlockStateContainer for BlockStateRedstoneOre {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for lit_i in self.lit.all() {
        out.push(BlockStateRedstoneOre { lit: lit_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedstoneOre { lit: self.lit.state_from(value.get("lit")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedstoneTorch {
    pub lit: bool,
}

impl BlockStateContainerImpl for BlockStateRedstoneTorch {}

impl BlockStateContainer for BlockStateRedstoneTorch {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for lit_i in self.lit.all() {
        out.push(BlockStateRedstoneTorch { lit: lit_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedstoneTorch { lit: self.lit.state_from(value.get("lit")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedstoneWallTorch {
    pub facing: FacingHorizontal,
    pub lit: bool,
}

impl BlockStateContainerImpl for BlockStateRedstoneWallTorch {}

impl BlockStateContainer for BlockStateRedstoneWallTorch {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for lit_i in self.lit.all() {
        out.push(BlockStateRedstoneWallTorch { facing: facing_i,lit: lit_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedstoneWallTorch { facing: self.facing.state_from(value.get("facing")?)?,lit: self.lit.state_from(value.get("lit")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStoneButton {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateStoneButton {}

impl BlockStateContainer for BlockStateStoneButton {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateStoneButton { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStoneButton { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


pub type BlockStateSnow = BlockStateNone;

pub type BlockStateIce = BlockStateNone;

pub type BlockStateSnowBlock = BlockStateNone;

pub type BlockStateCactus = BlockStateNone;

pub type BlockStateClay = BlockStateNone;

pub type BlockStateSugarCane = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateJukebox {
    pub has_record: bool,
}

impl BlockStateContainerImpl for BlockStateJukebox {}

impl BlockStateContainer for BlockStateJukebox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for has_record_i in self.has_record.all() {
        out.push(BlockStateJukebox { has_record: has_record_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJukebox { has_record: self.has_record.state_from(value.get("has_record")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateOakFence {}

impl BlockStateContainer for BlockStateOakFence {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateOakFence { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakFence { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


pub type BlockStatePumpkin = BlockStateNone;

pub type BlockStateNetherrack = BlockStateNone;

pub type BlockStateSoulSand = BlockStateNone;

pub type BlockStateGlowstone = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateNetherPortal {
    pub axis: HorizontalAxis,
}

impl BlockStateContainerImpl for BlockStateNetherPortal {}

impl BlockStateContainer for BlockStateNetherPortal {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateNetherPortal { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateNetherPortal { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCarvedPumpkin {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateCarvedPumpkin {}

impl BlockStateContainer for BlockStateCarvedPumpkin {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateCarvedPumpkin { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCarvedPumpkin { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJackOLantern {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateJackOLantern {}

impl BlockStateContainer for BlockStateJackOLantern {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateJackOLantern { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJackOLantern { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateCake = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateRepeater {
    pub facing: FacingHorizontal,
    pub locked: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateRepeater {}

impl BlockStateContainer for BlockStateRepeater {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for locked_i in self.locked.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateRepeater { facing: facing_i,locked: locked_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRepeater { facing: self.facing.state_from(value.get("facing")?)?,locked: self.locked.state_from(value.get("locked")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


pub type BlockStateWhiteStainedGlass = BlockStateNone;

pub type BlockStateOrangeStainedGlass = BlockStateNone;

pub type BlockStateMagentaStainedGlass = BlockStateNone;

pub type BlockStateLightBlueStainedGlass = BlockStateNone;

pub type BlockStateYellowStainedGlass = BlockStateNone;

pub type BlockStateLimeStainedGlass = BlockStateNone;

pub type BlockStatePinkStainedGlass = BlockStateNone;

pub type BlockStateGrayStainedGlass = BlockStateNone;

pub type BlockStateLightGrayStainedGlass = BlockStateNone;

pub type BlockStateCyanStainedGlass = BlockStateNone;

pub type BlockStatePurpleStainedGlass = BlockStateNone;

pub type BlockStateBlueStainedGlass = BlockStateNone;

pub type BlockStateBrownStainedGlass = BlockStateNone;

pub type BlockStateGreenStainedGlass = BlockStateNone;

pub type BlockStateRedStainedGlass = BlockStateNone;

pub type BlockStateBlackStainedGlass = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateOakTrapdoor {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateOakTrapdoor {}

impl BlockStateContainer for BlockStateOakTrapdoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateOakTrapdoor { facing: facing_i,half: half_i,open: open_i,powered: powered_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakTrapdoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceTrapdoor {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceTrapdoor {}

impl BlockStateContainer for BlockStateSpruceTrapdoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSpruceTrapdoor { facing: facing_i,half: half_i,open: open_i,powered: powered_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceTrapdoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchTrapdoor {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBirchTrapdoor {}

impl BlockStateContainer for BlockStateBirchTrapdoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBirchTrapdoor { facing: facing_i,half: half_i,open: open_i,powered: powered_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchTrapdoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleTrapdoor {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateJungleTrapdoor {}

impl BlockStateContainer for BlockStateJungleTrapdoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateJungleTrapdoor { facing: facing_i,half: half_i,open: open_i,powered: powered_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleTrapdoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaTrapdoor {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaTrapdoor {}

impl BlockStateContainer for BlockStateAcaciaTrapdoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateAcaciaTrapdoor { facing: facing_i,half: half_i,open: open_i,powered: powered_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaTrapdoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakTrapdoor {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakTrapdoor {}

impl BlockStateContainer for BlockStateDarkOakTrapdoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDarkOakTrapdoor { facing: facing_i,half: half_i,open: open_i,powered: powered_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakTrapdoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateStoneBricks = BlockStateNone;

pub type BlockStateMossyStoneBricks = BlockStateNone;

pub type BlockStateCrackedStoneBricks = BlockStateNone;

pub type BlockStateChiseledStoneBricks = BlockStateNone;

pub type BlockStateInfestedStone = BlockStateNone;

pub type BlockStateInfestedCobblestone = BlockStateNone;

pub type BlockStateInfestedStoneBricks = BlockStateNone;

pub type BlockStateInfestedMossyStoneBricks = BlockStateNone;

pub type BlockStateInfestedCrackedStoneBricks = BlockStateNone;

pub type BlockStateInfestedChiseledStoneBricks = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateBrownMushroomBlock {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateBrownMushroomBlock {}

impl BlockStateContainer for BlockStateBrownMushroomBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for down_i in self.down.all() {
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateBrownMushroomBlock { down: down_i,east: east_i,north: north_i,south: south_i,up: up_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrownMushroomBlock { down: self.down.state_from(value.get("down")?)?,east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedMushroomBlock {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateRedMushroomBlock {}

impl BlockStateContainer for BlockStateRedMushroomBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for down_i in self.down.all() {
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateRedMushroomBlock { down: down_i,east: east_i,north: north_i,south: south_i,up: up_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedMushroomBlock { down: self.down.state_from(value.get("down")?)?,east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMushroomStem {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateMushroomStem {}

impl BlockStateContainer for BlockStateMushroomStem {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for down_i in self.down.all() {
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateMushroomStem { down: down_i,east: east_i,north: north_i,south: south_i,up: up_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMushroomStem { down: self.down.state_from(value.get("down")?)?,east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateIronBars {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateIronBars {}

impl BlockStateContainer for BlockStateIronBars {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateIronBars { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateIronBars { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateGlassPane {}

impl BlockStateContainer for BlockStateGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


pub type BlockStateMelon = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateAttachedPumpkinStem {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateAttachedPumpkinStem {}

impl BlockStateContainer for BlockStateAttachedPumpkinStem {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateAttachedPumpkinStem { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAttachedPumpkinStem { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAttachedMelonStem {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateAttachedMelonStem {}

impl BlockStateContainer for BlockStateAttachedMelonStem {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateAttachedMelonStem { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAttachedMelonStem { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStatePumpkinStem = BlockStateNone;

pub type BlockStateMelonStem = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateVine {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateVine {}

impl BlockStateContainer for BlockStateVine {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateVine { east: east_i,north: north_i,south: south_i,up: up_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateVine { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakFenceGate {
    pub facing: FacingHorizontal,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateOakFenceGate {}

impl BlockStateContainer for BlockStateOakFenceGate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for in_wall_i in self.in_wall.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateOakFenceGate { facing: facing_i,in_wall: in_wall_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakFenceGate { facing: self.facing.state_from(value.get("facing")?)?,in_wall: self.in_wall.state_from(value.get("in_wall")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrickStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBrickStairs {}

impl BlockStateContainer for BlockStateBrickStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBrickStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrickStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStoneBrickStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateStoneBrickStairs {}

impl BlockStateContainer for BlockStateStoneBrickStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateStoneBrickStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStoneBrickStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMycelium {
    pub snowy: bool,
}

impl BlockStateContainerImpl for BlockStateMycelium {}

impl BlockStateContainer for BlockStateMycelium {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for snowy_i in self.snowy.all() {
        out.push(BlockStateMycelium { snowy: snowy_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMycelium { snowy: self.snowy.state_from(value.get("snowy")?)? })
    }
}


pub type BlockStateLilyPad = BlockStateNone;

pub type BlockStateNetherBricks = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateNetherBrickFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateNetherBrickFence {}

impl BlockStateContainer for BlockStateNetherBrickFence {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateNetherBrickFence { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateNetherBrickFence { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateNetherBrickStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateNetherBrickStairs {}

impl BlockStateContainer for BlockStateNetherBrickStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateNetherBrickStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateNetherBrickStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateNetherWart = BlockStateNone;

pub type BlockStateEnchantingTable = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateBrewingStand {
    pub has_bottle_0: bool,
    pub has_bottle_1: bool,
    pub has_bottle_2: bool,
}

impl BlockStateContainerImpl for BlockStateBrewingStand {}

impl BlockStateContainer for BlockStateBrewingStand {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for has_bottle_0_i in self.has_bottle_0.all() {
        
    for has_bottle_1_i in self.has_bottle_1.all() {
        
    for has_bottle_2_i in self.has_bottle_2.all() {
        out.push(BlockStateBrewingStand { has_bottle_0: has_bottle_0_i,has_bottle_1: has_bottle_1_i,has_bottle_2: has_bottle_2_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrewingStand { has_bottle_0: self.has_bottle_0.state_from(value.get("has_bottle_0")?)?,has_bottle_1: self.has_bottle_1.state_from(value.get("has_bottle_1")?)?,has_bottle_2: self.has_bottle_2.state_from(value.get("has_bottle_2")?)? })
    }
}


pub type BlockStateCauldron = BlockStateNone;

pub type BlockStateEndPortal = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateEndPortalFrame {
    pub eye: bool,
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateEndPortalFrame {}

impl BlockStateContainer for BlockStateEndPortalFrame {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for eye_i in self.eye.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStateEndPortalFrame { eye: eye_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateEndPortalFrame { eye: self.eye.state_from(value.get("eye")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateEndStone = BlockStateNone;

pub type BlockStateDragonEgg = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateRedstoneLamp {
    pub lit: bool,
}

impl BlockStateContainerImpl for BlockStateRedstoneLamp {}

impl BlockStateContainer for BlockStateRedstoneLamp {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for lit_i in self.lit.all() {
        out.push(BlockStateRedstoneLamp { lit: lit_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedstoneLamp { lit: self.lit.state_from(value.get("lit")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCocoa {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateCocoa {}

impl BlockStateContainer for BlockStateCocoa {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateCocoa { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCocoa { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSandstoneStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSandstoneStairs {}

impl BlockStateContainer for BlockStateSandstoneStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSandstoneStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSandstoneStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateEmeraldOre = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateEnderChest {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateEnderChest {}

impl BlockStateContainer for BlockStateEnderChest {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateEnderChest { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateEnderChest { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateTripwireHook {
    pub attached: bool,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateTripwireHook {}

impl BlockStateContainer for BlockStateTripwireHook {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for attached_i in self.attached.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateTripwireHook { attached: attached_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTripwireHook { attached: self.attached.state_from(value.get("attached")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateTripwire {
    pub attached: bool,
    pub disarmed: bool,
    pub east: bool,
    pub north: bool,
    pub powered: bool,
    pub south: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateTripwire {}

impl BlockStateContainer for BlockStateTripwire {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for attached_i in self.attached.all() {
        
    for disarmed_i in self.disarmed.all() {
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for powered_i in self.powered.all() {
        
    for south_i in self.south.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateTripwire { attached: attached_i,disarmed: disarmed_i,east: east_i,north: north_i,powered: powered_i,south: south_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTripwire { attached: self.attached.state_from(value.get("attached")?)?,disarmed: self.disarmed.state_from(value.get("disarmed")?)?,east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,powered: self.powered.state_from(value.get("powered")?)?,south: self.south.state_from(value.get("south")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


pub type BlockStateEmeraldBlock = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceStairs {}

impl BlockStateContainer for BlockStateSpruceStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSpruceStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBirchStairs {}

impl BlockStateContainer for BlockStateBirchStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBirchStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateJungleStairs {}

impl BlockStateContainer for BlockStateJungleStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateJungleStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCommandBlock {
    pub conditional: bool,
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateCommandBlock {}

impl BlockStateContainer for BlockStateCommandBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for conditional_i in self.conditional.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStateCommandBlock { conditional: conditional_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCommandBlock { conditional: self.conditional.state_from(value.get("conditional")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateBeacon = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateCobblestoneWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateCobblestoneWall {}

impl BlockStateContainer for BlockStateCobblestoneWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateCobblestoneWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCobblestoneWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMossyCobblestoneWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateMossyCobblestoneWall {}

impl BlockStateContainer for BlockStateMossyCobblestoneWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateMossyCobblestoneWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMossyCobblestoneWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


pub type BlockStateFlowerPot = BlockStateNone;

pub type BlockStatePottedOakSapling = BlockStateNone;

pub type BlockStatePottedSpruceSapling = BlockStateNone;

pub type BlockStatePottedBirchSapling = BlockStateNone;

pub type BlockStatePottedJungleSapling = BlockStateNone;

pub type BlockStatePottedAcaciaSapling = BlockStateNone;

pub type BlockStatePottedDarkOakSapling = BlockStateNone;

pub type BlockStatePottedFern = BlockStateNone;

pub type BlockStatePottedDandelion = BlockStateNone;

pub type BlockStatePottedPoppy = BlockStateNone;

pub type BlockStatePottedBlueOrchid = BlockStateNone;

pub type BlockStatePottedAllium = BlockStateNone;

pub type BlockStatePottedAzureBluet = BlockStateNone;

pub type BlockStatePottedRedTulip = BlockStateNone;

pub type BlockStatePottedOrangeTulip = BlockStateNone;

pub type BlockStatePottedWhiteTulip = BlockStateNone;

pub type BlockStatePottedPinkTulip = BlockStateNone;

pub type BlockStatePottedOxeyeDaisy = BlockStateNone;

pub type BlockStatePottedCornflower = BlockStateNone;

pub type BlockStatePottedLilyOfTheValley = BlockStateNone;

pub type BlockStatePottedWitherRose = BlockStateNone;

pub type BlockStatePottedRedMushroom = BlockStateNone;

pub type BlockStatePottedBrownMushroom = BlockStateNone;

pub type BlockStatePottedDeadBush = BlockStateNone;

pub type BlockStatePottedCactus = BlockStateNone;

pub type BlockStateCarrots = BlockStateNone;

pub type BlockStatePotatoes = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateOakButton {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateOakButton {}

impl BlockStateContainer for BlockStateOakButton {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateOakButton { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakButton { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceButton {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceButton {}

impl BlockStateContainer for BlockStateSpruceButton {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateSpruceButton { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceButton { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchButton {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateBirchButton {}

impl BlockStateContainer for BlockStateBirchButton {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateBirchButton { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchButton { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleButton {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateJungleButton {}

impl BlockStateContainer for BlockStateJungleButton {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateJungleButton { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleButton { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaButton {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaButton {}

impl BlockStateContainer for BlockStateAcaciaButton {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateAcaciaButton { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaButton { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakButton {
    pub face: Face,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakButton {}

impl BlockStateContainer for BlockStateDarkOakButton {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateDarkOakButton { face: face_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakButton { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


pub type BlockStateSkeletonSkull = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateSkeletonWallSkull {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateSkeletonWallSkull {}

impl BlockStateContainer for BlockStateSkeletonWallSkull {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateSkeletonWallSkull { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSkeletonWallSkull { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateWitherSkeletonSkull = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateWitherSkeletonWallSkull {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateWitherSkeletonWallSkull {}

impl BlockStateContainer for BlockStateWitherSkeletonWallSkull {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateWitherSkeletonWallSkull { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateWitherSkeletonWallSkull { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateZombieHead = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateZombieWallHead {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateZombieWallHead {}

impl BlockStateContainer for BlockStateZombieWallHead {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateZombieWallHead { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateZombieWallHead { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStatePlayerHead = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStatePlayerWallHead {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStatePlayerWallHead {}

impl BlockStateContainer for BlockStatePlayerWallHead {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePlayerWallHead { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePlayerWallHead { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateCreeperHead = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateCreeperWallHead {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateCreeperWallHead {}

impl BlockStateContainer for BlockStateCreeperWallHead {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateCreeperWallHead { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCreeperWallHead { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateDragonHead = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateDragonWallHead {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateDragonWallHead {}

impl BlockStateContainer for BlockStateDragonWallHead {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateDragonWallHead { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDragonWallHead { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAnvil {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateAnvil {}

impl BlockStateContainer for BlockStateAnvil {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateAnvil { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAnvil { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateChippedAnvil {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateChippedAnvil {}

impl BlockStateContainer for BlockStateChippedAnvil {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateChippedAnvil { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateChippedAnvil { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDamagedAnvil {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateDamagedAnvil {}

impl BlockStateContainer for BlockStateDamagedAnvil {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateDamagedAnvil { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDamagedAnvil { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateTrappedChest {
    pub facing: FacingHorizontal,
    pub type_: ChestType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateTrappedChest {}

impl BlockStateContainer for BlockStateTrappedChest {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateTrappedChest { facing: facing_i,type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTrappedChest { facing: self.facing.state_from(value.get("facing")?)?,type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateLightWeightedPressurePlate = BlockStateNone;

pub type BlockStateHeavyWeightedPressurePlate = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateComparator {
    pub facing: FacingHorizontal,
    pub mode: ComparatorMode,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateComparator {}

impl BlockStateContainer for BlockStateComparator {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for mode_i in self.mode.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateComparator { facing: facing_i,mode: mode_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateComparator { facing: self.facing.state_from(value.get("facing")?)?,mode: self.mode.state_from(value.get("mode")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDaylightDetector {
    pub inverted: bool,
}

impl BlockStateContainerImpl for BlockStateDaylightDetector {}

impl BlockStateContainer for BlockStateDaylightDetector {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for inverted_i in self.inverted.all() {
        out.push(BlockStateDaylightDetector { inverted: inverted_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDaylightDetector { inverted: self.inverted.state_from(value.get("inverted")?)? })
    }
}


pub type BlockStateRedstoneBlock = BlockStateNone;

pub type BlockStateNetherQuartzOre = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateHopper {
    pub enabled: bool,
    pub facing: FacingDownable,
}

impl BlockStateContainerImpl for BlockStateHopper {}

impl BlockStateContainer for BlockStateHopper {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for enabled_i in self.enabled.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStateHopper { enabled: enabled_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateHopper { enabled: self.enabled.state_from(value.get("enabled")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateQuartzBlock = BlockStateNone;

pub type BlockStateChiseledQuartzBlock = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateQuartzPillar {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateQuartzPillar {}

impl BlockStateContainer for BlockStateQuartzPillar {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateQuartzPillar { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateQuartzPillar { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateQuartzStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateQuartzStairs {}

impl BlockStateContainer for BlockStateQuartzStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateQuartzStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateQuartzStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateActivatorRail {
    pub powered: bool,
    pub shape: StraightRailShape,
}

impl BlockStateContainerImpl for BlockStateActivatorRail {}

impl BlockStateContainer for BlockStateActivatorRail {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for powered_i in self.powered.all() {
        
    for shape_i in self.shape.all() {
        out.push(BlockStateActivatorRail { powered: powered_i,shape: shape_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateActivatorRail { powered: self.powered.state_from(value.get("powered")?)?,shape: self.shape.state_from(value.get("shape")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDropper {
    pub facing: Facing,
    pub triggered: bool,
}

impl BlockStateContainerImpl for BlockStateDropper {}

impl BlockStateContainer for BlockStateDropper {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for triggered_i in self.triggered.all() {
        out.push(BlockStateDropper { facing: facing_i,triggered: triggered_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDropper { facing: self.facing.state_from(value.get("facing")?)?,triggered: self.triggered.state_from(value.get("triggered")?)? })
    }
}


pub type BlockStateWhiteTerracotta = BlockStateNone;

pub type BlockStateOrangeTerracotta = BlockStateNone;

pub type BlockStateMagentaTerracotta = BlockStateNone;

pub type BlockStateLightBlueTerracotta = BlockStateNone;

pub type BlockStateYellowTerracotta = BlockStateNone;

pub type BlockStateLimeTerracotta = BlockStateNone;

pub type BlockStatePinkTerracotta = BlockStateNone;

pub type BlockStateGrayTerracotta = BlockStateNone;

pub type BlockStateLightGrayTerracotta = BlockStateNone;

pub type BlockStateCyanTerracotta = BlockStateNone;

pub type BlockStatePurpleTerracotta = BlockStateNone;

pub type BlockStateBlueTerracotta = BlockStateNone;

pub type BlockStateBrownTerracotta = BlockStateNone;

pub type BlockStateGreenTerracotta = BlockStateNone;

pub type BlockStateRedTerracotta = BlockStateNone;

pub type BlockStateBlackTerracotta = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateWhiteStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateWhiteStainedGlassPane {}

impl BlockStateContainer for BlockStateWhiteStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateWhiteStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateWhiteStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOrangeStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateOrangeStainedGlassPane {}

impl BlockStateContainer for BlockStateOrangeStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateOrangeStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOrangeStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMagentaStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateMagentaStainedGlassPane {}

impl BlockStateContainer for BlockStateMagentaStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateMagentaStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMagentaStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightBlueStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateLightBlueStainedGlassPane {}

impl BlockStateContainer for BlockStateLightBlueStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateLightBlueStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightBlueStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateYellowStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateYellowStainedGlassPane {}

impl BlockStateContainer for BlockStateYellowStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateYellowStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateYellowStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLimeStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateLimeStainedGlassPane {}

impl BlockStateContainer for BlockStateLimeStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateLimeStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLimeStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePinkStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStatePinkStainedGlassPane {}

impl BlockStateContainer for BlockStatePinkStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStatePinkStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePinkStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGrayStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateGrayStainedGlassPane {}

impl BlockStateContainer for BlockStateGrayStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateGrayStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGrayStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightGrayStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateLightGrayStainedGlassPane {}

impl BlockStateContainer for BlockStateLightGrayStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateLightGrayStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightGrayStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCyanStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateCyanStainedGlassPane {}

impl BlockStateContainer for BlockStateCyanStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateCyanStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCyanStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpleStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStatePurpleStainedGlassPane {}

impl BlockStateContainer for BlockStatePurpleStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStatePurpleStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpleStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlueStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateBlueStainedGlassPane {}

impl BlockStateContainer for BlockStateBlueStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateBlueStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlueStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrownStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateBrownStainedGlassPane {}

impl BlockStateContainer for BlockStateBrownStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateBrownStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrownStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGreenStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateGreenStainedGlassPane {}

impl BlockStateContainer for BlockStateGreenStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateGreenStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGreenStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateRedStainedGlassPane {}

impl BlockStateContainer for BlockStateRedStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateRedStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlackStainedGlassPane {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateBlackStainedGlassPane {}

impl BlockStateContainer for BlockStateBlackStainedGlassPane {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateBlackStainedGlassPane { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlackStainedGlassPane { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaStairs {}

impl BlockStateContainer for BlockStateAcaciaStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateAcaciaStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakStairs {}

impl BlockStateContainer for BlockStateDarkOakStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDarkOakStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateSlimeBlock = BlockStateNone;

pub type BlockStateBarrier = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateIronTrapdoor {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateIronTrapdoor {}

impl BlockStateContainer for BlockStateIronTrapdoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateIronTrapdoor { facing: facing_i,half: half_i,open: open_i,powered: powered_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateIronTrapdoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStatePrismarine = BlockStateNone;

pub type BlockStatePrismarineBricks = BlockStateNone;

pub type BlockStateDarkPrismarine = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStatePrismarineStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePrismarineStairs {}

impl BlockStateContainer for BlockStatePrismarineStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePrismarineStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePrismarineStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePrismarineBrickStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePrismarineBrickStairs {}

impl BlockStateContainer for BlockStatePrismarineBrickStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePrismarineBrickStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePrismarineBrickStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkPrismarineStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDarkPrismarineStairs {}

impl BlockStateContainer for BlockStateDarkPrismarineStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDarkPrismarineStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkPrismarineStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePrismarineSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePrismarineSlab {}

impl BlockStateContainer for BlockStatePrismarineSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePrismarineSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePrismarineSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePrismarineBrickSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePrismarineBrickSlab {}

impl BlockStateContainer for BlockStatePrismarineBrickSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePrismarineBrickSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePrismarineBrickSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkPrismarineSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDarkPrismarineSlab {}

impl BlockStateContainer for BlockStateDarkPrismarineSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDarkPrismarineSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkPrismarineSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateSeaLantern = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateHayBlock {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateHayBlock {}

impl BlockStateContainer for BlockStateHayBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateHayBlock { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateHayBlock { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


pub type BlockStateWhiteCarpet = BlockStateNone;

pub type BlockStateOrangeCarpet = BlockStateNone;

pub type BlockStateMagentaCarpet = BlockStateNone;

pub type BlockStateLightBlueCarpet = BlockStateNone;

pub type BlockStateYellowCarpet = BlockStateNone;

pub type BlockStateLimeCarpet = BlockStateNone;

pub type BlockStatePinkCarpet = BlockStateNone;

pub type BlockStateGrayCarpet = BlockStateNone;

pub type BlockStateLightGrayCarpet = BlockStateNone;

pub type BlockStateCyanCarpet = BlockStateNone;

pub type BlockStatePurpleCarpet = BlockStateNone;

pub type BlockStateBlueCarpet = BlockStateNone;

pub type BlockStateBrownCarpet = BlockStateNone;

pub type BlockStateGreenCarpet = BlockStateNone;

pub type BlockStateRedCarpet = BlockStateNone;

pub type BlockStateBlackCarpet = BlockStateNone;

pub type BlockStateTerracotta = BlockStateNone;

pub type BlockStateCoalBlock = BlockStateNone;

pub type BlockStatePackedIce = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateSunflower {
    pub half: Flower,
}

impl BlockStateContainerImpl for BlockStateSunflower {}

impl BlockStateContainer for BlockStateSunflower {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for half_i in self.half.all() {
        out.push(BlockStateSunflower { half: half_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSunflower { half: self.half.state_from(value.get("half")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLilac {
    pub half: Flower,
}

impl BlockStateContainerImpl for BlockStateLilac {}

impl BlockStateContainer for BlockStateLilac {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for half_i in self.half.all() {
        out.push(BlockStateLilac { half: half_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLilac { half: self.half.state_from(value.get("half")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRoseBush {
    pub half: Flower,
}

impl BlockStateContainerImpl for BlockStateRoseBush {}

impl BlockStateContainer for BlockStateRoseBush {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for half_i in self.half.all() {
        out.push(BlockStateRoseBush { half: half_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRoseBush { half: self.half.state_from(value.get("half")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePeony {
    pub half: Flower,
}

impl BlockStateContainerImpl for BlockStatePeony {}

impl BlockStateContainer for BlockStatePeony {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for half_i in self.half.all() {
        out.push(BlockStatePeony { half: half_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePeony { half: self.half.state_from(value.get("half")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateTallGrass {
    pub half: Flower,
}

impl BlockStateContainerImpl for BlockStateTallGrass {}

impl BlockStateContainer for BlockStateTallGrass {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for half_i in self.half.all() {
        out.push(BlockStateTallGrass { half: half_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTallGrass { half: self.half.state_from(value.get("half")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLargeFern {
    pub half: Flower,
}

impl BlockStateContainerImpl for BlockStateLargeFern {}

impl BlockStateContainer for BlockStateLargeFern {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for half_i in self.half.all() {
        out.push(BlockStateLargeFern { half: half_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLargeFern { half: self.half.state_from(value.get("half")?)? })
    }
}


pub type BlockStateWhiteBanner = BlockStateNone;

pub type BlockStateOrangeBanner = BlockStateNone;

pub type BlockStateMagentaBanner = BlockStateNone;

pub type BlockStateLightBlueBanner = BlockStateNone;

pub type BlockStateYellowBanner = BlockStateNone;

pub type BlockStateLimeBanner = BlockStateNone;

pub type BlockStatePinkBanner = BlockStateNone;

pub type BlockStateGrayBanner = BlockStateNone;

pub type BlockStateLightGrayBanner = BlockStateNone;

pub type BlockStateCyanBanner = BlockStateNone;

pub type BlockStatePurpleBanner = BlockStateNone;

pub type BlockStateBlueBanner = BlockStateNone;

pub type BlockStateBrownBanner = BlockStateNone;

pub type BlockStateGreenBanner = BlockStateNone;

pub type BlockStateRedBanner = BlockStateNone;

pub type BlockStateBlackBanner = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateWhiteWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateWhiteWallBanner {}

impl BlockStateContainer for BlockStateWhiteWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateWhiteWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateWhiteWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOrangeWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateOrangeWallBanner {}

impl BlockStateContainer for BlockStateOrangeWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateOrangeWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOrangeWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMagentaWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateMagentaWallBanner {}

impl BlockStateContainer for BlockStateMagentaWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateMagentaWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMagentaWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightBlueWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateLightBlueWallBanner {}

impl BlockStateContainer for BlockStateLightBlueWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLightBlueWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightBlueWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateYellowWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateYellowWallBanner {}

impl BlockStateContainer for BlockStateYellowWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateYellowWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateYellowWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLimeWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateLimeWallBanner {}

impl BlockStateContainer for BlockStateLimeWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLimeWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLimeWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePinkWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStatePinkWallBanner {}

impl BlockStateContainer for BlockStatePinkWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePinkWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePinkWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGrayWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateGrayWallBanner {}

impl BlockStateContainer for BlockStateGrayWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateGrayWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGrayWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightGrayWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateLightGrayWallBanner {}

impl BlockStateContainer for BlockStateLightGrayWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLightGrayWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightGrayWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCyanWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateCyanWallBanner {}

impl BlockStateContainer for BlockStateCyanWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateCyanWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCyanWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpleWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStatePurpleWallBanner {}

impl BlockStateContainer for BlockStatePurpleWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePurpleWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpleWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlueWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBlueWallBanner {}

impl BlockStateContainer for BlockStateBlueWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBlueWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlueWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrownWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBrownWallBanner {}

impl BlockStateContainer for BlockStateBrownWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBrownWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrownWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGreenWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateGreenWallBanner {}

impl BlockStateContainer for BlockStateGreenWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateGreenWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGreenWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateRedWallBanner {}

impl BlockStateContainer for BlockStateRedWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateRedWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlackWallBanner {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBlackWallBanner {}

impl BlockStateContainer for BlockStateBlackWallBanner {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBlackWallBanner { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlackWallBanner { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateRedSandstone = BlockStateNone;

pub type BlockStateChiseledRedSandstone = BlockStateNone;

pub type BlockStateCutRedSandstone = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateRedSandstoneStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateRedSandstoneStairs {}

impl BlockStateContainer for BlockStateRedSandstoneStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateRedSandstoneStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedSandstoneStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOakSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateOakSlab {}

impl BlockStateContainer for BlockStateOakSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateOakSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOakSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceSlab {}

impl BlockStateContainer for BlockStateSpruceSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSpruceSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBirchSlab {}

impl BlockStateContainer for BlockStateBirchSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBirchSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateJungleSlab {}

impl BlockStateContainer for BlockStateJungleSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateJungleSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaSlab {}

impl BlockStateContainer for BlockStateAcaciaSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateAcaciaSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakSlab {}

impl BlockStateContainer for BlockStateDarkOakSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDarkOakSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateStoneSlab {}

impl BlockStateContainer for BlockStateStoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateStoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoothStoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSmoothStoneSlab {}

impl BlockStateContainer for BlockStateSmoothStoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSmoothStoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoothStoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSandstoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSandstoneSlab {}

impl BlockStateContainer for BlockStateSandstoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSandstoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSandstoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCutSandstoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateCutSandstoneSlab {}

impl BlockStateContainer for BlockStateCutSandstoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateCutSandstoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCutSandstoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePetrifiedOakSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePetrifiedOakSlab {}

impl BlockStateContainer for BlockStatePetrifiedOakSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePetrifiedOakSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePetrifiedOakSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCobblestoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateCobblestoneSlab {}

impl BlockStateContainer for BlockStateCobblestoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateCobblestoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCobblestoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrickSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBrickSlab {}

impl BlockStateContainer for BlockStateBrickSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBrickSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrickSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStoneBrickSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateStoneBrickSlab {}

impl BlockStateContainer for BlockStateStoneBrickSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateStoneBrickSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStoneBrickSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateNetherBrickSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateNetherBrickSlab {}

impl BlockStateContainer for BlockStateNetherBrickSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateNetherBrickSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateNetherBrickSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateQuartzSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateQuartzSlab {}

impl BlockStateContainer for BlockStateQuartzSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateQuartzSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateQuartzSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedSandstoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateRedSandstoneSlab {}

impl BlockStateContainer for BlockStateRedSandstoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateRedSandstoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedSandstoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCutRedSandstoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateCutRedSandstoneSlab {}

impl BlockStateContainer for BlockStateCutRedSandstoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateCutRedSandstoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCutRedSandstoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpurSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePurpurSlab {}

impl BlockStateContainer for BlockStatePurpurSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePurpurSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpurSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateSmoothStone = BlockStateNone;

pub type BlockStateSmoothSandstone = BlockStateNone;

pub type BlockStateSmoothQuartz = BlockStateNone;

pub type BlockStateSmoothRedSandstone = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceFenceGate {
    pub facing: FacingHorizontal,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceFenceGate {}

impl BlockStateContainer for BlockStateSpruceFenceGate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for in_wall_i in self.in_wall.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateSpruceFenceGate { facing: facing_i,in_wall: in_wall_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceFenceGate { facing: self.facing.state_from(value.get("facing")?)?,in_wall: self.in_wall.state_from(value.get("in_wall")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchFenceGate {
    pub facing: FacingHorizontal,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateBirchFenceGate {}

impl BlockStateContainer for BlockStateBirchFenceGate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for in_wall_i in self.in_wall.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateBirchFenceGate { facing: facing_i,in_wall: in_wall_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchFenceGate { facing: self.facing.state_from(value.get("facing")?)?,in_wall: self.in_wall.state_from(value.get("in_wall")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleFenceGate {
    pub facing: FacingHorizontal,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateJungleFenceGate {}

impl BlockStateContainer for BlockStateJungleFenceGate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for in_wall_i in self.in_wall.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateJungleFenceGate { facing: facing_i,in_wall: in_wall_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleFenceGate { facing: self.facing.state_from(value.get("facing")?)?,in_wall: self.in_wall.state_from(value.get("in_wall")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaFenceGate {
    pub facing: FacingHorizontal,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaFenceGate {}

impl BlockStateContainer for BlockStateAcaciaFenceGate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for in_wall_i in self.in_wall.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateAcaciaFenceGate { facing: facing_i,in_wall: in_wall_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaFenceGate { facing: self.facing.state_from(value.get("facing")?)?,in_wall: self.in_wall.state_from(value.get("in_wall")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakFenceGate {
    pub facing: FacingHorizontal,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakFenceGate {}

impl BlockStateContainer for BlockStateDarkOakFenceGate {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for in_wall_i in self.in_wall.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateDarkOakFenceGate { facing: facing_i,in_wall: in_wall_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakFenceGate { facing: self.facing.state_from(value.get("facing")?)?,in_wall: self.in_wall.state_from(value.get("in_wall")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceFence {}

impl BlockStateContainer for BlockStateSpruceFence {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateSpruceFence { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceFence { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateBirchFence {}

impl BlockStateContainer for BlockStateBirchFence {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateBirchFence { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchFence { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateJungleFence {}

impl BlockStateContainer for BlockStateJungleFence {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateJungleFence { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleFence { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaFence {}

impl BlockStateContainer for BlockStateAcaciaFence {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateAcaciaFence { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaFence { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakFence {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakFence {}

impl BlockStateContainer for BlockStateDarkOakFence {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateDarkOakFence { east: east_i,north: north_i,south: south_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakFence { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSpruceDoor {
    pub facing: FacingHorizontal,
    pub half: Flower,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateSpruceDoor {}

impl BlockStateContainer for BlockStateSpruceDoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for hinge_i in self.hinge.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateSpruceDoor { facing: facing_i,half: half_i,hinge: hinge_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSpruceDoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,hinge: self.hinge.state_from(value.get("hinge")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBirchDoor {
    pub facing: FacingHorizontal,
    pub half: Flower,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateBirchDoor {}

impl BlockStateContainer for BlockStateBirchDoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for hinge_i in self.hinge.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateBirchDoor { facing: facing_i,half: half_i,hinge: hinge_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBirchDoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,hinge: self.hinge.state_from(value.get("hinge")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJungleDoor {
    pub facing: FacingHorizontal,
    pub half: Flower,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateJungleDoor {}

impl BlockStateContainer for BlockStateJungleDoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for hinge_i in self.hinge.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateJungleDoor { facing: facing_i,half: half_i,hinge: hinge_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJungleDoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,hinge: self.hinge.state_from(value.get("hinge")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAcaciaDoor {
    pub facing: FacingHorizontal,
    pub half: Flower,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateAcaciaDoor {}

impl BlockStateContainer for BlockStateAcaciaDoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for hinge_i in self.hinge.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateAcaciaDoor { facing: facing_i,half: half_i,hinge: hinge_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAcaciaDoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,hinge: self.hinge.state_from(value.get("hinge")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDarkOakDoor {
    pub facing: FacingHorizontal,
    pub half: Flower,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateDarkOakDoor {}

impl BlockStateContainer for BlockStateDarkOakDoor {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for hinge_i in self.hinge.all() {
        
    for open_i in self.open.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateDarkOakDoor { facing: facing_i,half: half_i,hinge: hinge_i,open: open_i,powered: powered_i })
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDarkOakDoor { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,hinge: self.hinge.state_from(value.get("hinge")?)?,open: self.open.state_from(value.get("open")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateEndRod {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateEndRod {}

impl BlockStateContainer for BlockStateEndRod {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateEndRod { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateEndRod { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateChorusPlant {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateChorusPlant {}

impl BlockStateContainer for BlockStateChorusPlant {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for down_i in self.down.all() {
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateChorusPlant { down: down_i,east: east_i,north: north_i,south: south_i,up: up_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateChorusPlant { down: self.down.state_from(value.get("down")?)?,east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


pub type BlockStateChorusFlower = BlockStateNone;

pub type BlockStatePurpurBlock = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpurPillar {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStatePurpurPillar {}

impl BlockStateContainer for BlockStatePurpurPillar {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStatePurpurPillar { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpurPillar { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpurStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePurpurStairs {}

impl BlockStateContainer for BlockStatePurpurStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePurpurStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpurStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateEndStoneBricks = BlockStateNone;

pub type BlockStateBeetroots = BlockStateNone;

pub type BlockStateGrassPath = BlockStateNone;

pub type BlockStateEndGateway = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateRepeatingCommandBlock {
    pub conditional: bool,
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateRepeatingCommandBlock {}

impl BlockStateContainer for BlockStateRepeatingCommandBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for conditional_i in self.conditional.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStateRepeatingCommandBlock { conditional: conditional_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRepeatingCommandBlock { conditional: self.conditional.state_from(value.get("conditional")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateChainCommandBlock {
    pub conditional: bool,
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateChainCommandBlock {}

impl BlockStateContainer for BlockStateChainCommandBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for conditional_i in self.conditional.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStateChainCommandBlock { conditional: conditional_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateChainCommandBlock { conditional: self.conditional.state_from(value.get("conditional")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateFrostedIce = BlockStateNone;

pub type BlockStateMagmaBlock = BlockStateNone;

pub type BlockStateNetherWartBlock = BlockStateNone;

pub type BlockStateRedNetherBricks = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateBoneBlock {
    pub axis: Axis,
}

impl BlockStateContainerImpl for BlockStateBoneBlock {}

impl BlockStateContainer for BlockStateBoneBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for axis_i in self.axis.all() {
        out.push(BlockStateBoneBlock { axis: axis_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBoneBlock { axis: self.axis.state_from(value.get("axis")?)? })
    }
}


pub type BlockStateStructureVoid = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateObserver {
    pub facing: Facing,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateObserver {}

impl BlockStateContainer for BlockStateObserver {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateObserver { facing: facing_i,powered: powered_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateObserver { facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateShulkerBox {}

impl BlockStateContainer for BlockStateShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateWhiteShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateWhiteShulkerBox {}

impl BlockStateContainer for BlockStateWhiteShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateWhiteShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateWhiteShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOrangeShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateOrangeShulkerBox {}

impl BlockStateContainer for BlockStateOrangeShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateOrangeShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOrangeShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMagentaShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateMagentaShulkerBox {}

impl BlockStateContainer for BlockStateMagentaShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateMagentaShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMagentaShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightBlueShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateLightBlueShulkerBox {}

impl BlockStateContainer for BlockStateLightBlueShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLightBlueShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightBlueShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateYellowShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateYellowShulkerBox {}

impl BlockStateContainer for BlockStateYellowShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateYellowShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateYellowShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLimeShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateLimeShulkerBox {}

impl BlockStateContainer for BlockStateLimeShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLimeShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLimeShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePinkShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStatePinkShulkerBox {}

impl BlockStateContainer for BlockStatePinkShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePinkShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePinkShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGrayShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateGrayShulkerBox {}

impl BlockStateContainer for BlockStateGrayShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateGrayShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGrayShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightGrayShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateLightGrayShulkerBox {}

impl BlockStateContainer for BlockStateLightGrayShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLightGrayShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightGrayShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCyanShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateCyanShulkerBox {}

impl BlockStateContainer for BlockStateCyanShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateCyanShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCyanShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpleShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStatePurpleShulkerBox {}

impl BlockStateContainer for BlockStatePurpleShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePurpleShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpleShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlueShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateBlueShulkerBox {}

impl BlockStateContainer for BlockStateBlueShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBlueShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlueShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrownShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateBrownShulkerBox {}

impl BlockStateContainer for BlockStateBrownShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBrownShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrownShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGreenShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateGreenShulkerBox {}

impl BlockStateContainer for BlockStateGreenShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateGreenShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGreenShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateRedShulkerBox {}

impl BlockStateContainer for BlockStateRedShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateRedShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlackShulkerBox {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateBlackShulkerBox {}

impl BlockStateContainer for BlockStateBlackShulkerBox {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBlackShulkerBox { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlackShulkerBox { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateWhiteGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateWhiteGlazedTerracotta {}

impl BlockStateContainer for BlockStateWhiteGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateWhiteGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateWhiteGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateOrangeGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateOrangeGlazedTerracotta {}

impl BlockStateContainer for BlockStateOrangeGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateOrangeGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateOrangeGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMagentaGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateMagentaGlazedTerracotta {}

impl BlockStateContainer for BlockStateMagentaGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateMagentaGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMagentaGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightBlueGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateLightBlueGlazedTerracotta {}

impl BlockStateContainer for BlockStateLightBlueGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLightBlueGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightBlueGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateYellowGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateYellowGlazedTerracotta {}

impl BlockStateContainer for BlockStateYellowGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateYellowGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateYellowGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLimeGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateLimeGlazedTerracotta {}

impl BlockStateContainer for BlockStateLimeGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLimeGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLimeGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePinkGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStatePinkGlazedTerracotta {}

impl BlockStateContainer for BlockStatePinkGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePinkGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePinkGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGrayGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateGrayGlazedTerracotta {}

impl BlockStateContainer for BlockStateGrayGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateGrayGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGrayGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLightGrayGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateLightGrayGlazedTerracotta {}

impl BlockStateContainer for BlockStateLightGrayGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLightGrayGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLightGrayGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCyanGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateCyanGlazedTerracotta {}

impl BlockStateContainer for BlockStateCyanGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateCyanGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCyanGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePurpleGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStatePurpleGlazedTerracotta {}

impl BlockStateContainer for BlockStatePurpleGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStatePurpleGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePurpleGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlueGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBlueGlazedTerracotta {}

impl BlockStateContainer for BlockStateBlueGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBlueGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlueGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrownGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBrownGlazedTerracotta {}

impl BlockStateContainer for BlockStateBrownGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBrownGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrownGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGreenGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateGreenGlazedTerracotta {}

impl BlockStateContainer for BlockStateGreenGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateGreenGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGreenGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateRedGlazedTerracotta {}

impl BlockStateContainer for BlockStateRedGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateRedGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlackGlazedTerracotta {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBlackGlazedTerracotta {}

impl BlockStateContainer for BlockStateBlackGlazedTerracotta {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBlackGlazedTerracotta { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlackGlazedTerracotta { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateWhiteConcrete = BlockStateNone;

pub type BlockStateOrangeConcrete = BlockStateNone;

pub type BlockStateMagentaConcrete = BlockStateNone;

pub type BlockStateLightBlueConcrete = BlockStateNone;

pub type BlockStateYellowConcrete = BlockStateNone;

pub type BlockStateLimeConcrete = BlockStateNone;

pub type BlockStatePinkConcrete = BlockStateNone;

pub type BlockStateGrayConcrete = BlockStateNone;

pub type BlockStateLightGrayConcrete = BlockStateNone;

pub type BlockStateCyanConcrete = BlockStateNone;

pub type BlockStatePurpleConcrete = BlockStateNone;

pub type BlockStateBlueConcrete = BlockStateNone;

pub type BlockStateBrownConcrete = BlockStateNone;

pub type BlockStateGreenConcrete = BlockStateNone;

pub type BlockStateRedConcrete = BlockStateNone;

pub type BlockStateBlackConcrete = BlockStateNone;

pub type BlockStateWhiteConcretePowder = BlockStateNone;

pub type BlockStateOrangeConcretePowder = BlockStateNone;

pub type BlockStateMagentaConcretePowder = BlockStateNone;

pub type BlockStateLightBlueConcretePowder = BlockStateNone;

pub type BlockStateYellowConcretePowder = BlockStateNone;

pub type BlockStateLimeConcretePowder = BlockStateNone;

pub type BlockStatePinkConcretePowder = BlockStateNone;

pub type BlockStateGrayConcretePowder = BlockStateNone;

pub type BlockStateLightGrayConcretePowder = BlockStateNone;

pub type BlockStateCyanConcretePowder = BlockStateNone;

pub type BlockStatePurpleConcretePowder = BlockStateNone;

pub type BlockStateBlueConcretePowder = BlockStateNone;

pub type BlockStateBrownConcretePowder = BlockStateNone;

pub type BlockStateGreenConcretePowder = BlockStateNone;

pub type BlockStateRedConcretePowder = BlockStateNone;

pub type BlockStateBlackConcretePowder = BlockStateNone;

pub type BlockStateKelp = BlockStateNone;

pub type BlockStateKelpPlant = BlockStateNone;

pub type BlockStateDriedKelpBlock = BlockStateNone;

pub type BlockStateTurtleEgg = BlockStateNone;

pub type BlockStateDeadTubeCoralBlock = BlockStateNone;

pub type BlockStateDeadBrainCoralBlock = BlockStateNone;

pub type BlockStateDeadBubbleCoralBlock = BlockStateNone;

pub type BlockStateDeadFireCoralBlock = BlockStateNone;

pub type BlockStateDeadHornCoralBlock = BlockStateNone;

pub type BlockStateTubeCoralBlock = BlockStateNone;

pub type BlockStateBrainCoralBlock = BlockStateNone;

pub type BlockStateBubbleCoralBlock = BlockStateNone;

pub type BlockStateFireCoralBlock = BlockStateNone;

pub type BlockStateHornCoralBlock = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadTubeCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadTubeCoral {}

impl BlockStateContainer for BlockStateDeadTubeCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadTubeCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadTubeCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadBrainCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadBrainCoral {}

impl BlockStateContainer for BlockStateDeadBrainCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadBrainCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadBrainCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadBubbleCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadBubbleCoral {}

impl BlockStateContainer for BlockStateDeadBubbleCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadBubbleCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadBubbleCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadFireCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadFireCoral {}

impl BlockStateContainer for BlockStateDeadFireCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadFireCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadFireCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadHornCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadHornCoral {}

impl BlockStateContainer for BlockStateDeadHornCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadHornCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadHornCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateTubeCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateTubeCoral {}

impl BlockStateContainer for BlockStateTubeCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateTubeCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTubeCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrainCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBrainCoral {}

impl BlockStateContainer for BlockStateBrainCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBrainCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrainCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBubbleCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBubbleCoral {}

impl BlockStateContainer for BlockStateBubbleCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBubbleCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBubbleCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateFireCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateFireCoral {}

impl BlockStateContainer for BlockStateFireCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateFireCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateFireCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateHornCoral {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateHornCoral {}

impl BlockStateContainer for BlockStateHornCoral {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateHornCoral { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateHornCoral { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadTubeCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadTubeCoralFan {}

impl BlockStateContainer for BlockStateDeadTubeCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadTubeCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadTubeCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadBrainCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadBrainCoralFan {}

impl BlockStateContainer for BlockStateDeadBrainCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadBrainCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadBrainCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadBubbleCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadBubbleCoralFan {}

impl BlockStateContainer for BlockStateDeadBubbleCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadBubbleCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadBubbleCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadFireCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadFireCoralFan {}

impl BlockStateContainer for BlockStateDeadFireCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadFireCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadFireCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadHornCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadHornCoralFan {}

impl BlockStateContainer for BlockStateDeadHornCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadHornCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadHornCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateTubeCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateTubeCoralFan {}

impl BlockStateContainer for BlockStateTubeCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateTubeCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTubeCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrainCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBrainCoralFan {}

impl BlockStateContainer for BlockStateBrainCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBrainCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrainCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBubbleCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBubbleCoralFan {}

impl BlockStateContainer for BlockStateBubbleCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBubbleCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBubbleCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateFireCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateFireCoralFan {}

impl BlockStateContainer for BlockStateFireCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateFireCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateFireCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateHornCoralFan {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateHornCoralFan {}

impl BlockStateContainer for BlockStateHornCoralFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateHornCoralFan { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateHornCoralFan { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadTubeCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadTubeCoralWallFan {}

impl BlockStateContainer for BlockStateDeadTubeCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadTubeCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadTubeCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadBrainCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadBrainCoralWallFan {}

impl BlockStateContainer for BlockStateDeadBrainCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadBrainCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadBrainCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadBubbleCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadBubbleCoralWallFan {}

impl BlockStateContainer for BlockStateDeadBubbleCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadBubbleCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadBubbleCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadFireCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadFireCoralWallFan {}

impl BlockStateContainer for BlockStateDeadFireCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadFireCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadFireCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDeadHornCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDeadHornCoralWallFan {}

impl BlockStateContainer for BlockStateDeadHornCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDeadHornCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDeadHornCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateTubeCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateTubeCoralWallFan {}

impl BlockStateContainer for BlockStateTubeCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateTubeCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateTubeCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrainCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBrainCoralWallFan {}

impl BlockStateContainer for BlockStateBrainCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBrainCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrainCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBubbleCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateBubbleCoralWallFan {}

impl BlockStateContainer for BlockStateBubbleCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateBubbleCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBubbleCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateFireCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateFireCoralWallFan {}

impl BlockStateContainer for BlockStateFireCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateFireCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateFireCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateHornCoralWallFan {
    pub facing: FacingHorizontal,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateHornCoralWallFan {}

impl BlockStateContainer for BlockStateHornCoralWallFan {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateHornCoralWallFan { facing: facing_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateHornCoralWallFan { facing: self.facing.state_from(value.get("facing")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSeaPickle {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSeaPickle {}

impl BlockStateContainer for BlockStateSeaPickle {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSeaPickle { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSeaPickle { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateBlueIce = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateConduit {
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateConduit {}

impl BlockStateContainer for BlockStateConduit {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateConduit { waterlogged: waterlogged_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateConduit { waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateBambooSapling = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateBamboo {
    pub leaves: Leaves,
}

impl BlockStateContainerImpl for BlockStateBamboo {}

impl BlockStateContainer for BlockStateBamboo {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for leaves_i in self.leaves.all() {
        out.push(BlockStateBamboo { leaves: leaves_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBamboo { leaves: self.leaves.state_from(value.get("leaves")?)? })
    }
}


pub type BlockStatePottedBamboo = BlockStateNone;

pub type BlockStateVoidAir = BlockStateNone;

pub type BlockStateCaveAir = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateBubbleColumn {
    pub drag: bool,
}

impl BlockStateContainerImpl for BlockStateBubbleColumn {}

impl BlockStateContainer for BlockStateBubbleColumn {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for drag_i in self.drag.all() {
        out.push(BlockStateBubbleColumn { drag: drag_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBubbleColumn { drag: self.drag.state_from(value.get("drag")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePolishedGraniteStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePolishedGraniteStairs {}

impl BlockStateContainer for BlockStatePolishedGraniteStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePolishedGraniteStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePolishedGraniteStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoothRedSandstoneStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSmoothRedSandstoneStairs {}

impl BlockStateContainer for BlockStateSmoothRedSandstoneStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSmoothRedSandstoneStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoothRedSandstoneStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMossyStoneBrickStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateMossyStoneBrickStairs {}

impl BlockStateContainer for BlockStateMossyStoneBrickStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateMossyStoneBrickStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMossyStoneBrickStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePolishedDioriteStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePolishedDioriteStairs {}

impl BlockStateContainer for BlockStatePolishedDioriteStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePolishedDioriteStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePolishedDioriteStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMossyCobblestoneStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateMossyCobblestoneStairs {}

impl BlockStateContainer for BlockStateMossyCobblestoneStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateMossyCobblestoneStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMossyCobblestoneStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateEndStoneBrickStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateEndStoneBrickStairs {}

impl BlockStateContainer for BlockStateEndStoneBrickStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateEndStoneBrickStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateEndStoneBrickStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStoneStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateStoneStairs {}

impl BlockStateContainer for BlockStateStoneStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateStoneStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStoneStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoothSandstoneStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSmoothSandstoneStairs {}

impl BlockStateContainer for BlockStateSmoothSandstoneStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSmoothSandstoneStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoothSandstoneStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoothQuartzStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSmoothQuartzStairs {}

impl BlockStateContainer for BlockStateSmoothQuartzStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSmoothQuartzStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoothQuartzStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGraniteStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateGraniteStairs {}

impl BlockStateContainer for BlockStateGraniteStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateGraniteStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGraniteStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAndesiteStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateAndesiteStairs {}

impl BlockStateContainer for BlockStateAndesiteStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateAndesiteStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAndesiteStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedNetherBrickStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateRedNetherBrickStairs {}

impl BlockStateContainer for BlockStateRedNetherBrickStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateRedNetherBrickStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedNetherBrickStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePolishedAndesiteStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePolishedAndesiteStairs {}

impl BlockStateContainer for BlockStatePolishedAndesiteStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePolishedAndesiteStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePolishedAndesiteStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDioriteStairs {
    pub facing: FacingHorizontal,
    pub half: Stairs,
    pub shape: StairShape,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDioriteStairs {}

impl BlockStateContainer for BlockStateDioriteStairs {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for half_i in self.half.all() {
        
    for shape_i in self.shape.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDioriteStairs { facing: facing_i,half: half_i,shape: shape_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDioriteStairs { facing: self.facing.state_from(value.get("facing")?)?,half: self.half.state_from(value.get("half")?)?,shape: self.shape.state_from(value.get("shape")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePolishedGraniteSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePolishedGraniteSlab {}

impl BlockStateContainer for BlockStatePolishedGraniteSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePolishedGraniteSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePolishedGraniteSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoothRedSandstoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSmoothRedSandstoneSlab {}

impl BlockStateContainer for BlockStateSmoothRedSandstoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSmoothRedSandstoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoothRedSandstoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMossyStoneBrickSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateMossyStoneBrickSlab {}

impl BlockStateContainer for BlockStateMossyStoneBrickSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateMossyStoneBrickSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMossyStoneBrickSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePolishedDioriteSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePolishedDioriteSlab {}

impl BlockStateContainer for BlockStatePolishedDioriteSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePolishedDioriteSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePolishedDioriteSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMossyCobblestoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateMossyCobblestoneSlab {}

impl BlockStateContainer for BlockStateMossyCobblestoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateMossyCobblestoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMossyCobblestoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateEndStoneBrickSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateEndStoneBrickSlab {}

impl BlockStateContainer for BlockStateEndStoneBrickSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateEndStoneBrickSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateEndStoneBrickSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoothSandstoneSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSmoothSandstoneSlab {}

impl BlockStateContainer for BlockStateSmoothSandstoneSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSmoothSandstoneSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoothSandstoneSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoothQuartzSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateSmoothQuartzSlab {}

impl BlockStateContainer for BlockStateSmoothQuartzSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateSmoothQuartzSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoothQuartzSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGraniteSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateGraniteSlab {}

impl BlockStateContainer for BlockStateGraniteSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateGraniteSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGraniteSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAndesiteSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateAndesiteSlab {}

impl BlockStateContainer for BlockStateAndesiteSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateAndesiteSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAndesiteSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedNetherBrickSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateRedNetherBrickSlab {}

impl BlockStateContainer for BlockStateRedNetherBrickSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateRedNetherBrickSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedNetherBrickSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePolishedAndesiteSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStatePolishedAndesiteSlab {}

impl BlockStateContainer for BlockStatePolishedAndesiteSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStatePolishedAndesiteSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePolishedAndesiteSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDioriteSlab {
    pub type_: SlabType,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateDioriteSlab {}

impl BlockStateContainer for BlockStateDioriteSlab {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for type_i in self.type_.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateDioriteSlab { type_: type_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDioriteSlab { type_: self.type_.state_from(value.get("type")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBrickWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateBrickWall {}

impl BlockStateContainer for BlockStateBrickWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateBrickWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBrickWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStatePrismarineWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStatePrismarineWall {}

impl BlockStateContainer for BlockStatePrismarineWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStatePrismarineWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStatePrismarineWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedSandstoneWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateRedSandstoneWall {}

impl BlockStateContainer for BlockStateRedSandstoneWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateRedSandstoneWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedSandstoneWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateMossyStoneBrickWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateMossyStoneBrickWall {}

impl BlockStateContainer for BlockStateMossyStoneBrickWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateMossyStoneBrickWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateMossyStoneBrickWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateGraniteWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateGraniteWall {}

impl BlockStateContainer for BlockStateGraniteWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateGraniteWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGraniteWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateStoneBrickWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateStoneBrickWall {}

impl BlockStateContainer for BlockStateStoneBrickWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateStoneBrickWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStoneBrickWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateNetherBrickWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateNetherBrickWall {}

impl BlockStateContainer for BlockStateNetherBrickWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateNetherBrickWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateNetherBrickWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateAndesiteWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateAndesiteWall {}

impl BlockStateContainer for BlockStateAndesiteWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateAndesiteWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateAndesiteWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateRedNetherBrickWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateRedNetherBrickWall {}

impl BlockStateContainer for BlockStateRedNetherBrickWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateRedNetherBrickWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateRedNetherBrickWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSandstoneWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateSandstoneWall {}

impl BlockStateContainer for BlockStateSandstoneWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateSandstoneWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSandstoneWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateEndStoneBrickWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateEndStoneBrickWall {}

impl BlockStateContainer for BlockStateEndStoneBrickWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateEndStoneBrickWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateEndStoneBrickWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateDioriteWall {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

impl BlockStateContainerImpl for BlockStateDioriteWall {}

impl BlockStateContainer for BlockStateDioriteWall {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for east_i in self.east.all() {
        
    for north_i in self.north.all() {
        
    for south_i in self.south.all() {
        
    for up_i in self.up.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        
    for west_i in self.west.all() {
        out.push(BlockStateDioriteWall { east: east_i,north: north_i,south: south_i,up: up_i,waterlogged: waterlogged_i,west: west_i })
    }
    
    }
    
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateDioriteWall { east: self.east.state_from(value.get("east")?)?,north: self.north.state_from(value.get("north")?)?,south: self.south.state_from(value.get("south")?)?,up: self.up.state_from(value.get("up")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)?,west: self.west.state_from(value.get("west")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateScaffolding {
    pub bottom: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateScaffolding {}

impl BlockStateContainer for BlockStateScaffolding {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for bottom_i in self.bottom.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateScaffolding { bottom: bottom_i,waterlogged: waterlogged_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateScaffolding { bottom: self.bottom.state_from(value.get("bottom")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLoom {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateLoom {}

impl BlockStateContainer for BlockStateLoom {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateLoom { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLoom { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBarrel {
    pub facing: Facing,
    pub open: bool,
}

impl BlockStateContainerImpl for BlockStateBarrel {}

impl BlockStateContainer for BlockStateBarrel {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for open_i in self.open.all() {
        out.push(BlockStateBarrel { facing: facing_i,open: open_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBarrel { facing: self.facing.state_from(value.get("facing")?)?,open: self.open.state_from(value.get("open")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateSmoker {
    pub facing: FacingHorizontal,
    pub lit: bool,
}

impl BlockStateContainerImpl for BlockStateSmoker {}

impl BlockStateContainer for BlockStateSmoker {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for lit_i in self.lit.all() {
        out.push(BlockStateSmoker { facing: facing_i,lit: lit_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateSmoker { facing: self.facing.state_from(value.get("facing")?)?,lit: self.lit.state_from(value.get("lit")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBlastFurnace {
    pub facing: FacingHorizontal,
    pub lit: bool,
}

impl BlockStateContainerImpl for BlockStateBlastFurnace {}

impl BlockStateContainer for BlockStateBlastFurnace {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for lit_i in self.lit.all() {
        out.push(BlockStateBlastFurnace { facing: facing_i,lit: lit_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBlastFurnace { facing: self.facing.state_from(value.get("facing")?)?,lit: self.lit.state_from(value.get("lit")?)? })
    }
}


pub type BlockStateCartographyTable = BlockStateNone;

pub type BlockStateFletchingTable = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateGrindstone {
    pub face: Face,
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateGrindstone {}

impl BlockStateContainer for BlockStateGrindstone {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for face_i in self.face.all() {
        
    for facing_i in self.facing.all() {
        out.push(BlockStateGrindstone { face: face_i,facing: facing_i })
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateGrindstone { face: self.face.state_from(value.get("face")?)?,facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLectern {
    pub facing: FacingHorizontal,
    pub has_book: bool,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateLectern {}

impl BlockStateContainer for BlockStateLectern {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for has_book_i in self.has_book.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateLectern { facing: facing_i,has_book: has_book_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLectern { facing: self.facing.state_from(value.get("facing")?)?,has_book: self.has_book.state_from(value.get("has_book")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


pub type BlockStateSmithingTable = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateStonecutter {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateStonecutter {}

impl BlockStateContainer for BlockStateStonecutter {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateStonecutter { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStonecutter { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBell {
    pub attachment: Attachment,
    pub facing: FacingHorizontal,
    pub powered: bool,
}

impl BlockStateContainerImpl for BlockStateBell {}

impl BlockStateContainer for BlockStateBell {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for attachment_i in self.attachment.all() {
        
    for facing_i in self.facing.all() {
        
    for powered_i in self.powered.all() {
        out.push(BlockStateBell { attachment: attachment_i,facing: facing_i,powered: powered_i })
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBell { attachment: self.attachment.state_from(value.get("attachment")?)?,facing: self.facing.state_from(value.get("facing")?)?,powered: self.powered.state_from(value.get("powered")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateLantern {
    pub hanging: bool,
}

impl BlockStateContainerImpl for BlockStateLantern {}

impl BlockStateContainer for BlockStateLantern {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for hanging_i in self.hanging.all() {
        out.push(BlockStateLantern { hanging: hanging_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateLantern { hanging: self.hanging.state_from(value.get("hanging")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateCampfire {
    pub facing: FacingHorizontal,
    pub lit: bool,
    pub signal_fire: bool,
    pub waterlogged: bool,
}

impl BlockStateContainerImpl for BlockStateCampfire {}

impl BlockStateContainer for BlockStateCampfire {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        
    for lit_i in self.lit.all() {
        
    for signal_fire_i in self.signal_fire.all() {
        
    for waterlogged_i in self.waterlogged.all() {
        out.push(BlockStateCampfire { facing: facing_i,lit: lit_i,signal_fire: signal_fire_i,waterlogged: waterlogged_i })
    }
    
    }
    
    }
    
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateCampfire { facing: self.facing.state_from(value.get("facing")?)?,lit: self.lit.state_from(value.get("lit")?)?,signal_fire: self.signal_fire.state_from(value.get("signal_fire")?)?,waterlogged: self.waterlogged.state_from(value.get("waterlogged")?)? })
    }
}


pub type BlockStateSweetBerryBush = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateStructureBlock {
    pub mode: StructureMode,
}

impl BlockStateContainerImpl for BlockStateStructureBlock {}

impl BlockStateContainer for BlockStateStructureBlock {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for mode_i in self.mode.all() {
        out.push(BlockStateStructureBlock { mode: mode_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateStructureBlock { mode: self.mode.state_from(value.get("mode")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateJigsaw {
    pub facing: Facing,
}

impl BlockStateContainerImpl for BlockStateJigsaw {}

impl BlockStateContainer for BlockStateJigsaw {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateJigsaw { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateJigsaw { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateComposter = BlockStateNone;

#[derive(Default, Clone, Debug)]
pub struct BlockStateBeeNest {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBeeNest {}

impl BlockStateContainer for BlockStateBeeNest {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBeeNest { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBeeNest { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


#[derive(Default, Clone, Debug)]
pub struct BlockStateBeehive {
    pub facing: FacingHorizontal,
}

impl BlockStateContainerImpl for BlockStateBeehive {}

impl BlockStateContainer for BlockStateBeehive {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        
    for facing_i in self.facing.all() {
        out.push(BlockStateBeehive { facing: facing_i })
    }
    
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(BlockStateBeehive { facing: self.facing.state_from(value.get("facing")?)? })
    }
}


pub type BlockStateHoneyBlock = BlockStateNone;

pub type BlockStateHoneycombBlock = BlockStateNone;

