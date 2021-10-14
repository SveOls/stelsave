#![feature(int_abs_diff)]

use std::iter::Iterator;
use std::error::Error;
use std::collections::HashMap;

use regex::Regex;
use bmp::{Image, Pixel};

mod file_analysis;
mod fltk_wrapper;
use fltk::{prelude::*, *};

fn main() {
    let save = "2200.01.01";

    let data = Galaxy::new(&mut file_analysis::analyse(save).unwrap()).unwrap();
    let world = World::new(&data);

    fltk_wrapper::test(&world);

}



/// Contains all stuff
#[derive(Debug, Clone)]
pub struct World {
    date: usize,
    systems: Vec<(Star, Vec<usize>)>,
}

impl World {
    fn new(inp: &Galaxy) -> World {
        let mut systems = Vec::new();
        for i in inp.systems.iter().flatten() {
            systems.push(Star::new(&inp.bodies, i));
        }
        World {
            date: inp.time.unwrap(),
            systems
        }
    }
}

#[derive(Debug, Clone)]
struct Star {
    id: usize,
    name: String,
    planets: Vec<Planet>,
    coordinate: (f64, f64)
}

impl Star {
    fn new(bodies: &Option<Vec<Bodies>>, system: &System) -> (Star, Vec<usize>) {
        (Star {
            id:         system.id.clone().unwrap_or(0),
            name:       system.name.clone().unwrap_or(String::new()),
            coordinate: system.coordinate.clone().unwrap_or((0.0, 0.0)),
            planets:    bodies.iter().flatten().filter(|x| x.system == system.id).map(|x| Planet::new(x)).collect()
        }, {
            match &system.hyperlanes {
                Some(a) => a.clone(),
                None => Vec::new()
            }
        })
    }
}

#[derive(Debug, Clone)]
struct Planet {
    id: usize,
    name: String,
    coordinate: (f64, f64),
    typ: BodyType,
}

impl Planet {
    fn new(inp: &Bodies) -> Planet {
        Planet {
            id:         inp.id.clone().unwrap_or(0),
            name:       inp.name.clone().unwrap_or(String::new()),
            coordinate: inp.coordinate.unwrap_or((0.0, 0.0)),
            typ:        inp.planet_type.unwrap_or(BodyType::Placeholder)
        }
    }
}






// struct Saves {
//     saves: Vec<Galaxy>,
//     _endgamecrisis: Option<Crisis>
// }

// impl Saves {
//     fn new() -> Saves {
//         let ret = Saves {
//             saves: Vec::new(),
//             _endgamecrisis: None
//         };
//         ret
//     }
//     fn push(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box::<dyn Error>> {
//         Ok(self.saves.push(Galaxy::new(data)?))
//     }
// }

// enum Crisis {
//     _Placeholder,
// }

struct Galaxy {
    time:       Option<usize>,
    species:    Option<HashMap<Species, String>>,
    empires:    Option<Vec<Empire>>,
    systems:    Option<Vec<System>>,
    bodies:     Option<Vec<Bodies>>,
    pops:       Option<Vec<(Pop, usize)>>,
    deposits:   Option<Vec<Deposit>>,
    version:    Option<[usize; 3],>
}

impl Galaxy {
    fn new(data: &mut impl Iterator<Item = String>) -> Result<Galaxy, Box::<dyn Error>> {
        let id_reg = Regex::new(r#"^version="([0-9A-z'\s.-]+)"$|^date="([0-9.]+)"$"#).unwrap();

        let mut ret = Galaxy {
            time:       None,
            species:    None,
            empires:    None,
            systems:    None,
            bodies:     None,
            pops:       None,
            deposits:   None,
            version:    None
        };



        let mut date;
        let mut datedit;
        let mut version;
        let mut veredit;
        while let Some(a) = data.next() {
            // println!("entering: {}", a);
            if let Some(b) = id_reg.captures(&a) {
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                    version = c.split(|x| x == '.' || x == 'v');
                    version.next();
                    veredit = ret.version.get_or_insert([0, 0, 0]);
                    veredit[0] = version.next().unwrap().parse()?;
                    veredit[1] = version.next().unwrap().parse()?;
                    veredit[2] = version.next().unwrap().parse()?;
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str())) {
                    date = c.split('.');
                    datedit = ret.time.get_or_insert(0);
                    *datedit += (date.next().unwrap().parse::<usize>()? - 2200) * 360;
                    *datedit += (date.next().unwrap().parse::<usize>()? - 1) * 30;
                    *datedit += date.next().unwrap().parse::<usize>()? - 1;
                }
            }
            if ret.time.is_some() && ret.version.is_some() {
                break;
            }
        }





