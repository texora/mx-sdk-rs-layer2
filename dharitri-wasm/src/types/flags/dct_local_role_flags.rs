use super::DctLocalRole;
use bitflags::bitflags;

bitflags! {
    pub struct DctLocalRoleFlags: u64 {
        const NONE                  = 0b00000000;
        const MINT                  = 0b00000001;
        const BURN                  = 0b00000010;
        const NFT_CREATE            = 0b00000100;
        const NFT_ADD_QUANTITY      = 0b00001000;
        const NFT_BURN              = 0b00010000;
        const NFT_ADD_URI           = 0b00100000;
        const NFT_UPDATE_ATTRIBUTES = 0b01000000;
        const TRANSFER              = 0b10000000;
    }
}

impl DctLocalRoleFlags {
    pub fn has_role(&self, role: &DctLocalRole) -> bool {
        *self & role.to_flag() != DctLocalRoleFlags::NONE
    }

    pub fn iter_roles(&self) -> impl Iterator<Item = &DctLocalRole> {
        DctLocalRole::iter_all().filter(move |role| self.has_role(role))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_flags_has_role() {
        let flags = DctLocalRoleFlags::MINT;
        assert!(flags.has_role(&DctLocalRole::Mint));
        let flags = DctLocalRoleFlags::MINT | DctLocalRoleFlags::BURN;
        assert!(flags.has_role(&DctLocalRole::Mint));
        let flags = DctLocalRoleFlags::NONE;
        assert!(!flags.has_role(&DctLocalRole::Mint));
        let flags = DctLocalRoleFlags::BURN;
        assert!(!flags.has_role(&DctLocalRole::Mint));
    }

    #[test]
    fn test_flags_iter_role() {
        let flags = DctLocalRoleFlags::MINT;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&DctLocalRole>>(),
            alloc::vec![&DctLocalRole::Mint],
        );

        let flags = DctLocalRoleFlags::MINT | DctLocalRoleFlags::BURN;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&DctLocalRole>>(),
            alloc::vec![&DctLocalRole::Mint, &DctLocalRole::Burn],
        );

        let flags = DctLocalRoleFlags::NONE;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&DctLocalRole>>(),
            Vec::<&DctLocalRole>::new(),
        );
    }
}
