use cilly::lexer::*;

fn main() {
    let test = 
    r#"
fun fact(n){
    if(n==0)
        return 1;
    else
        return n * fact(n-1);
}
print(fact(10));
fun k(x){
    fun ky(y){
        return x + y;
    }
    return ky;
}
var ky = k(3);
print(ky(5));
fun fib0(n){
    if(n < 2)
        return n;
    else
        return fib0(n-1) + fib0(n-2);
}
fun fib(n){
    var f0 = 0;
    var f1 = 1;
    while(n > 0){
        var t = f1;
        f1 = f0 + f1;
        f0 = t;
        n = n - 1;
    }
    return f0;
}
print(fib(10),"hello world");
fun make_count(n){
    fun inc(){
        n = n + 1;
        return n;
    }
    return inc;
}
fun make_dog(){
    var weight = 10;
    fun eat(m){
        weight = m + weight;        
    }
    fun get(){
        return weight;
    }
    fun dispatch(m){
        if(m == "eat"){
            return eat;
        } else if (m == "get"){
            return get();
        }
    }
    return dispatch;
}
var dog = make_dog();
var eat = dog("eat");
eat(10);
print(dog("get"));
eat(20);
print(dog("get"));
var c1 = make_count(1);
var c2 = make_count(1);
print(c1(), c1(), c1(), c2());
print(2*10!);
    "#;
    let input = "fun add(a, b) { return a + b; }";
    
    let mut lexer = Lexer::new(test);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }
    println!("{:?}", tokens);
}