        while let Some(a) = data.next() {
            match a.as_str() {
                "species_db={" => {
                    ret.render_species(data)?
                }
                "pop={" => {
                    ret.render_pops(data)?
                }
                "galactic_object={" => {
                    ret.render_stars(data)?
                }
                "planets={" => {
                    data.next();
                    ret.render_bodies(data)?
                }
                "country={" => {
                    ret.render_empires(data)?
                }
                "deposit={" => {
                    ret.render_deposits(data)?
                }
                _ => {}
            }
        }
        Ok(ret)
    }
    fn render_deposits(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        // println!("yo");
        let mut temp;
        while let Some(a) = Deposit::new(data)? {
            temp = self.deposits.get_or_insert(Vec::new());
            temp.push(a);
        }
        // todo!();
        Ok(())
    }
    fn render_empires(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        // println!("yo");
        let mut temp;
        while let Some(a) = Empire::new(data)? {
            temp = self.empires.get_or_insert(Vec::new());
            temp.push(a);
        }
        // if let Some(a) = &self.empires {
        //     for i in a.iter() {
        //         println!("{:?}", i);
        //     }
        // }
        // todo!();
        Ok(())
    }
    fn render_bodies(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        // println!("yo");
        let mut temp;
        while let Some(a) = Bodies::new(data)? {
            temp = self.bodies.get_or_insert(Vec::new());
            temp.push(a);
        }
        Ok(())
    }
    fn render_stars(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        let mut temp;
        while let Some(a) = System::new(data)? {
            temp = self.systems.get_or_insert(Vec::new());
            temp.push(a);
        }
        // for i in self.systems.iter() {
        //     println!("{:?}, {:?}: \t{:?}", i.id, i.name, i.class);
        // }
        Ok(())
    }
    fn render_species(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        let mut temp;
        while let Some(a) =  Species::new(data)? {
            temp = self.species.get_or_insert(HashMap::new());
            temp.insert(a.0, a.1);
        }
        Ok(())
    }
    fn render_pops(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        let mut temp;
        while let Some(a) = Pop::new(data)? {
            temp = self.pops.get_or_insert(Vec::new());
            temp.push(a);
        }
        Ok(())
    }
}

#[derive()]
struct Empire {
    name:   Option<String>,
    id:     Option<usize>,
    ethos:  Option<Vec<Ethic>>,
    capital: Option<usize>,
    planets: Option<Vec<usize>>,
}

impl Empire {
    fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<Empire>, Box<dyn Error>> {
        let mut ret = Empire {
            name: None,
            id: None,
            ethos: None,
            capital: None,
            planets: None,
        };
        let id_reg = Regex::new(r#"^\t\tname="([0-9A-z'\s-]+)"|^\t([0-9]+)=\{$|ethos=\{|capital=([0-9]+)|owned_planets=\{|\t\t}|\t}|^}"#).unwrap();

        while let Some(a) = data.next() {
            // println!("entering: {}", a);
            if let Some(b) = id_reg.captures(&a) {
                if let Some("\t\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    continue
                }
                if let Some("\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{:?}\n", ret);
                    return Ok(Some(ret))
                }
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{}", a);
                    return Ok(None)
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                    ret.name = Some(c.to_owned());
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    ret.id = Some(c);
                }
                if let Some(c) = b.get(3).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    ret.capital = Some(c);
                }
                if Some("owned_planets={") == b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_owned_planets(data)?;
                }
                if Some("ethos={") == b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_ethos(data)?;
                }
            }

        }
        unreachable!();
    }
    fn to_ethos(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        let id_reg = Regex::new(r#"}"#).unwrap();

        let mut temp;

        while let Some(a) = data.next() {
            if let Some(_) = id_reg.captures(&a) {
                return Ok(())
            }
            temp = self.ethos.get_or_insert(Vec::new());
            temp.append(&mut Ethic::to_methic(&a));
        }
        Ok(())
    }
    fn to_owned_planets(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        if let Some(a) = data.next() {
            if "\t\t\t}" != a {
                self.planets = Some(a.trim().split(' ').map(|x| x.parse().unwrap()).collect());
            }
        }
        Ok(())
    }
}


