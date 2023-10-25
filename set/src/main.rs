fn main() {
    // 创建一个集合
    let groups: Vec<i32> = Vec::new();

    // 创建并初始化一个集合
    let mut groups = vec![1,2,3];

    // 向集合中添加元素
    groups.push(4);

    // 访问集合元素
    println!("集合第一项：{}", &groups[0]);
    // 判断元素是否存在于集合中
    println!("第一项元素是否存在于集合中：{:?}", groups.get(0));

    // 禁止在存在仍需要使用的集合引用时改变集合的大小，因为此时集合的地址已经重新分配了
    let ele = &groups[0];
    groups.push(7);
    // println!("ele: {}", ele);

    // 遍历集合中的元素
    for ele in &mut groups {
        *ele += 1;
    }
    println!("groups: {:?}", groups);

    // 集合默认只能存储同一种类型的元素，但可以使用枚举来让它存储多种类型的元素
    #[derive(Debug)]
    enum DataStructure {
        Int(i32),
        Float(f32),
        Double(f64)
    }
    let data = vec![DataStructure::Int(1),DataStructure::Float(1.0),DataStructure::Double(1.000000000000000000000000)];
    println!("data: {:?}", data);
}
