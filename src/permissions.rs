use thiserror::Error;

/// The symbolic representations for each digit representation. To get a symbol, use the digit as
/// index. For example, to get "r--" use 4.
const SYMBOLIC: &[&str] = &["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];

/// The numeric value of a single symbol, where r = 4, w = 2, x = 1. The sum of all symbols is the
/// octal representation. For example, "rw-" sums to 6.
const RWX: [u8; 3] = [4, 2, 1];

/// Errors during permission conversions.
#[derive(Error, Debug, PartialEq)]
pub enum PermissionError {
    #[error("the permission `{0}` is not a valid symbolic representation")]
    InvalidSymbolic(String),
    #[error("the permission `{0}` is not a valid octal representation")]
    InvalidOctal(String),
}

/// Result type for permission errors.
pub type Result<T> = std::result::Result<T, PermissionError>;

/// A Permission represents a unix file permission in octal form.
/// The `is_dir` field is used when reading a symbolic representation, otherwise it is false.
#[derive(Debug, Default)]
pub struct Permission {
    is_dir: bool,
    owner: u8,
    group: u8,
    other: u8,
}

impl Permission {
    /// Convert a symbolic representation to a Permission struct.
    ///
    /// # Examples
    ///
    /// "drwxr--r--" = Permission { is_dir: true, owner: 7, group: 4, other: 4}
    pub fn from_symbolic(input: String) -> Result<Self> {
        if input.chars().count() != 10 {
            return Err(PermissionError::InvalidSymbolic(input));
        }
        let text_vec = input.chars().collect::<Vec<_>>();
        // grab directory and chunk into threes
        let dir = text_vec[0];
        let owner = text_vec[1..4].iter();
        let group = text_vec[4..7].iter();
        let other = text_vec[7..10].iter();

        Ok(Permission {
            is_dir: dir == 'd',
            owner: Permission::convert_rwx(owner),
            group: Permission::convert_rwx(group),
            other: Permission::convert_rwx(other),
        })
    }

    /// Convert an iterator of a single permission entity to a single digit.
    ///
    /// # Examples
    ///
    /// ['-','-','-'] = 0
    /// ['r','-','x'] = 5
    fn convert_rwx<'a, I>(rwx: I) -> u8
    where
        I: Iterator<Item = &'a char>,
    {
        rwx.enumerate()
            .map(|(i, d)| match d {
                '-' => 0,
                _ => RWX[i],
            })
            .sum()
    }

    /// Create a Permission struct from an octal permission representation. The `is_dir` field will
    /// be false.
    ///
    /// # Examples
    ///
    /// 640 = Permission{is_dir: false, owner: 6, group: 4, other: 0}
    pub fn from_octal(input: u16) -> Result<Self> {
        if input > 777 {
            return Err(PermissionError::InvalidOctal(input.to_string()));
        }

        // Parse the input digit by digit, from right to left.
        let mut i = input;
        let other = (i % 10) as u8;
        i /= 10;
        let group = (i % 10) as u8;
        i /= 10;
        let owner = (i % 10) as u8;

        Ok(Permission {
            is_dir: false,
            other,
            group,
            owner,
        })
    }

    /// Convert to octal representation.
    /// "640"
    pub fn to_octal(&self) -> String {
        format!("{}{}{}", self.owner, self.group, self.other)
    }

    /// Convert to symbolic representation.
    /// "-rw-r-----"
    pub fn to_symbolic(&self) -> String {
        let dir = match self.is_dir {
            true => "d",
            false => "-",
        };
        format!(
            "{}{}{}{}",
            dir,
            SYMBOLIC[self.owner as usize],
            SYMBOLIC[self.group as usize],
            SYMBOLIC[self.other as usize]
        )
    }
}

#[test]
fn test_permission_from_symbolic() {
    struct TestCase<'a> {
        input: &'a str,
        want: &'a str,
    }
    let tests = vec![
        TestCase {
            input: "drwxrwxr-x",
            want: "775",
        },
        TestCase {
            input: "-rw-r-----",
            want: "640",
        },
        TestCase {
            input: "drwxr--r--",
            want: "744",
        },
        TestCase {
            input: "--w-r-----",
            want: "240",
        },
        TestCase {
            input: "----------",
            want: "000",
        },
        TestCase {
            input: "-----",
            want: "",
        },
    ];
    for t in tests {
        let perm = Permission::from_symbolic(t.input.to_string());
        let perm = match perm {
            Ok(p) => p,
            Err(e) => {
                assert_eq!(e, PermissionError::InvalidSymbolic(t.input.to_string()));
                continue;
            }
        };

        let octal = perm.to_octal();
        assert_eq!(t.want, octal);

        let symbolic = perm.to_symbolic();
        assert_eq!(t.input, symbolic);
    }
}

#[test]
fn test_permission_from_octal() {
    struct TestCase<'a> {
        input: &'a str,
        want: &'a str,
    }
    let tests = vec![
        TestCase {
            input: "775",
            want: "-rwxrwxr-x",
        },
        TestCase {
            input: "640",
            want: "-rw-r-----",
        },
        TestCase {
            input: "240",
            want: "--w-r-----",
        },
        TestCase {
            input: "000",
            want: "----------",
        },
        TestCase {
            input: "778",
            want: "",
        },
    ];
    for t in tests {
        let i = t.input.parse().unwrap();
        let perm = Permission::from_octal(i);
        let perm = match perm {
            Ok(p) => p,
            Err(e) => {
                assert_eq!(e, PermissionError::InvalidOctal(t.input.to_string()));
                continue;
            }
        };

        let octal = perm.to_octal();
        assert_eq!(t.input, octal);

        let symbolic = perm.to_symbolic();
        assert_eq!(t.want, symbolic);
    }
}
