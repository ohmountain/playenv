use std::env;
use std::fmt;
use std::collections::BTreeMap;

struct PathRes {
    res: BTreeMap<String, String>
}

impl fmt::Display for PathRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.res.len() == 0 {
            try!(writeln!(f, "Play Enviroments"));
            try!(writeln!(f, ""));
            try!(writeln!(f, "Useage:"));
            try!(writeln!(f, "     playenv <args>"));

            try!(writeln!(f, ""));
            try!(writeln!(f, "Example:"));
            try!(writeln!(f, "     playenv HOME ANDROID_HOME"));

            try!(writeln!(f, "     ["));
            try!(writeln!(f, "         ANDROID_HOME ==> /home/renshan/.android"));
            try!(writeln!(f, "                 HOME ==> /home/renshan"));
            try!(writeln!(f, "     ]"));

            return writeln!(f, "");
        }

        try!(writeln!(f, ""));
        try!(writeln!(f, "["));

        let mut max_len = 0;

        for (k, _) in &self.res {
            if k.len() > max_len {
                max_len = k.len();
            }
        }

        for (k, v) in &self.res {
            try!(writeln!(f, "    {k:>0max_len$} ==> {v}", k=k, max_len=max_len, v=v));
        }

        writeln!(f, "]")
    }
}

fn main() {
    let args: Vec<String> = get_args();

    let mut res: PathRes = PathRes { res: BTreeMap::new() };


    for arg in &args {
        res.res.insert(arg.clone(), get_var(arg));
    }

    println!("{}", res);
}

fn get_args() -> Vec<String> {
    env::args().skip(1).map(|x|{
        x
    }).collect()
}

fn get_var(var: &String) -> String {
    match env::var(var) {
        Ok(val) => val,
        Err(_)  => String::from("None")
    }
}

#[cfg(test)]
mod test {

    use get_var;
    use get_args;
    use std::env;

    #[ test ]
    fn test_get_args() {
        assert_eq!(true, 0 == get_args().len());
    }

    #[ test ]
    fn test_get_var() {
        assert_eq!(String::from("None"), get_var(&("Fuck".into())));
        assert_eq!(env::var("HOME").unwrap_or("None".into()), get_var(&("HOME".into())));
    }
}
