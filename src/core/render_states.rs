use napi_derive::napi;

#[napi]
pub enum Cull {
    None,
    Back,
    Front,
    FrontAndBack,
}

#[napi]
pub enum DepthTest {
    Never,
    Less,
    Equal,
    Lequal,
    Greater,
    Notequal,
    Gequal,
    Always,
}
