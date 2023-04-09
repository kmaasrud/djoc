mod html;
mod pdf;

#[derive(Clone, Default)]
pub struct Builder {
    pub number_sections: bool,
}
