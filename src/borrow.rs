use std::io;



/*
為了保障記憶體安全所以有了ownership的觀念,該釋放時就會自動釋放

但 move(shallow copy) ownership太麻煩了且我們不希望heap被release掉
1. 我們可以使用.clone()
2. borrow by & or &mut -->如此一來pointer釋放掉後原本的heap仍具有ownership 
在任何時候，我們要嘛只能有一個&mut，要嘛可以有任意數量的&。
&必須永遠有效。

*/

fn main() {
    let heap = String::from("dynamic String object memory space");
    let h = takes_and_gives_back(heap);
    takes_ownership(h);
    


    let stack = "fixed string array memory space";
    makes_copy(stack);
    let _stack2 = stack;
    let _stack3 = stack;
    // stack = "new string array value";
    /* ^^^ cannot assign twice to immutable variable */

/* reference and borrow */

    let mut name = String::from("taiwan");
    borrow_modify(&mut name); /* passed by reference */

    println!("new name is: {}", name);

    let us_name = String::from("United States of America");
    let length = borrow_peek(&us_name);  
    println!("length of usa is {}", length);



/* avoid race condiction */
    {
    let _r1 = &name;
    }

    let _r2 = &mut name;
  
    
}

fn takes_ownership(owned_str: String){
    println!("{}",owned_str);
    
    
}


fn makes_copy(s: &str){
    println!("the copy of stack is heare :{}", s);
    // s = "new string array value modified in function scope";
    /* ^^^cannot assign to immutable argument */

}

fn takes_and_gives_back(s: String)-> String{
    s /* return */
}

fn borrow_modify(s: &mut String){
    s.push_str(", china");
    /* s is a pointer */

}

fn borrow_peek(s: &String) -> usize {
    s.len()
}











