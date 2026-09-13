//! #70 wardrobe catalog. Names only. Not a VRM asset pack.

use crate::GaianError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutfitCategory {
    Casual,
    Professional,
    Cultural,
    Athletic,
    Fantasy,
    CustomFromOwnPhoto,
}

impl OutfitCategory {
    pub fn all() -> [OutfitCategory; 6] {
        [
            Self::Casual,
            Self::Professional,
            Self::Cultural,
            Self::Athletic,
            Self::Fantasy,
            Self::CustomFromOwnPhoto,
        ]
    }

    pub fn sample_item(self) -> &'static str {
        match self {
            Self::Casual => "plain-shirt",
            Self::Professional => "plain-jacket",
            Self::Cultural => "owner-provided-only",
            Self::Athletic => "plain-kit",
            Self::Fantasy => "plain-cloak",
            Self::CustomFromOwnPhoto => "owner-garment",
        }
    }
}

pub fn load_custom(owner_garment_photo: bool) -> Result<&'static str, GaianError> {
    if !owner_garment_photo {
        return Err(GaianError::NotSelf);
    }
    Ok("owner-garment")
}

pub fn cultural_preset_warning() -> &'static str {
    "Do not ship stereotyped cultural presets. Head coverings are first-class items. Cultural dress is owner-chosen, not a costume default."
}
