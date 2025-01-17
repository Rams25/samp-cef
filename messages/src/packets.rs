use crate::impl_into_packet;
pub use crate::proto::packets::*;

impl_into_packet!(RequestJoin, PacketId::REQUEST_JOIN);
impl_into_packet!(JoinResponse, PacketId::JOIN_RESPONSE);
impl_into_packet!(CreateBrowser<'a>, PacketId::CREATE_BROWSER);
impl_into_packet!(DestroyBrowser, PacketId::DESTROY_BROWSER);
impl_into_packet!(EmitEvent<'a>, PacketId::EMIT_EVENT);
impl_into_packet!(HideBrowser, PacketId::HIDE_BROWSER);
impl_into_packet!(FocusBrowser, PacketId::FOCUS_BROWSER);
impl_into_packet!(AlwaysListenKeys, PacketId::ALWAYS_LISTEN_KEYS);
impl_into_packet!(BrowserCreated, PacketId::BROWSER_CREATED);
impl_into_packet!(Got, PacketId::GOT);
impl_into_packet!(OpenConnection, PacketId::OPEN_CONNECTION);
impl_into_packet!(CreateExternalBrowser<'a>, PacketId::CREATE_EXTERNAL_BROWSER);
impl_into_packet!(AppendToObject, PacketId::APPEND_TO_OBJECT);
impl_into_packet!(RemoveFromObject, PacketId::REMOVE_FROM_OBJECT);
impl_into_packet!(ToggleDevTools, PacketId::TOGGLE_DEV_TOOLS);
impl_into_packet!(SetAudioSettings, PacketId::SET_AUDIO_SETTINGS);