#[derive(Debug)]
struct System {
    id:         Option<usize>,
    name:       Option<String>,
    bodies:     Option<Vec<usize>>,
    class:      Option<StarType>,
    coordinate: Option<(f64, f64)>,
    hyperlanes: Option<Vec<usize>>
}

impl System {
    fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<System>, Box<dyn Error>> {
        let mut ret = System {
            name:       None,
            bodies:     None,
            class:      None,
            coordinate: None,
            id:         None,
            hyperlanes: None,
        };
        let id_reg = Regex::new(r#"\t\tplanet=([0-9]+)|\t([0-9]+)=\{|\t\tcoordinate=\{|\t\thyperlane=\{|\t\tname="([0-9A-z'\s-]+)"|\t\tstar_class="([0-9_A-z'\s-]+)"|\t\t\t}|\t\t}|\t}|}"#).unwrap();


        let mut temp;
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                if let Some("\t\t}" | "\t\t\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    continue
                }
                if let Some("\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{:?}", ret);
                    return Ok(Some(ret))
                }
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(None)
                }
                if Some("\t\tcoordinate={") ==   b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_coordinate(data);
                }
                if Some("\t\thyperlane={") ==   b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_hyperlanes(data);
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    temp = ret.bodies.get_or_insert(Vec::new());
                    temp.push(c);
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.id = Some(c);
                }
                if let Some(c) = b.get(3).map_or(None, |m| Some(m.as_str().to_owned())) {
                    ret.name = Some(c);
                }
                if let Some(c) = b.get(4).map_or(None, |m| Some(StarType::to_startype(m.as_str()) )) {
                    ret.class = Some(c);
                }
            }

        }
        unreachable!();
    }
    fn to_hyperlanes(&mut self, data: &mut impl Iterator<Item = String>) {
        let id_reg = Regex::new(r#"to=([0-9]+)|^\t\t\}"#).unwrap();

        let mut temp;
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                if  Some("\t\t}") == b.get(0).map_or(None, |m| Some(m.as_str())) {
                    break;
                }
                if let Some(xfir) = b.get(1).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    temp = self.hyperlanes.get_or_insert(Vec::new());
                    temp.push(xfir);
                }
            }

        }
    }
    fn to_coordinate(&mut self, data: &mut impl Iterator<Item = String>) {
        let id_reg = Regex::new(r#"(\})|x=(-?[0-9.]+)|y=(-?[0-9.]+)"#).unwrap();

        let mut temp;
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                if  Some("}") == b.get(1).map_or(None, |m| Some(m.as_str())) {
                    break;
                }
                if let Some(xfir) = b.get(2).map_or(None, |m| Some(m.as_str().parse::<f64>().unwrap())) {
                    temp = self.coordinate.get_or_insert((0.0, 0.0));
                    temp.0 = xfir;
                }
                if let Some(yfir) = b.get(3).map_or(None, |m| Some(m.as_str().parse::<f64>().unwrap())) {
                    temp = self.coordinate.get_or_insert((0.0, 0.0));
                    temp.1 = yfir;
                }
            }

        }
    }
}

