static mut STASH: &i32 = &128;

fn f<'a>(p: &'static i32)
{
  unsafe {
    STASH = p;
  }
}

fn smallest(v: &[i32]) -> &i32 {
  let mut s = &v[0];
  for r in &v[1..] {
    if *r < *s { s = r; }
  }
  s
}

struct S<'a>{
  r: &'a i32
}


fn main()
{
  let s: i32;
  let s1;
  
  {
    let x = 10;
    s1 = S{ r: &x };
  }
  {
    let parabola = [9,4,1,0,1,4,9];
    let s = smallest(&parabola);
    println!("S: {}", *s);
  }
}

