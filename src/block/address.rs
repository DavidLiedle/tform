//! Address composite block.

use crate::block::Block;
use crate::field::{Field, Select, TextInput};
use crate::validation::rules::Pattern;

/// US state abbreviations.
const US_STATES: &[(&str, &str)] = &[
    ("AL", "Alabama"),
    ("AK", "Alaska"),
    ("AZ", "Arizona"),
    ("AR", "Arkansas"),
    ("CA", "California"),
    ("CO", "Colorado"),
    ("CT", "Connecticut"),
    ("DE", "Delaware"),
    ("FL", "Florida"),
    ("GA", "Georgia"),
    ("HI", "Hawaii"),
    ("ID", "Idaho"),
    ("IL", "Illinois"),
    ("IN", "Indiana"),
    ("IA", "Iowa"),
    ("KS", "Kansas"),
    ("KY", "Kentucky"),
    ("LA", "Louisiana"),
    ("ME", "Maine"),
    ("MD", "Maryland"),
    ("MA", "Massachusetts"),
    ("MI", "Michigan"),
    ("MN", "Minnesota"),
    ("MS", "Mississippi"),
    ("MO", "Missouri"),
    ("MT", "Montana"),
    ("NE", "Nebraska"),
    ("NV", "Nevada"),
    ("NH", "New Hampshire"),
    ("NJ", "New Jersey"),
    ("NM", "New Mexico"),
    ("NY", "New York"),
    ("NC", "North Carolina"),
    ("ND", "North Dakota"),
    ("OH", "Ohio"),
    ("OK", "Oklahoma"),
    ("OR", "Oregon"),
    ("PA", "Pennsylvania"),
    ("RI", "Rhode Island"),
    ("SC", "South Carolina"),
    ("SD", "South Dakota"),
    ("TN", "Tennessee"),
    ("TX", "Texas"),
    ("UT", "Utah"),
    ("VT", "Vermont"),
    ("VA", "Virginia"),
    ("WA", "Washington"),
    ("WV", "West Virginia"),
    ("WI", "Wisconsin"),
    ("WY", "Wyoming"),
    ("DC", "District of Columbia"),
];

/// A composite block for US addresses.
pub struct AddressBlock {
    prefix: String,
    title: Option<String>,
    required: bool,
}

impl AddressBlock {
    /// Creates a new address block with the given prefix.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            title: None,
            required: false,
        }
    }

    /// Sets the block title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Marks all fields in this block as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    fn field_id(&self, name: &str) -> String {
        format!("{}_{}", self.prefix, name)
    }
}

impl Block for AddressBlock {
    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn fields(&self) -> Vec<Box<dyn Field>> {
        let mut fields: Vec<Box<dyn Field>> = Vec::new();

        // Street Address 1
        let mut street1 = TextInput::new(self.field_id("street1"), "Street Address")
            .placeholder("123 Main St");
        if self.required {
            street1 = street1.required();
        }
        fields.push(Box::new(street1));

        // Street Address 2
        let street2 = TextInput::new(self.field_id("street2"), "Address Line 2")
            .placeholder("Apt, Suite, Unit, etc. (optional)");
        fields.push(Box::new(street2));

        // City
        let mut city = TextInput::new(self.field_id("city"), "City")
            .placeholder("City");
        if self.required {
            city = city.required();
        }
        fields.push(Box::new(city));

        // State
        let mut state = Select::new(self.field_id("state"), "State");
        for (abbr, name) in US_STATES {
            state = state.option(*abbr, format!("{} ({})", name, abbr));
        }
        if self.required {
            state = state.required();
        }
        fields.push(Box::new(state));

        // ZIP Code
        let mut zip = TextInput::new(self.field_id("zip"), "ZIP Code")
            .placeholder("12345 or 12345-6789")
            .validator(Box::new(Pattern::zip_code()));
        if self.required {
            zip = zip.required();
        }
        fields.push(Box::new(zip));

        fields
    }
}