#[derive(Debug)]
enum StarType {
    Placeholder,
}

impl StarType {
    fn to_startype(inp: &str) -> StarType {
        match inp {
            _ => StarType::Placeholder
        }
    }
}



struct Bodies {
    system:     Option<usize>,
    coordinate: Option<(f64, f64)>,
    id:         Option<usize>,
    planet_type: Option<BodyType>,
    orbits:     Option<usize>,
    moons:      Option<Vec<usize>>,
    size:       Option<usize>,
    name:       Option<String>,
    deposits:   Option<Vec<usize>>,
    districts:  Option<Vec<District>>,
    pops:       Option<Vec<usize>>,
    owner:      Option<usize>,
}

impl Bodies {
    fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<Bodies>, Box<dyn Error>> {

        let mut ret = Bodies {
            system: None,                      //
            id: None,                          //
            planet_type: None,  //
            orbits: None,
            moons: None,
            size: None,
            name: None,            //
            deposits: None,         //
            districts: None,           //
            coordinate: None,              //
            pops: None,
            owner: None
        };
        let id_reg = Regex::new(r#"^\t\t([0-9]+)=\{|\t\t\tcoordinate=\{|\t\t\tdeposits=\{|\t\t\tmoons=\{|\t\t\tpop=\{|\t\t\tname="([_0-9A-z'\s-]+)"|\t\t\tplanet_class="([A-z'\s-]+)"|\t\t\tplanet_size=([0-9]+)|\t\t\tmoon_of=([0-9]+)|\t\t\t}|\t\t}|\t}|}|owner=([0-9]+)|\t\tdistrict="([0-9_A-z'\s-]+)""#).unwrap();

        // println!("entering");
        let mut temp;
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                // println!("{:?}", b);

                if  Some("\t\t\t}")     == b.get(0).map_or(None, |m| Some(m.as_str())) ||
                    Some("\t\t\t\t}")   == b.get(0).map_or(None, |m| Some(m.as_str()))  {
                    continue
                }
                if Some("\t\t}") ==        b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{:?}", ret);
                    if ret.id > Some(25) {
                        return Ok(None)
                    }
                    return Ok(Some(ret))
                }
                if Some("\t}") ==          b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(None)
                }
                if Some("\t\t\tdeposits={") ==          b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_deposits(data)?
                }
                if Some("\t\t\tcoordinate={") ==          b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_coordinate(data)?
                }
                if Some("\t\t\tmoons={") ==          b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_moons(data)?
                }
                if Some("\t\t\tpop={") ==          b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.to_pops(data)?
                }
                if let Some(c) =            b.get(1).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.id = Some(c)
                }
                if let Some(c) =            b.get(2).map_or(None, |m| Some(m.as_str())) {
                    ret.name = Some(c.to_owned())
                }
                if let Some(c) =            b.get(3).map_or(None, |m| Some(m.as_str())) {
                    ret.planet_type = Some(BodyType::to_bodytype(c))
                }
                if let Some(c) =            b.get(4).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.size = Some(c)
                }
                if let Some(c) =            b.get(5).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.orbits = Some(c)
                }
                if let Some(c) =            b.get(6).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.owner = Some(c)
                }
                if let Some(c) = b.get(7).map_or(None, |m| Some(m.as_str() )) {
                    temp = ret.districts.get_or_insert(Vec::new());
                    temp.push(District::to_district(c));
                }
            }

        }
        unreachable!();
    }
    fn to_coordinate(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box::<dyn Error>> {
        let id_reg =    Regex::new(r#"(\})|x=(-?[0-9.]+)|y=(-?[0-9.]+)|origin=([0-9]+)"#).unwrap();

        let mut temp;
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                if  Some("}") == b.get(1).map_or(None, |m| Some(m.as_str())) {
                    break;
                }
                if let Some(xfir) = b.get(2).map_or(None, |m| Some(m.as_str().parse::<f64>().unwrap())) {
                    temp = self.coordinate.get_or_insert((0.0, 0.0));
                    temp.0 = xfir;
                }
                if let Some(yfir) = b.get(3).map_or(None, |m| Some(m.as_str().parse::<f64>().unwrap())) {
                    temp = self.coordinate.get_or_insert((0.0, 0.0));
                    temp.1 = yfir;
                }
                if let Some(a) = b.get(4).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    self.system = Some(a)
                }
            }

        }



        Ok(())
    }
    fn to_deposits(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box::<dyn Error>> {
        if let Some(a) = data.next() {
            if "\t\t\t}" != a {
                self.deposits = Some(a.trim().split(' ').map(|x| x.parse().unwrap()).collect());
            }
        }
        Ok(())
    }
    fn to_moons(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box::<dyn Error>> {
        if let Some(a) = data.next() {
            if "\t\t\t}" != a {
                self.moons = Some(a.trim().split(' ').map(|x| x.parse().unwrap()).collect());
            }
        }
        Ok(())
    }
    fn to_pops(&mut self, data: &mut impl Iterator<Item = String>) -> Result<(), Box::<dyn Error>> {
        if let Some(a) = data.next() {
            if "\t\t\t}" != a {
                self.pops = Some(a.trim().split(' ').map(|x| x.parse().unwrap()).collect());
            }
        }
        Ok(())
    }
}


#[derive(Debug, Copy, Clone)]
enum BodyType {
    // Asteroid,
    // HabPlanet(HabPlanetType),
    Placeholder,
    // UnInit
}

impl BodyType {
    fn to_bodytype(inp: &str) -> BodyType {
        match inp {
            _ => BodyType::Placeholder
        }
    }
}

#[derive(Debug)]
struct Pop {
    id: usize,
    species: usize,
    ethic: Option<Ethic>,
    enslaved: bool,
    job: Option<Employment>
}

impl Pop {
    fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<(Pop, usize)>, Box<dyn Error>> {
        let id_reg =    Regex::new(r#"\t\tspecies=([0-9]+)|[\t]([0-9]+)=\{|\t\tethos=\{|\t\tenslaved=([a-z]+)|\t\tjob="([A-z\s]+)"|\t\t}|\t}|}|\t\tplanet=([0-9]+)"#).unwrap();
        let mut ret = Pop {
            id: 0,
            species: 0,
            ethic: None,
            enslaved: false,
            job: None
        };

        let mut i = 0;
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                if let Some("\t\t}") =      b.get(0).map_or(None, |m| Some(m.as_str())) {
                    continue
                }
                if let Some("\t}") =        b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(Some((ret, i)))
                }
                if let Some("}") =          b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(None)
                }
                if let Some(c) =            b.get(1).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.species = c
                }
                if let Some(c) =            b.get(2).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.id = c
                }
                if Some("\t\tethos={") ==   b.get(0).map_or(None, |m| Some(m.as_str())) {
                    ret.ethic = Some(Ethic::to_ethic(&data.next().unwrap()));
                }
                if let Some(c) =            b.get(3).map_or(None, |m| Some(match m.as_str() { "no" => false, "yes" => true, _ => panic!() })) {
                    ret.enslaved = c
                }
                if let Some(c) =            b.get(4).map_or(None, |m| Some(Employment::to_employment(m.as_str()))) {
                    ret.job = Some(c)
                }
                if let Some(c) =            b.get(5).map_or(None, |m| Some(m.as_str().parse().unwrap())) {
                    i = c
                }
            }

        }
        unreachable!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Ethic {
    Militarist,
    Pacifist,
    Spiritualist,
    Materialist,
    Egalitarian,
    Authoritarian,
    Xenophile,
    Xenophobe,
    Gestalt,
}

