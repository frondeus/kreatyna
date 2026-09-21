// Compile and run both the macro declarations and their checked-in expansion.
// Some generated helpers are intentionally unused in this focused fixture.
#[allow(dead_code)]
mod declared {
    include!("../fixtures/green_path.rs");

    mod tests {
        include!("green_path_checks.rs");
    }
}

#[allow(dead_code)]
mod expanded {
    include!("../../macros/tests/expanded/green_path.rs");

    mod tests {
        include!("green_path_checks.rs");
    }
}
