use crate::generated::types::{
    PnAssetChangeEnumeration, PnAssetTypeEnumeration, PnChannelAccumulativeEnumeration,
    PnChannelDirectionEnumeration, PnChannelMaintenanceEnumeration, PnChannelSpecifierEnumeration,
    PnChannelTypeEnumeration,
};

// Defaults generally have to be implemented manually if they are needed. It's usually domain
// specific to guess what reasonable default values actually are, using 0 just isn't always safe or
// possible.
impl Default for PnAssetChangeEnumeration {
    fn default() -> Self {
        Self::INSERTED
    }
}

impl Default for PnChannelAccumulativeEnumeration {
    fn default() -> Self {
        Self::SINGLE
    }
}

impl Default for PnChannelDirectionEnumeration {
    fn default() -> Self {
        Self::MANUFACTURER_SPECIFIC
    }
}

impl Default for PnChannelMaintenanceEnumeration {
    fn default() -> Self {
        Self::FAULT
    }
}

impl Default for PnChannelTypeEnumeration {
    fn default() -> Self {
        Self::UNSPECIFIC
    }
}

impl Default for PnChannelSpecifierEnumeration {
    fn default() -> Self {
        Self::ALL_DISAPPEARS
    }
}

impl Default for PnAssetTypeEnumeration {
    fn default() -> Self {
        Self::DEVICE
    }
}