impl Ethic {
    fn to_ethic(inp: &str) -> Ethic {
        let id_reg =    Regex::new(r#"ethic="([a-z_]+)""#).unwrap();
        if let Some(b) = id_reg.captures(inp) {
            if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                match c {
                    "ethic_militarist" =>   Ethic::Militarist,
                    "ethic_pacifist" =>     Ethic::Pacifist,
                    "ethic_spiritualist" => Ethic::Spiritualist,
                    "ethic_materialist" =>  Ethic::Materialist,
                    "ethic_egalitarian" =>  Ethic::Egalitarian,
                    "ethic_authoritarian" => Ethic::Authoritarian,
                    "ethic_xenophile" =>    Ethic::Xenophile,
                    "ethic_xenophobe" =>    Ethic::Xenophobe,
                    _ =>                    panic!("1: {}", inp),
                }
            } else {
                panic!("2: {}", inp);
            }
        } else {

            panic!("3: {}", inp);
        }

    }
    fn to_methic(inp: &str) -> Vec<Ethic> {
        let id_reg =    Regex::new(r#"ethic="([a-z_]+)""#).unwrap();
        if let Some(b) = id_reg.captures(inp) {
            if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                match c {
                    "ethic_militarist" =>   vec![Ethic::Militarist],
                    "ethic_pacifist" =>     vec![Ethic::Pacifist],
                    "ethic_spiritualist" => vec![Ethic::Spiritualist],
                    "ethic_materialist" =>  vec![Ethic::Materialist],
                    "ethic_egalitarian" =>  vec![Ethic::Egalitarian],
                    "ethic_authoritarian" => vec![Ethic::Authoritarian],
                    "ethic_xenophile" =>    vec![Ethic::Xenophile],
                    "ethic_xenophobe" =>    vec![Ethic::Xenophobe],
                    "ethic_fanatic_militarist" =>   vec![Ethic::Militarist, Ethic::Militarist],
                    "ethic_fanatic_pacifist" =>     vec![Ethic::Pacifist, Ethic::Pacifist],
                    "ethic_fanatic_spiritualist" => vec![Ethic::Spiritualist, Ethic::Spiritualist],
                    "ethic_fanatic_materialist" =>  vec![Ethic::Materialist, Ethic::Materialist],
                    "ethic_fanatic_egalitarian" =>  vec![Ethic::Egalitarian, Ethic::Egalitarian],
                    "ethic_fanatic_authoritarian" => vec![Ethic::Authoritarian, Ethic::Authoritarian],
                    "ethic_fanatic_xenophile" =>    vec![Ethic::Xenophile, Ethic::Xenophile],
                    "ethic_fanatic_xenophobe" =>    vec![Ethic::Xenophobe, Ethic::Xenophobe],
                    "ethic_gestalt_consciousness" => vec![Ethic::Gestalt],
                    _ =>                    panic!("1a: {}", inp),
                }
            } else {
                panic!("2a: {}", inp);
            }
        } else {

            panic!("3a: {}", inp);
        }

    }
}


