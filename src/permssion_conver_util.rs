pub(crate) fn file_type_number_to_symbol(file_type: &str) -> String {
    // returns the file type
    match file_type {
        "040" => "d", // directory
        "120" => "l", // symlink
        "020" => "c", // character device
        "060" => "b", // block device
        "010" => "p", // fifo
        "140" => "s", // socket
        _ => "-",     // 100 for regular file
    }
    .to_string()
}
pub(crate) fn symbol_to_file_type_number(symbol_type: &str) -> String {
    // returns the file type
    match symbol_type {
        "d" => "040", // directory
        "l" => "120", // symlink
        "c" => "020", // character device
        "b" => "060", // block device
        "p" => "010", // fifo
        "s" => "140", // socket
        _ => "100",   // regular file
    }
    .to_string()
}
pub(crate) fn digit_to_permission(num: &str) -> String {
    // returns the corresponding permission where the wildcard maps to 0 or else
    match num {
        "1" => "--x",
        "2" => "-w-",
        "3" => "-wx",
        "4" => "r--",
        "5" => "r-x",
        "6" => "rw-",
        "7" => "rwx",
        _ => "---", // if 0 or else
    }
    .to_string()
}

pub(crate) fn permission_to_digit(perm: &str) -> String {
    // returns the corresponding permission where the wildcard maps to 0 or else
    match perm {
        "--x" => "1",
        "-w-" => "2",
        "-wx" => "3",
        "r--" => "4",
        "r-x" => "5",
        "rw-" => "6",
        "rwx" => "7",
        _ => "0", // if all are empty
    }
    .to_string()
}
