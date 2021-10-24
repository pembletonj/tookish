
pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize
}

impl TabsState {

    pub fn from_strs(titles: Vec<&str>) -> TabsState {
        TabsState {
            titles: titles.iter().map(|t| {
                String::from(*t)
            }).collect(),
            index: 0
        }
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn next(&mut self) {
        if self.index < self.titles.len() - 1 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

}