#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Species(usize);

impl Species {
    fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<(Species, String)>, Box::<dyn Error>> {
        let id_reg =    Regex::new(r#"^\t\tname="([A-z'\s-]+)"$|^[\t]([0-9]+)=\{$|^\t}$|^}$"#).unwrap();
        // let name_reg =  Regex::new(\t\tname="([A-z\s]+)").unwrap();
        // let bad =       Regex::new("^\\}$").unwrap();

        // let spec = Species(if let Some(a) = data.next() {
        //     println!("oh: {:?}", id_reg.captures(&a).unwrap());
        //     id_reg.captures(&a).unwrap().get(2).map_or(0, |m| { println!("herte: {:?}", m); m.as_str().parse::<usize>().unwrap()})
        // } else {
        //     return Ok(None);
        // });
        let mut ret = (Species(0), "".to_owned());
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                if let Some("\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(Some(ret))
                }
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(None)
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str())) {
                    ret.1 = c.to_owned()
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    ret.0 = Species(c)
                }
            }

        }
        // return Ok(Some(()))

        // unreachable!();
        Ok(Some(ret))
    }
}

#[derive(Debug)]
enum Employment {
    Placeholder,
    // more
}

impl Employment {
    fn to_employment(inp: &str) -> Employment {
        match inp {
            _ => Employment::Placeholder
        }
    }
}

