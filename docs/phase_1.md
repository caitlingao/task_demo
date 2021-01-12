## 第一阶段：基本功能开发分析设计
### Task 数据结构

|字段名|字段类型|字段含义说明|默认值|备注|
|-----|------|-----|---|----|
|id|i32|id| |主键，自增长|
|content|String|task 内容| |不允许为空|
|finished|bool|是否完成|false| |
|created_at|datatime|创建时间| |不允许为空|
|updated_at|datatime|修改时间| |不允许为空|

### 实现思路
1. src/models/task.rs 文件定义 Task 结构
```Rust
pub struct Task {
    pub id: i32,
    pub content: String,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
```
2. src/database.rs 文件定义 task 的添加、完成、查看list等方法。
```Rust
pub fn add_task(content: &str) -> Result<()>{
    let id = tasks.len()  as i32 + 1;
    let task = Task::new(content, id);
    tasks.push(task);
}

pub fn finish_task(id: i32) -> Result<()>{
    match tasks.iter_mut().find(|task| task.id == id) {
        Some(task) => {
            task.finished = true;
            task.updated_at = chrono::offset::Utc::now();
        }
        None => {
            println!("task does not exist");
            return Ok(());
        }
    }
}

pub fn get_tasks() -> Result<()> {
    let finished_tasks: Vec<Task> = tasks
        .iter()
        .filter(|task| task.finished)
        .cloned()
        .collect();
    let unfinished_tasks: Vec<Task> = tasks
        .iter()
        .filter(|task| !task.finished)
        .cloned()
        .collect();

    for task in unfinished_tasks.iter() {
        println!("{id}. {content}", id = task.id, content = task.content);
    }

    for task in finished_tasks.iter() {
        println!("{id}. [Done]{content}", id = task.id, content = task.content);
    }
}

pub fn get_unfinished_tasks() -> Result<()> {
    let unfinished_tasks: Vec<Task> = tasks
        .iter()
        .filter(|task| !task.finished)
        .cloned()
        .collect();

    for task in unfinished_tasks.iter() {
        println!("{id}. {content}", id = task.id, content = task.content);
    }
}
```

3. 相关要点：
- 使用tasks.json文件存储添加的 task 数据，json 文件内容结构如下：
```json
[
    {
        "id":1,
        "content":"my first task",
        "finished":false,
        "created_at":"2021-01-12T13:55:04.107825",
        "updated_at":"2021-01-12T13:55:04.107825"
    }
]
```
- 使用 serde_json 解析 json 文件内容，格式化需要修改的数据
```Rust
// 读取文件中数据
let string_data = fs::read_to_string(&path).expect("Unable to read file");
// 解析数据
tasks = serde_json::from_str(&string_data)?;

// 格式化数据
let json = serde_json::to_string_pretty(&tasks)?;
// 写数据进 json 文件
fs::write(&path, &json).expect("Unable write to file");
```
- 查看 list 数据使用 --all 参数时，对 tasks 按照 finished 进行排序，未完成的数据优先展示
- 查看 list 数据时，根据数据的数量返回 items/item
