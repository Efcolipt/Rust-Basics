#![allow(unused)]

fn f1() -> Result<u32, String> {
    println!("f1");
    // Err("error".to_string())
    Ok(1)
}

fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}

fn f3() -> Result<u32, bool> {
    println!("f3");
    Ok(3)
}

fn f_match() -> Result<u32, String> {
    let res1 = f1();
    let x1 = match res1 {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };

    let res2 = f2();
    let x2 = match res2 {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(x1 + x2)
}

fn f_question() -> Result<u32, String> {
    let x1 = f1()?;
    let x2 = f2()?;

    let res3 = f3();
    
    let x3 = match res3 {
        Ok(x) => x,
        Err(err) => {
            return Err("f3 error".to_string());
        }
    };

    Ok(x1 + x2)
}

fn main() -> Result<(), String> {
    // let res = f1();

    // match res {
    //     Ok(x) => println!("x = {x}"),
    //     Err(err) => println!("{:?}", err),
    // }

    let z = f_question();

    match z {
        Ok(val) => println!("z = {val}"),
        Err(err) => println!("{:?}", err),
    }

    let x = f1()?;

    println!("x = {x}");

    Ok(())
}
