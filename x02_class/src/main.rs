mod cls; // 模块名来自于那个文件名
use cls::ClassName; // 导出来的类

fn main() {
    let object = ClassName::new(1024);
    object.public_method();  // 调用了公共方法
}