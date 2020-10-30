// use std::io::stdin;
use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;


fn main() -> std::io::Result<()>  {
    let args = std::env::args(); // 透过环境变量，提取参数
    println!("{:?}", args);
    // 测试提取环境参数 cargo run aaa bbb ccc

    // let mut str_buf = String::new();
    // stdin().read_line(&mut str_buf)
    //     .expect("Failed to read line.");
    // println!("Your input line is \n{}", str_buf);
    // 引用 stdin库，获取输入字符串

    let pwd = std::env::current_dir();
    println!("{:?}", pwd); // 返回的是 Ok(())
    let dir = std::env::current_dir().unwrap();
    println!("{}", dir.display());
    let test_file = dir.display().to_string() + &String::from("/../test_file/test.txt");
    println!("{}", test_file);
    // 获取$PWD，拼接测试文件路径。

    let text = fs::read_to_string(&test_file).unwrap();
    println!("{}", text);
    // fs库读取文件，

    let content = fs::read(&test_file).unwrap();
    println!("{:?}", content);
    // 二进制方式读取，输出

    let mut buffer = [0u8; 5]; // 缓冲区是一个 u8 数组
    let mut file = fs::File::open(&test_file).unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    // buffer方式读取文件, 加了一个库 use std::io::prelude::*;


    let test_file = dir.display().to_string() + &String::from("/../tmp4TestFile.txt");
    println!("{}", test_file);
    // 新的测试文件，测试写入操作

    fs::write(&test_file, "this will write into file").unwrap();
    // 直接写入 - 一次性写入 直接删除文件内容 不存在就创建

    let mut file = fs::File::create(&test_file).unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
    // 流方式写入 ，结果同上，直接覆盖, 

    // File 类中不存在 append 静态方法，但是我们可以使用 OpenOptions 来实现用特定方法打开文件
    let mut file2 = OpenOptions::new().append(true).open(&test_file)?;
    file2.write(b" APPEND WORD")?;
    // Ok(())
    // 对文件追加内容
    // 问题1： 文件操作有异常问题，所以main上需要抛出对应的Result
    // 问题2：OK(()) 就是异常的返回，这里就是么有分号结尾
    // 问题3：55，56行上的末尾问号，是异常甩给最后OK的意思

    // 测试继续
    let mut file = OpenOptions::new().read(true).write(true).open(&test_file)?;
    file.write(b"COVER")?;
    Ok(())
    // 这次字符串从头写入，还用到了权限，read write -> 结果 是 “COVERRUST PROGRAM APPEND WORD”
}