// #[derive(Debug)]
// enum HabPlanetType {
//     // Dry,
//     // Wet,
//     // Cold,

//     // Ecumenopolis,
//     // Gaia,
//     // Tomb,
//     // Relic,
//     // Hive,
//     // Machine,
//     // Habitat,
//     // Ringworld,

//     // Tropical,
//     // Ocean,
//     // Continental,
//     // Desert,
//     // Savanna,
//     // Arid,
//     // Alpine,
//     // Arctic,
//     // Tundra,

//     Placeholder
// }


#[derive(Debug, PartialEq, Eq, Hash)]
enum District {
    City,
    Farming,
    Generator,
    Industrial,
    Mining,
    Placeholder
    // more
}


// district="district_city"
// district="district_farming"
// district="district_farming"
// district="district_farming"
// district="district_generator"
// district="district_industrial"

impl District {
    fn to_district(inp: &str) -> District {
        match inp {
            "district_city"         => District::City,
            "district_farming"      => District::Farming,
            "district_generator"    => District::Generator,
            "district_industrial"   => District::Industrial,
            "district_mining"       => District::Mining,
            _                       => District::Placeholder
        }
    }
}

struct Deposit {
    id: Option<usize>,
    version: Option<DepositType>,
}

impl Deposit {
    fn new(data: &mut impl Iterator<Item = String>) -> Result<Option<Deposit>, Box::<dyn Error>> {
        let id_reg =    Regex::new(r#"^\t([0-9]+)=\{$|^\t\ttype="([_0-9A-z'\s-]+)"|^\t}$|^\}$"#).unwrap();
        // let name_reg =  Regex::new(\t\tname="([A-z\s]+)").unwrap();
        // let bad =       Regex::new("^\\}$").unwrap();

        // let spec = Species(if let Some(a) = data.next() {
        //     println!("oh: {:?}", id_reg.captures(&a).unwrap());
        //     id_reg.captures(&a).unwrap().get(2).map_or(0, |m| { println!("herte: {:?}", m); m.as_str().parse::<usize>().unwrap()})
        // } else {
        //     return Ok(None);
        // });
        let mut ret = Deposit {
            id: None,
            version: None
        };
        while let Some(a) = data.next() {
            if let Some(b) = id_reg.captures(&a) {
                if let Some("\t}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    // println!("{:?}", ret);
                    return Ok(Some(ret))
                }
                if let Some("}") = b.get(0).map_or(None, |m| Some(m.as_str())) {
                    return Ok(None)
                }
                if let Some(c) = b.get(2).map_or(None, |m| Some(m.as_str())) {
                    ret.version = Some(DepositType::to_deposittype(c))
                }
                if let Some(c) = b.get(1).map_or(None, |m| Some(m.as_str().parse::<usize>().unwrap())) {
                    // if c == 1026 {
                    //     return Ok(None);
                    // }
                    ret.id = Some(c)
                }
            }

        }
        // return Ok(Some(()))

        unreachable!();
        // Ok(Some(ret))
    }
}


#[derive(Debug)]
enum DepositType {
    Placeholder
}

impl DepositType {
    fn to_deposittype(inp: &str) -> DepositType {
        match inp {
            _ => DepositType::Placeholder
        }
    }
}

// impl HabPlanetType {
//     fn new(inp: &str) -> Result<HabPlanetType, Box<dyn Error>> {
//         match inp {
//             _ => todo!()
//         }
//     }
// }


impl std::fmt::Debug for Empire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.id {
            Some(a) => writeln!(f, "Empire {} {{", a)?,
            None    => writeln!(f, "NoID {{")?,
        }
        match &self.name {
            Some(a) => writeln!(f, "\tName: {}", a)?,
            None    => writeln!(f, "\tNo Name")?,
        }
        match &self.ethos {
            Some(a) => {
                writeln!(f, "\tEthos {{")?;
                for i in a.iter() {
                    writeln!(f, "\t\t{:?}", i)?;
                }
                writeln!(f, "\t}}")?;
            },
            None    => writeln!(f, "\tNo Ethos")?,
        }
        match &self.capital {
            Some(a) => writeln!(f, "\tCapital: {}", a)?,
            None    => writeln!(f, "\tNo Capital")?,
        }
        match &self.planets {
            Some(a) => {
                writeln!(f, "\tPlanets {{")?;
                for i in a.iter() {
                    writeln!(f, "\t\t{}", i)?;
                }
                writeln!(f, "\t}}")?;
            },
            None    => writeln!(f, "\tNo Planets")?,
        }

