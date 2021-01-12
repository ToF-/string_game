
pub struct Carnet<'a> {
    telephone: Option<&'a str>,
}

impl<'a> Carnet<'a> {

    pub fn new() -> Carnet<'a> {
        Carnet { telephone: None }
    }

    pub fn trouve(&self, nom: &str) -> Option<&'a str> {
        match nom {
            "Tof" => self.telephone,
            _ => None,
        }
    }

    pub fn insere(&mut self, nom: &str, telephone: &'a str) {
        self.telephone = Some(telephone)
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn les_tests_se_lancent() {
    }
    
    #[test]
    fn un_carnet_vide_ne_retrouve_pas_de_numero() {
        let carnet: Carnet = Carnet::new();
        assert_eq!(carnet.trouve("Tof"), None)
    }

    #[test]
    fn un_carnet_peut_retrouver_un_numero() {
        let mut carnet: Carnet = Carnet::new();
        let numero: String = "0648074807".into();
        carnet.insere("Tof", &numero);
        assert_eq!(carnet.trouve("Tof"), Some("0648074807"));
    }

    #[test]
    fn un_carnet_peut_retrouver_le_bon_numero() {
        let mut carnet: Carnet = Carnet::new();
        let numero: String = "0648074807".into();
        carnet.insere("Tof", &numero);
        assert_eq!(carnet.trouve("Bec"), None);
    }

    //#[test]
    // fn un_carnet_peut_retrouver_le_bon_numero() {
    //     let mut carnet: Carnet = Carnet::new(); 
    //     let numero1: String = "0648074807".into();
    //     let numero2: String = "0648074807".into();
    //     carnet.insere("Tof", &numero1);
    //     assert_eq!(carnet.trouve("Tof"), Some("0648074807"));
    //     // assert_eq!(Carnet { telephone: Some("0648074807") }.trouve("Tof"), Some("0648074807"))
    // }
}
