use super::entity::Ship;
use super::parse::Decodable;

#[derive(PartialEq, Debug)]
pub struct Player {
    pub id: i32,
    pub ships: Box<[Ship]>,
}

impl Player {
    pub fn all_ships(&self) -> &[Ship] {
        &self.ships
    }

    pub fn get_ship(&self, ship_id: i32) -> Option<&Ship> {
        for ship in self.all_ships() {
            if ship.id == ship_id {
                return Some(ship)
            }
        }
        None
    }
}

impl Decodable for Player {
    fn parse<'a, I>(tokens: &mut I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {

        let id = i32::parse(tokens);
        let ships = Box::parse(tokens);

        Self { id, ships }
    }
}
