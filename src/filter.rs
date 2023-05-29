pub trait Filter {
    fn filter(&self) -> String;
}

pub struct AndFilter {
    filters: Vec<Box<dyn Filter>>,
}

impl AndFilter {
    pub fn new() -> Self {
        AndFilter {
            filters: Vec::new(),
        }
    }

    pub fn add(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }
}

impl Filter for AndFilter {
    fn filter(&self) -> String {
        let mut filter = String::from("(&");
        for f in &self.filters {
            filter.push_str(&f.filter());
        }
        filter.push(')');
        filter
    }
}

pub struct OrFilter {
    filters: Vec<Box<dyn Filter>>,
}

impl OrFilter {
    pub fn new() -> Self {
        OrFilter {
            filters: Vec::new(),
        }
    }

    pub fn add(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }
}

impl Filter for OrFilter {
    fn filter(&self) -> String {
        let mut filter = String::from("(|");
        for f in &self.filters {
            filter.push_str(&f.filter());
        }
        filter.push(')');
        filter
    }
}

pub struct EqFilter {
    attribute: String,
    value: String,
}

impl Filter for EqFilter {
    fn filter(&self) -> String {
        format!("({}={})", self.attribute, self.value)
    }
}

pub struct NotFilter {
    filter: Box<dyn Filter>,
}

impl NotFilter {
    pub fn new(filter: Box<dyn Filter>) -> Self {
        NotFilter { filter }
    }
}

impl Filter for NotFilter {
    fn filter(&self) -> String {
        format!("(!{})", self.filter.filter())
    }
}

pub struct LikeFilter {
    attribute: String,
    value: String,
}

impl Filter for LikeFilter {
    fn filter(&self) -> String {
        format!("({}~={})", self.attribute, self.value)
    }
}