        write!(f, "}}")
    }
}


impl std::fmt::Debug for Bodies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.id {
            Some(a) => writeln!(f, "Body {} {{", a)?,
            None    => writeln!(f, "NoID {{")?,
        }
        match &self.name {
            Some(a) => writeln!(f, "\tName: {}", a)?,
            None    => writeln!(f, "\tNoName")?,
        }
        match &self.pops {
            Some(a) => writeln!(f, "\tPopulation: {}", a.len())?,
            None    => writeln!(f, "\tNoPop")?,
        }
        match &self.owner {
            Some(a) => writeln!(f, "\tOwner: {}", a)?,
            None    => writeln!(f, "\tNoOwner")?,
        }
        match &self.size {
            Some(a) => writeln!(f, "\tSize: {}", a)?,
            None    => writeln!(f, "\tNoSize")?,
        }
        match &self.coordinate {
            Some(a) => writeln!(f, "\tPosition: x={}, y={}", a.0, a.1)?,
            None    => writeln!(f, "\tNoPosition")?,
        }
        match &self.planet_type {
            Some(a) => writeln!(f, "\tType: {:?}", a)?,
            None    => writeln!(f, "\tNoType")?,
        }
        match &self.orbits {
            Some(a) => writeln!(f, "\tOrbiting: {}", a)?,
            None    => writeln!(f, "\tNoOrbit")?,
        }
        match &self.pops {
            Some(a) => {
                writeln!(f, "\tPops {{")?;
                write!(f, "\t\t")?;
                for i in a.iter() {
                    write!(f, "{} ", i)?;
                }
                writeln!(f, "\n\t}}")?;
            }
            None    => writeln!(f, "\tNoPops")?,
        }
        match &self.moons {
            Some(a) => {
                writeln!(f, "\tMoons {{")?;
                write!(f, "\t\t")?;
                for i in a.iter() {
                    write!(f, "{} ", i)?;
                }
                writeln!(f, "\n\t}}")?;
            }
            None    => writeln!(f, "\tOrbitsNothing")?,
        }
        match &self.deposits {
            Some(a) => {
                writeln!(f, "\tDeposits {{")?;
                write!(f, "\t\t")?;
                for i in a.iter() {
                    write!(f, "{} ", i)?;
                }
                writeln!(f, "\n\t}}")?;
            }
            None    => writeln!(f, "\tNoDeposits")?,
        }
        match &self.districts {
            Some(a) => {
                let mut temp: HashMap<&District, usize> = HashMap::new();
                writeln!(f, "\tDistricts {{")?;
                for i in a.iter() {
                    *temp.entry(i).or_insert(0) += 1;
                }
                for (key, val) in temp.iter() {
                    writeln!(f, "\t\t{} {:?}", val, key)?;
                }
                writeln!(f, "\t}}")?;
            }
            None    => writeln!(f, "\tNoDistricts")?,
        }
        write!(f, "}}")
    }
}


impl std::fmt::Debug for Deposit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.id {
            Some(a) => writeln!(f, "Deposit {} {{", a)?,
            None    => writeln!(f, "NoID {{")?,
        }
        match &self.version {
            Some(a) => writeln!(f, "\tType: {:?}", a)?,
            None    => writeln!(f, "\tNo Type")?,
        }
        write!(f, "}}")
    }
}

impl std::fmt::Display for Ethic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Placeholder")
    }
}
