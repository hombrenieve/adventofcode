#[path="../common.rs"]
mod common;

const RESOURCES: usize = 4;

#[derive(Debug, Clone, Copy)]
enum Resource {
    Ore(i32),
    Clay(i32),
    Obsidian(i32),
    Geode(i32)
}

impl Resource {
    fn id(&self) -> usize {
        match self {
            Resource::Ore(_) => 0,
            Resource::Clay(_) => 1,
            Resource::Obsidian(_) => 2,
            Resource::Geode(_) => 3
        }
    }

    fn value(&self) -> i32 {
        match self {
            Resource::Ore(x) => *x,
            Resource::Clay(x) => *x,
            Resource::Obsidian(x) => *x,
            Resource::Geode(x) => *x
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pool {
    counts: [i32; RESOURCES]
}

impl Pool {
    fn new() -> Pool {
        Pool {
            counts: [0; RESOURCES]
        }
    }
    fn can_pay(&self, cost: &Cost) -> bool {
        let mut can = true;
        for r in cost {
            can &= self.counts[r.id()] >= r.value().abs();
        }
        can
    }
    fn pay(&mut self, cost: &Cost) {
        for r in cost {
            self.counts[r.id()] -= r.value();
        }
    }
    fn income(&mut self, element: &Resource) {
        self.counts[element.id()] += element.value();
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    class: Resource,
}

impl Robot {
    fn new(resource: Resource) -> Robot {
        Robot { class: resource }
    }
    fn produce(&self) -> Resource {
        self.class
    }
}

#[derive(Debug, Clone, Copy)]
struct Hangar {
    robots: [i32; RESOURCES]
}

impl Hangar {
    fn new() -> Hangar {
        Hangar { robots: [1, 0, 0, 0] }
    }
    fn produce(&mut self, pool: &mut Pool) {
        pool.income(&Robot::new(Resource::Ore(self.robots[Resource::Ore(0).id()])).produce());
        pool.income(&Robot::new(Resource::Clay(self.robots[Resource::Clay(0).id()])).produce());
        pool.income(&Robot::new(Resource::Obsidian(self.robots[Resource::Obsidian(0).id()])).produce());
        pool.income(&Robot::new(Resource::Geode(self.robots[Resource::Geode(0).id()])).produce());
    }
    fn add(&mut self, robot: Robot) {
        self.robots[robot.class.id()] += 1;
    }
}

type Cost = Vec<Resource>;

type Blueprint = [Cost; RESOURCES];

#[derive(Debug, Clone, Copy)]
struct Factory<'a> {
    blueprint: &'a Blueprint
}

impl<'a> Factory<'a> {
    fn can_build(&self, class: Resource, pool: &Pool) -> bool {
        pool.can_pay(&self.blueprint[class.id()])
    }
    fn build(&self, class: Resource, pools: &mut Pool, hangar: &mut Hangar) {
        pools.pay(&self.blueprint[class.id()]);
        hangar.add(Robot::new(class));
    }
}

#[derive(Debug, Clone, Copy)]
struct Game<'a> {
    factory: Factory<'a>,
    pool: Pool,
    hangar: Hangar
}


fn geodes_opened(bl: &Blueprint) -> usize {
    let mut game = Game {
        factory: Factory { blueprint: bl },
        pool: Pool::new(),
        hangar: Hangar::new()
    };
    for _ in 1..=24 {
        game.hangar.produce(&mut game.pool);
        //factory.build
    }
    0
}


fn main() {
    let mut blueprints = Vec::new();
    while let Some(line) = common::read_line() {
        let spl = line.split(" ").collect::<Vec<&str>>();
        let bl = [
                vec![ Resource::Ore(spl[6].parse::<i32>().unwrap()) ],
                vec![ Resource::Ore(spl[12].parse::<i32>().unwrap()) ],
                vec![ Resource::Ore(spl[18].parse::<i32>().unwrap()), Resource::Clay(spl[21].parse::<i32>().unwrap()) ],
                vec![ Resource::Ore(spl[27].parse::<i32>().unwrap()), Resource::Obsidian(spl[30].parse::<i32>().unwrap()) ]
        ];
        blueprints.push(bl);
    }
    println!("Geodes opened: {:?}", blueprints.iter().map(|bl| geodes_opened(bl)).collect::<Vec<usize>>());
}