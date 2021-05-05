use std::collections::HashMap;
use time::PrimitiveDateTime;

/// Represents any supported by lib platform (messenger, social network, etc).
///
/// It should never be constructed outside of the implementation.
#[derive(PartialEq, Eq, Hash)]
pub struct Platform {
    pub name: &'static str,
}
impl Platform {
    pub const VK : Platform = Platform {
        name: "VK",
    };
    pub const TELEGRAM : Platform = Platform {
        name: "Telegram",
    };
}

/// Platform-dependent id of any entity.
pub struct PlatformId {
    pub value: u128,
    pub platform: &'static Platform,
}
impl PlatformId {
    pub fn new(value: u128, platform: &'static Platform) -> PlatformId {
        return PlatformId { value, platform }
    }
}

/// User of platform(s). Combines accounts from few platforms.
pub struct User {
    id: u128,
    platform_ids: HashMap<&'static Platform, u128>,
    pub username: String,
}
impl User {
    pub fn set_platform_id(&mut self, id: &PlatformId) {
        self.platform_ids.insert(id.platform, id.value);
    }
    pub fn get_platform_id(&self, platform: &'static Platform) -> Option<PlatformId> {
        match self.platform_ids.get(platform) {
            Some(i) => {
                Some(PlatformId { value: *i, platform: platform })
            }
            None => None
        }
    }
}

/// Private or group conversation between users.
pub struct Chat<'a> {
    id: PlatformId,
    pub participants: Vec<&'a User>,
}

/// Text message sent in chat.
pub struct Message<'a> {
    id: PlatformId,
    pub chat: &'a Chat<'a>,
    pub sender: &'a User,
    pub text: String,
    pub datetime: PrimitiveDateTime,
}