use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, ContentArrangement, Row, Table};

pub trait TableDisplay {
    fn init() -> Self;
    fn set_headers<T: Into<Row>>(&mut self, rows: T) -> &mut Self;
    fn rows<T: ToString>(&mut self, cells: Vec<T>) -> &mut Self;
}

impl TableDisplay for Table {
    fn init() -> Self {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table
    }

    fn set_headers<T: Into<Row>>(&mut self, rows: T) -> &mut Self {
        let cells = rows.into().cell_iter().map(move |cell|
            cell.clone().add_attribute(comfy_table::Attribute::Italic)
                .add_attribute(comfy_table::Attribute::Bold)
        ).collect::<Vec<Cell>>();
        self.set_header(Row::from(cells));
        self
    }

    fn rows<T: ToString>(&mut self, cells: Vec<T>) -> &mut Self {
        self.add_row(cells.into_iter().map(|x| Cell::new(x.to_string())).collect::<Vec<Cell>>());
        self
    }
}