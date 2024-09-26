
pub struct Mahsulot {
    pub id: u32,
    pub nomi: String,
    pub narxi: u32
}
impl Mahsulot {
    pub fn new(id: u32, nomi: String, narxi: u32) -> Self {
        Mahsulot { id, nomi, narxi }
    }
}

pub struct Buyurtma {
    pub user_id: i64,
    pub mahsulot_nomi: String,
    pub miqdori: String
}
impl Buyurtma {
    pub fn new(user_id: i64, mahsulot_nomi: String, miqdori: String) -> Self {
        Buyurtma { user_id, mahsulot_nomi, miqdori }
    }
}