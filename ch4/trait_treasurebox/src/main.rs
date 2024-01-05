trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

struct JewelyBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelyBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }

    fn check(&self) {
        println!("宝箱の中身は、{}円相当の宝石です。", self.price);
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    fn open(&self, _key: i32) -> bool {
        return true;
    }
    fn check(&self) {
        println!("罠だった！{}のダメージを受けた！", self.damage);
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("宝箱は開かなかった。");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelyBox{price: 10000, key_no: 1};
    let box2 = TrapBox{damage: 100};
    let box3 = JewelyBox{price: 20, key_no: 2};
    let my_key = 1;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}