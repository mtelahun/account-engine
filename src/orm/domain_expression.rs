/// Domain operators.
const DOP_AND: &'static str = "&";

const DOMAIN_OPERATORS: [&'static str; 1] = [DOP_AND];

/// Term Operators
const TOP_EQ: &'static str = "=";
const TOP_NEQ: &'static str = "!=";
const TOP_IN: &'static str = "in";

const TERM_OPERATORS: [&'static str; 3] = [TOP_EQ, TOP_NEQ, TOP_IN];

const DUMMY_LEAF: [bool; 1] = [true];
