## 装箱(box)
`Box<T>`

装箱将数据存储在堆上，并在栈上保留一个指向堆数据的指针

### 场景
- 拥有一个无法在编译时确定大小的类型, 但又想要在一个要求固定尺寸的上下文环境中使用这个类型的值  --- 指针的大小是固定的
- 需要传递大量数据的所有权, 但又不希望产生大量数据的复制行为时  --- 只复制指针
- 希望拥有一个实现了指定trait的类型值, 但又不关心具体的类型时