## 第四阶段：支持数据库持久化开发分析设计
### 增加TaskDTO数据结构
|字段名|字段类型|字段含义说明|默认值|备注|
|-----|------|-----|---|----|
|user_id|i32|user_id| ||
|content|String|task 内容| |不允许为空|
|finished|bool|是否完成|false| |

### 增加UserDTO数据结构用于插入用户
|字段名|字段类型|字段含义说明|默认值|备注|
|-----|------|-----|---|----|
|username|String|用户名| |不允许为空|
|email|String|邮箱| |不允许为空|
|password|String|密码|不允许为空| |

### 增加LoginDTO数据结构用于接收登录参数
|字段名|字段类型|字段含义说明|默认值|备注|
|-----|------|-----|---|----|
|email|String|邮箱| |不允许为空|
|password|String|密码|不允许为空| |

### 实现思路
1. src/models/task.rs 文件进行修改，增加以下方法
- find_all 数据库中查找所有数据；
- find_unfinished 数据库中查找未完成的数据；
- insert 插入数据进数据库；
- finish_task 修改数据库中数据为已完成；
- mul_insert 向数据库批量插入数据；

2. src/models/user.rs 文件进行修改，增加以下方法
- login 验证用户登录；
- mul_insert 向数据库批量插入数据；

3. src/services/task_service.rs 文件添加如下方法对数据库返回的数据进行处理，向前端返回：
- get_tasks
- get_unfinished_tasks
- add_task
- finish_task
- export_tasks
- import_tasks
- init_tasks

4. src/services/account_service.rs 文件添加如下方法，对数据库返回数据进行处理，向前端返回：
- login
- logout
- import_users

方法实现思路与 phase_3 中相同，只是将操作对象由本地文件更换为数据库，不再提供伪代码

5. 实现细节
- 导入用户与task数据时，需要对本地文件中数据去重；
- 数据库用户表中，密码不可以存明文，借助 bcrypt 对密码进行加密和验证；