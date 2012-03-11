use socket;
use std;
import std::io::println;

type Response = {status:int,content:str};


fn get(url:str) -> Response {
	ret request(url,"GET");
}

fn post(url:str)
fn request(url:str,method:str) -> Response{
	println("hello world");
	let r:Response = {status:200,content:"hello world"};
	ret r;
}


#[test]
fn test_get() {
	get("http://www.baidu.com");
}

#[test]
fn test_post(){
	println("test post");
}