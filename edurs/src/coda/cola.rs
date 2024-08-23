/* Hustler's Project
 * --------------------------------------------------------------
 */
mod crook {
    pub struct Hood {
        pub blkid: u32,      /* public field */
        blkname: String,     /* private field */
    }

    impl Hood {
        pub fn from_same_hood(&self, other: &Self) -> bool {
            self.blkid == other.blkid
        }

        pub fn show_blk(&self) {
            println!("The block {} - {}",
                self.blkid,
                self.blkname);
        }

        pub fn set_blk(id: u32, name: String) -> Hood {
            Hood {
                blkid: id,
                blkname: name,
            }
        }
    }
}

use crook::Hood;

pub fn check_spot() {
    let blk_0 = Hood::set_blk(112, "Tech Zone".to_string());
    let blk_1 = Hood::set_blk(101, "Wu Hou Destrict".to_string());

    blk_0.show_blk();
    blk_1.show_blk();

    println!("They're from the same hood ? {}",
        blk_0.from_same_hood(&blk_1));
}
